[![CI](https://github.com/laminarforge/laminarforge-cad/actions/workflows/ci.yml/badge.svg)](https://github.com/laminarforge/laminarforge-cad/actions/workflows/ci.yml)
[![Release](https://github.com/laminarforge/laminarforge-cad/actions/workflows/release.yml/badge.svg)](https://github.com/laminarforge/laminarforge-cad/actions/workflows/release.yml)

# laminarforge-cad

Parametric CAD models for the LaminarForge open-source diagnostics platform, written in Rust using the [`vcad`](https://crates.io/crates/vcad) crate. Core LAMP/CRISPR device CAD is now built around a sealed disposable diagnostic cartridge rather than loose PCR tubes. Shared constants live in `src/lib.rs`, device and validation generators live in `src/bin/`, and PCB routing lives in `src/pcb/`.

## Setup

```bash
# Clone the repository
git clone <repo-url>
cd laminarforge-cad

# Build all models
cargo build --release

# Generate the sealed disposable cartridge prototype
cargo run --release --bin diagnostic_cartridge

# Generate the reusable device assembly visualization
cargo run --release --bin assembly

# Run tests
cargo test --release
```

## CI / Release

- Every push and PR against `main` runs `cargo fmt --check`, `cargo clippy -D warnings`, `cargo build --release`, and `cargo test --release` (see `.github/workflows/ci.yml`).
- Tagging a release (`git tag vX.Y.Z && git push --tags`) builds all bins, zips `output/*.stl`, and attaches the archive to the GitHub release (see `.github/workflows/release.yml`).
