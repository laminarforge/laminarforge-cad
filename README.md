[![CI](https://github.com/laminarforge/laminarforge-cad/actions/workflows/ci.yml/badge.svg)](https://github.com/laminarforge/laminarforge-cad/actions/workflows/ci.yml)
[![Release](https://github.com/laminarforge/laminarforge-cad/actions/workflows/release.yml/badge.svg)](https://github.com/laminarforge/laminarforge-cad/actions/workflows/release.yml)

# laminarforge-cad

Parametric CAD models for the LaminarForge open-source diagnostics platform, written in Rust using the [`vcad`](https://crates.io/crates/vcad) crate. 45+ binaries in `src/bin/`, shared constants in `src/lib.rs`, PCB routing module in `src/pcb/`.

## Setup

```bash
# Clone the repository
git clone <repo-url>
cd laminarforge-cad

# Build all models
cargo build --release

# Generate a single model (outputs STL to ./output/)
cargo run --release --bin tube_holder

# Run tests
cargo test --release
```

## CI / Release

- Every push and PR against `main` runs `cargo fmt --check`, `cargo clippy -D warnings`, `cargo build --release`, and `cargo test --release` (see `.github/workflows/ci.yml`).
- Tagging a release (`git tag vX.Y.Z && git push --tags`) builds all bins, zips `output/*.stl`, and attaches the archive to the GitHub release (see `.github/workflows/release.yml`).
