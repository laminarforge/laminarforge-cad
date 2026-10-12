# LaminarForge Repo Instructions

## Branch And Release Policy

`main` is the single canonical branch and release source for this repository.

- Do not create `dev`, `staging`, `prod`, `master`, or other long-lived release branches.
- Do not add dev-first, staging-first, or prod-branch deployment workflows.
- Production output is produced from `main`; `prod` is not a branch or separate release lane.

## Build

AWS, GitHub Actions and Namespace build processing are retired. GitHub is source control only; do not restore or dispatch cloud build workflows.

Publish the workspace to canonical `main`; push is the sole normal compilation
request. Do not invoke MCP build/setup/deploy submissions, `laminarforge_build`
check/build/run actions, compiler/Make commands or indirect compilation wrappers.
The registered `cad-priming-dev` recipe compiles only
`chip_priming_tubing_fixture`. Additional targets require deliberate recipe and
pipeline registration changes; do not bypass missing coverage with manual builds.

- Read `docs/local-build-pipeline.md` and `docs/cad-iteration.md`. Record the
  observed job ID, exact SHA and immutable artifact receipt; publication alone is
  not queue admission, compilation or delivery proof. Report pending checkpoints.
- Retain status, logs, pause and cancel controls. Cargo uses normal compiler
  parallelism within the pipeline's single running compilation job.
- Runtime `execute` requires its supported executable receipt. A pipeline
  `compiled-binary` receipt does not grant geometry execution authority. If the
  adapter or matching receipt is unavailable, report execution pending; do not
  call legacy build/run actions to create it.
- Verify the actual tool/service and geometry behavior with observable evidence.
  Do not add unit, mock, stub, snapshot, synthetic-fixture or implementation-
  mirroring test files without Alex's explicit request. No package-wide `run_all`.
- Read `docs/cad-runtime-contract.md`. The MCP enforces `cad-workflow.json`: new or changed frozen model entry points require a reviewed runtime config/verifier contract. Never refresh a frozen model hash to bypass migration.
- New adjustable models must take validated runtime parameters and an explicit
  output directory. Generators and verifiers must consume the same configuration;
  reject missing/unknown fields and invalid dimensions before generating geometry.
- Record the configuration, source/binary identity and geometry verification with
  delivered outputs. A successful process exit alone is not manufacturing validation.

## Protected network boundary

Product compilation and CAD execution must not invoke network maintenance.
WireGuard, its LaunchDaemons, routing/forwarding, PF and associated DuckDNS live
in the separate `network-operations` scope. Never change, reinstall, restart or
reapply them unless Alex explicitly requests that exact maintenance in the
current conversation. Do not run broad setup workflows. Pipeline intake is
outbound polling; no router changes or inbound webhook exposure are required.

## PCBA Routing And Release

The mandatory policy is [`docs/pcba_routing_and_release_standard.md`](docs/pcba_routing_and_release_standard.md). Board-specific files may add fixtures and evidence, but may not weaken or fork it.

- Start from a tracked-clean, fetched canonical `origin/main`; pin and record every source/tool identity. Compile through canonical pushes; use receipt-authorized execution and approved PCB review tools.
- Keep routing proposals, DSN/SES imports, and validation scratch-only. Promote separately only after zero-delta DRC, connectivity, structural-identity, visual, electrical, and build gates pass.
- Run the exact fresh-input 1→10→50 Freerouting matrix with a 1,200-second cap. Stop at the first clean budget; allow no more than three fresh promotion attempts from the same input identity.
- Keep responses bounded: compact ≤10 KiB by default, one candidate ≤25 KiB in detail, and exact-ID retrieval for full evidence. Truncation is a stop condition.
- Fail closed on stale/dirty identity, parser/tool errors, DRC/dangling/open regressions, blocking ERC, BOM/CPL/footprint or fabrication blockers, nondeterministic packages, or missing manufacturer-portal approval. Any source, placement, population, stackup, schematic, or copper change invalidates prior routing evidence.
