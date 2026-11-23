#!/usr/bin/env python3
"""ORICO-only engineering-plan evidence ledger. No device or motion commands."""
import argparse
import contextlib
import datetime
import fcntl
import hashlib
import json
import os
from pathlib import Path
import re
import sys
import tempfile
import time
import uuid
import zipfile

ROOT = Path('/Volumes/ORICO/MacMiniBuilds/projects-output')
INVOCATION = str(uuid.uuid4())
MAX_ATTACHMENT = 100 * 1024 * 1024


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'), ensure_ascii=False).encode('utf-8')


def digest(data):
    return hashlib.sha256(data).hexdigest()


def timestamp():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def location(value, existing=False):
    if not ROOT.is_dir():
        raise ValueError('ORICO output root is unavailable')
    p = Path(value)
    if not p.is_absolute():
        raise ValueError('Use an absolute ORICO path')
    p = p.resolve(strict=existing)
    if ROOT.resolve() not in p.parents:
        raise ValueError('Record storage must be beneath the ORICO output root')
    return p


def durable_write(path, data, exclusive=True):
    with path.open('xb' if exclusive else 'wb') as stream:
        stream.write(data)
        stream.flush()
        os.fsync(stream.fileno())


@contextlib.contextmanager
def locked(run):
    if not run.is_dir() or not (run / 'plan.json').is_file():
        raise ValueError('Not an initialized engineering plan')
    with (run / '.lock').open('a+b') as stream:
        fcntl.flock(stream, fcntl.LOCK_EX)
        yield


def inspect(run):
    metadata_bytes = (run / 'plan.json').read_bytes()
    metadata = json.loads(metadata_bytes)
    config_bytes = (run / 'configuration.json').read_bytes()
    if digest(config_bytes) != metadata['configuration_sha256']:
        raise ValueError('Configuration snapshot hash mismatch')
    raw = (run / 'events.jsonl').read_bytes()
    if not raw or not raw.endswith(b'\n'):
        raise ValueError('Missing or incomplete event ledger; no automatic repair performed')
    previous = None
    events = []
    for sequence, line in enumerate(raw.splitlines(), 1):
        event = json.loads(line)
        event_hash = event.pop('sha256')
        if event['sequence'] != sequence or event['previous_sha256'] != previous or digest(canonical(event)) != event_hash:
            raise ValueError('Event chain mismatch at sequence {}'.format(sequence))
        event['sha256'] = event_hash
        previous = event_hash
        events.append(event)
        if event['kind'] == 'artifact_attached':
            payload = event['payload']
            name = payload['stored_file']
            if Path(name).name != name:
                raise ValueError('Invalid attachment reference')
            path = run / 'artifacts' / name
            data = path.read_bytes()
            if len(data) != payload['bytes'] or digest(data) != payload['sha256']:
                raise ValueError('Attachment hash mismatch: ' + name)
    if events[0]['kind'] != 'plan_created' or events[0]['payload']['plan_sha256'] != digest(metadata_bytes):
        raise ValueError('Plan identity mismatch')
    if (metadata['record_kind'] != 'engineering_plan' or metadata['hardware_run_started'] is not False
            or metadata['measurement_samples_recorded'] is not False):
        raise ValueError('Unsupported record kind or hardware state')
    return metadata, events


def append(run, events, kind, payload):
    event = {'sequence': len(events) + 1, 'previous_sha256': events[-1]['sha256'] if events else None,
             'recorded_at_utc': timestamp(), 'host_monotonic_ns': time.monotonic_ns(),
             'clock_scope': INVOCATION, 'kind': kind, 'payload': payload}
    event['sha256'] = digest(canonical(event))
    with (run / 'events.jsonl').open('ab') as stream:
        stream.write(canonical(event) + b'\n')
        stream.flush()
        os.fsync(stream.fileno())
    return event


def initialize(args):
    root = location(args.root)
    config = Path(args.config).read_bytes()
    json.loads(config)
    if not re.fullmatch(r'[0-9a-f]{40}', args.source_sha):
        raise ValueError('Source SHA must be the exact 40-character lowercase commit SHA')
    if not args.label.strip() or len(args.label) > 200:
        raise ValueError('Use a nonempty label up to 200 characters')
    identifier = str(uuid.uuid4())
    run = root / identifier
    metadata = {'schema_version': 1, 'record_kind': 'engineering_plan', 'plan_id': identifier,
                'label': args.label, 'created_at_utc': timestamp(), 'source_sha': args.source_sha,
                'configuration_sha256': digest(config), 'hardware_run_started': False,
                'measurement_samples_recorded': False,
                'claim': 'Design/evidence organization only. No physical run or device observation is implied.'}
    root.mkdir(parents=True, exist_ok=True)
    run.mkdir()
    (run / 'artifacts').mkdir()
    durable_write(run / 'configuration.json', config)
    encoded = json.dumps(metadata, indent=2, ensure_ascii=False).encode('utf-8') + b'\n'
    durable_write(run / 'plan.json', encoded)
    with locked(run):
        append(run, [], 'plan_created', {'plan_sha256': digest(encoded)})
        inspect(run)
    return {'success': True, 'plan_directory': str(run), 'plan_id': identifier,
            'hardware_run_started': False}


def attach(args):
    run = location(args.run_dir, existing=True)
    source = Path(args.file)
    if not args.label.strip() or len(args.label) > 200:
        raise ValueError('Use a nonempty artifact label up to 200 characters')
    with source.open('rb') as stream:
        before = os.fstat(stream.fileno())
        if before.st_size > MAX_ATTACHMENT:
            raise ValueError('Attachment exceeds the 100 MiB evidence limit')
        data = stream.read(MAX_ATTACHMENT + 1)
        after = os.fstat(stream.fileno())
    if len(data) != before.st_size or (before.st_size, before.st_mtime_ns) != (after.st_size, after.st_mtime_ns):
        raise ValueError('Source changed during capture')
    file_hash = digest(data)
    suffix = source.suffix if re.fullmatch(r'\.[A-Za-z0-9]{1,10}', source.suffix) else '.bin'
    name = file_hash + suffix
    with locked(run):
        _, events = inspect(run)
        target = run / 'artifacts' / name
        if target.exists():
            if digest(target.read_bytes()) != file_hash:
                raise ValueError('Existing content-addressed attachment is corrupt')
        else:
            temp = run / 'artifacts' / ('.capture-' + str(uuid.uuid4()))
            durable_write(temp, data)
            os.replace(temp, target)
        event = append(run, events, 'artifact_attached', {'label': args.label,
                       'original_filename': source.name, 'stored_file': name,
                       'sha256': file_hash, 'bytes': len(data)})
        inspect(run)
    return {'success': True, 'sequence': event['sequence'], 'artifact_sha256': file_hash}


def note(args):
    run = location(args.run_dir, existing=True)
    if not args.text.strip() or len(args.text) > 4000:
        raise ValueError('Use a nonempty note up to 4000 characters')
    with locked(run):
        _, events = inspect(run)
        event = append(run, events, 'engineering_note', {'text': args.text})
        inspect(run)
    return {'success': True, 'sequence': event['sequence']}


def verification(metadata, events):
    return {'success': True, 'plan_id': metadata['plan_id'], 'source_sha': metadata['source_sha'],
            'event_count': len(events), 'artifact_count': sum(e['kind'] == 'artifact_attached' for e in events),
            'last_event_sha256': events[-1]['sha256'], 'hardware_run_started': False,
            'measurement_samples_recorded': False,
            'claim': 'Local hash consistency, not a signature or proof of physical performance'}


def verify(args):
    run = location(args.run_dir, existing=True)
    with locked(run):
        receipt = verification(*inspect(run))
    if args.expected_head is not None and args.expected_head != receipt['last_event_sha256']:
        raise ValueError('The verified ledger ends at a different event than the supplied receipt')
    return receipt


def resume_text(metadata, config, events):
    lines = [metadata['label'], '', 'Engineering plan ' + metadata['plan_id'],
             'Recorded through ' + events[-1]['recorded_at_utc'],
             'Plan source ' + metadata['source_sha'], 'Last event ' + events[-1]['sha256'],
             'Hardware runs and measurement samples recorded by this plan: none.']
    if isinstance(config, dict):
        checklist = config.get('checklist')
        if isinstance(checklist, list) and checklist:
            lines.extend(['', 'Checklist in the saved configuration'])
            for step in checklist:
                if isinstance(step, dict) and isinstance(step.get('complete'), bool) and 'title' in step:
                    mark = 'x' if step['complete'] else ' '
                    lines.append('[{}] {}. {}'.format(mark, step.get('step', ''), step['title']))
                else:
                    lines.append(json.dumps(step, ensure_ascii=False))
        actions = config.get('next_actions')
        if isinstance(actions, list) and actions:
            lines.extend(['', 'Next actions in the saved configuration'])
            lines.extend('- ' + str(action) for action in actions)
    lines.extend(['', 'Recorded notes and files'])
    for event in events[1:]:
        detail = event['payload']
        lines.append('{}  {}'.format(event['sequence'], event['recorded_at_utc']))
        if event['kind'] == 'engineering_note':
            lines.append(detail['text'])
        elif event['kind'] == 'artifact_attached':
            lines.extend([detail['label'], '{}  ({} bytes)'.format(detail['original_filename'], detail['bytes']),
                          'artifacts/' + detail['stored_file']])
        else:
            lines.append(event['kind'])
    return ('\n'.join(lines) + '\n').encode('utf-8')


def handoff(args):
    run = location(args.run_dir, existing=True)
    with locked(run):
        metadata, events = inspect(run)
        receipt = verification(metadata, events)
        head = receipt['last_event_sha256']
        config = json.loads((run / 'configuration.json').read_bytes())
        extra = {'RESUME.txt': resume_text(metadata, config, events),
                 'verification.json': (json.dumps(receipt, indent=2) + '\n').encode('utf-8')}
        files = {'plan.json', 'configuration.json', 'events.jsonl', 'artifacts/'}
        files.update('artifacts/' + event['payload']['stored_file']
                     for event in events if event['kind'] == 'artifact_attached')
        destination = run / ('handoff-' + head + '.zip')
        if destination.exists():
            verify_handoff(destination, receipt, files, extra)
        else:
            with tempfile.TemporaryDirectory(dir=run, prefix='.handoff-') as staging:
                temporary = Path(staging) / 'handoff.zip'
                with zipfile.ZipFile(temporary, 'x', zipfile.ZIP_DEFLATED) as archive:
                    for name in sorted(files):
                        archive.write(run / name, arcname=name)
                    for name, data in extra.items():
                        archive.writestr(name, data)
                verify_handoff(temporary, receipt, files, extra)
                with temporary.open('rb') as stream:
                    os.fsync(stream.fileno())
                os.replace(temporary, destination)
                directory_fd = os.open(run, os.O_RDONLY)
                try:
                    os.fsync(directory_fd)
                finally:
                    os.close(directory_fd)
    return {**receipt, 'archive_path': str(destination)}


def verify_handoff(path, receipt, files, extra):
    with zipfile.ZipFile(path) as archive:
        names = archive.namelist()
        expected = files.union(extra)
        if len(names) != len(expected) or set(names) != expected:
            raise ValueError('Handoff member list does not match the ledger')
        root = zipfile.Path(archive)
        if verification(*inspect(root)) != receipt:
            raise ValueError('Handoff does not match the verified source ledger')
        for name, data in extra.items():
            if (root / name).read_bytes() != data:
                raise ValueError('Handoff contents differ for ' + name)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command', required=True)
    p = commands.add_parser('create-plan')
    for option in ['root', 'config', 'source-sha', 'label']:
        p.add_argument('--' + option, required=True)
    p.set_defaults(action=initialize)
    p = commands.add_parser('attach')
    for option in ['run-dir', 'file', 'label']:
        p.add_argument('--' + option, required=True)
    p.set_defaults(action=attach)
    p = commands.add_parser('note')
    p.add_argument('--run-dir', required=True)
    p.add_argument('--text', required=True)
    p.set_defaults(action=note)
    p = commands.add_parser('verify')
    p.add_argument('--run-dir', required=True)
    p.add_argument('--expected-head', help='Require the exact last_event_sha256 from a retained receipt')
    p.set_defaults(action=verify)
    p = commands.add_parser('handoff', help='Export a verified ZIP with a readable resume sheet')
    p.add_argument('--run-dir', required=True)
    p.set_defaults(action=handoff)
    args = parser.parse_args()
    try:
        print(json.dumps(args.action(args), sort_keys=True))
    except (ValueError, OSError, KeyError, TypeError, zipfile.BadZipFile) as error:
        print(json.dumps({'success': False, 'error': str(error)}), file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())
