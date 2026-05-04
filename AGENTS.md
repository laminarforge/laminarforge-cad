# LaminarForge Repo Instructions

## Branch And Release Policy

`main` is the single canonical branch and release source for this repository.

- Do not create `dev`, `staging`, `prod`, `master`, or other long-lived release branches.
- Do not add dev-first, staging-first, or prod-branch deployment workflows.
- Production output is produced from `main`; `prod` is not a branch or separate release lane.

## Build

Use `mcp__agentic-mcp__laminarforge_build` for CAD/build work in this repository.
