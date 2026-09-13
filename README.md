# laminarforge-cad

Parametric CAD models for the LaminarForge open-source diagnostics platform, written in Rust using the [`vcad`](https://crates.io/crates/vcad) crate. Core LAMP/CRISPR device CAD is now built around a sealed disposable diagnostic cartridge rather than loose PCR tubes. Shared constants live in `src/lib.rs`, device and validation generators live in `src/bin/`, and PCB routing lives in `src/pcb/`.

## Local iteration

Use the local `laminarforge_build` MCP tool with an explicit `bin`:

| Change | Action | Purpose |
| --- | --- | --- |
| Rust source validation | `check` | Check one target without code generation. |
| Rust source needs execution | `run`, profile `dev` | Build and run one target. |
| Runtime TOML only, executable already built from the intended source | `execute`, profile `dev` | Generate outputs without invoking Cargo. |
| Final optimized output | `run`, profile `release` | Explicitly optimize the selected target. |

The priming fixture accepts runtime TOML. Other models vary: some already read
configuration, while others use compiled constants. See [the iteration guide](docs/cad-iteration.md)
for exact tool arguments and verification requirements, and [the workflow audit](docs/cad-workflow-audit-2026-09-13.md)
for the remaining improvements.

## Source and delivery

`main` is canonical. Builds and CAD generation run locally on the Mac. AWS,
GitHub Actions and Namespace processing are retired; GitHub stores source only.
Pushing or tagging does not build or publish model outputs. Validate each
changed model and deliver its explicit output files through the local workflow.
PCB manufacturing output must also pass the [routing and release standard](docs/pcba_routing_and_release_standard.md).
