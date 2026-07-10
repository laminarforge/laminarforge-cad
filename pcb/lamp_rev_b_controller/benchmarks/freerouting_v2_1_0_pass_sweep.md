# Rev B Freerouting 2.1.0 pass sweep

Ticket: `T-E6F292B9`

Harness commit: `825d826069ef81be9277422627161febf4fef219`

## Result

Conclusion **(a): a longer stock Freerouting run is enough on the observed sample**. The isolated pass-10 run imported into a disposable KiCad board with physical DRC `0`, dangling warnings `0`, and real unconnected `0`. The planned pass-50 run was skipped under the requested early-stop rule.

| Pass budget | Observed | Runtime | Freerouting routed / remaining | SES nets / wires / segments / vias | SES length | Post-import errors / dangling / real unconnected |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 1 (`exact_budget`) | 8.132 s | 30 / 13 | 76 / 735 / 941 / 150 | 4814.874062 mm | 1 / 1 / 12 |
| 10 | 10 (`exact_budget`) | 15.165 s | 41 / 2 | 76 / 488 / 1008 / 167 | 5492.455980 mm | 0 / 0 / 0 |
| 50 | skipped | — | — | — | — | clean pass-10 stopping criterion met |

- Pass-1 SES SHA-256: `e6be32f1ac94557781fb322e7814f4ffafc1b43865af1ae8e615d30c2114ebc8`
- Pass-10 SES SHA-256: `2d7a75b19f8459060b59737febd99df5e0e87a38f214b2f25aef0ad9c419526a`
- Input board SHA-256 before and after: `2e2722f4e5b4acc72965f36f6fecc1a5f201de26a8b326ed44ce2b1f0d21d884`
- Generated fresh-input DSN SHA-256 for both runs: `63f45876adb1d40e7fe629fd10340aca65b6791570441f99f2b40c20ed02449e`
- Freerouting runtime consumed by the sweep: `23.297 s`, below both the 20-minute per-run and 35-minute total bounds.

Freerouting's own pass-10 statistics still label two connections incomplete, while KiCad reports only two raw self-zone entries, both excluded by the existing identical-zone filter, leaving zero real unconnected. KiCad is the final connectivity/physical-DRC gate for this benchmark.

## Reproducibility and retained evidence

Exact bit reproduction is not available: stock Freerouting 2.1.0 uses an unseeded `java.util.Random` for autoroute shuffling. The new pass-1 SES hash differs from the earlier committed pass-1 baseline (`9aa9f765d439…`), even though the board, DSN, toolchain, threads, and exposed routing settings match. Therefore this result establishes that stock pass 10 *can* finish cleanly, not that every unseeded pass-10 run is guaranteed to do so.

Full effective settings, command arguments, validation details, and raw stdout/stderr logs are retained under `freerouting_v2_1_0_pass_sweep/pass_1/` and `pass_10/`. No SES, imported routes, or generated boards were promoted. The canonical PCB remains byte-for-byte unchanged.
