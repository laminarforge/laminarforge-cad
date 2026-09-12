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

Exa research verified against primary sources:
- https://doc.rust-lang.org/cargo/reference/profiles.html
- https://doc.rust-lang.org/cargo/reference/timings.html
- https://doc.rust-lang.org/cargo/commands/cargo-check.html
- https://github.com/mozilla/sccache/blob/main/docs/Rust.md

sccache does not cache final-linked Rust binaries and requires incremental
compilation disabled. Do not disable incremental development merely to enable
a global wrapper. Preserve repository-lock safeguards.
