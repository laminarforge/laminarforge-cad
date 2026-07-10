# Freerouting baseline: lamp_rev_b_controller_current_route_closure

- Freerouting: `v2.1.0` at `1c1edc12bd20fbb328c2a52efbeb5f6a785849de`
- Input board SHA-256: `2e2722f4e5b4acc72965f36f6fecc1a5f201de26a8b326ed44ce2b1f0d21d884`
- Generated DSN SHA-256: `63f45876adb1d40e7fe629fd10340aca65b6791570441f99f2b40c20ed02449e`
- Raw SES SHA-256: `9aa9f765d4390cdcbebf1e4b3f7b8696382e40714140c92260dffe43e77b3932`
- Runtime: `7951` ms (bound `180` s)
- Autorouter passes: `1` observed (bound `1`)
- Freerouting routed/unrouted connections: `30` / `13`
- SES: `76` nets, `723` wires, `917` segments, `152` vias, `4906.355217` mm trace length
- KiCad before import: `0` physical DRC (`0` errors, `0` warnings), `43` real unconnected
- KiCad after import: `5` physical DRC (`0` errors, `5` warnings), `10` real unconnected
- Random seed: unsupported by Freerouting 2.1.0 (`null`)
- Canonical board modified: `false`
- Fixture safe for public upstream use: `true`

Freerouting's raw `BoardStatistics` report labels a `1182 × 942 mm` board and `48658.82 mm` trace length, while the DSN source is `118 × 94 mm` and independent SES geometry totals `4906.355217 mm`. This consistent factor-of-ten discrepancy points to resolution handling in statistics and should not replace the SES-derived metrics above.

The most valuable first upstream enhancement remains an injectable random seed with the effective seed included in CLI output. Freerouting 2.1.0 calls `Collections.shuffle(..., new Random())`, so otherwise identical bounded runs can produce different SES hashes, via counts, and closure results. A separate CLI defect leaves `-mp 1` disconnected from the batch autorouter's `stop_pass_no`; this harness sets both and verifies the observed bound.
