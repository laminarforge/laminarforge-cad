# CAD iteration

Use `laminarforge_build` with executor `github`, an explicit binary, and action
`check`, `build`, `run`, or `benchmark`. The default action is targeted `check`.
The executor records an exact main SHA and publishes timing/RSS evidence as a
GitHub Actions artifact. Check avoids code generation, but does not verify geometry.
Use `run` for geometry and `release` explicitly for final optimized output.

Development builds use incremental compilation with opt-level 1 for model code
and opt-level 2 for dependencies. Each isolated GitHub worker uses two Cargo jobs;
it does not compete with agents for the Mac mini's 16 GB. GitHub runner capacity
and billing still limit total concurrency. No claim of unlimited parallelism.

The priming fixture reads `models/chip_priming_tubing_fixture.toml` at runtime.
All values are required, in millimeters; invalid values and unknown keys fail.
The compiled CLI also accepts `--config PATH --output-dir PATH`. Reusing the
executable after editing TOML does not invoke Cargo. `model_config` on the MCP
run request supplies a job-specific TOML definition without a source commit.
It currently supports this fixture only; the rest of CAD is not yet data-driven.

`benchmark` measures targeted check, build, warm build, generation, and a second
generation after a 1 mm X-margin change. The second generation calls the same
executable directly, verifies its bytes did not change, validates all four binary
STLs, and verifies the base widened 2 mm. Peak RSS is recorded by GNU time for each
phase, alongside elapsed time, compiler identity, source SHA and features.
An initial build with a restored cache is not labeled a cold-build measurement.
The generated fixture is an engineering model, not manufacturing acceptance.

GitHub jobs currently compile/check the selected target on each invocation;
Cargo reuses compatible cached work. This removes package-wide builds and the
need for a compiler inside the API container. It is not a resident CAD service.
The runtime-data design can later be placed on a warm BOSGAME worker without
changing the model contract.

Exa research, September 12, 2026:
- https://doc.rust-lang.org/cargo/reference/profiles.html — profiles, incremental
  compilation, dependency overrides, and generics caveats.
- https://doc.rust-lang.org/cargo/reference/timings.html — compiler scheduling
  evidence; pair with OS memory measurements.
- https://doc.rust-lang.org/cargo/commands/cargo-check.html — check skips final
  code generation and cannot substitute for geometry verification.
- https://github.com/mozilla/sccache/blob/main/docs/Rust.md — Rust incremental
  compilation must be disabled for sccache, and final-linked binaries are not
  cacheable. Keep incremental development rather than enabling an ineffective
  global wrapper. Host lock-inheritance safeguards remain intact.

GEV, PCB routing and interactive FreeCAD remain explicit configured-host
operations. There is no automatic fallback to a Mac or AWS compiler.
