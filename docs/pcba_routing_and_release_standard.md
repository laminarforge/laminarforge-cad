# LaminarForge PCBA Routing and Release Standard

Status: mandatory repository policy  
Source post-mortem: artifact `A-8469937E`  
Research ticket: `T-2ED14E2C`  
Historical Rev B routing ticket: `T-352986EA`  
Effective: 2026-07-11

This standard applies to every LaminarForge PCBA routing change and manufacturer release. Board-specific directories may provide fixtures, configurations, and evidence indexes, but must not weaken or fork this policy. In this document, **must**, **require**, **stop**, and **promote nothing** are release gates rather than suggestions.

## Executive conclusion

Rev B did not reach zero by making the custom one-trace proposer progressively smarter. It reached zero only after the team (1) established a clean and current canonical input, (2) added enough physical routing space and manually closed many obvious routes, (3) repaired and bounded the DSN/SES/MCP toolchain, (4) built an isolated benchmark that pinned the board, Freerouting binary, Java/KiCad identities, settings, hashes, pass count, runtime, and scratch validation, and then (5) tested stock Freerouting as a whole-board iterative router. On the 43-real-open canonical board, one pass was not acceptable; ten passes produced a scratch-imported board with KiCad physical DRC 0, dangling warnings 0, and real unconnected 0. The result was promoted only after structural identity checks and fresh KiCad, contract, and build gates.

The durable lesson is: route quality is a gated experiment, not an open-count contest. A candidate that closes an airwire but adds DRC is a failure. A router that reports remaining connections is not the release authority. A board that is route-clean is not automatically electrically or manufacturer-release ready. The canonical checkout, full input identity, independent KiCad checks, visual plots, BOM/CPL/ERC/fabrication gates, deterministic packaging, and portal review are separate checkpoints.

## Evidence notation

- **Causal fact** means directly supported by a ticket, artifact, committed benchmark, tool run, or git commit.
- **Inference** means the most plausible explanation consistent with those facts but not proven by an instrumented controlled comparison.
- **Recommendation** means the reusable standard derived from the evidence.

The Rev B findings below retain that distinction. The mandatory workflow later in this document is the recommendation derived from them.

## Rev B post-mortem

### 1. Source package and deterministic route seed: July 1–3

**Causal facts**

- Commit `6f36827` (2026-07-01) added the Rev B PCBA source package.
- Artifact `A-448F1D06` records an early clean baseline with ERC 0, physical DRC 0, and 214 real unconnected items. It also records that read-only agents had inspected stale local commit `4563e04`; implementation was required to start at `8e06d01` or later.
- Commits `8e06d01` and `936a515` added schematic capture and routing-seed schema v2. The route source of truth became durable TOML, not hand-edited KiCad traces.
- Git history then shows a long series of small reductions: 172, 159, 149, 144, 138, 133, 123, 119, 116, 111, 105, 100, 94, 91, 88, 87, 86, 85, 78, 76, and 75 opens, followed by named I2C routes. This was real progress, but it was labor-intensive singleton closure.
- Ticket `T-352986EA` was created 2026-07-01, briefly marked blocked and unblocked, briefly marked done on 2026-07-03, reopened 108 seconds later, and finally closed on 2026-07-10. The audit trail proves status churn; it does not prove a technical cause by itself.

**Inference**

The deterministic seed/materializer was a necessary foundation, but the commit cadence is evidence of a locally greedy workflow: each accepted route reduced the count, while the remaining problem became increasingly dominated by congested, interacting nets.

### 2. Structural stall and north-board relief: July 3–7

**Causal facts**

- At commit `81b349b`, the board baseline was 70 active opens, physical DRC 0, 118 × 88 mm, four copper layers.
- Artifact `A-32EA841C` records eight scratch candidates. Every candidate closed its intended airwire, but every one changed physical DRC from 0 to a nonzero value:
  - HEATER0_GATE: 70→69 opens, DRC 0→8
  - HEATER_ARM: 70→69, DRC 0→5
  - CAM_CS_N: 70→69, DRC 0→3
  - 3V3_BST: 70→69, DRC 0→8
  - USB_CC2: 70→69, DRC 0→5
  - LATCH_LOCKED_N: 70→69, DRC 0→15
  - LED_PLUS: 70→69, DRC 0→4
  - HEATER1_LOW: 70→69, DRC 0→14
- The coordinator correctly kept route writers closed and requested structural analysis rather than promoting any of these.
- Artifact `A-A716CFFD` ranked +6 mm north growth first, +8 mm only if needed; it deferred extra layers and locked-component moves.
- Commit `cced62e` (2026-07-04) promoted north board growth. The eventual outline was 118 × 94 mm. Commit `a01a3b` fixed bounds checks for the grown board.
- No evidence shows that board growth alone closed the board. It was a prerequisite that relieved edge pressure.

**Inference**

The repeated pattern—airwire closure plus DRC regression across unrelated net families—shows that the stall was structural congestion, not an inability to serialize another trace. The north growth helped create perimeter routing space later used by successful controlled routes.

### 3. MCP routing infrastructure and failed autorouter handoffs: July 3–9

**Causal facts**

- The MCP repository added geometry/search, observability, scratch validation, autorouter bridge, visual overlays, and electrical/fabrication tools in grouped commits on July 3: `3a23707`, `bdc5c52`, `9288591`, `9407f1e`, `56147e4`.
- Commit `b3149ed` made legacy DSN/SES paths fail closed.
- July 8 required many concrete interoperability fixes before DSN/SES results were trustworthy: DSN placement, network pin tokens, padstacks, circle/through-stack serialization, router-log failure handling, SES coordinate scale, via anchor paths, scratch handoff, per-net acceptance, and coordinate transforms (`d3d38c2` through `e4647cc`).
- Artifact `A-A55D8559` records a fresh detached worktree because local `main` was behind `origin/main`. Its 20/60/120-pass batch was not promotable: one run had a Freerouting normalization error; all three SES files failed parser acceptance on malformed one-point paths; therefore zero scratch candidates were applied.
- On July 10, bounded/scoped DSN export still needed context-net preservation, target-net replacement semantics, non-target copper protection, coordinate review, and keeping target copper routable (`5b84a87`, `33086c3`, `ceac573`, `9a0bb8d`).
- These fixes are prerequisites, not evidence that a custom algorithm routed the final board.

**Inference**

Earlier long autorouter runs did not establish that “more passes do not work.” They exercised a changing and sometimes invalid export/import path. Only the later isolated benchmark made pass-count comparisons meaningful.

### 4. Manual controlled closure to 43 and the custom one-trace proposer: July 8–9

**Causal facts**

- Git history on July 8 contains many individually promoted routes, reducing the remaining problem to 44 active opens.
- Artifact `A-29F36F80` records the successful HEATER0_GATE perimeter route: one scratch candidate, active opens 44→43, physical DRC 0→0, followed by independent scratch validation, DRC delta, and unconnected delta. Commit `1c3c08a` promoted it.
- MCP commit `0ad7e14` added `laminarforge_pcb_local_route_proposer`. The implementation enumerates a small fixed family: straight same-layer, x/y doglegs, or layer transitions with a via at an endpoint/corner. It ranks skeletons using endpoint pressure, length/detour, and an approximate clearance model. Its own schema says KiCad scratch DRC remains authoritative and canonical write is forbidden.
- At the 43-open clean baseline, generic one-lane attempts failed:
  - `3V3_BST`, artifact `A-B20FE39C`: 43→42, DRC 0→4; all three local shapes were already reported blocked by eight nearby objects.
  - `+3V3`, artifact `A-8CBB5944`: 43→42, DRC 0→12.
  - `THERM_CH_3`, artifact `A-93B57D95`: 43→42, DRC 0→8.
  - `LID_CLOSED_N`, artifact `A-CAF8ED22`: rejected on nonzero physical DRC.
- None mutated canonical CAD; failure memory was recorded.
- The detailed proposer response itself became an operational blocker: 62,310 bytes/about 17,112 tokens and renderer truncation. Commit `b064ea6` introduced compact summaries, a one-candidate bounded detail path, and exact `candidate_id` retrieval. Artifact `A-231C1E06` measured 9,062-byte summary output, 20,673-byte bounded detail, and 25,156-byte selected full evidence.

**Why the proposer failed — causal facts**

1. It proposed route *skeletons*, not a global rip-up/reroute solution.
2. Its path family was deliberately small and orthogonal.
3. Its clearance model was approximate; zone handling used conservative bounding boxes, and scratch KiCad DRC was explicitly authoritative.
4. The selected 43-open candidates consistently closed the target airwire but created 4–12 DRC violations.
5. It had no authority to move existing copper or components, reserve a multi-net channel, or iteratively negotiate interacting routes.

**Inference**

The proposer optimized the wrong unit of work for the late-stage problem. A single geometric shortcut can look good by distance/open-count while being impossible in the existing copper field. The successful manual HEATER0_GATE route used board-specific intent—an existing same-net In2 entry and a right-edge perimeter dogleg—to avoid a known collision. The generic proposer did not search a comparable state space. Improving its scoring alone would not turn it into a whole-board autorouter.

### 5. Controlled stock Freerouting benchmark and final routing: July 9–10

**Causal facts**

- Commit `368a014` added a Rust-only isolated benchmark. Commit `f304382` recorded a pinned Freerouting 2.1.0 baseline. Commit `825d826` made pass budget explicit and authoritative in both public `-mp` and internal `router.stop_pass_no`, with observed-pass verification.
- The harness pinned Freerouting 2.1.0 revision `1c1edc12…`, JAR SHA-256 `2c07d58f…`, one autorouter thread, optimizer disabled, sequential item selection, fanout disabled, a 1,200-second per-run cap, canonical board and fresh DSN hashes, and disposable import and KiCad validation.
- The required 1/10/50 sweep at commit `825d826` produced:
  - Pass 1: 8.132 s, Freerouting 30 routed/13 remaining, post-import physical violations 2 (1 warning), real unconnected 12.
  - Pass 10: 15.165 s, 41/2, physical DRC 0, dangling warnings 0, real unconnected 0.
  - Pass 50: skipped under the first-clean stopping rule.
- The input board SHA-256 and DSN SHA-256 were identical across the sweep. The canonical board remained byte-identical.
- Stock Freerouting 2.1.0 is unseeded. The pass-1 SES hash differed from the earlier baseline under otherwise fixed exposed controls, so the evidence proves possibility, not bit-for-bit or every-run reproducibility.
- The promotion run started from base commit `ba91540`. A default macOS `java` preflight failed before routing and produced no SES; it did not consume one of the three routed attempts. The installed OpenJDK 26 binary was then used.
- The first fresh routed 10-pass promotion attempt succeeded: 14.318 s, observed passes 10, Freerouting 42 routed/1 remaining, SES 76 nets/488 wires/982 segments/170 vias, KiCad physical DRC 0, warnings/dangling 0, raw unconnected 3, reviewed identical self-zone 3, real unconnected 0. Attempts 2 and 3 were skipped under the first-clean rule.
- Commit `ea5410b` promoted the result. Artifact `A-C90D0D06` and the committed promotion benchmark record:
  - all 38 touched candidates accepted by SES review;
  - exact preservation of 155 components, 409 pads, 80 net entries/79 named nets, 22 layers, 4 zones, 118 × 94 mm outline, placement, pad nets, constraints, route families, and rules;
  - only routed copper changed, 601→982 segments and 113→170 vias; and
  - final route report DRC 0, real unconnected 0; contract check and MCP build passed.

**Why ten stock passes succeeded — causal facts**

- It operated on the full current board rather than one isolated net.
- It had ten verified autorouter passes, while one pass was empirically insufficient.
- It could iteratively rip up and reroute interacting copper across the whole routing problem.
- It started from the already improved 43-open, zero-DRC, grown-board canonical state.
- The DSN/SES path and independent KiCad import checks were finally trustworthy.

**Inference**

The extra passes gave the stock global router enough negotiation cycles to escape conflicts that singleton straight/dogleg proposals could not. The result does not show that “10 is universally optimal,” nor that every unseeded 10-pass attempt will be clean. It shows that 10 was the first tested clean budget for this fixed fixture and should be the standard second escalation point.

### 6. Why visual, electrical, and fabrication verification mattered

**Causal facts**

- A one-pass route reduced connectivity dramatically but imported with physical/dangling violations. Open-count improvement alone would have created false confidence.
- Freerouting reported two remaining connections in the pass-10 sweep and one in the promotion run, while KiCad found only reviewed identical-self-zone entries and zero real opens. Conversely, raw KiCad unconnected was nonzero. Neither raw counter was sufficient without classification; KiCad physical connectivity was the release authority.
- After the routing ticket closed, fabrication-readiness work still found and corrected source/electrical issues, including U1 exposed-pad grounding and the source-less HEATER_SUPPLY_SENSE interface. Artifact `A-9F7C4851` records final physical DRC 0, active unconnected 0, 154 item-scoped accepted ERC findings, blocking ERC 0, BOM/CPL/footprint issues 0, and fab gate ready.
- The later ADC route reconciliation used F.Cu and mirrored B.Cu plots. Visual inspection confirmed branch separation, the thermistor detour around J23, and preservation of unrelated autorouted copper.
- The deterministic manufacturer-handoff flow at commit `57b1de7` generates Gerbers, drills, fabrication/assembly PDFs, BOM/CPL/DNP files, top/bottom renders, STEP, manifests and hashes twice and promotes only byte-identical output.
- The repository still separates electrical fabrication-clean state from order/upload readiness. Missing manufacturer/order profile, physical stackup/tolerances, finish, colors, impedance policy, part MPN data, and portal-preview approval remain fail-closed blockers. Artifact `A-D87AC12B` says the portal must inspect rendered layers, drill map, BOM mapping, CPL overlay, rotations, side assignment, exact part matches, substitutions, panelization, and edge/tooling changes.

**Inference**

Machine checks prove connectivity and encoded design rules; visual and portal checks prove that the encoded/exported representation matches human intent and manufacturer interpretation. Neither can replace the other.

## Mandatory workflow

### Checkpoint 0 — ticket and source authority

1. Work under one routing/release ticket.
2. Fetch `origin/main`. Require a tracked-clean checkout with `HEAD == origin/main`. Do not use ignored reports as fresh evidence.
3. If local `main` is behind, create a detached scratch worktree pinned to the exact origin commit. Never silently benchmark stale local `main`.
4. Record commit SHA, board/project/rules/contract/placement/routing-seed hashes, KiCad version, Java version, Freerouting version/revision/JAR hash.
5. Stop if any required identity is missing or differs from the declared fixture.

### Checkpoint 1 — clean baseline

1. Run the source contract/check binary through `mcp__agentic-mcp__laminarforge_build`.
2. Generate fresh KiCad DRC/connectivity and ERC reports with approved MCP review tools.
3. Require physical DRC 0 before routing. Record raw unconnected, reviewed self-zone entries, and active/real unconnected separately.
4. Run constraint/netclass/stack/layer/outline/component/pad identity audits.
5. Stop and repair the design/toolchain before routing if baseline DRC is nonzero, reports are stale, parser warnings are unexplained, or electrical source intent is unresolved.

### Checkpoint 2 — isolated benchmark fixture

1. Export a fresh full-board DSN to a new empty output directory; never reuse a previous output directory.
2. Protect existing non-target copper and preserve context nets. If doing a bounded target experiment, explicitly make only the target copper routable.
3. Cap runtime at exactly 1,200 seconds per run and verify observed pass count.
4. Store large DSN/SES/log/report payloads on disk or as artifacts; inline only summaries and hashes.
5. Reject any router log error, timeout, malformed path, coordinate-scale mismatch, unknown net/layer, canonical hash change, or observed pass-count mismatch.

### Checkpoint 3 — required 1→10→50 pass matrix

Run fresh-input stock Freerouting in this exact order:

| Stage | Pass budget | Required decision |
|---|---:|---|
| Baseline | 1 | Measure closure, DRC, dangling, runtime, and SES metrics; promote only if fully clean. |
| Standard escalation | 10 | Run only if pass 1 is not clean; stop the sweep if KiCad is fully clean. |
| Extended escalation | 50 | Run only if pass 10 is not clean and no parser/tool failure exists. |
| Structural stop | none | If pass 50 is not clean, stop pass escalation and open a structural routing decision. |

For each row record runtime; observed passes; Freerouting routed/remaining; SES nets/wires/segments/vias/length/hash; before/after physical DRC, warnings/dangling, raw/self-zone/real opens; and unchanged canonical hash.

Do not infer a pass trend from runs with different board/DSN hashes or rejected SES. Do not run 60/120/“more until lucky” after a failed 50-pass matrix without a new ticketed hypothesis.

### Checkpoint 4 — promotion attempts

1. Use the first clean budget from the matrix.
2. Permit a maximum of three fresh routed attempts from the same canonical input identity because stock Freerouting 2.1.0 is unseeded.
3. A Java/tool preflight failure that produces no SES is not a routed attempt; fix the explicit dependency and retry.
4. First clean attempt wins; skip remaining attempts.
5. If all three routed attempts fail, promote nothing. Record all results and return to structural/toolchain analysis.

### Checkpoint 5 — scratch-only import and promotion gate

Routing and SES work must remain scratch-only until every condition below passes. A candidate may be promoted only when all are true:

- SES parser/review accepted; no unknown nets/layers or malformed geometry.
- Scratch physical DRC remains 0; DRC delta is 0.
- Dangling-route warnings are 0.
- Expected active-open reduction is achieved; final promotion requires active/real unconnected 0.
- Any ignored raw unconnected item is individually classified as the exact reviewed identical-self-zone case; no blanket subtraction.
- Component/reference/value/footprint/placement/rotation/side and every pad geometry/net match the baseline.
- Layer count/order, zones, outline, board dimensions, contract nets, netclasses, widths, route families, test-point and rule coverage match.
- Only the authorized copper delta exists.
- Independent fresh KiCad report, route report, constraint audit, contract check, focused tests, and MCP build pass.

Canonical copper must not be written by proposal, benchmark, import, or validation tools. Promotion is a separate, explicitly reviewed action after scratch evidence is complete.

### Checkpoint 6 — visual and electrical approval

1. Generate F.Cu and mirrored B.Cu plots plus top/bottom board renders.
2. Inspect route separation, plane/return continuity, local detours, copper-to-edge, testpoint access, connector escape, high-current widths, and preservation of unrelated copper.
3. Run domain reviews for USB/differential, I2C, power rails, heater/current paths, ground return, and thermal zones as applicable.
4. Run ERC with item-scoped, reasoned acceptances. Require blocking ERC 0. A raw ERC number alone is not pass/fail.
5. Any source, population, placement, schematic, stackup, or copper change invalidates prior route evidence and returns to Checkpoint 0.

### Checkpoint 7 — fabrication, deterministic package, and portal approval

1. Require BOM/CPL/footprint issue count 0, missing BOM 0, missing CPL 0, and footprint mismatch 0.
2. Generate Gerbers, PTH/NPTH drills/maps, fabrication and assembly drawings, paste, IPC-356 if available, DNP/no-substitution files, renders, STEP, source snapshot, manifest, and `SHA256SUMS`.
3. Generate the package twice from the same clean canonical commit; require normalized byte identity.
4. Run the fabrication-readiness gate and require release blockers 0.
5. Independently review Gerber layers, outline, drills/slots, mask/paste, polarity/origin, BOM mapping, CPL overlay/rotation/side, DNPs, substitutions, panelization, tooling, and edge changes in the selected manufacturer portal.
6. Do not upload or order until the manufacturer/order profile and portal approval record are explicit. Electrical fab-ready is not order-ready.

## Bounded output and evidence rules

- Default tool response: compact summary, target ≤10 KiB.
- Detailed response: at most one bounded candidate, target ≤25 KiB.
- Full evidence: retrieve exactly one `candidate_id`; do not inline all candidates.
- Persist logs with run IDs and expose bounded excerpts by line/byte cap.
- Persist arrays, DRC entries, DSN/SES, and reports as files/artifacts; inline counts, top blockers, hashes, and paths.
- Never let renderer truncation become the evidence record. If a response truncates, stop and retrieve by ID/path before deciding.

## Anti-patterns

- Counting “open decreased” as success when DRC increased.
- Running or promoting from stale local `main`.
- Comparing pass counts across different board/DSN hashes.
- Reusing a nonempty output directory.
- Treating Freerouting's remaining count or raw KiCad unconnected count as the sole authority.
- Treating a custom fixed-skeleton proposer as a global autorouter.
- Repeating a known blocked geometry without a structural change.
- Escalating pass count after parser/tool errors.
- Importing an SES that failed parser acceptance.
- Dumping tens of kilobytes of candidate arrays inline.
- Hand-editing canonical KiCad copper without durable source/identity evidence.
- Claiming fabrication/order readiness from DRC/connectivity alone.
- Inventing manufacturer settings or accepting portal auto-mapping without review.
- Allowing routing evidence to survive a source, placement, population, stackup, schematic, or copper change.

## Stop conditions

Stop immediately and promote nothing when any of these occurs:

- canonical checkout is dirty, stale, or not bound to a recorded commit/hash set;
- baseline physical DRC is nonzero;
- router exceeds the 1,200-second timeout or pass budget, logs an error, or observed passes do not match;
- SES is missing, malformed, uses unknown nets/layers, or fails scale/import review;
- scratch DRC delta is nonzero or dangling warnings appear;
- active opens do not decrease as expected;
- structural/identity diff shows unauthorized change;
- output truncation hides the candidate or validation evidence;
- pass 50 remains unclean;
- three fresh promotion attempts at the winning budget all fail; or
- blocking ERC, constraint conflicts, BOM/CPL/footprint issues, fabrication blockers, nondeterministic packaging, or unapproved portal mapping remains.

## Required durable evidence bundle

Every PCBA routing ticket must retain:

1. Ticket timeline and decision/status notes.
2. Canonical commit and all input/tool hashes/versions.
3. Baseline DRC/ERC/connectivity/constraint summary.
4. Benchmark configuration and 1/10/50 matrix result JSON/Markdown.
5. Bounded stdout/stderr logs and run IDs.
6. DSN/SES hashes and retained raw payload locations.
7. Scratch apply/validate, DRC delta, unconnected delta, SES review, and structural diff run IDs.
8. Accepted/rejected candidate artifact with explicit reason.
9. F.Cu/B.Cu plots or visual-review record.
10. Final KiCad, route report, contract check, tests/build, ERC, BOM/CPL, constraint, and fabrication-gate evidence.
11. Gerber/drill/assembly/BOM/CPL render/portal checklist and approval record.
12. Final board/package SHA-256 and source commit.

## Rev B fixture and evidence index

The Rev B board-specific benchmark configuration is [`../pcb/lamp_rev_b_controller/freerouting_benchmark.toml`](../pcb/lamp_rev_b_controller/freerouting_benchmark.toml). Its operating notes and committed sweep/promotion evidence are indexed from the [Rev B controller README](../pcb/lamp_rev_b_controller/README.md).

- `T-352986EA`: close Rev B routing; final description and status history.
- `T-23164347`, `A-32EA841C`, `A-A716CFFD`: eight DRC-regressing candidates and structural escape decision.
- `A-A55D8559`: stale-main avoidance and rejected 20/60/120 DSN/SES retest.
- `A-29F36F80`: successful controlled HEATER0_GATE perimeter route, 44→43, DRC 0.
- `A-B20FE39C`, `A-8CBB5944`, `A-CAF8ED22`, `A-93B57D95`: 43-open singleton rejections.
- `A-231C1E06`: proposer output cap and measured response sizes.
- `T-7E497D30`, `A-1279B899`, commits `368a014`/`f304382`: isolated benchmark and upstream findings.
- `T-E6F292B9`, `A-7655E067`, commits `825d826`/`ba91540`: 1/10/50 matrix and pass-10 clean evidence.
- `A-C90D0D06`, commit `ea5410b`: promoted zero-unconnected result.
- `T-C7BBBEA3`, `A-9F7C4851`, commit `9cd44a5`: later electrical/fabrication reconciliation.
- `A-D87AC12B`, commit `57b1de7`: provisional portal controls and deterministic manufacturer handoff.

Future PCB directories must carry a small board-specific benchmark configuration and evidence index that points here; they must not copy and diverge from this general policy.
