# Mac-local compile-only queue

Canonical `main` pushes are the normal request for the registered local pipeline.
The worker invokes the root `Makefile` at the queued SHA, using the recipe from
that same commit. Agents publish through the workspace lifecycle and inspect
status/logs; they do not invoke the recipe or manual compiler commands.

The explicit group `cad-priming-dev` compiles only the existing reviewed runtime
binary `chip_priming_tubing_fixture`. It preserves the repository's `dev` profile
(incremental model compilation, optimized dependencies) and normal Cargo
parallelism. The recipe verifies its existing `cad-workflow.json` enrollment,
runtime configuration and verifier before compiling. It neither changes model
contracts nor executes generators, verifiers, installers or network setup.

The worker supplies `BUILD_JOB_ID`, `BUILD_COMMIT_SHA`, `BUILD_REPOSITORY`,
`BUILD_TARGET`, `BUILD_CACHE_DIR`, `BUILD_OUTPUT_DIR`, `BUILD_BASE_SHA` (empty for
the first compatible build), and `BUILD_TOOLCHAIN`. Cache and output directories
are absolute and distinct. Cargo keeps its stable target directory under the
registered cache. Each job receives a copied, read-only binary and a version-1
`build-receipt.json` with the exact job, repository, SHA, `dev` profile, toolchain
identity, artifact path and SHA-256. Artifact kind is `compiled-binary`.
`phases.ndjson`, `changed-paths.txt`, the worker's build log and Cargo timings
provide observable checkpoints.

This is compilation only. A compiler artifact is not a geometry result,
manufacturing validation, installation or delivery. Runtime execution still
requires the existing CAD configuration/verifier contract and appropriate MCP
execution receipt; the pipeline receipt is not a substitute for that authority.

Activation and real compilation remain pending. The pipeline must explicitly
allow this repository, group, `dev` profile and compiled-binary artifact before
claiming success. Verification requires actual canonical pushes and this real
compiler flowing through the queue, including repeated waiting pushes grouped
into one newest-commit build while another repository's running job continues.
