# 16-Slot Cassette No-Cell Bench Validation Protocol

Protocol ID: LF-CAS-A7-P001
Revision: A1
Ticket: T-D713D22E
Digital baseline: fbb0006e9f5ad4fd02b2b0aecd88e4c043c54f6e
Companion forms: docs/sixteen_slot_cassette_no_cell_bench_validation_data_sheets.md
Readiness checklist: LF-CAS-A7-RC001 revision A0,
docs/sixteen_slot_cassette_hp0_hp2_readiness_checklist.md

## Document Status

This is the repository-versioned engineering execution protocol for the first physical LaminarForge
16-slot, 4 x 4 cassette. It converts the A7 validation specification into a
bench-executable sequence with fixed sample definitions, measurement controls,
data records, acceptance decisions, and failure responses.

The protocol is versioned as an execution template, but it is not a
QMS-approved procedure and no physical execution is authorized yet. The current
A7 CAD outputs are dry layout mockups: the surrogate and restriction pieces are
solid, the bubble-station ports are not flow passages, and no quantitative
dye/hold-up station exists. Full-cassette wet testing is blocked until hold
points HP-0 through HP-2 are independently approved. Limited empty-bench
commissioning may begin only after HP-0 and SAF-E01. HP-1 functional or gasket
coupon wet qualification may begin only after the as-built checks close and
SAF-E02 is independently approved.

Core A7 execution uses only an approved nonhazardous, nonbiological liquid.
Every fluid and cleaning agent requires an SDS, PPE/disposal review, and
materials-compatibility check. A hazardous or flammable E2 cleaner requires a
separate approved SOP and risk assessment. This protocol does not validate
sterility, aseptic processing, live-cell use, AAV containment, recirculation,
or mixed-condition routing.

## 1. Governing Scope

- One cassette is one condition: within each trial, all sixteen paths share one
  declared cassette-wide source, recipe, pressure/flow profile, condition ID,
  and waste identity. Prospectively declared cassette-wide tracer, rinse, and
  E2 phases are diagnostic phases, never per-lane experimental conditions.
- S01-S16 are spatial positions and readout lanes inside one cassette. They are
  not sixteen independent cassette samples.
- Restriction, blockage, bypass, artificial-leak, and bubble articles are
  diagnostic challenges, not separate experimental conditions.
- The cassette-specific A0, A3, A6, A7, and A8 requirements control this work.
  A generic 2 x maximum-operating-pressure note elsewhere in the repository
  does not override the cassette-specific 1.5 x liquid proof requirement.
- The full cassette is never the first burst article. Production-intent gasket
  coupons must pass isolated liquid leak and destructive liquid burst gates
  before the cassette is hydraulically challenged.
- The closed carrier/closure/lid stack is 41.35 mm high. The 31.35 mm value is
  the carrier overall height and must not be used as the closed-stack fixture
  clearance.
- Environmental mapping and cleaning/carryover are separately labeled
  extensions. They may be required by the intended-use release, but they are
  not silently represented as core A7 hydraulic evidence.

Passing this protocol permits media-only planning for the tested configuration
and claim level only. It does not itself authorize a media-only run.

## 2. Controlling References And Resolved Conflicts

| Topic | Controlling source | Protocol resolution |
| --- | --- | --- |
| Mechanical geometry and slot map | A0 interface specification and shared Rust contract | Use the released 4 x 4, row-major S01-S16 configuration and released drawings. |
| Gasket and closure | A3 gasket specification | Measure actual gasket free height; require both 20-30% squeeze and the 1.68-1.92 mm nominal-height guard band. |
| Disposable path and ports | A6 fluid-path specification | Use one pressure-limited, single-pass harness and the G0-G3, M0-M6, W0-W4 map. |
| Ordered hydraulic gates | A7 fixture specification | Preserve the eleven A7 stages; later evidence cannot erase an earlier failure. |
| Proof pressure | A3/A6/A7 | Installed liquid proof is 1.5 x MOP and below every installed component proof rating. |
| Isolated gasket qualification | A3/A7 | Liquid pressure is max(35 kPa gauge, 1.5 x MOP). |
| Destructive margin | A3 | Qualify isolated coupons before full-cassette wet work at max(3 x isolated-gasket pressure, 100 kPa gauge). |
| Dock drainage | A0/A8 | Test segmented openings and intentional bridges as modeled; do not claim a continuous sump or gravity capacity. |
| Environmental work | A3 plus manufacturing-readiness plan | Run as extension E1 with its own approved limits; do not call it incubator qualification. |
| Cleaning/carryover | A4 plus manufacturing-readiness plan | Run as extension E2 with a frozen nonbiological tracer, recipe, analytical limit, and controls. |

Manufacturing remains blocked until the D2/D4 gasket groove has its required
entry break and vendor-approved corner geometry and the D7 worst-case stack
closes against the actual gasket lot. This protocol does not waive those
drawing blockers.

## 3. Definitions And Claim Levels

### 3.1 Count definitions

- n_article: independently manufactured reusable cassette/dock assemblies.
- n_harness: independently assembled disposable harnesses.
- n_lane: spatial paths within one cassette; n_lane = 16.
- Measurement repeat: a repeated observation on the same physical
  configuration.
- Independent wet preparation: a new fluid preparation, full drain/reset,
  instrument check, prime, and run ID. It is not a new hardware article.
- Closure cycle: one complete open, inspect, reseat, close, torque, settle, and
  witness sequence.
- Connector cycle: one mate/demate operation counted separately and only for a
  connector rated for reconnection.
- Challenge trial: one controlled perturbation at one frozen location and
  setting.

### 3.2 Claim levels

| Claim | Minimum physical sample | Required repetition | Permitted conclusion |
| --- | --- | --- | --- |
| FA-1 first article | One cassette/dock serial, one gasket assembly from one identified lot, one 16-surrogate nominal set; n_harness = 1 for a reconnect-rated harness or n_harness = 4 for the fixed single-use plan (checkpoint 1 plus fresh harnesses at 5/10/25) | Complete protocol; three independent wet preparations for quantitative prime/bubble/flow/recovery/waste tests where specified | The tested serialized configuration and each identified harness passed its executed first-article A7 engineering gates; the conclusion does not accept the gasket or harness lot. |
| DV-3 internal engineering confirmation | One cassette/dock serial. Reconnect-rated: n_harness = 3 full-core builds. Single-use: n_full_core = 3 (H1 plus H2/H3) plus checkpoint-only H5/H10/H25, n_harness total = 6. Record n_lot separately. | H1 completes FA-1/core; H2/H3 each repeat topology and three-preparation prime/bubble/installed-leak/nine-interval-flow/dye/waste gates at the same frozen closure state; the designated reconnect-rated H1 or separate single-use H5/H10/H25 plan completes endurance. | Harness-build repeatability is supported for the three full-core builds. Checkpoint-only harnesses are not counted as full-core replicates. Three builds from one lot do not support a three-lot conclusion; this is not formal design validation or population reliability. |
| Manufacturing confirmation | Three independently manufactured cassette/dock sets. Pair each set with one reconnect-rated harness (3 total), or with the four-harness single-use FA-1 checkpoint plan (12 total). Record n_lot for both; require n_lot = 3 only for a lot-to-lot statement. | One complete approved qualification per set, including endurance, with the sample plan revisited after pilot variance. | Broader design/manufacturing confirmation may be assessed; formal process capability still requires a separate plan. |

FA-1 is the minimum current A7 gate. The final report must state the claim
level prominently. Sixteen lanes and repeated readings may not be counted as
independent cassette articles.

## 4. Roles And Approval Independence

| Role | Responsibility |
| --- | --- |
| Design authority | Owns released geometry, intended use, MOP, hardware ratings, and configuration changes. |
| Metrology lead | Owns measurement methods, traceability, uncertainty budgets, and dimensional decision rules. |
| Fluidics validation lead | Owns fixture commissioning, frozen stimuli, hydraulic execution, and raw data. |
| Test operator | Executes the approved sequence without unapproved adjustment. |
| Safety reviewer | Independently approves pressure containment, relief, shielding, spill capacity, electrical separation, and safe depressurization; does not design, calculate, execute, or technically own the rig evidence being approved. |
| Quality/release reviewer | Independently approves hold points, deviations, invalidations, retests, and final disposition; does not originate, calculate, execute, or technically own the evidence being approved. |
| Records custodian | Owns the immutable executed-record location, access control, retention, native-file preservation, hashes, and manifest completeness. |

The test operator may not self-approve a deviation, invalidation, acceptance
limit change, or final release. The safety reviewer must be distinct from the
pressure-rig designer and operator for the approved rig. The quality reviewer
may own the gate decision, but not any technical input used to release it.

## 5. Test Articles And Minimum Sample Sizes

### 5.1 Core FA-1 articles

| Article | Minimum | Sampling meaning |
| --- | ---: | --- |
| Reusable carrier/lid/window/bulkhead/dock set | 1 serialized matched set | Qualifies this set only. |
| Production-intent disposable harness | 1 serialized reconnect-rated assembly, or 4 serialized assemblies for the fixed single-use checkpoint plan | Initial harness stays connected through P1-P3; single-use replacements occur only at checkpoints 5/10/25. Qualifies each listed harness only. |
| Gasket | 1 serialized production-intent assembly from 1 identified lot | Free height and lot tolerance are measured before closure; evidence qualifies the assembly/configuration, not the lot. |
| Nominal flow-through surrogates | 16 uniquely identified articles | Complete spatial census of S01-S16. |
| Low-resistance coupons | 4 | One challenge in each row and column. |
| High-resistance coupons | 4 | One challenge in each row and column. |
| Blocked coupons | 4 | One challenge in each row and column. |
| Bypass coupons | 4 | One challenge in each row and column. |
| 20/25/30% squeeze ladder | 1 set per gasket material and lot | Three height references, not three independent gasket samples. |
| Isolated gasket leak coupon | 1 per gasket material and lot | Coupon/configuration evidence only. |
| Destructive burst coupon | 1 per gasket material and lot | Separate from all release hardware. |
| Gasket reassembly coupon | 1 per gasket material and lot | Same coupon through closure/reassembly cycles 1, 5, 10, and 25; this is not a fluid-connector cycling article. |
| Humid-soak coupon | 1 per gasket material and lot when E1 is in scope | E1-specific prerequisite; not required to close core HP-1. |
| Artificial-leak control | 1 isolated, permanently marked control | Detector verification only; never release-installed. |
| Evaporation blanks | At least 3 matched covered vessels per gravimetric run | Correct or bound collection loss. |

If multiple gasket materials or lots are evaluated, duplicate the complete
coupon set. The FA-1 minimum is intentionally not a statistical reliability
sample. A broader claim requires the manufacturing-confirmation plan.

### 5.2 Coupon characterization

Every functional nominal or fault coupon receives at least three repeated
steady-state measurements at the frozen flow condition. Record pressure,
temperature, flow, calculated resistance, uncertainty, and its approved band.
Repeated readings characterize measurement repeatability; they do not create
independent physical samples.

### 5.3 Rev C chip-lot metrology

Measure 100% of the sixteen intended Rev C chips plus every intended spare
before a pocket drawing is released. The physical protocol uses surrogates, but
the run cover must link the surrogate interface revision to the measured
production chip-lot record.

## 6. Required Equipment And Measurement Capability

Specific manufacturer/model downselection belongs in immutable DS-02A1. Final
selected-equipment capability, ratings, calibration plan, and projected
uncertainty belong in a new immutable DS-02A2; A2 cites but never mutates A1.
Exact received serials, current certificates, as-found/as-left checks, and
pre-SAF commissioning close in DS-02B. Post-SAF bubble/dye/evaporation method
controls close later in DS-02C. Each stage cites its predecessor one way.
Missing selection, range, calibration plan/state, or uncertainty is an HP-0
failure.

| Equipment class | Minimum quantity/capability | Release requirement |
| --- | --- | --- |
| Pressure-limited pump/controller | One, with command logging, hard maximum, isolation, liquid-compatible relief, and safe vent/drain | Range covers MOP and proof without saturation. MOP is derived from the lowest applicable manufacturer-rated, temperature-derated working limit plus approved engineering margin; an n = 1 coupon burst is margin evidence, not an operating rating. |
| Pressure measurement | Six simultaneous channels: common upstream, R1-R4, waste/backpressure | Synchronized raw capture at 1 Hz or faster for decay tests; range places test values away from sensor endpoints. |
| Pressure reference/check source | One traceable reference covering all test pressures | Pre/post zero and span checks; expanded uncertainty supports the decision rule. |
| Data acquisition and clock | One synchronized logger | Raw native export, monotonic timestamps, command/status capture, software/firmware version, and file hash. |
| Flow collection | Sixteen labeled, covered collection vessels and nests | S01-S16 identity cannot be swapped; matched evaporation blanks included. |
| Input-flow reference | Traceable source balance, calibrated positive-displacement delivery, or reference flow measurement | Measures actual interval input independently of pump command; range and uncertainty support the total-recovery band. |
| Balance | One or more balances plus traceable check weights | Readability at most one tenth of the smallest expected net mass; expanded uncertainty no more than 25% of the nearest mass-related acceptance band where practical. |
| Temperature | Fluid and ambient probes for core work | Record temperature for density, pressure stabilization, and long holds. |
| Environmental mapping extension | Sixteen fixed slot-plane probes plus chamber reference and ambient probe | Required only for E1; loaded and unloaded maps remain separate. |
| Bubble injection | Calibrated positive-displacement tool or fixed loop at common and R1-R4 locations | Frozen slug volume, uncertainty, injection geometry, and clearing limit. |
| Bubble evidence | Fixed lighting/camera and visible upstream, row, inlet, and outlet witness zones; optional sensors | Optical detection limit is frozen; video is time-linked to raw data. |
| Dye quantification | Calibrated absorbance/colorimetric method or other validated quantitative method | Photograph alone cannot support quantitative recovery unless validated; blank, standards, independent check, LOD/LOQ, and recovery are recorded. |
| Dimensional inspection | D8-approved CMM/plate/indicator, micrometer, optical comparator, profilometer, pins/gauges as required | Current calibration and uncertainty for every critical feature and witness. |
| Torque | Calibrated driver covering the selected M4 torque and tolerance | Torque is recorded but cannot substitute for stop/witness evidence. |
| Leak controls | Matched sealed blank, characterized artificial-leak control, dye-visible tray/witness paper, and smallest practical trapped liquid/residual-gas volume | Blank volume/compliance bounds the DUT configuration. The positive control is at or below the smallest rejectable leak and brackets each campaign; controls remain isolated from release hardware. |
| Waste station | Adjustable, measured high/nominal/low elevation and reverse-head/backpressure capability | Captured W4 relief/overflow, anti-siphon challenge, and secondary containment. |
| Fluid preparation | Calibrated mass/volume tools, thermometer, density method, and viscosity method when viscosity-adjusted fluid is used | Recipe, lot, density, viscosity, dye concentration, and temperature are traceable. |
| Safety | Separately rated installed-system and destructive-coupon rigs; shield, spill kit, electrical separation, emergency stop, liquid relief, and secondary containment | Ratings, relief tolerances, abort limits, SDS/PPE/disposal, and chemical compatibility are approved; containment is at least 110% of the frozen maximum releasable liquid inventory. |

The project target is expanded measurement uncertainty no greater than 25% of
the applicable tolerance band where practical. This is a LaminarForge
measurement-capability rule, not a universal ISO requirement. If it cannot be
met, use a tighter guard band or better method before execution.

## 7. Prerequisite Hold Points

### HP-0 — Controlled inputs frozen

The quality reviewer must confirm every required value in DS-00 and DS-01 plus
the immutable DS-02A1 downselection and DS-02A2 final-capability snapshots before HP-0 can
pass. HP-0 itself authorizes no wet work. The controlled closure workflow and
acquisition gaps are tracked in LF-CAS-A7-RC001 revision A0. SAF-E01 may then
authorize only its exact bounded empty-bench commissioning scope. Functional or
gasket coupons additionally require the as-built HP1-04 checks and SAF-E02;
full-cassette wet work additionally needs HP-1 and HP-2 approval:

- Released drawing set, STEP manifest, source commit, and calculation version.
- Cassette, dock, lid, window, bulkhead, gasket, fastener, harness, surrogate,
  coupon, fixture, software, and port-map revisions.
- Fault-insertion manifold/architecture, reference nodes, hydraulic-equivalence
  evidence, blinded actuation method, and proof that all sixteen challenges can
  be changed without opening the cassette or disturbing its gasket, harness,
  source condition, or closure-cycle count.
- One nonbiological condition ID.
- Fluid recipe, density, viscosity, dye concentration, and test temperature.
- MOP, installed proof pressure, isolated gasket pressure, burst target,
  maximum permitted coupon-rig pressure, pressure target tolerance, peak and
  overshoot limits, abort triggers, installed-rig and destructive-rig relief
  setpoints/tolerances, differential pressure basis, and proof/burst rating of
  every component and fixture.
- A guard-banded safety hierarchy demonstrating that proof target is achieved,
  proof plus uncertainty remains at or below the weakest installed proof
  rating, the installed-rig relief lower tolerance permits the proof hold, and
  its upper tolerance/maximum overshoot remains below the lowest safe installed
  limit. The destructive-rig relief lower tolerance permits the burst target,
  while its upper tolerance, hard cap, maximum overshoot, and abort limit remain
  below the rig's lowest safe rating; a coupon surviving the target is a lower-
  bound result and is not forced to failure beyond a rig rating.
- The MOP basis is capped by every supplier-rated, temperature-derated working
  pressure and remains no more than 50% of the lowest credible derated
  failure/burst/connector limit after approved engineering margin. The 1.5 x
  proof checkpoint and an n = 1 coupon result cannot serve as that operating
  limit by themselves.
- Pump/control mode, nominal flow, ramp, objective stabilization rule,
  commanded setpoint and acceptable mean/total/per-row/per-lane flow bands,
  traceable measured-input method and collected-versus-measured-input recovery
  band, collection time or target input volume, pressure-drift
  reference/analysis windows, near-zero threshold and absolute drift limit,
  lane mass/flow quantification limits, position-bias decision rule, and DAQ
  sample rate.
- Prime maximum time, maximum system/row/lane volume, recovery band, and their
  uncertainties.
- Bubble gas amount or delivered volume at a stated pressure and temperature,
  uncertainty, injection locations, detection limit, maximum clearing time,
  and maximum clearing volume.
- Pressure-decay source-isolation boundary, effective liquid/gas volumes,
  permissible residual gas, compliance record, thermal-equilibration rule,
  exact acquisition/windows, matched-blank band, artificial-leak magnitude and
  band, witness detection limit, permitted temperature change, and uncertainty
  model.
- Nonpressurized perimeter-challenge liquid head or volume, fill/application
  locations, cassette orientation, dwell, tracer concentration and visual
  threshold, expected gutter/drain destinations, captured-volume recovery
  band, and migration/escape decision limit.
- Waste elevations, reverse head/pressure, dwell, siphon detection limit,
  W4 measured stimulus/released-volume method, captured-volume recovery band,
  escaped-volume decision limit, and containment capacity.
- Post-dye tracer-clear baseline locations, method, decision limit, one-sided
  upper-bound rule, and recovery uncertainty required before waste testing.
- Pre-injection system/carryover-blank sampling map, decision limit, one-sided
  upper-bound rule, and recovery uncertainty required before every A7-9 trial;
  the P2/P3 blank also proves the preceding preparation's waste tracer cleared.
- Quantitative dye method, calibration range, analytical recovery, LOD/LOQ,
  residual method, numeric one-chip-dose volume, condition volume, and
  resulting physical hold-up upper-bound limit and tracer-equivalent-loss
  reporting rule.
- Final torque/tolerance, receiver configuration, settling rule, and witness
  map.
- Connector reuse classification and rated cycle limit.
- Measurement uncertainty budget and conformity decision rule.
- SDS, PPE, chemical compatibility, disposal route, spill/relief/containment
  review, and the separate SOP/risk assessment when a hazardous or flammable
  E2 cleaner is proposed.
- E1 temperature/RH/evaporation limits and E2 cleaning/carryover limits if
  either extension is in scope.
- Raw-file naming, immutable storage location, and hash method.

An unset required value is not a deviation; it is an automatic NOT READY
result.

HP-0 must be closed in this order: define provisional performance and range
requirements; downselect exact components and instruments; demonstrate selected
ratings and measurement capability; then freeze final methods, limits,
guard bands, calibration plans, and DS-01/DS-02A1/A2. A method cannot be called
final before the selected hardware passes that cross-check.

### HP-1 — Functional bench and coupons qualified

The actual gasket material lot and all production-intent HP-1 gasket coupons
must first receive a limited independent incoming release against their
controlled drawings, critical dimensions, finish, stop plane, materials, and
traceability. This HP-1 article release does not replace HP-2 incoming
inspection of the cassette set.

- Functional flow-through nominal and fault coupons exist. The current solid
  CAD tokens do not qualify.
- A non-disturbing, externally accessible fault-insertion architecture is
  hydraulically equivalent at the frozen reference nodes and can select every
  randomized challenge without opening or retorquing the cassette, demating
  the harness, changing the cassette-wide source condition, or revealing the
  key. Without it, A7-8 is NOT READY.
- A quantitative dye/mass-closure and physical hold-up station exists.
- Six mandatory pressure channels, sixteen collection positions, bubble
  injection, waste challenge, leak witness, and relief/containment are
  functional.
- Every flowing challenge coupon has an approved finite resistance band and
  uncertainty; every blocked coupon has approved one-sided flow/resistance and
  pressure-signature limits.
- The sealed blank and artificial-leak control demonstrate usable leak-method
  resolution.
- A smallest-prohibited-bubble positive control at representative upstream,
  row-high-point, and chip-inlet witness positions demonstrates the frozen
  optical method's detection capability under representative lighting and
  pressure.
- Pressure/safety, balance, temperature, clock, torque, and dimensional
  prechecks pass in DS-02B before SAF-E02; bubble, dye, and evaporation method
  controls pass separately in DS-02C after SAF-E02.
- D7 closes at every required witness using the measured gasket lot.
- Production-intent squeeze, isolated leak, and destructive burst coupons
  pass before the cassette is wetted.

Execute the HP-1 coupon program in this order only after HP-0, SAF-E01 bounded
empty-bench commissioning, the as-built HP1-04 safety checks, and SAF-E02:

1. Measure the 20/25/30% squeeze-ladder references at every specified location
   and compare measured height/squeeze with the manufactured reference and D7
   prediction using separate height and squeeze uncertainties.
2. Condition the isolated leak coupon at the released closure setting, isolate
   the source, and hold dyed liquid at Pgasket using the Section 8.2 timing and
   controls. It must meet the 5% decay gate and show no dye migration.
3. On a separate destructive rig and coupon, ramp liquid to the burst target
   without exceeding the frozen maximum permitted pressure, overshoot, relief,
   or abort limits. Surviving the target records a lower bound only; do not ramp
   to failure beyond any rig, fixture, or instrument rating.
4. Cycle the gasket reassembly coupon through 1, 5, 10, and 25 approved coupon
   closure/reassembly cycles, inspecting and repeating the isolated liquid hold
   at every checkpoint. It must meet the initial dimensional, closure, and leak
   limits without damage or adjustment. These are not connector mate/demate
   cycles.

### HP-2 — Incoming inspection and metrology passed

- Vendor FAI and independent incoming measurements pass the released drawings.
- All identities, revisions, lots, certificates, and calibration states
  reconcile.
- Seal lands are clean and undamaged; no burr, coating buildup, clogged port,
  residue, corrosion, or label conflict exists.
- Sixteen surrogates seat without forcing.
- Datum A support, rear/left/front registration, D1 round locator, D2 relieved
  locator, and D3/D4 non-locating witness behavior pass.
- The dock does not twist the carrier beyond the released limit and does not
  hide optical, leak-witness, drain, label, or handling regions.

## 8. General Measurement And Decision Rules

### 8.1 Conformity rule

Use expanded uncertainty U and the preapproved decision rule:

- Upper limit L: PASS only when measured result + U is less than or equal to L.
- Lower limit L: PASS only when measured result - U is greater than or equal to L.
- Two-sided limits [Llow, Lhigh]: PASS only when the complete interval
  [result - U, result + U] lies inside the limits.
- An interval overlapping a limit is recorded as HOLD (metrologically
  indeterminate), not as PASS or article FAIL. Improve the method or remeasure
  only under an approved scope; never relabel the overlap as compliance.

Every individual required lane, trial, checkpoint, and control must pass. Do
not pool data, discard an unexplained outlier, use a majority vote, or average
away a failure.

### 8.2 Pressure decay

After the approved stabilization rule is met, isolate the pump/controller with
the approved liquid-rated valve. No active pressure makeup is permitted during
the hold, and the approved relief path remains available. Acquire at least 661
seconds after isolation:

- Differential pressure is delta P = Pinternal - Pexternal. Record both sides
  or the verified gauge reference; absolute pressure may be logged but is not
  the denominator for this engineering decay gate.
- P0 is the mean differential pressure from t = 0 through 60 seconds.
- P10 is the mean differential pressure from t = 600 through 660 seconds. The
  window centers are exactly ten minutes apart.
- Decay percent = 100 x (P0 - P10) / P0. P0 must be positive and above its
  frozen quantification threshold.

Remove visible gas before the hold and remain within the frozen residual-gas
bound and fluid/ambient temperature-change limit. The matched sealed blank must
represent or conservatively bound the DUT's effective volume and compliance;
otherwise characterize the difference before use. The uncertainty budget
includes pressure calibration/resolution/repeatability, P0/P10 estimation,
timing, ambient reference, temperature change, blank/compliance mismatch, and
residual gas. The artificial-leak and sealed-blank controls must remain within
their frozen bands before and after the campaign.

Apply the 5% criterion to the raw DUT differential-pressure decay; do not
subtract a blank to manufacture a passing result. No full-cassette pneumatic or
full-perimeter burst test is allowed.
Any pressure rise outside the matched-blank/thermal stability band is HOLD as
metrologically indeterminate unless an assignable method cause is demonstrated,
even when the signed decay calculation is below 5%. Peak pressure plus
uncertainty must also remain below the frozen safe
maximum; an overshoot or abort-limit excursion is a safety FAIL.

### 8.3 Gravimetric flow

For each covered collection vessel, use at least three matched blanks over the
same time, cover geometry, and local collection environment. For each blank,
signed blank delta mass = final mass - initial mass; evaporation therefore has
a negative sign. Record both raw masses, exposure times, location, mapping to
lanes, and uncertainty.

- Corrected lane mass = gross - tare - mapped signed blank delta mass.
- Collected volume = corrected lane mass / fluid density at the recorded
  temperature.
- Lane flow qi = collected volume / collection time.
- Row flow is the sum of its four lane flows.
- For the full spatial census, population standard deviation is
  sqrt(sum((x - mean)^2) / N), and CV is 100 x population standard deviation /
  mean. Use N = 16 for slot CV and N = 4 for row CV. Calculate both separately
  for every collection interval using the versioned calculation method.

Every qi used in a CV must be positive and above the frozen quantification
limit. Propagate U(CV) with the approved joint covariance or Monte Carlo model;
shared density, timing, balance, and blank terms must not be counted as
independent sixteen times. Record coverage factor and coverage probability.
Also calculate interval per-lane/per-row/mean/total flow and
collected-versus-measured-input recovery against their prospectively frozen
two-sided bands. Measured input comes from the approved traceable source
balance, calibrated displacement, or reference-flow method—not pump command
alone. CV alone cannot demonstrate adequate delivery.

Report each lane even when the summary CV passes. A below-quantification lane
is a failure, not zero-valued data.

### 8.4 Pressure drift

Freeze the reference window, analysis window, pressure basis, near-zero
threshold, and decision mode for each mandatory channel. For a positive
reference above the near-zero threshold, drift percent = 100 x
max(abs(P(t) - Pref)) / abs(Pref). Apply the uncertainty guard band to the 5%
upper limit. For zero, near-zero, or negative gauge/differential references, do
not divide by Pref; use the prospectively approved absolute kPa deviation limit
that represents the intended operating band, with uncertainty.

### 8.5 Dye recovery

- Input tracer mass Min = Cin x Vin after approved blank, dilution, density,
  and recovery corrections. For every mutually exclusive fraction j,
  Mj = Cj x Vj after its approved corrections.
- Freeze routing states and collection boundaries so a tracer mass appears in
  exactly one ledger destination. Record every fraction's start/stop time,
  volume, concentration, dilution, recovery, uncertainty, and destination.
- Signed mass bias = sum(Mj) - Min. Preserve over-recovery; do not clip it.
  Closure error percent = 100 x abs(signed mass bias) / Min.
- Tracer-equivalent unresolved loss = max(0, Min - sum(Mj)) / Cin. This is not
  automatically hydraulic dead volume because adsorption and analytical loss
  are confounded.
- Quantify physical retained/hold-up volume with a separate, mutually exclusive
  inlet/outlet/rinse/residual volumetric ledger. Report the conservative
  one-sided upper bound, including recovery and uncertainty, when a residual is
  nondetect. Do not call tracer non-closure alone dead volume.

## 9. Ordered Test Procedure

The A7 stages below are the acceptance groups. Physical execution follows this
nested order so the three wet preparations remain genuinely independent. HOLD
or FAIL stops downstream work. An INVALID result requires approved
investigation before repetition and remains in the record.

### 9.1 Controlling campaign matrix

1. Execute A7-1 through A7-3 once. A7-2 is closure cycle/checkpoint 1; do not
   create a second nominal cycle 1.
2. After HP-3, qualify the matched blank/artificial leak, all sixteen isolated
   slot loops, and the nonpressurized perimeter challenge before preparation
   P1. These fixture/loop checks are outside the three installed campaigns.
3. Keep the initial production harness connected through P1-P3. If its
   connector is single-use, do not demate it between preparations; drain/reset
   through the approved route.
4. Execute each preparation in the following order, with a complete drain,
   reset, independent fluid preparation, and instrument precheck before the
   next preparation. Within each preparation, an independent reviewer must
   record a prep-local A7-4/5/6/7 approval after the third nominal interval and
   before A7-8, A7-9, or A7-10. This is the authorized nested implementation of
   the group-level sequence; formal HP-4 closes only after all three prep-local
   approvals:

| Preparation | Exact in-preparation order |
| --- | --- |
| P1 | A7-4 prime; A7-5 one common plus R1-R4 bubble trial; A7-6 one installed proof hold; A7-7 three nominal intervals; A7-9 one dye/hold-up trial; verified tracer-clear baseline; A7-10 one trial at every in-scope waste state; drain/reset. |
| P2 | A7-4 prime; A7-5 one common plus R1-R4 bubble trial; A7-6 one installed proof hold; A7-7 three nominal intervals; A7-8 all sixteen blinded fault placements with a verified nominal restoration after each; A7-9 one dye/hold-up trial; verified tracer-clear baseline; A7-10 one trial at every in-scope waste state; drain/reset. |
| P3 | A7-4 prime; A7-5 one common plus R1-R4 bubble trial; A7-6 one installed proof hold; A7-7 three nominal intervals; A7-9 one dye/hold-up trial; verified tracer-clear baseline; A7-10 one trial at every in-scope waste state; final drain. |

5. Close HP-4 only after the P1-P3 prime, bubble, installed integrity, and nine
   nominal-flow intervals pass. Close HP-5 only after the P2 fault matrix plus
   all three dye/hold-up and all three replicates of every waste state pass.
6. Continue closure cycles 2-25 without resetting the mechanical counter. Wet
   endurance checkpoints occur at cycles 5, 10, and 25. A reconnect-rated
   harness remains the designated harness; a single-use connector uses the
   prospectively identified fresh harness assigned to each checkpoint. Thus
   FA-1 n_harness is 1 for reconnect-rated hardware or 4 for the fixed
   single-use checkpoint plan.

### A7-1 — Incoming inspection

1. Reconcile DS-00/01, commissioned DS-02B, post-SAF DS-02C, HP-2 DS-03B, and immutable
   DS-04A/DS-04B snapshots, including all serials, revisions, lots, and
   certificates.
2. Inspect all seal, datum, optical, drain, connector, label, and handling
   regions under fixed lighting.
3. Seat and remove each uniquely identified surrogate once.
4. Verify all sixteen carrier receivers, sixteen lid fasteners, nine internal
   stops, perimeter stops, and unused-port caps.

Acceptance:

- 16/16 surrogates seat and remove without binding, damage, or unapproved
  force.
- No obstruction, datum damage, gasket defect, clogged port, coating buildup,
  residue, corrosion, or mislabel exists.
- Every lot and identity is traceable. An untracked part fails the stage.

### A7-2 — Dry assembly, compression, and dock fit

1. Measure the gasket free height at the approved lot map.
2. Assemble on datums using the A2 paired cross-pattern: finger-tight pass, low
   seating pass, and initial final pass. Record every setting for all sixteen
   fasteners and every accessible compression witness.
3. Hold at test temperature for at least ten minutes. Record actual start/end
   times and witness heights for a qualifying five-minute stability window; do
   not proceed if stability is not demonstrated within thirty minutes.
4. Perform one repeated final pass only after stability is demonstrated, record
   every fastener setting, and remeasure all required witnesses.
5. After the repeated final pass, demonstrate a new five-minute stability
   window within the same thirty-minute maximum and remeasure every required
   witness. Apply separate expanded uncertainties to compressed height,
   five-minute change, and calculated squeeze.
6. Verify the repeated pass did not invalidate the squeeze, nominal-height, or
   stability limits; if it changes a witness beyond its guard-banded limit,
   return the assembly to HOLD rather than beginning the wet sequence.
7. Verify hard-stop contact, lid alignment, view openings, window retention,
   drain visibility, labels, and free/docked support.

Preferred witness sample is all sixteen per-slot witnesses plus five perimeter
witnesses. If all sixteen are inaccessible, the preapproved minimum slot map is
S01, S04, S06, S07, S10, S11, S13, and S16 plus four perimeter corners and one
defined perimeter reference.

Acceptance:

- Actual squeeze = 100 x (free height - compressed height) / free height is
  20-30% at every measured location after uncertainty; 25% is the target.
- The nominal-height guard band is 1.68-1.92 mm after uncertainty.
- Witness change is no more than 0.02 mm over five minutes.
- All lands/stops contact the common 7.35 mm closure plane.
- No chip, window, port, label, datum, hard stop, gasket land, gutter, drain,
  or witness region is obstructed or damaged.
- Released dock-twist/flatness limits pass both free and docked.

### A7-3 — Dry harness topology

1. Trace reservoir, pump/controller, bubble window, common supply, R1-R4,
   S01-S16 inlets, S01-S16 outlets, row wastes, W0/W1/W3/W4, sensors, caps,
   relief, and containment from source to destination.
2. Scan or photograph every keyed interface and cap state.
3. Verify supplier bend radii, strain relief, dock motion, optical keepouts,
   seals, datums, stops, labels, and leak/drain visibility.
4. Have a second person independently reconcile the physical route against the
   released map.

Acceptance:

- 100% of ports, branches, pigtails, sensors, caps, and waste routes reconcile.
- No route crosses or loads a keepout.
- Any swap-capable ambiguous connector is a design failure. Labels or training
  alone do not rescue the design.

Hold point HP-3: independent dry-stack/topology approval.

### A7-4 — Prime-to-waste

Run P1-P3 under the controlling checkpoint-1 matrix for the FA-1 quantitative
record. At endurance checkpoints 5, 10, and 25, one complete prime is the
minimum; do not add a duplicate checkpoint-1 prime.

1. Complete pressure zero/span, balance, temperature, and fluid-preparation
   checks.
2. Prime common upstream M0/M2 to W1.
3. Prime R1-R4 one row at a time.
4. Prime S01-S16 through the nominal surrogates and collect output.
5. Record volume, time, all pressures, fluid temperature, and recovered waste.

Acceptance for every preparation/checkpoint:

- 16/16 inlet and outlet witnesses are bubble-free.
- No visible bubble remains at common upstream, any row high point, or any
  vertical high point.
- No manual tapping, tube squeezing, rerouting, or unplanned manipulation is
  needed. Such intervention fails the trial and opens a design deviation.
- System, row, and lane prime times and volumes remain within their frozen
  upper limits after uncertainty, and recovered waste remains within its
  frozen two-sided recovery band.
- Prime pressure remains within its approved band without overshoot, and every
  required record and control is complete.

### A7-5 — Bubble challenge

Minimum for the initial checkpoint-1 FA-1 harness, and separately for each H2/H3
DV-3 confirmation harness:

- Three common-upstream trials.
- Three trials at each of R1, R2, R3, and R4.
- Total: fifteen repeated trials, each starting from a verified primed state.

For FA-1, distribute these across the three independent wet preparations: each
preparation contains one common, R1, R2, R3, and R4 challenge. The completed
campaign therefore has three trials at every injection location without
pretending they are independent hardware articles.

1. Before P1 and after P3, present the smallest prohibited bubble at one
   representative upstream, row-high-point, and chip-inlet witness (six method
   checks total). The method must detect 6/6 under the frozen conditions.
2. For each trial, inject the frozen bubble volume at the approved location and
   flow condition.
3. Record injection volume/uncertainty and synchronized video.
4. Run approved prime/debubble mode to W1/W3.
5. Record clearing time, clearing volume, all mandatory pressures, and every
   downstream witness state.

Acceptance for all fifteen trials:

- Clearing time plus its uncertainty and clearing volume plus its uncertainty
  are within their frozen upper limits.
- It routes to W1/W3 and never reaches a chip-inlet witness.
- No trapped bubble remains at a row high point.
- Delivered gas stimulus at the frozen pressure/temperature is inside its band
  after uncertainty, and the smallest-prohibited-bubble method controls confirm
  optical validity.

Fifteen trials are repeated challenges on one system, not n_article = 15.
The planned single-use checkpoint harnesses at cycles 5, 10, and 25 execute
only the A7-11 topology, prime, installed-proof, and three-interval flow subset;
they are not represented as fully A7-qualified harnesses.

### A7-6 — Liquid integrity and pressure decay

Fixture/coupon qualification under HP-1 occurs before this stage. Use liquid,
minimum practical trapped gas, relief, secondary containment, and shielding as
approved.

1. Verify the matched sealed blank and characterized artificial-leak positive
   control against their frozen bands.
2. Confirm the correct liquid rig, isolation boundary, effective liquid/gas
   volumes, residual-gas bound, compliance record, thermal state, matched
   blank, source-isolation valve, relief route, and safe abort limits.
3. Test each of sixteen isolated per-slot gasket loops once. Inability to
   isolate any loop is NOT READY for this protocol, not N/A.
4. Perform a low-head, nonpressurized perimeter-containment challenge; do not
   pressurize the large perimeter footprint as a burst article.
5. For each pressurized hold, reach the target within its frozen tolerance,
   record peak/overshoot, isolate the pressure source with no active makeup,
   and acquire through t = 660 seconds with P0 at t = 0-60 and P10 at
   t = 600-660 seconds.
6. Run three installed-system proof holds for FA-1.
7. Repeat one installed-system hold at endurance cycles 5, 10, and 25; the
   P1-P3 holds are the checkpoint-1 evidence.
8. Repeat the matched blank and artificial-leak positive control after the
   final campaign hold.

Pressures:

- Isolated gasket/coupon qualification:
  Pgasket = max(35 kPa gauge, 1.5 x MOP).
- Installed system: Pproof = 1.5 x MOP, below every installed component proof
  rating.
- Coupon burst under HP-1:
  Pburst target = max(3 x Pgasket, 100 kPa gauge).

Acceptance for every pressurized coupon, loop, and installed hold:

- The target minus uncertainty is achieved, peak plus uncertainty stays below
  the frozen safe maximum, and no abort trigger occurs.
- The source is isolated with no active makeup. Raw differential-pressure
  decay between windows centered ten minutes apart plus uncertainty is no more
  than 5%, and any pressure rise remains inside the frozen blank/thermal band.
- No visible dye appears at chip interfaces, adjacent slots, optical regions,
  gasket perimeter, bulkhead, connector/waste handoffs, gutter, drain, or
  underside witness.
- No lid lift, uncontrolled extrusion, fastener/insert movement, cracked
  surrogate, relief failure, or structural damage occurs.
- All controls pass before and after the campaign.
- Every installed component supports the proof pressure; otherwise the
  component fails selection or MOP must be prospectively reduced.
- The destructive coupon sustains the target without lid lift, rig damage, or
  uncontrolled gasket extrusion. If a separately authorized ramp continues to
  failure, the failure pressure must exceed the target and remain within every
  rig and instrument rating.

The nonpressurized perimeter challenge is not evaluated by pressure decay. It
passes only when dyed liquid shows no migration outside the intended perimeter
containment and all released gutter/drain capture routes remain complete and
observable.

### A7-7 — Nominal flow balance

Run three separate steady-state collection intervals per FA-1 wet preparation,
for nine initial FA-1 intervals total.
At cycles 5, 10, and 25, run three intervals after a complete prime; the nine
P1-P3 intervals are the checkpoint-1 evidence.

1. Install the approved, independently characterized nominal surrogate set.
2. Reach the frozen steady-state criterion.
3. Tare sixteen covered vessels and matched evaporation blanks.
4. Collect all sixteen outputs for the frozen time or target input volume while
   independently measuring actual interval input.
5. Record raw lane/blank/source masses or reference-input readings,
   temperatures, density, time, and all six mandatory pressure traces.
6. Calculate corrected lane flows, row totals, slot CV, row CV, and
   channel-by-channel drift for each interval.

Acceptance for every interval:

- Every lane, row, interval mean, and total flow remains inside its frozen
  two-sided delivery band after uncertainty.
- Collected-versus-measured-input recovery remains inside its frozen two-sided
  band after uncertainty.
- Slot CV plus uncertainty is no more than 10%.
- Row CV plus uncertainty is no more than 10%.
- Every mandatory pressure channel passes its frozen relative 5% or near-zero
  absolute-kPa drift rule after uncertainty.
- Every lane is above the quantitative method limit.
- The frozen spatial-bias metric/test remains inside its approved threshold. A
  flagged row/corner/position pattern is HOLD pending the preapproved decision
  path, even if CV passes.

Do not pool intervals or forty-eight lane values to rescue a failed interval.

Hold point HP-4: independent approval of prime, bubble, integrity, and nominal
flow evidence.

### A7-8 — Restriction, occlusion, and bypass localization

First characterize every physical coupon with at least three steady-state
readings at the frozen reference nodes and condition. For nominal, low, high,
and bypass flow-through coupons, R = delta P / Q with stated units; freeze a
finite band and uncertainty. A blocked coupon with Q at or below LOQ does not
have a finite measured R: freeze a one-sided maximum-flow decision limit and
report R greater than or equal to delta P / Qupper, with its pressure-signature
band and one-sided uncertainty.

Before execution, freeze a scalar class discriminant z from named raw features
or a covariance-aware multivariate classifier, including its training evidence,
decision boundaries, and calculation version. For scalar class means a and b,
the separation statistic is abs(mean(z_a) - mean(z_b)) /
sqrt(u_a^2 + u_b^2 - 2 cov(a,b)), where u is the predictive standard
uncertainty at k = 1; the nearest-class value must be at least 3. Predictive u
includes instrument uncertainty, within-coupon repeatability, between-coupon or
class variation, and baseline drift. It is not the standard error of a class
mean. A multivariate method must freeze an equivalent covariance-aware
three-predictive-standard-uncertainty threshold and nonoverlapping predictive
class bands.

Run one fault at a time with fifteen nominal surrogates:

| Fault | Required placements |
| --- | --- |
| Low resistance | S01, S06, S11, S16 |
| High resistance | S02, S05, S12, S15 |
| Blocked | S03, S08, S09, S14 |
| Bypass | S04, S07, S10, S13 |

This sixteen-trial matrix places each class once in every row and column and
challenges every slot exactly once. Restore and verify the nominal baseline
before the next trial. An independent challenge installer randomizes order and
holds a concealed placement/class key. The analyst runs the frozen classifier
and records class plus slot before the key is revealed. All changes occur in
the qualified external fault-insertion architecture; do not open the lid,
replace an in-pocket surrogate, retorque the closure, disturb the harness, or
change the cassette-wide source condition. If the released fixture cannot do
this, stop as NOT READY and redesign the fixture or prospective cycle/sample
plan.

Acceptance:

- 16/16 affected slot and row locations are identified without visual
  inspection or the concealed key.
- Every flowing coupon remains within its frozen finite resistance band; every
  blocked coupon satisfies its one-sided flow/resistance and pressure-signature
  bounds.
- Every expected class signature is separated from its nearest adjacent class
  by the frozen covariance-aware three-standard-uncertainty rule.
- No fault creates an uncontrolled leak, pressure overshoot, or invalid
  downstream state.
- The randomization key, installer identity, analyst pre-unblinding result,
  reveal time, and raw classifier output reconcile.

### A7-9 — Dye recovery, mass closure, and physical hold-up

Perform three complete injection/flush/recovery trials for FA-1, one per
independent wet preparation.

1. Verify analytical blank, independent check standard, recovery control, LOD,
   LOQ, decision limit, and one-sided nondetect upper-bound method.
2. Before injection, collect the system/carryover blank across the frozen
   relevant sampling map. Its one-sided upper bound, including recovery and
   uncertainty, must be below the residual acceptance limit; otherwise HOLD.
3. Quantify Cin, Vin, and Min with uncertainty.
4. Inject through M3 or the released dye inlet.
5. Flush using the released A6 prime/rinse route.
6. Under the frozen mutually exclusive routing states, quantify each collected
   fraction's start/stop time, Cj, Vj, dilution, recovery, uncertainty, and
   unique ledger destination. Do not sum S01-S16 and downstream W0/W3 when they
   represent the same fluid.
7. Quantify connector, filter, bubble element, dead-leg, gasket-interface,
   dry-structure, drain/leak-capture, and residual-extract locations using the
   frozen sampling map.
8. Calculate signed tracer bias, absolute closure error, tracer-equivalent
   unresolved loss, and the separate physical inlet/outlet/rinse/residual
   volumetric hold-up ledger and one-sided upper bound.

Acceptance for all three trials:

- Closure error plus uncertainty is no more than 10%.
- The one-sided upper bound on physical retained/hold-up volume, including
  recovery and uncertainty, is below the smaller of the frozen numeric
  one-chip-dose volume and 10% of the frozen cassette-condition volume.
- Tracer-equivalent unresolved loss is reported separately and cannot be
  relabeled physical dead volume without independent volumetric evidence.
- No mapped or unidentified location contains dye above its prospectively
  validated visual or analytical detection threshold. A nondetect passes only
  when its one-sided upper bound, including recovery and uncertainty, meets the
  location limit and the method decision limit is below that acceptance limit.
- Every analytical control and recovery check passes.

No operator or integrator may waive the physical hold-up upper-bound gate.

### A7-10 — Waste backflow, siphon, and overflow

Run one replicate of every in-scope state in each of the three independently
identified wet preparations. Within each preparation, use this fixed order:

1. Normal nominal-elevation reference.
2. Lowest planned waste elevation/siphon state.
3. Highest planned waste elevation/backpressure state.
4. Applied reverse-head or reverse-pressure state.
5. Pump-off hold at the worst approved elevation.
6. W4 relief/overflow state when W4 is installed; W4 is always last because it
   intentionally challenges containment.

Full W4 scope therefore produces eighteen preparation/state records: three
normal references plus fifteen challenge trials. If W4 is prospectively N/A,
retain its three rows as N/A and execute fifteen records total.

Before every state, restore the nominal configuration, flush, and demonstrate
the state-local tracer-clear baseline with the frozen one-sided upper-bound
rule. Assign a unique preparation/state/waste/tracer identity. After every
state, collect and reconcile the fluid, inspect containment, and complete the
approved reset before the next state. A nonclear baseline or incomplete reset
is HOLD and blocks the next state.

Record exact elevation, head/pressure, dwell, uncertainty, initial/final
levels, reverse-tracer result, siphon volume, W4 released and captured volumes,
relief route, and containment.

Acceptance for every trial:

- No reverse tracer reaches chip outlets or row branches.
- The one-sided upper bound on uncommanded siphon volume or reverse tracer,
  including measurement recovery and uncertainty, remains below the frozen
  acceptance limit; the validated detection/decision limit must itself be
  below that acceptance limit.
- W4 and every overflow route remain within secondary containment. Measured
  captured-versus-released volume passes its frozen recovery band, and the
  one-sided upper bound on escaped volume, including uncertainty, is below its
  frozen limit.
- No dry cassette structure, label, electrical equipment, or bench surface is
  wetted.
- Waste ID remains linked to cassette and condition IDs.

Hold point HP-5: independent approval of fault, recovery, and waste evidence.

### A7-11 — Repeat assembly and reconnection endurance

Use the same reusable cassette, gasket, fastener/receiver set, and nominal
surrogates through twenty-five closure cycles unless a failure requires
quarantine. If the released connector is explicitly rated for repeated
no-cell reconnection, use the designated endurance harness and count every
mate/demate. If the connector is single-use, use the prospectively assigned
fresh harness at each wet checkpoint and repeat topology before wetting; that
planned harness change does not reset the reusable mechanical cycle counter.

1. At every cycle, open, inspect, reseat, close, torque, settle, and inspect.
2. Treat A7-2 plus P1-P3 as checkpoint 1. At cycles 5, 10, and 25, repeat
   compression, prime, installed liquid integrity, and three-interval nominal
   flow gates; do not execute a duplicate cycle-1 checkpoint.
3. Count connector cycles separately. Do not reconnect a single-use connector.
4. Record cuts, flattening, debris, coating wear, window damage, label
   degradation, screw/insert changes, and route changes.

Acceptance:

- Every checkpoint meets the same cycle-1 limits.
- No gasket cut/crack/twist, compression loss, thread/insert damage, connector
  loosening, cracked surrogate, window damage, corrosion, or label/route
  degradation occurs.
- No unapproved part replacement occurs.

Unplanned replacement of a gasket, fastener, insert, surrogate, connector, or
harness after a failure creates a new configuration. The durability counter
for the replaced item resets to cycle 1; the original sequence remains failed
or terminated. Prospectively assigned single-use checkpoint harnesses follow
their approved sample plan and are not treated as corrective-action
replacements.

Hold point HP-6: cycle-25 and final physical-inspection approval.

## 10. Separately Scoped Extensions

Complete the core sequence through HP-6 before either extension. If both are
run on the same FA-1 article, the fixed order is E1, its repeat gates, then E2
and its repeat gates; the conclusion is combined sequential exposure because
the two effects are confounded. To claim each effect separately, assign
separate prospectively identified articles. HP-7 follows all in-scope
extensions.

### E1 — Humid 37 C material/dock conditioning

E1 is required when the intended-use release includes humid 37 C docking. It is
not incubator qualification.

- Freeze temperature, RH, duration, probe positions, stabilization, logging
  interval, evaporation limit, condensation rule, and loaded configuration in
  DS-01.
- Use sixteen fixed slot-plane probes plus chamber reference and ambient probe
  for every loaded E1 gate.
- Run an unloaded chamber confirmation separately from the representative
  loaded cassette map; freeze and pass its own stability, spatial, overshoot,
  and recovery limits before loading the article.
- For FA-1, run one 24-hour commissioning exposure. Three separately prepared
  power-cycle exposures on the same serialized hardware support environmental
  run-repeatability only. A manufacturing-confirmation conclusion requires the
  Section 3 independently manufactured article plan.
- Measure the humid-soak coupon before and after exposure and execute its
  frozen post-soak compression and isolated liquid-hold gates. The coupon is
  exposed once as part of E1; its executed result is required to close E1, not
  core HP-1.
- Record per-position min/max/mean, temporal fluctuation, spatial range,
  overshoot, stabilization, recovery, condensate, and evaporation.
- After exposure, repeat full DS-05B compression/closure, one installed proof
  hold, three nominal-flow intervals, label scan, window inspection, and
  corrosion/residue inspection.

Every slot must remain within the prospectively approved limits after
uncertainty. No uncontrolled condensation or dry-structure wetting is allowed.

### E2 — Nonbiological cleaning and carryover

E2 is required when the intended-use release includes a reusable cleaning
process. It is not sterility, decontamination, or AAV-clearance validation.
The A6 wetted harness remains disposable and is never released for cleaning and
reuse. E2 applies only to prospectively declared reusable, nonwetted
carrier/lid/dock/window surfaces and their slot-associated sampling units.
Gasket lands or dry connector exteriors are included only if explicitly
declared reusable and nonwetted. Tubing and wetted connector interiors are
excluded. Any future reusable wetted component requires a separate protocol
revision.

- Freeze nonbiological tracer, loading, dwell/dry time, cleaning agent,
  concentration, temperature, pH, contact time, agitation/flow, flush volume,
  direct/rinse sampling, analytical recovery, LOD/LOQ, carryover limit, and
  material list.
- Include clean blanks, process blanks, deliberately dirty positive controls,
  and recovery spikes with frozen expected control-response bands: at least one
  of each control type in every cleaning cycle.
- Run every declared reusable surface sampling unit through three separately
  prepared soil-clean-blank cycles on the same serialized surfaces. These are
  repeated process executions, not n_article = 3.
- Report results below LOD as below the method limit, never as zero.
- A deliberately dirty control must produce its prospectively expected
  detectable/high response, and recovery controls must pass.
- After the third cycle, repeat visual/dimensional inspection, label scan,
  gasket inspection, full DS-05B compression/closure, one installed proof hold,
  three nominal-flow intervals, and one complete dye/hold-up trial using a
  newly identified clean disposable harness.

Acceptance:

- Every declared reusable surface sampling unit is at or below the frozen
  carryover limit after uncertainty. For a nondetect, the one-sided upper
  bound/reporting limit, including recovery and uncertainty, must be no more
  than the acceptance limit, and the LOQ/decision limit must be below it.
- No residue, corrosion, retained dye, label lift, gasket damage, window
  cracking/clouding, surface-finish failure, dimensional failure, or hydraulic
  degradation occurs.
- Recleaning until a sample passes is prohibited.

## 11. Failure, Investigation, And Retest Rules

### 11.1 Status values

- PASS: prerequisites, controls, raw data, uncertainty, and every criterion pass.
- FAIL: the article/configuration produced an out-of-limit result.
- INVALID: a demonstrated method, instrument, identity, or data-record failure
  prevents interpretation. INVALID is not PASS and remains in history.
- HOLD: investigation or disposition is pending; downstream work is blocked.
- NOT READY: a prerequisite or frozen input is missing.
- N/A: allowed only when prospectively approved for a genuinely optional
  feature or extension.

### 11.2 Failure classes

| Class | Examples |
| --- | --- |
| S — Safety/containment | Overpressure, uncontrolled spill/leak, wet electrical equipment, relief failure, window break, hardware lift, uncontrolled extrusion. |
| I — Identity/topology/data | Wrong part/lot/route/condition, missing calibration or raw trace, timestamp discontinuity, overwritten data. |
| M — Method/instrument | Failed control, saturation, balance drift, uncertain stimulus, unstable environment, invalid calculation. |
| C — Component/assembly | Damaged gasket, connector, harness, fastener, insert, surrogate, coupon, or workmanship. |
| D — Design/durability | Fit, compression, bubble, leak, flow, recovery, backflow, drainage, cleaning, or endurance outside limits. |

### 11.3 Immediate response

1. Stop commanded flow and depressurize through the approved safe path.
2. Isolate electrical power if liquid threatens energized equipment.
3. Contain fluid and preserve the physical state.
4. Do not retorque, tap, reroute, clean, adjust, or disassemble except for
   immediate safety.
5. Save native raw traces, video/photos, fluid fractions, operator notes, and
   calculation state.
6. Assign a deviation ID and quarantine the cassette, harness, waste, coupons,
   and failed parts.
7. Identify the last verified-good hold point and place dependent downstream
   evidence on HOLD.

### 11.4 Investigation and invalidation

- Preserve the original result. Never delete, overwrite, relabel, or average it
  with later data.
- Check controls, fixture integrity, calibration, environment, identity,
  operator actions, fluid preparation, software/version, and calculation
  history.
- INVALID requires a documented, scientifically demonstrated assignable
  measurement-system cause and quality approval.
- Without a demonstrated assignable method cause, treat the observation as an
  article/design/manufacturing/process failure.
- A failed post-campaign instrument check triggers impact assessment of every
  result since the last known-good check.

### 11.5 Retest limits

- No immediate try-again run is allowed.
- Root cause, corrective action, change-impact assessment, and a preapproved
  retest scope are required first.
- One planned confirmation retest is allowed per corrective action. Recurrence
  returns the design/configuration to review.
- An original failure remains part of the release record even after successful
  corrective-action confirmation.
- A prospective protocol/specification revision is required to change a limit.
  Limits may never change after results are seen to convert a failure to pass.

Minimum restart scope:

| Change/failure | Required restart |
| --- | --- |
| Instrument/calibration failure | Repair/calibrate, repeat controls, and repeat all measurements since the last known-good check. |
| Harness/connector replacement | Topology, prime, bubble, installed leak, flow, dye recovery, and waste stages. |
| Gasket/closure/fastener/insert rework | Incoming inspection, compression, integrity, prime, and flow; endurance resets. |
| Surrogate replacement | Independent characterization, fit, nominal baseline, and affected challenges. |
| Fault coupon replacement | Independent characterization and every affected placement challenge. |
| Fluid/dye/temperature/pump profile change | New configuration and every affected wet stage. |
| Drawing/material/geometry change | New configuration and full protocol from HP-0. |
| Calculation software change | Preserve raw data, version the new calculation, recompute affected results, and obtain independent review. |
| Leak failure | Requalify integrity before flow, recovery, waste, cleaning, or endurance evidence is accepted. |

Mandatory gates, one-condition semantics, pressure safety, raw-data retention,
and independent review cannot be waived by deviation.

## 12. Data And Record Requirements

Use every applicable companion data sheet. Each page carries protocol revision,
run ID, cassette serial, operator initials, reviewer initials, and page number.

The final searchable record contains:

- Protocol, drawing, CAD, software, firmware, and calculation revisions.
- Source commit and file manifest.
- All cassette, harness, gasket, tubing, connector, surrogate, coupon, fluid,
  dye, waste, equipment, and material identities/lots.
- Calibration certificates, uncertainty budgets, and as-found/as-left checks.
- Signed parameter-freeze sheet and hold-point approvals.
- Completed raw sheets, unfiltered native traces, derived calculations,
  photo/video index, and SHA-256 file hashes.
- Every aborted run, deviation, invalidation, failure, rejected part,
  corrective action, and retest.
- Independent gate matrix with no aggregate score.
- Explicit claim level and limitations.

Final disposition is one of:

- PASS — eligible for media-only planning for the stated configuration.
- HOLD — unresolved evidence or corrective action.
- REJECT — required gate failed or configuration withdrawn.

## 13. External Measurement And Quality Basis

These sources inform method control; they do not supply LaminarForge's product
acceptance limits:

| Source | Use in this protocol |
| --- | --- |
| ISO 10012:2026, Quality management — Requirements for measurement management systems, https://www.iso.org/standard/10012 | Controlled measuring equipment and measurement processes. |
| ISO/IEC 17025:2017, https://www.iso.org/standard/66912.html | Calibration/test competence, method control, records, and traceability principles. |
| NIST TN 2156, https://nvlpubs.nist.gov/nistpubs/TechnicalNotes/NIST.TN.2156.pdf | Metrological traceability belongs to the measurement result and documented chain. |
| JCGM 100:2008, https://www.bipm.org/documents/20126/2071204/JCGM_100_2008_E.pdf | Measurement-uncertainty evaluation and reporting. |
| ISO 14253-1:2017, https://www.iso.org/standard/70137.html and ISO/IEC Guide 98-4:2012, https://www.iso.org/standard/50465.html | ISO 14253-1 informs GPS/dimensional conformity only. Guide 98-4 plus the documented LaminarForge rule informs hydraulic and analytical decisions. |
| NIST liquid micro-flow standard, https://www.nist.gov/laboratories/tools-instruments/gravimetric-standard-liquid-micro-flow | Gravimetric micro-flow practice, temperature/density control, and evaporation suppression. |
| IEC 60068-3-6:2018, https://webstore.iec.ch/en/publication/29226 and IEC 60068-3-11:2007, https://webstore.iec.ch/en/publication/567 | Environmental chamber mapping and uncertainty concepts for extension E1. |
| ASTM G121-18, https://store.astm.org/g0121-18.html; ASTM G122-20, https://store.astm.org/g0122-20.html; ASTM E3106-22, https://store.astm.org/e3106-22.html | Informative coupon, cleaning-method-development, and lifecycle concepts for E2; no claimed conformance or material-compatibility qualification. Verify active editions at execution freeze. |
| ASTM E2930-13(2021), https://store.astm.org/e2930-13r21.html | Informative boundary check only: its gas/nondeformable-vessel scope does not match this liquid, compliant system. |
| FDA Process Validation guidance, https://www.fda.gov/files/drugs/published/Process-Validation--General-Principles-and-Practices.pdf; Data Integrity guidance, https://www.fda.gov/media/119267/download; OOS guidance, https://www.fda.gov/media/158416/download | Conservative protocol, raw-data, deviation, investigation, and retest practices only; these documents do not make this engineering bench a regulated CGMP process. |

ASTM E2930-13(2021) describes gas pressure-decay testing of nondeformable vessels.
The compliant tubing/gasket cassette uses dyed liquid and does not claim
E2930 conformity. Its 5% decay, flow CV, recovery, sample counts, endurance
cycles, environmental bands, and carryover limits are LaminarForge engineering
requirements frozen before testing.

## 14. Gate Summary

| Gate | Minimum pass condition |
| --- | --- |
| HP-0 | Every required input, limit, method, sample, uncertainty, and record field is prospectively frozen. |
| HP-1 | Functional fixture, coupons, controls, relief, containment, and measurement systems are qualified; production-intent leak/burst coupons pass. |
| HP-2 | Released configuration, FAI, incoming metrology, identities, and dry condition pass. |
| HP-3 | Fit, datum, dock, compression, closure, and topology pass. |
| HP-4 | Three prime/bubble preparations, liquid integrity, and three nominal-flow intervals in each preparation (nine initial intervals total) pass independently. |
| HP-5 | All sixteen fault placements, three recovery runs, and every waste state pass. |
| HP-6 | Cycles 1, 5, 10, and 25 and final inspection pass without replacement. |
| E1/E2 | Required extension limits and post-exposure hydraulic/mechanical gates pass when in scope. |
| HP-7 | Independent reviewer confirms complete raw evidence, closed deviations, correct claim level, and no aggregate scoring. |

No later gate can compensate for an earlier failure.
