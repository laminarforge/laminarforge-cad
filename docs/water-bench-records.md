# Water bench engineering-plan evidence recorder

`tools/water_bench_record.py` uses Python standard-library facilities to store
an engineering plan and its actual design evidence under
`/Volumes/ORICO/MacMiniBuilds/projects-output/`. No internal-disk fallback is
permitted. This tool does not connect to devices, command a pump, start an
experiment, or record fabricated mass samples. M08 hardware control and timed
instrument capture remain unimplemented.

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
