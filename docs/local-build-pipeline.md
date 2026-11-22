# Mac-local compile-only queue

Canonical `main` pushes are the normal request for the registered local pipeline.
The worker invokes the root `Makefile` at the queued SHA, using the recipe from
that same commit. Agents publish through the workspace lifecycle and inspect
status/logs; they do not invoke the recipe or manual compiler commands.

The explicit group `cad-priming-dev` compiles the reviewed runtime binaries
`chip_priming_tubing_fixture` and `water_transfer_dock`. It preserves the repository's `dev` profile
(incremental model compilation, optimized dependencies) and normal Cargo
parallelism. The recipe verifies its existing `cad-workflow.json` enrollment,
runtime configurations and self-verifiers before compiling. It neither changes model
contracts nor executes generators, verifiers, installers or network setup.

The worker supplies `BUILD_JOB_ID`, `BUILD_COMMIT_SHA`, `BUILD_REPOSITORY`,
`BUILD_TARGET`, `BUILD_CACHE_DIR`, `BUILD_OUTPUT_DIR`, `BUILD_BASE_SHA` (empty for
the first compatible build), and `BUILD_TOOLCHAIN`. Cache and output directories
are absolute and distinct. Cargo keeps its stable target directory under the
registered cache. Each job receives both copied, read-only binaries and a version-1
`build-receipt.json` with the exact job, repository, SHA, `dev` profile, toolchain
identity, artifact path and SHA-256. Artifact kind is `compiled-binary`.
`phases.ndjson`, `changed-paths.txt`, the worker's build log and Cargo timings
provide observable checkpoints.
Recipe phases include job ID, exact SHA, cache directory and elapsed seconds in
both stdout and `phases.ndjson`: started, compiling, compiled, artifact_copied,
and completed after the receipt is committed. Nonzero exits emit failed.

This is compilation only. A compiler artifact is not a geometry result,
manufacturing validation, installation or delivery. Runtime execution still
requires the existing CAD configuration/verifier contract and appropriate MCP
execution receipt; the pipeline receipt is not a substitute for that authority.

Intake uses outbound branch polling, with a normal 60-second interval. No public
webhook listener or router changes are needed. Build recipes and product setup
must not touch WireGuard, routing/forwarding, PF, DuckDNS or their services; that
separate maintenance scope requires Alex's exact instruction.

The pipeline must explicitly
allow this repository, group, `dev` profile and both compiled-binary targets before
claiming success. Record actual job IDs, exact SHA, receipt and observed result;
report any activation, compilation or execution checkpoint that is still pending.
Verification requires actual canonical pushes and this real
compiler flowing through the queue, including repeated waiting pushes grouped
into one newest-commit build while another repository's running job continues.

## Water-transfer dock enrollment

The dock is deliberately enrolled as a second runtime model. Registration must
require the exact artifact target set `chip_priming_tubing_fixture` and
`water_transfer_dock`; a source recipe change alone does not update the private
pipeline allowlist. Activate matching controller and execution configuration
through supported receipt-bound service controls while no older single-target
CAD job or receipt is pending. Do not interrupt unrelated running compilation.

Receipt acceptance compares the configured `artifact_targets` set against the
recipe's complete output set. A successful compiler exit does not pass that gate
when registration still lists only `chip_priming_tubing_fixture`. Keep both
`chip_priming_tubing_fixture` and `water_transfer_dock` in the controller and
execution-worker configuration. Apply registration repairs through receipt-bound
staging and drain-safe update controls, preserving the registered paths and
unrelated repository settings. Publish a canonical change to request a fresh
build after repairing registration; do not invoke a manual compiler retry.

The dock's outputs and execution checkpoints are described in
[water-transfer-dock.md](water-transfer-dock.md). Its mesh previews come from
the actual STL vertices. A compiled binary receipt does not establish that those
meshes have been generated or inspected.
