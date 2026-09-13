# Implemented local CAD safeguards — September 13, 2026

The audit's build and export safeguards are implemented, and a second mechanical
model now accepts runtime parameters. Rust remains the geometry implementation;
AWS, GitHub Actions and Namespace do not execute these builds.

## Executable verification

Agentic Flowstate commit `0d9c76786120962f75e583f63af53bf72a731491` adds local
build receipts. `execute` rejects a missing receipt, changed executable, changed
compiled inputs, changed repository source/build inputs, lockfiles, configuration
or toolchain identity. Runtime model files remain outside this compilation
identity unless Rust embeds them. Toolchain identity is read through rustup and
file hashes; execute does not invoke Cargo or rustc.

Targeted builds use `--locked` and retain the two-job limit and host compute lock.
Foreground run builds and then executes that exact receipt. Background runs
invalidate earlier receipts; a targeted build after completion registers the
new one. No-op Cargo builds can reuse the existing compiled result. USDZ
conversion is now a separate explicit action, avoiding implicit conversion work.

## Runtime heating platen

`heating_block` and `heating_block_verify` use the same five-field TOML contract
and geometry implementation. The supported parameters are body height, pocket
depth, X/Y pocket clearance and heater bore diameter. Other assembly interfaces
remain fixed. Missing, unknown, non-finite and out-of-range values fail.

Each generation produces STL, a three-view SVG and a manifest with config and
executable hashes, parameter values, mesh hash, bounds and triangle count. The
verifier checks the matching configuration, dimensions, hashes, preview and
byte-identical regenerated geometry. It does not certify thermal behavior,
watertightness or manufacturing suitability.

Validation evidence:
- Nominal STL is byte-identical to the original generator.
- A runtime height change from 16 to 18 mm increased the overall mesh height by
  exactly 2 mm with an unchanged executable hash.
- Execute-only variant generation took 17 ms and verification 13 ms in their
  processes before receipt installation. These exclude MCP startup and receipt
  checking and are not comparisons with compilation time.
- The verifier rejected the variant when supplied the nominal configuration.
- All three SVG views were rendered and visually inspected.

The integrated cartridge was not given arbitrary new runtime controls: inspection
found several existing parameter fields ignored by publication geometry. Its
parameter conversion and the coupon families remain separate geometry work.

## Required STEP exports

All 17 call sites now handle conversion failure explicitly. The converter helper
resolves the tool at runtime, removes stale same-stem STEP output, checks exit
status and STEP structure, and publishes through a temporary file. Missing or
failed conversion cannot report a successful required export.

Tests cover missing, failed and incomplete converters, removal of stale output,
and successful publication. A real integrated-cartridge STL/STEP generation and
its existing deterministic verifier also passed. Mesh-to-STEP output remains
faceted geometry, not an analytic CAD solid.

## Verification and evidence

Targeted source/receipt tests, run-and-feature tests, background lock/recovery
tests, parameter tests and converter tests passed. All modified converter callers
and both heating binaries passed targeted compilation checks. Runtime validation
artifacts and logs are stored at:

`/Users/jarvisgpt/projects/tmp/cad-safeguards/`

The delivery artifact records the final installed MCP check and canonical-source
run. Existing MCP clients load the installed executable/schema on reconnect;
client-owned processes are not killed or restarted.
