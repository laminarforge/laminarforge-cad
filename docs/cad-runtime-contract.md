# Runtime CAD contract

Local Rust executables construct geometry. Agents edit the model's runtime TOML
for supported dimensional changes and call receipt-authorized MCP `execute`.
Rust/dependency/toolchain changes require canonical publication and a matching
pipeline artifact before subsequent execution. GitHub stores source only. AWS,
GitHub Actions and Namespace do not build or process these models.

## Published model families

Every entry below accepts explicit `--config PATH --output-dir PATH`. Generators
and verifiers read the same required TOML fields, reject unknown fields and invalid
values, and produce/check mesh hashes, bounds, orthographic SVG previews and exact
regenerated STL bytes. These are engineering checks, not manufacturing approval.

| Generator | Config under models/ | Verification |
| --- | --- | --- |
| chip_priming_tubing_fixture | chip_priming_tubing_fixture.toml | same target with --verify |
| water_transfer_dock (default module) | water_transfer_dock.toml | same target with --verify |
| water_transfer_dock --module receiver | water_bench_receiver.toml | same selector/config with --verify |
| water_transfer_dock --module controller | water_bench_controller.toml | same selector/config with --verify |
| water_transfer_dock --module frame | water_bench_frame.toml | same selector/config with --verify; unpowered guide-frame review |
| water_transfer_dock --module carriage | water_bench_carriage.toml | same selector/config with --verify; dry mechanical review only |
| water_transfer_dock --module bench | water_bench_layout.toml | same selector/config with --verify; posed review assemblies only |
| water_transfer_dock --module source | water_bench_source_saddle.toml | same selector/config with --verify |
| water_transfer_dock --module outlet | water_bench_outlet_support.toml | same selector/config with --verify |
| water_transfer_dock --module coupon | water_bench_coupon.toml | same selector/config with --verify; review meshes only |
| water_transfer_dock | water_transfer_dock.toml | same target with --verify; see water-transfer-dock.md |
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
review the change and actual tool/service evidence before recording a new fixed source identity.

The registry is a workflow gate, not a semantic proof of arbitrary Rust. Library
geometry changes still require review, actual parameter/geometry verification and mating-interface
checks. Executable receipts separately cover source, dependencies and toolchain;
changing shared Rust invalidates prior executables. Existing untouched historical
models are not described as converted.

Package-wide `run_all` is rejected at MCP. Compilation is requested only by
canonical `main` pushes; normal agents do not invoke check/build/run compiler
actions. The registered `cad-priming-dev` group covers the priming fixture and
water-transfer dock.
Other targets need explicit recipe enrollment. The local pipeline serializes
compilation jobs while Cargo uses its normal parallelism without a custom jobs
cap. Native builds, including optional STEP kernels, stay local.

Each candidate gets its own output directory. A generator invalidates its old
manifest before writing a new publication. Verification must pass before promotion.
Runtime configuration edits do not invalidate the executable receipt. Outputs record
configuration and generator identity; the MCP receipt supplies source/toolchain
identity. Keep the tool result with the output evidence. Changing either inputs or
binary invalidates the associated verification.

## Cassette fabrication tools

`heated_microplate_cassette_v0` is a runtime target using
`models/heated_microplate_cassette_v0.toml`. Its verification is integrated into
its generation pass: it checks mesh closure, interference/travel, and (with the
`step` feature) analytic STEP round-trip volume and bounds, then writes
`verification.json`. Its self-referential registry verifier means rerunning that
checked generation into a new output directory, not passing `--verify`. The
`--validate-only` option checks dimensions without generating geometry.

`cassette_release` is a fixed-source packaging utility, not adjustable geometry.
It takes `--input-dir`, `--output-dir`, and `--source-dir`, validates the input
report and source hashes, and packages the existing artifacts. Its reviewed source
hash is registered separately. Keep both entries updated when changing these tools;
new model implementation commits must include their registry contract.

### Rev D plate frame contract

The heated cassette runtime configuration now requires `[joints]` coordinates.
The generator and validator share these values. Validation rejects the former
narrow housing, inadequate gasket/edge lands, colliding joint-hole patterns,
rear-hole intrusion and unreviewed roof/bezel thickness changes that would alter
commercial screw engagement. The geometry pass checks the four separate frame
parts, sampled drawer travel and the new joint hardware against non-parent solids.
Threaded parent engagement is intentionally excluded from the hardware check.
Tests exercise rejected dimensions and missing/unknown fields. This does not
validate coating-process capability, fastener preload or fluid temperature.

The fixed packaging utility's Rev D review changes only its release label and
adds the new joint module to the source snapshot. Its registry source identity
tracks those reviewed packaging changes; adjustable geometry remains runtime.
