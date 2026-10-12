# Local CAD iteration

AWS, GitHub Actions and Namespace processing are retired. GitHub is source
control only. Compilation follows canonical `main` pushes and the registered
recipe; see [the queue contract](local-build-pipeline.md). Normal agents do not
invoke compiler/Make commands or MCP check/build/run submission actions.

Development uses incremental compilation, opt-level 1 for model code and
opt-level 2 for dependencies. Release remains explicit final optimization.
The tracked Cargo.lock pins dependencies.

The priming fixture reads models/chip_priming_tubing_fixture.toml at runtime.
All values are required millimeters; missing, unknown and invalid values fail.
Its CLI accepts --config PATH and --output-dir PATH. After building it once,
editing the TOML and running the executable requires no Rust compilation.
See [the runtime contract](cad-runtime-contract.md) for all converted families and their verifiers.

After pipeline compilation and supported executable-receipt registration, use
execute-only geometry processing. If that adapter or receipt is missing, report
execution pending instead of invoking a legacy compiler action. After editing
only its TOML, reuse the receipt-authorized executable:

```json
{"repo":"laminarforge-cad","action":"execute","bin":"chip_priming_tubing_fixture","profile":"dev","args":["--config","models/chip_priming_tubing_fixture.toml","--output-dir","output/priming-fixture"]}
```

Use a separate output directory for each candidate. Check expected dimensions
and component outputs before promoting a candidate. The tool's `execute` action
requires a matching executable receipt. After changing source, dependencies,
features or the toolchain, publish `main` and wait for the exact new artifact and
supported execution receipt. A pipeline compile-only receipt does not by itself
authorize geometry execution. Do not reuse preview evidence as final release
evidence after changing the configuration.

Already compiled PCB tools that read TOML can also use `execute`, but all routing
and release gates still apply. Do not pass the fixture's CLI flags to binaries
that do not implement them.

If a running client has a stale schema, report reconnect pending rather than
bypassing pipeline policy with a different build action. Verify actual execution
and its observable output; do not create synthetic or mocked test scaffolding.

Exa research verified against primary sources:
- https://doc.rust-lang.org/cargo/reference/profiles.html
- https://doc.rust-lang.org/cargo/reference/timings.html
- https://doc.rust-lang.org/cargo/commands/cargo-check.html
- https://github.com/mozilla/sccache/blob/main/docs/Rust.md

sccache does not cache final-linked Rust binaries and requires incremental
compilation disabled. Do not disable incremental development merely to enable
a global wrapper. Preserve repository-lock safeguards.

## Heating platen and export checks

`heating_block` and `heating_block_verify` both accept `--config` and
`--output-dir`. Their default configuration is `models/heating_block.toml`.
Enroll and compile each required target through its canonical push recipe, then
use supported receipt-authorized execution for the generator and verifier with the same
configuration and output directory. The five exposed parameters control body
height, registration pocket depth, X/Y clearance and heater bore diameter.
Other cartridge/assembly dimensions remain fixed; these files are mechanical
prototypes and do not establish thermal or manufacturing qualification.

The generator writes `heating_block.stl`, an orthographic `heating_block.preview.svg`, and a manifest with configuration and
executable hashes, parameter values, mesh hash, triangle count and bounds.
The verifier checks the input identity, mesh dimensions, file hash, preview and
byte-identical regenerated geometry. A manifest from another configuration is
rejected. These checks do not establish watertightness or manufacturing fitness.

Required STEP conversion now fails on a missing converter, nonzero exit or
incomplete STEP output. `STLTOSTEP_BIN` can specify the installed converter;
otherwise its required location is the current user's `~/.local/bin/stltostp`.
Failed conversion removes any old same-stem STEP publication. Mesh-to-STEP
conversion produces faceted geometry, not analytic solid modeling.
