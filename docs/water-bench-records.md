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
