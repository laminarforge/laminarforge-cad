#!/usr/bin/env python3
"""List macOS USB-serial ports or record raw bytes from an identified bench balance."""
import argparse
import contextlib
import fcntl
import hashlib
import json
import os
from pathlib import Path
import select
import stat
import sys
import termios
import time
import uuid

from water_bench_record import canonical, location, timestamp


def inventory():
    if sys.platform != 'darwin':
        raise ValueError('This port inventory supports macOS.')
    ports = []
    for path in sorted(Path('/dev').glob('cu.*')):
        if path.name.startswith('cu.usbserial') and stat.S_ISCHR(path.lstat().st_mode):
            ports.append(str(path))
    return {'observed_at_utc': timestamp(), 'usb_serial_candidates': ports}


@contextlib.contextmanager
def receiver(port):
    fd = os.open(port, os.O_RDONLY | os.O_NOCTTY | os.O_NONBLOCK | os.O_NOFOLLOW)
    try:
        if not stat.S_ISCHR(os.fstat(fd).st_mode):
            raise ValueError('Selected port is no longer a character device.')
        fcntl.ioctl(fd, termios.TIOCEXCL)
        original = termios.tcgetattr(fd)
        controls = original[6].copy()
        controls[termios.VMIN] = 1
        controls[termios.VTIME] = 0
        flags = termios.CS8 | termios.CLOCAL | termios.CREAD
        settings = [0, 0, flags, 0, termios.B9600, termios.B9600, controls]
        try:
            termios.tcsetattr(fd, termios.TCSANOW, settings)
            actual = termios.tcgetattr(fd)
            mask = termios.CSIZE | termios.PARENB | termios.CSTOPB | termios.CRTSCTS | flags
            if (actual[:2] != settings[:2] or actual[2] & mask != flags
                    or actual[3:6] != settings[3:6]):
                raise ValueError('Serial driver did not accept the requested raw 9600 8N1 settings.')
            yield fd
        finally:
            termios.tcsetattr(fd, termios.TCSANOW, original)
    finally:
        os.close(fd)


def append_event(stream, kind, **fields):
    stream.write(canonical({'event': kind, **fields}) + b'\n')
    stream.flush()
    os.fsync(stream.fileno())


def capture(args):
    snapshot = inventory()
    ports = snapshot['usb_serial_candidates']
    if not ports:
        return {'success': False, 'status': 'no_usb_serial_candidate', **snapshot}
    if args.port not in ports or not args.instrument or not args.instrument.strip():
        return {'success': False, 'status': 'select_port_and_identify_balance', **snapshot}
    if not 1 <= args.seconds <= 120:
        raise ValueError('Duration must be between 1 and 120 seconds.')
    root = location(args.root)
    root.mkdir(parents=True, exist_ok=True)
    path = root / ('balance-' + str(uuid.uuid4()) + '.jsonl')
    byte_count = 0
    raw_hash = hashlib.sha256()
    with path.open('xb') as stream:
        append_event(stream, 'capture_requested', schema_version=1,
                     recorded_at_utc=timestamp(), port=args.port, instrument_label=args.instrument.strip(),
                     baud=9600, data_bits=8, parity='none', stop_bits=1, duration_seconds=args.seconds)
        print('Recording to ' + str(path), file=sys.stderr, flush=True)
        error = None
        try:
            with receiver(args.port) as fd:
                started = time.monotonic_ns()
                append_event(stream, 'port_configured', recorded_at_utc=timestamp(), monotonic_ns=started)
                deadline = started + args.seconds * 1_000_000_000
                while True:
                    remaining = deadline - time.monotonic_ns()
                    if remaining <= 0 or not select.select([fd], [], [], remaining / 1e9)[0]:
                        status = 'raw_bytes_received' if byte_count else 'no_bytes_received'
                        break
                    try:
                        data = os.read(fd, 4096)
                    except BlockingIOError:
                        continue
                    if not data:
                        status = 'port_closed'
                        break
                    append_event(stream, 'received', observed_at_utc=timestamp(),
                                 monotonic_ns=time.monotonic_ns(), offset=byte_count, raw_hex=data.hex())
                    raw_hash.update(data)
                    byte_count += len(data)
        except KeyboardInterrupt:
            status = 'interrupted'
        except (OSError, ValueError, termios.error) as exc:
            status, error = 'capture_error', str(exc)
        receipt = {'success': status == 'raw_bytes_received', 'status': status,
                   'byte_count': byte_count, 'raw_sha256': raw_hash.hexdigest()}
        if error:
            receipt['error'] = error
        append_event(stream, 'capture_ended', recorded_at_utc=timestamp(), **receipt)
    return {**receipt, 'record_path': str(path)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command', required=True)
    commands.add_parser('ports', help='List USB-serial candidates without opening them')
    command = commands.add_parser('capture', help='Listen to a physically identified balance')
    command.add_argument('--root', required=True, help='Directory under the ORICO project output root')
    command.add_argument('--port', help='Exact listed device traced to the balance')
    command.add_argument('--instrument', help='Model and physical asset or serial identifier')
    command.add_argument('--seconds', type=int, default=10)
    args = parser.parse_args()
    try:
        result = capture(args) if args.command == 'capture' else {'success': True, **inventory()}
    except (OSError, ValueError) as exc:
        result = {'success': False, 'error': str(exc)}
    print(json.dumps(result, sort_keys=True))
    return 0 if result['success'] else 1


if __name__ == '__main__':
    sys.exit(main())
