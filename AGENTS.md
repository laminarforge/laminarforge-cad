# LaminarForge Repo Instructions

## Branch And Release Policy

`main` is the single canonical branch and release source for this repository.

- Do not create `dev`, `staging`, `prod`, `master`, or other long-lived release branches.
- Do not add dev-first, staging-first, or prod-branch deployment workflows.
- Production output is produced from `main`; `prod` is not a branch or separate release lane.

## Build

AWS, GitHub Actions and Namespace build processing are retired. GitHub is source control only. Use local MCP execution; do not restore or dispatch cloud build workflows.

Use `mcp__agentic-mcp__laminarforge_build` for CAD/build work in this repository.

- Read `docs/cad-iteration.md`. Select one binary; use `check` for Rust validation,
  `run` with profile `dev` when changed Rust needs execution, and `execute` for
  runtime configuration changes after building the intended source once.
- `execute` requires a valid build receipt. After Rust, dependencies, features,
  or toolchain changes, build the target again before executing it. After a
  background build/run, register its receipt with one targeted `build`.
- Do not run package-wide release builds/tests or `run_all` for a single-model edit.
  Select tests for the affected geometry and contracts. Final optimization is explicit.
- New adjustable models must take validated runtime parameters and an explicit
  output directory. Generators and verifiers must consume the same configuration;
  reject missing/unknown fields and invalid dimensions before generating geometry.
- Record the configuration, source/binary identity and geometry verification with
  delivered outputs. A successful process exit alone is not manufacturing validation.

## PCBA Routing And Release

The mandatory policy is [`docs/pcba_routing_and_release_standard.md`](docs/pcba_routing_and_release_standard.md). Board-specific files may add fixtures and evidence, but may not weaken or fork it.

- Start from a tracked-clean, fetched canonical `origin/main`; pin and record every source/tool identity. Use MCP LaminarForge build and approved PCB review tools.
- Keep routing proposals, DSN/SES imports, and validation scratch-only. Promote separately only after zero-delta DRC, connectivity, structural-identity, visual, electrical, and build gates pass.
- Run the exact fresh-input 1→10→50 Freerouting matrix with a 1,200-second cap. Stop at the first clean budget; allow no more than three fresh promotion attempts from the same input identity.
- Keep responses bounded: compact ≤10 KiB by default, one candidate ≤25 KiB in detail, and exact-ID retrieval for full evidence. Truncation is a stop condition.
- Fail closed on stale/dirty identity, parser/tool errors, DRC/dangling/open regressions, blocking ERC, BOM/CPL/footprint or fabrication blockers, nondeterministic packages, or missing manufacturer-portal approval. Any source, placement, population, stackup, schematic, or copper change invalidates prior routing evidence.
