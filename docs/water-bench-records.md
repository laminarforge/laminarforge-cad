# Water bench engineering-plan evidence recorder

`tools/water_bench_record.py` uses Python standard-library facilities to store
an engineering plan and its actual design evidence under
`/Volumes/ORICO/MacMiniBuilds/projects-output/`. No internal-disk fallback is
permitted. Plan files contain design evidence. A separate receive-only utility
records raw serial output from a bench balance. Physical readout verification
is pending.

Commands:

```text
python3 tools/water_bench_record.py create-plan --root <absolute-ORICO-subdirectory> --config models/water_bench_hardware.json --source-sha <exact-40-character-SHA> --label <engineering-plan-label>
python3 tools/water_bench_record.py attach --run-dir <returned-directory> --file <actual-evidence-file> --label <artifact-description>
python3 tools/water_bench_record.py note --run-dir <returned-directory> --text <engineering-note>
python3 tools/water_bench_record.py verify --run-dir <returned-directory>
```

## Recover a checklist and its evidence

Run `handoff` on the saved plan directory. It returns the path to a ZIP.
Open `RESUME.txt` inside it for the checklist and next actions from the saved
configuration, including indented substeps where present, followed by dated notes
and the location of each recorded file.
Older plans without a checklist retain their notes and files.

```text
python3 tools/water_bench_record.py handoff --run-dir <plan-directory>
```

The ZIP includes the original plan, configuration, ledger and attachments.
The recorder holds the plan lock and reopens the archive to check those bytes
and the resume sheet before publishing it. Repeating the command for the same
recorded state verifies and reuses that ZIP. Adding a note or attachment gives
the next export its own filename, preserving the earlier one.

To restore, unpack the ZIP into a **new directory** beneath the ORICO output
root. Use the last event hash from the separately saved ticket or verification
receipt to check that the restored plan matches the intended checkpoint.

```text
python3 tools/water_bench_record.py verify --run-dir <restored-directory> --expected-head <last-event-sha256-from-saved-receipt>
```

Creation snapshots the real JSON configuration, source identity and UTC creation
time. It explicitly records `hardware_run_started: false` and
`measurement_samples_recorded: false`. An actual run will need a deliberate
future state/measurement contract; do not reinterpret these plan records as
completed water experiments.

Attachments are copied as content-addressed files, preserving original filename,
label, byte size and SHA-256. A bounded 100 MiB limit prevents accidental bulk
capture; reference large instrument recordings through a reviewed later storage
contract. Changing source size/mtime during capture fails. Copies are checked
again through the ledger verifier.

Writers take an exclusive advisory lock. Each event contains sequence, previous
hash, UTC record time, host monotonic time and invocation clock scope. Files are
flushed/fsynced before success is reported. Verification checks the configuration
snapshot, plan identity, complete newline-terminated event chain and attachment
bytes. Incomplete/corrupt records fail without an automatic rewrite or deletion.
The hashes detect accidental inconsistency; this is not a signed, immutable or
adversary-resistant record. Invocation-scoped monotonic times do not establish
device time, cross-reboot timing or UTC clock calibration.

Record actual device identity, raw frames/units/status, calibration, mass and
physical stop/reconnect observations only after the real hardware interfaces
exist. Do not mark unavailable balance flags as stable or infer a flow trace
from a commanded setting. Images of CAD are design references, not photographs
of installed hardware.

## Bench balance readout

`python3 tools/bench_balance_capture.py ports` lists macOS
`/dev/cu.usbserial*` character devices without opening them. Each path
identifies a possible adapter. Trace it to the balance before selecting it.
The Mac showed zero candidates on November 23.

```text
python3 tools/bench_balance_capture.py capture --root <absolute-ORICO-output-subdirectory> --port <listed-balance-port> --instrument "<model and serial or asset ID>" --seconds 10
```

The command opens the selected port read-only, configures the documented 9600
baud, 8 data bits, no parity and one stop bit, and listens for up to the chosen
duration (1–120 seconds). It sends no instrument commands. Each read is saved
as hex bytes with a byte offset and two host timestamps, UTC and monotonic.
A single read can contain part of a frame or several frames. The timestamps
describe when the host read the bytes. For physical verification, retain the
displayed value and unit alongside the matching output and instrument identity.

Each invocation that selects an identified port creates a new JSONL recording.
The final event records the outcome, the byte count and a SHA-256 covering the
concatenated raw bytes. An interruption or disconnect can leave a partial
record. A missing final event means the file is unfinished. An empty port
inventory exits with `no_usb_serial_candidate` before creating a recording.
