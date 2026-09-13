# Local CAD iteration

AWS, GitHub Actions and Namespace processing are retired. GitHub is source
control only. Use the local LaminarForge MCP tool with an explicit binary.

Development uses incremental compilation, opt-level 1 for model code and
opt-level 2 for dependencies. Release remains explicit final optimization.
The tracked Cargo.lock pins dependencies.

The priming fixture reads models/chip_priming_tubing_fixture.toml at runtime.
All values are required millimeters; missing, unknown and invalid values fail.
Its CLI accepts --config PATH and --output-dir PATH. After building it once,
editing the TOML and running the executable requires no Rust compilation.
This first conversion covers the priming fixture, not all CAD models.

Build and run the fixture once using this `laminarforge_build` input:

```json
{"repo":"laminarforge-cad","action":"run","bin":"chip_priming_tubing_fixture","profile":"dev","args":["--config","models/chip_priming_tubing_fixture.toml","--output-dir","output/priming-fixture"]}
```

After editing only its TOML, reuse the executable:

```json
{"repo":"laminarforge-cad","action":"execute","bin":"chip_priming_tubing_fixture","profile":"dev","args":["--config","models/chip_priming_tubing_fixture.toml","--output-dir","output/priming-fixture"]}
```

Use a separate output directory for each candidate. Check expected dimensions
and component outputs before promoting a candidate. The tool's `execute` action
requires a matching executable receipt. Use targeted `run` after changing
source, dependencies, features or the toolchain. After a background run, use
one targeted `build` to register its receipt before execute-only iteration. `check`
does not refresh the executable. Do not reuse preview evidence as final release
evidence after changing the configuration.

Already compiled PCB tools that read TOML can also use `execute`, but all routing
and release gates still apply. Do not pass the fixture's CLI flags to binaries
that do not implement them.

The installed MCP exposes `check`, `execute`, `profile` and `args`. If a running
client still shows the old schema, reconnect that client to load the installed
tool. Do not change the request to a full release build to bypass a stale schema.

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
Build each target once, then execute the generator and verifier with the same
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
