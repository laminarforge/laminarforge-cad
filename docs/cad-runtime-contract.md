# Runtime CAD contract

Local Rust executables construct geometry. Agents edit the model's runtime TOML
for supported dimensional changes and call MCP `execute`; they rebuild one target
only after Rust/dependency/toolchain changes. GitHub stores source only. AWS,
GitHub Actions and Namespace do not build or process these models.

## Published model families

Every entry below accepts explicit `--config PATH --output-dir PATH`. Generators
and verifiers read the same required TOML fields, reject unknown fields and invalid
values, and produce/check mesh hashes, bounds, orthographic SVG previews and exact
regenerated STL bytes. These are engineering checks, not manufacturing approval.

| Generator | Config under models/ | Verification |
| --- | --- | --- |
| chip_priming_tubing_fixture | chip_priming_tubing_fixture.toml | same target with --verify |
| heating_block | heating_block.toml | heating_block_verify |
| p0_cartridge_coupon_suite | p0_cartridge_coupons.toml | p0_cartridge_coupon_verify, same --stack selection |
| sixteen_slot_cassette_print_coupons | sixteen_slot_cassette_print_coupons.toml | same target with --verify |
| diagnostic_cartridge | diagnostic_cartridge.toml | same target with --verify |
| swab_integrated_sealed_diagnostic_cartridge | swab_integrated_sealed_diagnostic_cartridge.toml | swab_integrated_sealed_diagnostic_cartridge_verify |

The P0 family covers 11 coupon geometries across three material stacks plus the
shared alignment nest. Cassette coupons expose chip clearance, gasket groove
width, attachment overlap and stop clearance. Their fixed A0 mating dimensions
remain shared with the assembly; fit experiments do not silently revise it.
Diagnostic cartridge parameters vary height, bore, channels and alignment hole
fit within the fixed reader footprint. The integrated publication supports body
length/width only. Its internal topology and dose-envelope fields are required
fixed design assertions: unsupported changes fail instead of being ignored by
geometry. Model code remains necessary when changing topology or those fixed
interfaces.

## Enforced entry-point policy

`cad-workflow.json` enrolls the repository's existing binary entry points. The local
MCP checks it before check/build/run/execute. Runtime targets require a configuration
file and registered verifier; run/execute require explicit config and output flags.
Unconverted historical entry points are frozen to reviewed source hashes. Changing
one or adding a new target requires a deliberate contract update before compilation.
Do not refresh a frozen model hash to bypass migration. Convert adjustable geometry
and its verification together. For non-model reports or fixed validation utilities,
review the change and its tests before recording a new fixed source identity.

The registry is a workflow gate, not a semantic proof of arbitrary Rust. Library
geometry changes still require review, parameter/geometry tests and mating-interface
checks. Executable receipts separately cover source, dependencies and toolchain;
changing shared Rust invalidates prior executables. Existing untouched historical
models are not described as converted.

Package-wide `run_all` is rejected at MCP. Select one target; use `check` for type
validation and `run` only when execution is needed. Final release optimization is
explicit. All heavy CAD work shares the Mac host lock and compiler jobs are capped
at two. Native builds, including optional STEP kernels, stay local.

Each candidate gets its own output directory. A generator invalidates its old
manifest before writing a new publication. Verification must pass before promotion.
Runtime configuration edits do not invalidate the executable receipt. Outputs record
configuration and generator identity; the MCP receipt supplies source/toolchain
identity. Keep the tool result with the output evidence. Changing either inputs or
binary invalidates the associated verification.
