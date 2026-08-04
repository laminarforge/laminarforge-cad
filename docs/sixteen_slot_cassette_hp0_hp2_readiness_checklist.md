# 16-Slot Cassette HP-0 Through HP-2 Readiness Checklist

Checklist ID: LF-CAS-A7-RC001
Revision: A0
Ticket: T-BE6BF5A7
Protocol: LF-CAS-A7-P001 revision A1
Forms: LF-CAS-A7-P001-F revision A1
Planning baseline: artifact A-8E1805D4
Parent repository baseline reviewed: fc515b3
Controlled release commit: use the Git commit containing this revision and
record it in the executed release manifest; fc515b3 is not this document's
release identity.

## Purpose And Present Disposition

This checklist converts prerequisite hold points HP-0, HP-1, and HP-2 into an
owned readiness-control package for the first physical 16-slot, 4 x 4 cassette.
It identifies the predecessor for every action, the evidence that must exist,
and the minimum equipment and hardware that must be selected, acquired or
fabricated, received, calibrated, and commissioned.

As of the repository baseline above, all three hold points are NOT READY:

| Gate | Present state | Blocking facts |
| --- | --- | --- |
| HP-0 | NOT READY | D0-D9 are not released; D2/D4 gasket-groove entry/corner geometry and the actual-lot D7 stack remain open; no named independent approvers, approved equipment models, frozen pressure hierarchy, or executed DS-00/01/DS-02A1/A2 records exist. |
| HP-1 | NOT READY | Blocked by HP-0 and safety approval. The fixture CAD is a dry layout mockup. Its nominal/fault tokens are solid, it has no functional fault passages or quantitative dye/hold-up station, and no instrument, control, coupon, or pressure rig has executed qualification evidence. |
| HP-2 | NOT READY | Blocked by drawing release and physical parts. No released vendor drawing package, supplier FAI, accepted custom-part set, or independent incoming-metrology record exists. |

The repository does not contain an approved cassette-specific bought-component
BOM or an on-hand inventory record. In this checklist, UNVERIFIED GAP means
that the item must either be procured or be positively matched to existing
inventory with the required model, serial, range, rating, calibration,
uncertainty, and commissioning evidence. It does not assert that the item is
physically absent.

This is a readiness checklist, not executed test evidence and not a QMS by
itself. The executed DS forms remain the gate record.

## Non-Negotiable Gate Logic

| Authorization | Required predecessors |
| --- | --- |
| Order a full-size carrier/lid/dock set | D2/D4 manufacturing geometry closed, D7 worst-case stack closed, and controlled D0-D9 package released. |
| Wet-commission the empty bench/control loops | HP-0 PASS plus separate limited SAF-E01 test-specific authorization. |
| Wet a functional or gasket coupon | SAF-E01 remains valid, the as-built safety functions in HP1-04 pass, and SR/QR issue SAF-E02 for that exact rig and scope. |
| Wet the assembled cassette | HP-0 PASS, HP-1 PASS, and HP-2 PASS. |
| Accept later test evidence | Every predecessor gate remains valid for the same configuration; a later pass cannot erase an earlier failure. |

An unset value, unnamed required owner, missing evidence identity, expired
calibration, or unapproved substitution is NOT READY. It is not a deviation.
The checklist-row state uses the protocol dispositions; every row is initially
NOT READY until its evidence and approval close.

## Owners And Required Independence

Role codes in the tables below are mandatory accountable owners, not optional
review suggestions.

| Code | Accountable role | Named assignee now | Scope | Independence rule |
| --- | --- | --- | --- | --- |
| PR | Program owner | Alex Lewis | Budget, schedule, appointments, and stop-work enforcement. | May also hold DA/SL, but may not replace required independent SR or QR approval. |
| DA | Design authority | Alex Lewis, provisional until delegated in writing | Released geometry, intended use, MOP basis, configuration, D0-D9, and changes. | Cannot unilaterally disposition metrology or safety failures. |
| SL | Sourcing/build lead | Alex Lewis, provisional until delegated in writing | Supplier RFI/RFQ, purchase control, chain of custody, certificates, and substitutions. | Supplier self-inspection does not replace ML incoming inspection. |
| GD | Mechanical/GD&T owner | UNASSIGNED — blocker | Drawing tolerances, D7 stack, D8 feature scheme, and drawing/CAD agreement. | Must be competent for large precision plate and seal-interface definition. |
| ML | Metrology lead | UNASSIGNED — blocker | Methods, uncertainty, calibration, input metrology, FAI audit, and conformity decisions. | Must independently review supplier results. |
| FV | Fluidics validation lead | UNASSIGNED — blocker | Functional fixture, harness, frozen stimuli, commissioning, coupons, and raw hydraulic data. | May execute tests but cannot independently release own deviations. |
| TO | Test operator | UNASSIGNED — blocker before execution | Approved sequence, chain of custody, and contemporaneous records. | Cannot self-approve deviations, invalidations, changed limits, or release. |
| SR | Safety reviewer | UNASSIGNED — blocker | Pressure boundaries, relief, shield, spill, electrical separation, PPE, and safe depressurization. | Must be independent of the rig designer and operator for the safety evidence approved; may not author, calculate, execute, or technically own that evidence. |
| QR | Quality/release reviewer | UNASSIGNED — blocker | Hold points, N/A decisions, deviations, invalidations, retests, and final gate disposition. | Must be independent of every evidence owner, author, calculator, and operator for the evidence approved; may own the gate decision but not its technical inputs. |
| RC | Records custodian | UNASSIGNED — blocker | Immutable executed-record location, access control, retention, native files, hashes, and manifest completeness. | May not silently replace or overwrite an approved evidence object. |
| GA | Gasket applications reviewer | UNSELECTED supplier or independent seal engineer — blocker | Compound, tolerance, groove fill, entry/corner geometry, splice, compression set, and compatibility. | Supplier compound paperwork alone is insufficient. |
| SQ | Supplier-quality contact | UNASSIGNED until machining source is selected | Ballooned FAI, certificates, NCRs, and no-substitution control. | Vendor FAI and independent incoming results remain separate records. |

PR must record each appointment, competency basis, conflicts, signature method,
and effective date in HP0-E01. One person may hold multiple execution roles only
when the independence rules above remain satisfied. In every checklist row, the
Owner column names technical evidence owners. SR and QR are required reviewers
where stated in the action or evidence package; their review is not evidence
authorship.

## Evidence Control

Use one immutable evidence object per evidence ID and a row-specific disposition
entry for every checklist row. Multiple rows may cite one controlled package,
but no row may be closed by an unidentified aggregate folder. The recommended
record name is:

LF-CAS-A7-{RUN-OR-BUILD}-{GATE}-{EVIDENCE-ID}-R{REVISION}

Every evidence object must include its owner, reviewer, creation timestamp and
time zone, configuration/serial/lot scope, immutable location, and status. A
detached one-way manifest or sidecar created after signing must record the
object's SHA-256; the object must never contain or be modified to add its own
hash. A mutable working file or an unhashed aggregate folder cannot close a row.

| Evidence ID | Required package |
| --- | --- |
| HP0-E01 | Named-role and independence record. |
| HP0-E02 | Controlled D0-D9 release-candidate review followed by the final released index, editable-source identities, STEP/release manifests, source commit, hashes, verifier outputs, and signed independent release review. |
| HP0-E03 | Provisional then final configuration/BOM registry for cassette, dock, closure, gasket, harness, fixture, coupons, software, calculation, slot map, and G/M/W port map. |
| HP0-E04 | Two immutable input-metrology snapshots: preliminary measurements of all intended Rev C chips/spares and candidate gasket/closure/datum inputs, followed by final selected-lot dowel, screw, receiver, and gasket measurements. |
| HP0-E05 | Signed actual-lot D7 worst-case stack and witness map. |
| HP0-E06 | Controlled release candidate followed by final released functional-surrogate/fault-coupon, 20/25/30% squeeze-ladder, isolated-leak, destructive-burst, and reassembly-coupon drawings/material specifications; fault-manifold, dye/hold-up station, and fixture drawings/P&ID/BOM; hydraulic-equivalence analysis and reference nodes; blind key; and executed dry proof of all 16 non-disturbing selections. |
| HP0-E07 | Condition/fluid/chemistry, SDS, compatibility, PPE, disposal, and extension-scope package. |
| HP0-E08 | Provisional pressure/range procurement envelope followed by the exact selected-component rating matrix, final MOP basis, proof/burst hierarchy, relief tolerances, hard caps, overshoot, abort, and safe-depressurization calculations. |
| HP0-E09 | Provisional performance requirements followed by the final selected-system hydraulic, prime, bubble, leak, perimeter, waste, dye/hold-up, closure, and endurance methods and limits. |
| HP0-E10 | Immutable DS-02A1 downselection and DS-02A2 final-capability snapshots, calibration plan, measurement uncertainty budgets, and conformity rules. |
| HP0-E11 | Raw-record naming, native-file retention, immutable storage, access, retention, clock, and hash plan. |
| HP0-E12 | Detached signed HP-0 gate manifest referencing signed DS-00/01 and immutable HP-0 DS-02A1/A2 sidecar hashes one way. It grants no wet-work authorization and is never written back into those source records. |
| SAF-E01 | Separate SR/QR limited authorization for the exact as-designed bench, fluid, pressure boundary, and wet commissioning scope after HP-0; references HP0-E08/12 and expires on any configuration change. |
| SAF-E02 | Separate SR/QR as-built authorization for the exact functional/gasket-coupon rig and scope after HP1-04; records dry inspection, ratings, relief, hard cap, isolation, shield, containment, vent/drain, emergency stop, and permitted wet limits. |
| HP1-E01 | As-built bench P&ID/BOM, pressure boundaries, port map, software/firmware, and commissioning report. |
| HP1-E02A | Immutable DS-02B pre-SAF instrument and safety commissioning snapshot. |
| HP1-E02B | Immutable DS-02C post-SAF bubble, dye, and evaporation method-control snapshot. |
| HP1-E03 | Functional article inventory and characterization for all 32 nominal/fault coupons plus the separately released, limited incoming-acceptance record for the actual gasket lot and production-intent gasket coupons used in HP-1. |
| HP1-E04 | Sixteen-selection fault-manifold equivalence, concealment, actuation, and non-disturbance qualification. |
| HP1-E05 | Bubble-injection and optical detection qualification using immutable DS-07A pre-P1 method checks 1-3 only: upstream, representative row-high-point, and chip-inlet witness. Later DS-07B checks 4-6 remain campaign evidence. |
| HP1-E06 | Sealed blank, characterized artificial-leak control, and leak-method resolution qualification. |
| HP1-E07 | Quantitative dye/mass-closure and physical hold-up method validation. |
| HP1-E08 | Actual-lot D7 confirmation, squeeze ladder, isolated leak, destructive burst, and cycle 1/5/10/25 reassembly-coupon records. |
| HP1-E09 | Detached signed HP-1 gate manifest referencing HP0-E12, SAF-E01/02, HP1-E02A/B, the immutable DS-02B/DS-02C/DS-03A/DS-05A/DS-06A/DS-07A sidecar hashes, DS-08 method/coupon records, DS-10A/B, exact configuration identity, and every immutable attachment one way. It is never written back into source records. |
| HP2-E01 | Vendor ballooned FAI and raw data keyed to D8 feature IDs. |
| HP2-E02 | Independent incoming ballooned inspection and uncertainty record keyed to the same D8 IDs. |
| HP2-E03 | One-row-per-item identity, revision, lot, certificate, calibration, and chain-of-custody reconciliation. |
| HP2-E04 | Separate immutable DS-04A pre-hardware and DS-04B post-hardware/incoming-fit visual, cleanliness, fit, datum, dock-twist, and visibility evidence with predecessor hashes. |
| HP2-E05 | Detached signed HP-2 gate manifest referencing HP0-E12, immutable DS-03B/DS-04A/DS-04B sidecar hashes, D8, vendor FAI, independent incoming records, and exact configuration identity one way. It is never written back into source records. |

## HP-0 Checklist — Controlled Inputs Frozen

| ID | Required action and exact exit condition | Owner | Dependencies | Required evidence | Initial disposition |
| --- | --- | --- | --- | --- | --- |
| HP0-01 | Appoint every role above by name; document competency and independence. No UNASSIGNED required role remains. | PR | None | HP0-E01 | NOT READY |
| HP0-02 | Correct D2/D4 groove entry and corner geometry and issue a controlled D0-D9 release candidate with all seven STEP/reference files, draft manifests, and no-biological-claim notes. It is not orderable until HP0-15B. | DA, GD, GA | HP0-01; preliminary input measurements from HP0-04 | HP0-E02 release-candidate review | NOT READY |
| HP0-03 | Create a provisional serialized configuration registry covering cassette, carrier, lid, window, bulkhead, dock, gasket, sixteen M4 closures/receivers, D1/D2 pins, harness, surrogates, coupons, fixture, software, calculations, slot map, and G/M/W map. Final freeze occurs in HP0-15B. | DA | HP0-02 | HP0-E03 provisional registry; DS-00 draft | NOT READY |
| HP0-04 | Measure 100% of the 16 intended Rev C chips plus every declared spare and representative candidate gasket, 6 mm dowel, M4 closure, and receiver inputs to support the release candidate and procurement envelopes. Preserve the preliminary snapshot; it does not replace selected-lot metrology. | ML, GD | HP0-01; contracted or on-hand input-metrology method with current certificates and preliminary approved U | HP0-E04 preliminary snapshot; DS-00 draft | NOT READY |
| HP0-04A | After downselection, measure the exact intended gasket lot, dowels, screws, and receivers and reconcile manufacturer tolerances. The final pocket target uses measured Rev C lot maximum plus 0.80 mm per side while preserving at least 0.50 mm per-side clearance after tolerance. | ML, GD | HP0-04/15; exact selected lots/articles received | HP0-E04 final selected-lot snapshot | NOT READY |
| HP0-05 | At every witness, calculate guard-banded compressed height h_comp from the complete final selected actual-lot stack and prove 1.68 mm <= h_comp <= 1.92 mm. Separately prove 20% <= (h_free - h_comp) / h_free <= 30% using measured gasket free height and its uncertainty. Include worst-case flatness, coating, seating, stop/land height, lid deflection, and measurement uncertainty. | GD, ML, DA | HP0-02; HP0-04A | HP0-E05 | NOT READY |
| HP0-06 | Issue controlled release candidates for the functional surrogate/fault-coupon and gasket squeeze-ladder/leak/burst/reassembly-coupon drawings/material specifications plus the fault-manifold, dye/hold-up station, and fixture drawings/P&ID/BOM. Define hydraulic reference nodes/equivalence method/U and blind key, then execute dry proof that all sixteen selections change without opening/retorquing the cassette, demating the harness, changing source condition, changing closure count, or revealing the key. Final release occurs in HP0-15A; independent QR reviews without originating or executing the package. | FV, DA, ML | HP0-01; controlled dry prototype/manifold and surrogate interfaces available | HP0-E06 release-candidate/dry-proof record; DS-01 draft | NOT READY |
| HP0-07 | Assign one nonbiological condition ID and define provisional fluid recipe, density, viscosity, dye identity/concentration, temperature, batch method, SDS, compatibility, PPE, disposal, and spill-response requirements. Final selected-material safety approval occurs in HP0-15A. | FV | Candidate wetted-material architecture | HP0-E07 provisional package; DS-00/01 draft | NOT READY |
| HP0-08 | Define the provisional engineering envelope before downselection: intended MOP basis, installed-proof/Pgasket/Pburst formulas, maximum ceilings, target tolerances, hard-cap/overshoot/abort requirements, temperature derating, maximum releasable inventory, and nominal pressure/flow/mass ranges. These are procurement requirements, not final approved settings. | DA, FV, ML | HP0-06/07; intended claim | HP0-E08 provisional envelope; DS-01 draft | NOT READY |
| HP0-09 | Define provisional hydraulic and prime performance: control mode, command/ramp ranges, flow bands, stabilization, DAQ rate, pressure-drift logic, measured-input method, collection interval, prime volume/time/recovery, and lane/row/total decision rules. | FV, ML | HP0-08 | HP0-E09 provisional method requirements | NOT READY |
| HP0-10 | Define provisional bubble-method requirements: delivered amount/volume basis at stated pressure and temperature, target U, common/R1-R4 injection points, smallest prohibited bubble, optical detection limit, and clearing time/volume ranges. | FV, ML | HP0-08/09 | HP0-E09 provisional method requirements | NOT READY |
| HP0-11 | Define provisional leak/perimeter requirements: isolation boundary, Veff/residual-gas/compliance limits, equilibration and acquisition windows, matched blank, artificial-leak magnitude, visual threshold, perimeter stimulus, temperature allowance, and U target. SR independently reviews the prospective pressure-boundary requirements. | FV, ML | HP0-08 | HP0-E09 provisional method requirements | NOT READY |
| HP0-12 | Define provisional waste requirements: low/nominal/high elevations, reverse head/pressure, dwell, siphon limit, W4 stimulus/recovery, escaped-volume limit, destination segregation, and containment capacity >=110% of maximum releasable inventory. SR independently reviews the prospective safety requirements. | FV, ML | HP0-08/09 | HP0-E09 provisional method requirements | NOT READY |
| HP0-13 | Define provisional quantitative dye/mass-closure requirements: calibration range, blanks/checks, recovery, LOD/LOQ, residual method, carryover-blank map, tracer-clear map, one-chip dose, condition volume, and hold-up limit = min(one chip dose, 10% of condition volume). | FV, ML | HP0-08/09 | HP0-E09/10 provisional method requirements | NOT READY |
| HP0-14 | Define preliminary closure and service-life requirements: M4 torque/tolerance range, receiver/insert process, engagement, settling, witness map, gasket reuse, harness reuse, and connector cycle rating. | DA, GD, ML | HP0-02/04; candidate supplier records | HP0-E03/09 provisional requirements | NOT READY |
| HP0-15 | Downselect every mandatory bought item and contracted capability in the gap registers using the provisional envelopes from HP0-08 through HP0-14. Select custom suppliers by capability only; do not release the full-size custom-part order. Immutable DS-02A1 identifies manufacturer/model or controlled procurement identity, range, capacity, resolution, rating, software, calibration plan, and projected uncertainty. SR independently reviews all safety-critical selections. | ML, FV, SL | HP0-08 through HP0-14 | HP0-E10; immutable DS-02A1 downselection snapshot | NOT READY |
| HP0-15A | Using the exact selections and selected-lot metrology, close D7; finalize and release the HP0-E06 article/fixture package; complete the installed-component rating matrix and selected-equipment capability checks; resolve every mismatch; and freeze final HP0-07 through HP0-14 values, methods, limits, guard bands, calibration plan, and uncertainty budgets. Final ranges must avoid endpoint operation/saturation and support U <=25% of the nearest tolerance band where practical. Preserve DS-02A1 and issue a new immutable DS-02A2 capability snapshot; SR and independent QR review but do not author the technical package. | DA, FV, ML, GD | HP0-04A/05/15 | Final HP0-E05/06/07/08/09/10; final DS-01 and immutable DS-02A2 | NOT READY |
| HP0-15B | Update the release candidate for every exact selection and final D7 result; independently release D0-D9, all seven STEP/reference files, final BOM/configuration registry, manifests, hashes, verifier outputs, D8 inspection plan, and no-biological-claim notes. CAD, drawings, BOM, D7, and D8 agree before any full-size custom-part order. | DA, GD, GA | HP0-15A | Final HP0-E02/03/05 | NOT READY |
| HP0-16 | Freeze sample/claim plan: FA-1 minimum uses one matched cassette set; one reconnect-rated harness or four single-use harnesses; 16 nominal surrogates; four each low/high/blocked/bypass; and the full gasket-coupon set. | DA, FV | HP0-15B connector reuse decision; independent QR review | HP0-E03; DS-00 | NOT READY |
| HP0-17 | Freeze uncertainty budgets, covariance treatment, standard-u/expanded-U convention, coverage, guard bands, the conformity rule, and the rule for investigating supplier-versus-independent measurements that differ beyond combined uncertainty even when each independently conforms. Independent QR reviews without authoring or calculating the budgets. | ML | HP0-15A selected methods and instruments | HP0-E10; DS-01 | NOT READY |
| HP0-18 | Freeze native/raw file names, immutable location, access, retention, clock alignment, export, SHA-256, corrections, and one-file-per-manifest-row rules. Independent QR verifies the record-control result. | RC | RC appointed | HP0-E11; DS-00/01 | NOT READY |
| HP0-19 | Declare E1 and E2 in or out and document the technical basis. SR and independent QR prospectively approve every N/A. If E1 is in, add its 16 slot probes and humid-soak coupon; if hazardous/flammable E2 chemistry is in, approve a separate SOP and risk assessment. | DA, FV | Intended claim | HP0-E07/09 | NOT READY |
| HP0-20 | SR approves the prospective safety design and limits. DA, FV, ML, SR, RC, and independent QR sign HP-0 only after every row, DS-00/01, and immutable HP-0 DS-02A1/A2 snapshots are complete. HP-0 itself grants no wet-work authorization; SR/QR issue SAF-E01 separately for the exact limited empty-bench commissioning setup. | QR gate approver; SR safety approver | HP0-01 through HP0-19, including HP0-04A/15A/15B | HP0-E12; later SAF-E01 | NOT READY |

## HP-1 Checklist — Functional Bench And Coupons Qualified

HP-1 construction and dry checks begin only after HP-0. SAF-E01 permits only
the bounded empty-bench commissioning it names; no functional or gasket coupon
may be wetted until HP1-04 closes and SAF-E02 is signed for the as-built rig.

| ID | Required action and exact exit condition | Owner | Dependencies | Required evidence | Initial disposition |
| --- | --- | --- | --- | --- | --- |
| HP1-01 | Fabricate, receive, serialize, and pre-inspect 16 functional nominal surrogates plus four each low, high, blocked, and bypass coupons. Solid CAD tokens do not qualify. | FV, ML | HP-0; HP0-E06 released coupon drawings | HP1-E03; immutable DS-03A HP-1 article-qualification snapshot; DS-10A/B | NOT READY |
| HP1-01A | Before any HP-1 wet qualification, independently inspect and release the actual gasket material lot, squeeze ladder, isolated leak coupon, destructive burst coupon, and reassembly coupon against the controlled HP0-E06 drawings, wetted materials, critical dimensions, finish, stop plane, and traceability. Every critical passes; failures are reworked/reinspected or trigger prospective design revision. This limited article release does not replace full HP-2 incoming inspection of the cassette set. | ML | HP-0; HP0-E06; received gasket/coupon lot | HP1-E03; immutable DS-03A HP-1 article-qualification snapshot | NOT READY |
| HP1-02 | Build and dry-commission one external fault manifold. All 16 concealed selections actuate without disturbing cassette, gasket, harness, source, or closure count. Hydraulic equivalence is demonstrated later under SAF-E02 in HP1-07. Independent QR reviews but does not own or execute the evidence. | FV, ML | HP1-01; HP0-E06 | HP1-E04; DS-06A/10 | NOT READY |
| HP1-03 | Build the functional bench with six simultaneous pressure channels, sixteen collection positions, measured input, bubble injection, dye/hold-up, waste challenge, leak witness, relief, and containment. As-built P&ID and BOM match DS-06A. SR independently reviews the design. | FV | HP-0 equipment receipt | HP1-E01; DS-02B/06A | NOT READY |
| HP1-04 | Under SAF-E01, inspect and commission the empty bench at the exact bounded condition it authorizes: source limit, relief tolerance, calibrated rig pressure/peak capture, isolation/no-active-makeup valve, vent/drain, emergency stop, shield, spill/electrical controls, and both containments. After immutable DS-02B records every dry/as-built and permitted empty-bench check as PASS, independent SR and QR issue SAF-E02 before any functional or gasket coupon is wetted. | FV, ML | HP1-03; HP-0 PASS; SAF-E01 | HP1-E02A; immutable DS-02B; later SAF-E02 | NOT READY |
| HP1-05 | Calibrate/check all six pressure channels, reference, DAQ/clock, balance/check weights, input reference, fluid/ambient temperature, torque driver, and dimensional methods. Verify bubble-injector and dye-instrument identities, certificates, ranges, and readiness only; executed bubble/dye method controls wait for DS-02C after SAF-E02. | ML | HP1-03; equipment installed | HP1-E02A; immutable DS-02B | NOT READY |
| HP1-06 | Characterize each of the 32 physical nominal/fault coupons with at least three steady-state readings at the frozen condition. Flowing coupons have finite bands/U; blocked coupons have one-sided flow/resistance and pressure-signature limits. | FV, ML | HP1-01/03/04/05; SAF-E02 | HP1-E03; DS-10A/B | NOT READY |
| HP1-07 | Demonstrate coupon-class separation >=3 combined standard uncertainties and qualify manifold actuation/hydraulic equivalence at its approved bench reference nodes. Actual cassette 16/16 concealed fault localization remains A7-8 campaign evidence after HP-4 and cannot be claimed by HP-1. Independent QR reviews but does not own or execute the evidence. | FV, ML | HP1-02/04/06; SAF-E02 | HP1-E03/04; DS-06A/10A/B | NOT READY |
| HP1-08 | Qualify the bubble method using the smallest prohibited bubble at representative upstream, row-high-point, and chip-inlet witness positions under representative lighting and pressure. Immutable DS-02C records delivered-volume controls and DS-07A pre-P1 checks 1-3 pass; DS-07B checks 4-6 remain later campaign evidence. Independent QR reviews the three-of-three disposition. | FV, ML | HP1-03/04/05; SAF-E02 | HP1-E02B/05; DS-02C and DS-07A checks 1-3 | NOT READY |
| HP1-09 | Qualify the matched sealed blank and characterized artificial-leak control. Resolution reaches the smallest rejectable leak, controls bracket the qualification, and the positive control stays isolated from release hardware. Independent QR reviews without owning or executing the controls. | ML, FV | HP1-03/04/05; SAF-E02 | HP1-E06; dedicated DS-08 qualification records | NOT READY |
| HP1-10 | Validate the quantitative dye/mass-closure and physical hold-up station with blank, low/mid/high standards, independent check, recovery, LOD/LOQ, mutually exclusive fractions, and residual method. | FV, ML | HP1-03/04/05; SAF-E02 | HP1-E02B/07; immutable DS-02C method-control snapshot | NOT READY |
| HP1-11 | Confirm actual-lot D7 at every witness and measure the production-intent 20/25/30% squeeze ladder, keeping height and squeeze uncertainties separate. | GD, ML, DA | HP0-E05; HP1-01A | HP1-E08; immutable DS-05A | NOT READY |
| HP1-12 | On the non-destructive rig, pass the isolated production-intent gasket coupon at Pgasket: <=5% decay and no visible dye migration. SR independently approves the authorized rig and limits. | FV, ML | HP1-01A/04/05/09/11; SAF-E02 | HP1-E08; DS-08 coupon/method qualification | NOT READY |
| HP1-13 | On the separate shielded destructive rig, reach the frozen burst target without exceeding ceiling, overshoot, relief, or abort limits. Record survival as a lower bound; do not force failure beyond a rating. SR independently approves the authorized rig and limits. | FV, ML | HP1-01A/04/05/11; SAF-E02 | HP1-E08; DS-08 destructive burst record | NOT READY |
| HP1-14 | Run the same gasket reassembly coupon through closure/reassembly cycles 1, 5, 10, and 25. Each dimensional, closure, damage, and isolated liquid-hold checkpoint passes without adjustment. Independent QR reviews the completed sequence. | FV, ML | HP1-01A/12/13; SAF-E02 | HP1-E08; DS-05A squeeze ladder and DS-08 reassembly records | NOT READY |
| HP1-15 | Independent QR reconciles the exact scoped records in HP1-E09, all immutable attachments, controls, deviations, and retests without pulling later HP-3/P1-P3 blanks into this gate. DA, ML, FV, SR, RC, and QR sign HP-1. | QR gate approver | HP1-01/01A through HP1-14 | HP1-E09 | NOT READY |

## HP-2 Checklist — Incoming Inspection And Metrology Passed

HP-2 evaluates the physical production-intent configuration before the HP-3
assembled closure/topology gate. Execute immutable DS-04A pre-hardware and
DS-04B post-hardware/incoming-fit snapshots; do not reuse either later as the
HP-3 assembled-state DS-04C snapshot.

| ID | Required action and exact exit condition | Owner | Dependencies | Required evidence | Initial disposition |
| --- | --- | --- | --- | --- | --- |
| HP2-01 | Receive a ballooned vendor FAI keyed to every required D8 feature ID, with raw data, calibration IDs, material/finish certificates, lots, and NCRs. Every critical characteristic passes the released drawing. A failure requires rework plus reinspection or a prospective HP-0/drawing revision; use-as-is cannot close HP-2. | SQ, SL, DA | Released D0-D9; selected supplier; physical parts | HP2-E01; immutable DS-03B HP-2 incoming snapshot | NOT READY |
| HP2-02 | Independently measure 100% of first-article custom parts and all repeated critical features using the D8 method/U. Vendor and independent results remain separate and each conforms independently. Any difference beyond the frozen combined-U discrepancy rule is investigated and resolved even when both results individually pass. | ML | HP2-01; calibrated metrology | HP2-E02; D8 | NOT READY |
| HP2-03 | Reconcile one row per part, gasket, fastener, receiver, dowel, window, label, harness, coupon, and instrument: identity, revision, material/lot, certificate, calibration, and chain of custody. Independent QR reviews without originating the reconciliation. | ML | HP2-01/02 | HP2-E03; DS-03B | NOT READY |
| HP2-04 | Inspect seal lands, grooves, stops, bores, ports, windows, labels, and dry structure before hardware installation. No burr, coating buildup, clogged port, residue, corrosion, crack, damage, or label conflict exists. | ML, DA | HP2-02 | HP2-E04; immutable DS-04A pre-hardware snapshot | NOT READY |
| HP2-05 | Install only the released receivers, pins, captive screws, window retention, and labels; repeat affected flatness, position, finish, and cleanliness checks afterward and link the pre-hardware predecessor hash. | ML, TO | HP2-02/04; released install process | HP2-E02/04; immutable DS-04B post-hardware snapshot | NOT READY |
| HP2-06 | Complete the S01-S16 census. All 16 uniquely identified functional surrogates seat and remove without forcing, binding, rocking, or damage and meet the released clearance/force rule. | ML | HP2-05; functional surrogates | HP2-E04; DS-04B | NOT READY |
| HP2-07 | Datum A support, rear/left/front contacts, D1 round locator, D2 relieved locator, and D3/D4 non-locating witness behavior meet released limits. | ML, GD | HP2-05 | HP2-E04; DS-04B | NOT READY |
| HP2-08 | Measure free and docked carrier state. Dock-induced twist/flatness delta meets the released D8 limit, with no hidden spacer or unrecorded datum offset. | ML, GD | HP2-07 | HP2-E04; DS-04B | NOT READY |
| HP2-09 | With the incoming configuration docked, verify optical, leak-witness, gutter, segmented drain/bridge, label, connector, and handling regions remain visible and unobstructed. Independent QR reviews the evidence without owning or executing it. | DA, ML | HP2-08 | HP2-E04; DS-04B/photos | NOT READY |
| HP2-10 | DA, ML, RC, and independent QR sign HP-2 only after supplier FAI, independent D8 inspection, identity reconciliation, cleanliness, 16/16 fit, datums, dock twist, and visibility all pass. | QR gate approver | HP2-01 through HP2-09 | HP2-E05 | NOT READY |

## Mandatory Equipment Gap Register

All rows are currently UNVERIFIED GAP. Final manufacturer/model and usable
range cannot be approved until the relevant HP-0 method and pressure/flow range
are frozen. Candidate names are not purchase authorization.

### Gap-register accountability

SL owns purchase/rental/contract records and receipt traceability for every
gap. The named technical role owns the specification and capability evidence;
SR/QR remain independent reviewers rather than originators. DS-02A1/A2 or DS-03
must name the individual owner for each item before it can leave UNVERIFIED GAP.

| Register rows | Technical owner | Required independent review |
| --- | --- | --- |
| EQ-01, EQ-04/05, EQ-07, EQ-09/10, EQ-13/14, EQ-16 | FV, with ML for measurement capability | SR for pressure/wet safety; QR for evidence release. |
| EQ-02/03, EQ-06/08, EQ-11/12, EQ-17/18/19 | ML, with FV or GD for the applicable interface | SR for any pressure/chemical boundary; QR for evidence release. |
| EQ-15 and EQ-20 | FV and ML | SR independently approves the rig/station; QR releases the evidence. |
| HW-01 through HW-04 and HW-19 | DA and GD | ML for measurable characteristics; QR for release. |
| HW-05 and HW-06 | DA, GD, and GA | ML for dimensional evidence, SR for compatibility/safe use, and QR for release. |
| HW-07 | ML and GD | DA for interface use; QR for release. |
| HW-08 through HW-18 | FV, with DA/GD for mechanical interfaces | ML for characterization/metrology, SR for wet/pressure safety, and QR for release. |

| ID | Minimum quantity | Exact minimum capability | Gate | Close action |
| --- | ---: | --- | --- | --- |
| EQ-01 | 1 | Pressure-limited pump/controller with command logging, physical or validated hard maximum, isolation, liquid-compatible relief, and safe vent/drain. Range covers nominal operation and installed proof without endpoint operation or saturation. | HP-0/1 | Select model and pressure/flow configuration; record rating, software, calibration, and U; receive and commission. |
| EQ-02 | 6 simultaneous channels | Common upstream, R1, R2, R3, R4, and waste/backpressure. Synchronized raw capture >=1 Hz; range supports the frozen tests away from endpoints. Add one synchronized ambient-pressure reference if absolute sensors are selected. | HP-0/1 | Select six sensor/channel assemblies and any ambient reference; calibrate as a system. |
| EQ-03 | 1 or a controlled set | Traceable pressure reference/check capability covering every installed, Pgasket, and destructive-rig pressure through the frozen Pburst/ceiling, supporting pre/post zero/span checks and the U budget. | HP-0/1 | Procure, rent, or contract calibrated reference capacity for all pressure ranges. |
| EQ-04 | 1 | Synchronized DAQ/logger for all six pressure channels, command/status and required temperature/ambient inputs; monotonic timestamps, native export, software/firmware identity, and hashes. | HP-0/1 | Select channel count/interface and validate clock/raw export. |
| EQ-05 | 1 | Independent measured-input reference: traceable source balance, calibrated positive-displacement delivery, or reference flow method. Pump command alone is prohibited. | HP-0/1 | Select method after nominal input range is frozen; calibrate. |
| EQ-06 | >=1 balance plus one check-weight set | Readability <=1/10 of the smallest expected net mass; expanded U <=25% of the nearest mass-related acceptance band where practical; low/mid/high traceable checks. | HP-0/1 | Freeze expected mass, select balance and weights, calibrate and check. |
| EQ-07 | 16 vessels + 16 nest positions + at least 3 blank vessels/placements | Sixteen labeled covered S01-S16 collection vessels in sixteen non-swappable nest positions plus at least three matched covered evaporation blanks placed simultaneously in representative locations. | HP-1 | Select vessel/nest/blank geometry, serialize, tare, and verify compatibility. |
| EQ-08 | 2 core probes | One fluid-temperature and one ambient-temperature probe with range, resolution, response, calibration, and synchronized timestamps suitable for density and long pressure holds. | HP-0/1 | Select, calibrate, and commission. |
| EQ-09 | 1 system, 5 selectable locations | Calibrated positive-displacement bubble injector or fixed-loop system serving common upstream and R1-R4; frozen delivered gas amount/volume, P/T correction, and U. | HP-0/1 | Design/select loops/tool, calibrate delivered volume, and verify each location. |
| EQ-10 | 1 fixed system | Time-linked camera, fixed lighting, scale/field reference, and visible upstream, row-high-point, chip-inlet, and outlet witness zones; supports the smallest prohibited bubble and leak threshold. | HP-0/1 | Select camera/lighting/mount, validate detection limit and timestamps. |
| EQ-11 | 1 | Quantitative absorbance/colorimetric instrument or another validated method with blank, low/mid/high standards, independent check, recovery, calibration range, LOD/LOQ, and raw export. Photographs alone do not qualify unless validated quantitatively. | HP-0/1 | Select method/instrument after dye and range are frozen; validate. |
| EQ-12 | 1 tool set | Calibrated mass/volume preparation tools, thermometer, density method, and viscosity method if viscosity-adjusted fluid is used. | HP-0/1 | Define recipe range; select/calibrate tools. |
| EQ-13 | 1 | Adjustable, measured low/nominal/high waste-elevation and reverse-head/backpressure station with W4 stimulus/capture, anti-siphon challenge, safe drain, and secondary containment. | HP-0/1 | Design/fabricate, measure ranges, and commission. |
| EQ-14 | 1 installed-system rig | Installed-system pressure boundary with hard cap/interlock, liquid relief, source-isolation/no-makeup valve, vent/drain, emergency stop, leak tray, electrical separation, and containment >=110% of frozen maximum inventory. | HP-0/1 | Complete rated P&ID/BOM and SR commissioning. |
| EQ-15 | 1 separate destructive rig | Separately rated small-volume coupon rig with pressure source, calibrated pressure measurement, synchronized peak/overshoot/relief capture, hard ceiling, relief, overshoot/abort control, remote operation or safe separation, shield, vent/drain, and containment. Its weakest rating exceeds every permitted target/peak. | HP-0/1 | Design/select independently from EQ-14; SR and ML approve and commission. |
| EQ-16 | 1 set | Matched sealed blank, one permanently marked characterized artificial-leak control at or below the smallest rejectable leak, clear dye tray/witness paper, and minimized/characterized trapped gas and liquid volumes. | HP-0/1 | Fabricate/characterize and bracket each campaign. |
| EQ-17 | Access to 1 qualified system | CMM/vision or equivalent working envelope that accommodates the 869.04 x 691.92 x 76.00 mm maximum part envelope plus required fixturing, probe travel, edge access, and orientation, or an approved combination of large surface plate/height gauge/indicator and other methods, with U adequate for every D8 characteristic. | HP-0/2 | Reserve external accredited capacity or document qualified in-house capacity. |
| EQ-18 | 1 qualified set | Large surface plate/indicator method where used; micrometer for gasket/groove/stop measurements; optical comparator or toolmaker microscope for gasket and groove geometry; profilometer resolving Ra 0.8 um target/1.6 um maximum; selected pin/plug/thread gauges; and any D8-specific fixtures. | HP-0/2 | Finalize from D8, calibrate, and prove measurement capability. |
| EQ-19 | 1 | Calibrated torque driver covering the selected M4 torque and tolerance. Torque records closure effort but never substitutes for hard-stop/witness evidence. | HP-0/1/2 | Select only after M4 hardware/receiver strategy; calibrate. |
| EQ-20 | 1 complete station set | Spill kit, required PPE, chemical labels/SDS access, electrical separation, approved waste/disposal containers, and safe cleanup/depressurization provisions. | HP-0/1 | SR inventories and signs readiness. |

E1-only equipment is not required for core HP-1: if E1 is approved in scope,
add 16 fixed slot-plane temperature/RH probes, one chamber reference, one ambient
probe, a synchronized logger, at least three matched evaporation blanks per run,
and the approved environmental chamber/module.

## Mandatory Physical Hardware And Article Gap Register

| ID | FA-1 minimum quantity | Exact first-article requirement | Gate | Present state |
| --- | ---: | --- | --- | --- |
| HW-01 | 1 matched set | One serialized production-intent carrier, lid/clamp, retained window, bulkhead, and dock manufactured to released D0-D9. | HP-2 | UNVERIFIED GAP; no released orderable package or accepted set. |
| HW-02 | 16 + 16 + 2 | Sixteen M4 x 0.7 captive positive-drive lid screws; sixteen released stainless receiver/insert/nut-plate assemblies at the locked 3.30 mm pilots; exactly two selected 6 mm-family ground dowels for D1/D2. D3/D4 remain pin-free. | HP-0/2 | UNVERIFIED GAP; family assumptions only, no approved SKU. |
| HW-03 | 6 when D5 mounting is used | Released M5 dock-mount fasteners for the six 5.40 mm holes, including retention/washer strategy and installation torque. | HP-0/2 | UNVERIFIED GAP; D5 selection open. |
| HW-04 | 1 complete set | Mechanically retained window hardware defined by D3; adhesive-only retention is not the baseline. Final piece count follows the released D3 BOM. | HP-0/2 | UNVERIFIED GAP; retention design open. |
| HW-05 | 1 assembly from 1 identified lot | Production-intent 2.40 mm nominal gasket assembly providing 16 continuous per-slot loops plus one perimeter loop, with supplier compound, durometer, tolerance, splice/join, compression-set, compatibility, and lot records. | HP-0/1/2 | UNVERIFIED GAP; platinum-cured silicone 50-60A and EPDM 60-70A are candidates only. |
| HW-06 | 1 set per gasket material and lot | One three-reference 20/25/30% squeeze ladder with nominal 1.92/1.80/1.68 mm heights, one isolated leak coupon, one separate destructive burst coupon, and one reassembly coupon. Every coupon duplicates the final 1.80 x 3.20 mm groove, seal finish, 7.35 mm closure-stop plane logic, gasket, and fastener style. Add one humid-soak coupon only if E1 is in scope. | HP-1 | UNVERIFIED GAP; production-intent coupon drawings/material absent. |
| HW-07 | 16 plus every declared spare | Intended Rev C chips for 100% input metrology; all spares must also be measured before the pocket drawing is released. Physical A7 testing uses surrogates, but this lot record controls the interface. | HP-0 | UNVERIFIED GAP; count of spares must be declared. |
| HW-08 | 16 | Uniquely identified functional flow-through Rev C-footprint nominal surrogates, one for S01-S16. | HP-1/2 | UNVERIFIED GAP; current CAD pieces are solid. |
| HW-09 | 4 + 4 + 4 + 4 | Four low-resistance, four high-resistance, four blocked, and four bypass functional coupons, enabling one challenge in every row and column. | HP-1 | UNVERIFIED GAP; current CAD pieces are solid/uncharacterized. |
| HW-10 | 1 | Externally accessible, blinded, non-disturbing fault-insertion manifold with all 16 selections and frozen reference nodes. | HP-0/1 | UNVERIFIED GAP; no functional design/qualification. |
| HW-10A | 1 fixture mechanical set | Cassette datum nest with direct datum-A support and B/C/front registration, functional six-channel pressure bar, 16-position collection deck, common/R1-R4 bubble station, leak tray, adjustable waste/backpressure station, and run-record identity board. | HP-1 | UNVERIFIED GAP; current outputs are dry placement envelopes, not functional hardware. |
| HW-11 | 1 reconnect-rated or 4 single-use | One full-core production-intent harness if every connector is rated/released for reconnect checkpoints, otherwise one full-core harness plus fresh harnesses at checkpoints 5, 10, and 25. | HP-0/1/2 | UNVERIFIED GAP; tubing, connector, pump segment, and vendor open. |
| HW-11A | 1 per harness/run | Common condition reservoir or bag with frozen usable capacity, material/lot, condition identity, vent/headspace configuration, working/proof rating where pressurized, connector interface, and maximum releasable inventory contribution. | HP-0/1/2 | UNVERIFIED GAP; no selected vessel or capacity. |
| HW-12 | 1 complete harness topology per harness | Reservoir lead, optional approved filter, pump segment, common trunk, four labeled row feeds, sixteen inlet pigtails, sixteen outlet pigtails, four row-waste branches, common waste, prime bypass, compatible connectors/caps, and visible witness access at upstream, R1-R4, every S01-S16 inlet, every S01-S16 outlet, and every vertical high point. Use a keyed tray/combs, incompatible connectors, physical segregation, or validated scan control; labels and the 12 placeholder comb teeth alone are insufficient. | HP-0/1/2 | UNVERIFIED GAP; exact lengths, OD/ID, connector counts/SKUs, mistake-proofing, and caps freeze in the harness BOM. |
| HW-13 | 16 logical service positions | Bought connector, fitting, or controlled cap/plug at each used/unused G0-G3, M0-M6, and W0-W4 logical position. Placeholder CAD holes are not connector drawings. | HP-0/1/2 | UNVERIFIED GAP; final panel geometry and active-port state open. |
| HW-14 | 1 | One matched sealed-fixture blank, separate from release hardware. | HP-1 | UNVERIFIED GAP. |
| HW-15 | 1 | One permanently marked, characterized artificial-leak control, isolated from release testing. | HP-1 | UNVERIFIED GAP. |
| HW-16 | 1 | One quantitative dye/mass-closure and physical hold-up station with mutually exclusive S01-S16, W0/W3, rinse, leak/drain, and residual fractions. | HP-1 | UNVERIFIED GAP; station absent from current CAD. |
| HW-17 | 16 vessels + 16 nests + at least 3 blanks | Sixteen covered labeled collection vessels in sixteen non-swappable S01-S16 nest positions plus at least three matched simultaneous evaporation-blank vessels/placements. | HP-1 | UNVERIFIED GAP. |
| HW-18 | 1 complete segregated destination set | Prospectively frozen destinations for W0 main waste, separate W1 prime/purge, separable W3 dye/leak fractions, and captured W4 relief/overflow, with identity labels, at least one released anti-backflow/anti-siphon device or equivalent architecture, and secondary containment. A single bottle qualifies only if a validated switching/reset and mass-ledger scheme preserves every required separation. | HP-0/1 | UNVERIFIED GAP; container count and capacities remain open. |
| HW-19 | 1 controlled label set | One global cassette/condition code, one human-readable identity, S01-S16 labels, R1-R4 labels, G/M/W labels, harness/waste IDs, orientation mark, and unused-port status labels, compatible with the selected cleaning/humidity exposure. | HP-0/2 | UNVERIFIED GAP; final stock/adhesive not selected. |

The older output/system_bom_v2.csv and generic assorted M3/M4/M5 kits are not
cassette release evidence. They belong to older system scopes and do not prove
the required topology, quantities, ratings, calibration, uncertainty, relief,
dead volume, captive retention, or D8 conformity.

Equipment rows define required capability; hardware rows define the physical
article inventory. The following are cross-references, not duplicate purchase
quantities: EQ-07/HW-17, EQ-09/HW-10A, EQ-13/HW-10A, EQ-16/HW-14/HW-15, and
EQ-20/HW-18.

## Decisions That Must Precede An Exact MPN

The planning artifact contains useful candidate families, but none is approved
for purchase. Freezing an MPN before its range and interface are known would
create false readiness.

| Selection | Candidate direction already identified | Required decision before MPN freeze |
| --- | --- | --- |
| M4 captive closure | Southco F5-class captive screw; KATO Tangless or Heli-Coil-class receiver | D7 clamp/engagement requirement, lid thickness/counterbore, installation process, coating treatment, torque, and service life. |
| Datum pins | MISUMI 6 mm ground-dowel family or equivalent | D1 bore tolerance, D2 relieved seat, exact projection/engagement, finish, and replaceability. |
| Gasket | Apple Rubber engineering review first; Marco Rubber/Trelleborg alternates | Approved compound/durometer, manufactured tolerance, corner/entry geometry, groove fill, splice, compression set, cleaning compatibility, and lot evidence. |
| Pump/controller | Compare Fluigent Flow EZ and Elveflow OB1 classes | Frozen MOP/proof, nominal flow, response, channel/control architecture, wetted boundary, hard-limit behavior, logging, and U. |
| Connectors/tubing | Small compatible MicroCNX/AseptiQuik-S-class handoffs and weldable TPE/C-Flex versus silicone route | Frozen OD/ID, volume budget, pressure/temperature rating, reconnect or single-use plan, dead volume, pump segment, and no-cell versus later aseptic claim. |
| Machining/metrology | Fictiv large-parts, Xometry custom CNC, and one regional large-format shop with independent metrology | Released D0-D9, confirmed 869.04 x 691.92 mm capacity, large-plate warpage/coating plan, D8 FAI, traceability, and passed production-intent coupons. |

Candidate-family availability was checked 2026-08-04 against the manufacturers'
current pages for [Southco F5](https://southco.com/en_any_int/fasteners/captive-screws/captive-screws/f5-flush-captive-screws),
[KATO Tangless](https://katofastening.com/inserts/tangless.html),
[MISUMI MST6-10](https://us.misumi-ec.com/vona2/detail/110302390350/?HissuCode=MST6-10),
[Apple Rubber non-round face seals](https://www.applerubber.com/seal-design-guide/special-elastomer-applications/face-seal-applications-non-round/),
[Fluigent Flow EZ](https://www.fluigent.com/research/instruments/pressure-flow-controllers/flow-ez/),
[Elveflow OB1](https://elveflow.com/microfluidic-products/microfluidics-flow-control-systems/ob1-pressure-controller/),
and [CPC MicroCNX](https://www.cpcworldwide.com/Biopharma/Products/Aseptic-Sterile-Connectors/MicroCNX).
This check confirms that the named families are current, not that any model has
passed the HP-0 range, rating, interface, uncertainty, or compatibility gates.

## Execution Order

1. PR appoints GD, ML, FV, TO, SR, QR, RC, GA, and later SQ.
2. ML measures Rev C chips/spares and representative candidate gasket/dowel/
   closure inputs while GD corrects D2/D4 and prepares the D0-D9 release
   candidate and provisional configuration registry.
3. SL may issue non-orderable capability RFIs while DA/FV/ML define provisional
   performance, pressure, range, closure, and measurement requirements.
4. In parallel, FV designs the functional fault manifold, all functional
   coupons, dye/hold-up station, and complete bench P&ID/BOM.
5. Downselect exact bought items and contracted capabilities; receive and
   measure the selected gasket/dowel/closure lots; close final D7; confirm
   selected ratings/capability; then freeze final methods, limits, uncertainty,
   and DS-01/DS-02A1/A2 records.
6. Update and independently release D0-D9 and the final configuration/BOM. Only
   then release a full custom-part order, complete HP-0, and obtain the separate
   SAF-E01 limited empty-bench commissioning approval.
7. Build HP-1 and dry-inspect its safety functions; perform only the bounded
   SAF-E01 empty-bench checks; issue SAF-E02 for the as-built rig; then execute
   instrument checks, limited gasket/coupon incoming release, 32 coupon
   characterizations, manifold equivalence, bubble/leak/dye controls, actual-lot
   D7 and squeeze ladder, isolated leak, separate burst, and reassembly cycles.
8. After physical parts arrive, keep vendor FAI distinct from independent
   incoming inspection. Install hardware only after pre-install criticals pass,
   repeat distortion-sensitive checks, and close HP-2.
9. Independent SR/QR issue SAF-E01 and SAF-E02 for their exact bounded scopes;
   independent QR releases assembled-cassette wet work only after HP-0/1/2.

## Failure And Change Response

| Event | Required response |
| --- | --- |
| Missing value, owner, model/range, rating, calibration, U, hash, or approval | Record NOT READY. Do not open a deviation merely to start work. |
| D7 inequality fails or a contributor is missing | Stop drawing/RFQ release and full-size ordering. Revise geometry, tolerance, material, coating, closure, or measurement method and rerun the complete stack. |
| Supplier proposes a substitute | Quarantine the change. DA/ML/SR review ratings/interfaces; revise HP-0 configuration and affected drawings. Repeat affected coupon and incoming gates. |
| Calibration/control fails or expires | Stop. Place every measurement since the last known-good check on HOLD; repair/calibrate, repeat controls, and repeat affected evidence. |
| Relief, hard cap, shield, containment, emergency stop, or safe vent fails | Stop and depressurize safely. No wet work resumes until SR approves corrective action and full affected commissioning is repeated. |
| Functional coupon/manifold/bubble/leak/dye method fails | No cassette wetting. Determine root cause, control the revision, use fresh articles where damage or contamination is possible, and repeat the failed qualification plus dependent checks. |
| Squeeze, isolated leak, burst, or reassembly coupon fails | Reject the gasket/closure configuration for HP-1. Do not adjust acceptance limits after results; revise and restart the ordered coupon sequence as QR directs. |
| Vendor or incoming critical characteristic fails | Quarantine the part/set. Do not average repeated features, assemble around the failure, or accept use-as-is. Rework and reinspect to the current drawing, or prospectively revise the drawing/configuration and reopen HP-0 before new evidence. |
| Hardware installation changes flatness, datum, finish, or cleanliness | Remove from HP-2 PASS consideration; correct/rework under an approved process and repeat every affected post-install measurement. |
| Drawing, material, geometry, pressure boundary, harness topology, or software/calculation change | Create a new controlled configuration. Reopen HP-0 and invalidate/repeat every dependent HP-1/HP-2 record. |
| Operator error with no article effect | Preserve the original record, document invalidation and scope, and repeat only under QR-approved retest. The operator cannot self-approve. |
| Safety incident, uncontrolled leak/spill, wet electrical equipment, window/hardware damage, or unknown identity | Stop, isolate energy/pressure, contain, quarantine, preserve evidence, notify SR/QR, and identify the last verified-good hold point before any recovery plan. |

## Gate Signoff

| Gate | Accountable signers | Authorization produced |
| --- | --- | --- |
| HP-0 | DA, FV, ML, SR, RC, and independent QR | Controlled inputs frozen. This gate grants no wet-work authorization; SAF-E01 is separate and configuration-specific. |
| SAF-E01 | Independent SR and QR | Bounded empty-bench commissioning only; exact design/configuration, fluid, pressure limits, and expiry triggers stated. |
| SAF-E02 | Independent SR and QR | As-built functional/gasket-coupon wet work only; exact rig/configuration, limits, and expiry triggers stated. |
| HP-1 | DA, ML, FV, SR, RC, and independent QR | Functional bench, controls, 32 functional coupons, limited gasket/coupon incoming release, gasket sequence, and measurement systems qualified for the listed configuration. |
| HP-2 | DA, ML, RC, and independent QR | Vendor FAI and independent as-received/post-hardware conformance accepted. |

No signature may be dated before its last required evidence object. No aggregate
score, majority vote, waiver of a failed mandatory row, or undocumented
conditional pass is permitted.
