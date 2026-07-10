# Rev B Freerouting Benchmark

This harness converts the canonical `lamp_rev_b_controller.kicad_pcb` into an isolated Specctra DSN fixture, runs the repository-pinned Freerouting JAR with bounded settings, imports the SES into a disposable board copy, and runs KiCad physical DRC/connectivity on that copy. It never writes routed copper back to the canonical board or `routing_seed.toml`.

The fixture is safe to publish with a future upstream issue or pull request. It is derived only from this public repository's board geometry, footprints, net names, and routing and contains no credentials, patient data, firmware secrets, or private supplier terms.

## Reproduce

On macOS with Homebrew OpenJDK and KiCad on `PATH`:

```sh
PATH="/opt/homebrew/opt/openjdk/bin:$PATH" cargo run --release --bin lamp_rev_b_controller_freerouting_benchmark -- --pass-budget 1 --output-dir target/freerouting-benchmark/pass-1
```

Use `--java <path>` or `--kicad-cli <path>` when those executables are not on `PATH`. The output directory must be absent or empty. The run writes `result.json`, `summary.md`, the generated DSN, raw SES, Freerouting logs, disposable pre/post-import KiCad boards, and KiCad JSON DRC reports.

The pinned version, JAR hash, build revision, 20-minute per-run bound, routing settings, and public-fixture declaration live in `freerouting_benchmark.toml`. Every invocation requires an explicit positive `--pass-budget`. The harness passes that value through both public `-mp` and internal `router.stop_pass_no`, records it in `result.json`, and rejects missing, excess, or premature observed pass counts. An observed count below budget is valid only when Freerouting reports zero remaining connections.

## Determinism limitation

Freerouting 2.1.0 has no random-seed option. Its `BatchAutorouter` unconditionally calls `Collections.shuffle(..., new Random())`, including the single-threaded autorouter path. This harness therefore fixes every exposed control (one autorouter thread, an explicit pass budget, sequential optimizer selection, optimizer and fanout disabled) and records `random_seed_supported: false` / `random_seed: null`, but the raw SES hash is observational rather than guaranteed reproducible. Adding an injectable seed and reporting the effective seed is the leading upstream contribution target.

There is also a v2.1.0 CLI pass-limit defect: public `-mp` updates `router.max_passes`, while the batch autorouter stops on a separate `stop_pass_no` field that otherwise remains `999`. The harness sets both fields from the explicit budget and verifies the observed count exactly, except for a clean early completion with zero Freerouting connections remaining. A future upstream fix should make the public max-pass setting authoritative and add a regression test around CLI execution.

The bounded 1/10/50-pass convergence evidence is recorded in `benchmarks/freerouting_v2_1_0_pass_sweep.md`; the pass-50 run was skipped after pass 10 reached the clean KiCad stopping criterion.
