# 16-Slot Cassette No-Cell Bench Validation Data Sheets

Form set: LF-CAS-A7-P001-F
Revision: A1
Protocol: docs/sixteen_slot_cassette_no_cell_bench_validation_protocol.md
Readiness checklist: LF-CAS-A7-RC001 revision A0,
docs/sixteen_slot_cassette_hp0_hp2_readiness_checklist.md
Ticket: T-D713D22E

## Use Instructions

Duplicate this versioned blank form set for each run. Do not enter execution
data in the repository template. Every executed page must show protocol revision, run ID,
cassette serial, operator initials, independent-review initials, and page
number. Electronic records must retain the native raw file, an immutable
export, and SHA-256 hash.

Permitted status values are PASS, FAIL, INVALID, HOLD, NOT READY, and
prospectively approved N/A. Blank required fields are NOT READY. Corrections
must preserve the original entry and show initials/date/reason; do not erase or
overwrite the original.

This repository file is a versioned engineering template, not a QMS by itself.
The executed copy becomes controlled only when its record owner, immutable
storage location, access control, and retention rule are entered on DS-00.

### Reusable executed-page header

Copy this header to the top of every executed or duplicated DS page. A missing
required header field makes that page NOT READY and prevents its evidence from
closing a gate.

| Header field | Entry |
| --- | --- |
| Protocol ID/revision | |
| Form-set revision | |
| Readiness checklist ID/revision | LF-CAS-A7-RC001 revision A0 |
| Run ID | |
| Cassette serial/revision | |
| Harness ID/revision, when applicable | |
| Sheet ID and copy/trial/cycle identifier | |
| Operator initials/date-time/time zone | |
| Independent-review initials/date-time/time zone | |
| Page number / total pages | |
| Executed-record owner and immutable location | |

---

## DS-00 — Run Cover And Claim

| Field | Entry |
| --- | --- |
| Protocol ID/revision | |
| Form-set revision | |
| Readiness checklist ID/revision | LF-CAS-A7-RC001 revision A0 |
| Ticket ID | |
| Run ID | |
| Claim level: FA-1 / DV-3 / manufacturing confirmation | |
| Source commit | |
| Released drawing-set ID/revision | |
| STEP manifest ID/SHA-256 | |
| Release-manifest ID/SHA-256 | |
| Software/firmware/calculation revision manifest | |
| Cassette serial/revision | |
| Carrier/lid/window/bulkhead/dock serials and revisions | |
| Fastener/receiver set IDs, revisions, lots | |
| Harness serial/revision/lot | |
| Multi-harness/checkpoint assignment manifest, if applicable | |
| Gasket source material/lot | |
| Installed gasket assembly ID/revision | |
| Nominal surrogate-set revision | |
| Fault-coupon-set revision | |
| Fixture revision | |
| Slot-map and port-map revisions | |
| Rev C chip lot and metrology-record ID | |
| Intended Rev C chip count / measured count | |
| Spare Rev C chip count / measured count | |
| Rev C chip-lot metrology disposition | |
| D7 worst-case stack record/revision/disposition | |
| One-condition ID | |
| Test-fluid batch ID | |
| Operator | |
| Independent reviewer | |
| Safety reviewer | |
| Start date/time/time zone | |
| End date/time/time zone | |
| E1 environmental extension in scope? | |
| E2 cleaning/carryover extension in scope? | |
| Final disposition | |
| Final-report artifact ID | |
| Executed-record owner/immutable storage/retention rule | |
| Repository-template/non-QMS status acknowledged | |

### Claim limitations acknowledged

| Limitation | Operator initials/date | Reviewer initials/date |
| --- | --- | --- |
| Sixteen lanes are not sixteen independent cassette articles. | | |
| Repeated trials are not independent hardware articles. | | |
| No sterility, aseptic, live-cell, AAV-containment, recirculation, or mixed-condition claim. | | |
| FA-1 applies only to the listed serialized configuration. | | |

### Harness sample accounting

“Full-core” means topology through waste for P1-P3. “Checkpoint-only” means the
identified subset required at checkpoints 5/10/25. Select exactly one row and
reconcile every harness to the DS-00 manifest.

| Claim/connector plan | Required full-core harnesses | Required checkpoint-only harnesses | Required total | Selected? | Actual IDs/manifest | Status |
| --- | ---: | ---: | ---: | --- | --- | --- |
| FA-1, reconnect-rated | 1 | 0 | 1 | | | |
| FA-1, single-use | 1 | 3 | 4 | | | |
| DV-3, reconnect-rated | 3 | 0 | 3 | | | |
| DV-3, single-use | 3 | 3 | 6 | | | |
| Manufacturing confirmation, reconnect-rated | 3 | 0 | 3 | | | |
| Manufacturing confirmation, single-use | 3 | 9 | 12 | | | |

---

## DS-01 — Frozen Parameters And Acceptance Inputs

No wet work may begin until every applicable required row is approved.

| Parameter | Symbol | Frozen value/unit | Allowed tolerance or uncertainty | Source/rating record | Approver/date |
| --- | --- | --- | --- | --- | --- |
| Maximum operating pressure | MOP | | | | |
| Decay basis: differential/gauge pressure above ambient only | Delta P | | | | |
| Absolute-sensor conversion: Delta P = Psystem,abs - Pambient,abs; synchronized ambient reference and covariance method | | | | | |
| Installed liquid proof pressure = 1.5 x MOP | Pproof | | | | |
| Installed proof target tolerance | | | | | |
| Installed-system maximum permitted peak | | | | | |
| Isolated gasket pressure = max(35 kPa gauge, 1.5 x MOP) | Pgasket | | | | |
| Coupon burst target = max(3 x Pgasket, 100 kPa gauge) | Pburst | | | | |
| Installed-system relief setpoint and tolerance | Prelief | | | | |
| Pressure-source hard cap | | | | | |
| Coupon burst-rig maximum/test ceiling | | | | | |
| Coupon burst-rig weakest validated rating/component | | | | | |
| Coupon burst-rig relief lower bound / upper bound / hard cap | | | | | |
| Coupon burst-rig overshoot limit and abort trigger | | | | | |
| Pressure abort triggers and safe-depressurization route | | | | | |
| Pressure-source isolation/valve state and no-active-makeup rule | | | | | |
| Weakest validated component limit and component ID | Plimit | | | | |
| Credible derated failure/burst/connector limit after margin used for MOP <= 50% check; not proof or one-coupon result | | | | | |
| Effective isolated test volume and determination method | Veff | | | | |
| Maximum residual/trapped gas and verification method | | | | | |
| Compliance characterization method/range/record | | | | | |
| Pressure-hold stabilization band/rule | | | | | |
| Maximum fluid/ambient temperature change during hold | Delta T max | | | | |
| Decay acquisition duration and endpoint windows | | 11 min minimum; P0 = t 0-60 s, P10 = t 600-660 s | | | |
| Sealed-blank maximum decay/control band | | | | | |
| Calibrated artificial-leak control ID and leak rate/conductance | | | | | |
| Artificial-leak expected response band/signature | | | | | |
| Minimum detectable unacceptable leak and visual detection threshold | | | | | |
| Perimeter low-head stimulus liquid, volume/U, fill points, orientation, dwell, tracer threshold, capture/migration/drain limits | | | | | |
| Pump/control mode and command profile | | | | | |
| Nominal flow or pressure command | | | | | |
| Ramp limit | | | | | |
| Hydraulic stabilization rule | | | | | |
| Collection duration or target input volume | | | | | |
| DAQ sample rate | | | | | |
| Pressure-decay endpoint-center separation | | 600 s | | | |
| Pressure-decay upper limit | | 5% | | | |
| Pressure-drift upper limit | | 5% | | | |
| Pressure-drift reference/normalization mode by channel | | | | | |
| Near-zero threshold and absolute drift band by channel | | | | | |
| Slot-flow CV upper limit | | 10% | | | |
| Row-flow CV upper limit | | 10% | | | |
| Net-mass and lane-flow quantification limits | | | | | |
| Expected total/mean/per-lane nominal flow bands | | | | | |
| Expected per-row nominal flow bands R1-R4 | | | | | |
| Input-versus-output flow-recovery band | | | | | |
| Traceable measured-input reference method/instrument | | | | | |
| Persistent position-bias decision rule | | | | | |
| External non-disturbing fault-insertion manifold ID/revision | | | | | |
| Fault-manifold hydraulic-equivalence method/reference nodes/U | | | | | |
| Fault-manifold 16-selection, no-open/no-retorque/no-demate/no-source-change qualification | | | | | |
| Prime maximum volume, maximum time, pressure band, and recovery band | | | | | |
| Bubble volume | Vbubble | | | | |
| Bubble reference pressure/temperature and delivered-volume correction | | | | | |
| Bubble injection locations | | Common, R1-R4 | | | |
| Bubble optical detection limit | | | | | |
| Bubble maximum clearing time | | | | | |
| Bubble maximum clearing volume | | | | | |
| Waste low elevation/head | | | | | |
| Waste nominal elevation/head | | | | | |
| Waste high elevation/head | | | | | |
| Reverse pressure/head | | | | | |
| Waste challenge dwell | | | | | |
| Siphon-volume detection limit | | | | | |
| W4 overflow/relief stimulus | | | | | |
| W4 captured-volume recovery band | | | | | |
| W4 escaped-volume one-sided upper limit | | | | | |
| Waste reverse-tracer baseline/decision limit | | | | | |
| Frozen maximum releasable liquid inventory | | | | | |
| Secondary-containment capacity | | | | | |
| Containment capacity / maximum inventory ratio | | At least 110% | | | |
| One-chip-dose volume | Vchip | | | | |
| Cassette condition volume | Vcondition | | | | |
| Physical hold-up limit = min(Vchip, 0.10 x Vcondition) | Vhold,max | | | | |
| Dye/tracer identity and concentration | | | | | |
| Quantitative dye method/instrument/version | | | | | |
| Dye calibration range | | | | | |
| Dye method LOD/LOQ | | | | | |
| Dye reporting/decision limit and nondetect upper-bound rule | | | | | |
| Dye analytical recovery limits | | | | | |
| Residual-estimation method | | | | | |
| Persistent-retention validated visual/analytical threshold | | | | | |
| Pre-injection system/carryover-blank map, recovery rule, residual limit, and one-sided upper-bound rule | | | | | |
| Post-dye tracer-clear map, method, decision limit, and upper-bound rule before waste | | | | | |
| Fluid recipe, density, viscosity, temperature | | | | | |
| Final torque and tolerance | | | | | |
| Receiver configuration | | | | | |
| Closure settling rule | | At least 10 min; witness delta no more than 0.02 mm over 5 min; fail if not stable by 30 min | | | |
| Witness map | | | | | |
| Connector reuse classification/rated cycles | | | | | |
| Predictive class-separation method: z = abs(mu_a - mu_b) / sqrt(u_a^2 + u_b^2 - 2 cov_ab), k = 1; include instrument, within-coupon, between-coupon/class, and drift dispersion | | z at least 3 | | | |
| Measurement-uncertainty budget/method record | | | | | |
| Standard-u versus expanded-U convention, coverage factor k/probability | | | | | |
| Covariance/correlation treatment | | | | | |
| Conformity decision-rule version/record | | | | | |
| E1 temperature/RH/duration/log interval | | | | | |
| E1 stabilization, spatial range, temporal fluctuation, overshoot, recovery, RH, evaporation, and condensation definitions/limits | | | | | |
| E2 tracer loading/dwell and cleaning recipe | | | | | |
| E2 direct/rinse method, analytical reporting/decision limit, and carryover limit | | | | | |
| Fluid/cleaner SDS, PPE, disposal, and materials-compatibility review record | | | | | |
| Hazardous/flammable E2 chemistry SOP and risk-assessment IDs, or prospectively approved N/A | | | | | |
| Raw-file location/naming/hash method | | | | | |
| Calculation workbook/script version | | | | | |

### Installed-component pressure-rating matrix

Duplicate rows until every installed pressure/wetted component and every source,
relief, isolation, vent, and sensor boundary item is listed. Eight blank rows are
not a sample cap.

| Component ID | Function | Manufacturer/part/lot | MOP rating | Proof rating | Burst rating | Temperature derating | Source document | PASS/FAIL |
| --- | --- | --- | ---: | ---: | ---: | --- | --- | --- |
| | | | | | | | | |
| | | | | | | | | |
| | | | | | | | | |
| | | | | | | | | |
| | | | | | | | | |
| | | | | | | | | |
| | | | | | | | | |
| | | | | | | | | |

HP-0 disposition: __________
Design authority/date: __________
Fluidics validation lead/date: __________
Metrology lead/date: __________
Safety reviewer/date: __________
Quality reviewer/date: __________
Records custodian/date: __________

---

## DS-02 — Equipment, Calibration, And Measurement Controls

Duplicate equipment rows as needed; every instrument or safety function used by
the run must have its own identity and disposition.

Execute DS-02 as four separately signed immutable snapshots:

- DS-02A1 records the exact downselection decision at HP0-15.
- DS-02A2 cites A1 and records final selected-equipment capability, rating,
  calibration plan, and uncertainty at HP0-15A.
- DS-02B cites A2 and records exact received serials, current certificates,
  as-found/as-left checks, and only the pre-SAF commissioning controls needed
  before SAF-E02.
- DS-02C cites B and SAF-E02 and records post-SAF bubble, dye, and evaporation
  method controls before HP-1 release.

Never append a later stage to, replace, or re-hash an earlier signed file. For
DS-02A1/A2, mark as-found/as-left and both commissioning-control sections
prospectively N/A with the stage basis. For DS-02B, mark the DS-02C control
section prospectively N/A; sealed-blank/artificial-leak results belong only in
DS-08. For DS-02C, copy the applicable exact equipment identities from B and
mark unrelated rows prospectively N/A. Any blank required field is NOT READY.

| Snapshot-control field | Entry |
| --- | --- |
| Stage: DS-02A1 downselection / DS-02A2 final capability / DS-02B pre-SAF commissioning / DS-02C post-SAF method controls | |
| Snapshot record ID/revision | |
| Configuration/P&ID/BOM identity | |
| Immediate predecessor DS-02 record ID/revision/SHA-256; required for A2/B/C | |
| SAF-E02 ID/revision/SHA-256; required only for DS-02C | |
| Executed snapshot immutable location | |
| Detached sidecar-manifest ID/location assigned before signing; sidecar records the final file hash after signing | |

| Equipment ID | Function | Manufacturer/model/serial | Range/capacity | Resolution | Standard u / expanded U / k / coverage or budget link | Certificate/due date | Software/firmware | As-found check | As-left check | Status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| | Pump/controller | | | | | | | | | |
| | Upstream pressure | | | | | | | | | |
| | R1 pressure | | | | | | | | | |
| | R2 pressure | | | | | | | | | |
| | R3 pressure | | | | | | | | | |
| | R4 pressure | | | | | | | | | |
| | Waste/backpressure | | | | | | | | | |
| | Pressure reference | | | | | | | | | |
| | DAQ/clock | | | | | | | | | |
| | Balance | | | | | | | | | |
| | Check weights | | | | | | | | | |
| | Covered collection-vessel/nest set | | | | | | | | | |
| | Evaporation-blank vessel set, minimum three | | | | | | | | | |
| | Traceable measured-input mass/volume/displacement reference | | | | | | | | | |
| | Fluid-preparation mass/volume tools | | | | | | | | | |
| | Fluid-density method/instrument | | | | | | | | | |
| | Fluid-viscosity method/instrument | | | | | | | | | |
| | Fluid temperature | | | | | | | | | |
| | Ambient temperature | | | | | | | | | |
| | Bubble injector | | | | | | | | | |
| | Dye instrument | | | | | | | | | |
| | Torque driver | | | | | | | | | |
| | Dimensional instrument 1 | | | | | | | | | |
| | Dimensional instrument 2 | | | | | | | | | |
| | Waste elevation/head | | | | | | | | | |
| | Camera/bubble evidence | | | | | | | | | |
| | Installed-system relief device | | | | | | | | | |
| | Pressure-source hard-limit/interlock | | | | | | | | | |
| | Source-isolation/no-makeup valve | | | | | | | | | |
| | Safe vent/drain | | | | | | | | | |
| | Emergency stop | | | | | | | | | |
| | Secondary containment | | | | | | | | | |
| | Coupon burst rig/weakest pressure boundary | | | | | | | | | |
| | Coupon burst shield | | | | | | | | | |
| | Spill kit/electrical-separation control | | | | | | | | | |
| | E1 chamber reference/RH system, if applicable | | | | | | | | | |
| | E1 sixteen-probe slot-plane set, if applicable | | | | | | | | | |

### DS-02B pre-SAF commissioning controls

| Control | Control/equipment ID and configuration | Required target/band | Before result | After result | u/U, k/coverage | Raw file/photo | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Pressure zero/span | | | | | | | |
| Pressure-source isolation/no-active-makeup verification | | | | | | | |
| Relief/hard-cap/safe-vent functional check | | | | | | | |
| Emergency-stop functional check | | | | | | | |
| Burst-rig rating and shield check | | | | | | | |
| Secondary-containment capacity and 110% calculation | | | | | | | |
| Balance low/mid/high check | | | | | | | |
| Temperature check | | | | | | | |
| Clock/timestamp alignment | | | | | | | |
| Spill/electrical-separation/PPE readiness | | | | | | | |

Sealed-blank and artificial-leak qualification are not DS-02 controls; execute
and preserve them only in the dedicated DS-08 qualification records after
SAF-E02.

### DS-02C post-SAF method controls

| Control | Control/equipment ID and configuration | Required target/band | Before result | After result | u/U, k/coverage | Raw file/photo | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Evaporation-blank set readiness; simultaneous campaign results remain in DS-09 | | | | | | | |
| Bubble delivered-volume check at reference pressure/temperature | | | | | | | |
| Dye blank | | | | | | | |
| Dye low/mid/high standards and calibration fit/range | | | | | | | |
| Independent dye check | | | | | | | |
| Dye LOD/LOQ, recovery, and residual-method check | | | | | | | |

### DS-02A1 HP-0 downselection disposition

Required equipment downselected against provisional envelopes: __________
HP0-E10 downselection package ID/revision: __________
Sourcing/build lead/date: __________
Metrology lead/date: __________
Fluidics validation lead/date: __________
Safety reviewer/date: __________
Independent quality reviewer/date: __________
Records custodian/date: __________

### DS-02A2 HP-0 final-capability disposition

Predecessor DS-02A1 identity/hash verified unchanged: __________
Selected equipment capability, ratings, calibration plan, and projected U demonstrated: __________
HP0-E10 selection/capability package ID/revision: __________
Metrology lead/date: __________
Fluidics validation lead/date: __________
Safety reviewer/date: __________
Independent quality reviewer/date: __________
Records custodian/date: __________

### DS-02B pre-SAF commissioning disposition

Predecessor DS-02A2 identity/hash verified unchanged: __________
SAF-E01 limited empty-bench authorization ID/revision/hash: __________
DS-02B bounded commissioning disposition: __________
Metrology lead/date: __________
Fluidics lead/date: __________
Safety reviewer/date: __________
Independent quality reviewer/date: __________
Records custodian/date: __________

SAF-E02 is a detached downstream authorization that may be issued only after
the signed DS-02B snapshot and its sidecar hash exist. HP1-E09 is a later
detached gate manifest and may reference DS-02B, DS-02C, and SAF-E02 one way;
none is written back into DS-02B.

### DS-02C post-SAF method-control disposition

Predecessor DS-02B identity/hash verified unchanged: __________
SAF-E02 as-built functional/gasket-coupon authorization ID/revision/hash: __________
Bubble/dye/evaporation method-control disposition: __________
Metrology lead/date: __________
Fluidics validation lead/date: __________
Safety reviewer/date: __________
Independent quality reviewer/date: __________
Records custodian/date: __________

---

## DS-03 — Test Articles, Lots, And Pre-Inspection

For each supplier-made custom part, identify the vendor FAI here and link the
separate independent incoming/D8 record in the executed HP-2 manifest. Copying
vendor results into an internal field does not create independent inspection.

Execute DS-03A for the HP-1 functional articles, actual gasket lot, and
production-intent coupon set. Execute a new DS-03B for HP-2 incoming parts and
identity reconciliation. Each snapshot has its own signatures and detached
sidecar hash; DS-03B may reference DS-03A but may not mutate it. Mark rows outside the selected stage
with a prospectively approved N/A and basis rather than leaving them blank.

| Snapshot-control field | Entry |
| --- | --- |
| Stage: DS-03A HP-1 article qualification or DS-03B HP-2 incoming | |
| Snapshot record ID/revision | |
| Configuration/article/lot scope | |
| Predecessor DS-03A record ID/revision/SHA-256; required when applicable | |
| Executed snapshot immutable location | |
| Detached sidecar-manifest ID/location assigned before signing; sidecar records the final file hash after signing | |

| Item ID | Class/function | Part/revision | Material/lot | Supplier | Characterization/FAI record | Assigned role/slot | Pre-inspection | Status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| | Carrier | | | | | | | |
| | Lid/clamp | | | | | | | |
| | Window | | | | | | | |
| | Bulkhead | | | | | | | |
| | Dock | | | | | | | |
| | Gasket | | | | | | | |
| | Harness | | | | | | | |
| | Sealed-fixture matched blank | | | | | | | |
| | Covered collection-vessel/nest set | | | | | | | |
| | Evaporation-blank set, at least three | | | | | | | |
| | Squeeze ladder | | | | | | | |
| | Leak coupon | | | | | | | |
| | Burst coupon | | | | | | | |
| | Gasket reassembly coupon | | | | | | | |
| | Soak coupon | | | | | | | |
| | Artificial-leak control | | | | | | | |
| | Nominal surrogate set | | | | | S01-S16 | | |
| | Low-resistance set | | | | | Four placements | | |
| | High-resistance set | | | | | Four placements | | |
| | Blocked set | | | | | Four placements | | |
| | Bypass set | | | | | Four placements | | |
| | External non-disturbing fault-insertion manifold | | | | Hydraulic-equivalence/qualification record | All 16 selections | | |
| | Fluid batch | | | | | | | |
| | Dye batch | | | | | | | |
| | Waste container | | | | | | | |
| | Rev C chip-lot metrology record | | | | | Intended plus all spares | | |

Rev C intended count/measured/pass: __________
Rev C spare count/measured/pass: __________
Installed gasket assembly ID versus source-lot reconciliation: __________
Multi-harness/single-use checkpoint manifest reconciliation: __________

Snapshot disposition: __________
Fluidics validation lead/date; required for DS-03A: __________
Metrology lead/date: __________
Supplier/build lead/date; required for DS-03B: __________
Independent quality reviewer/date: __________
Records custodian/date: __________

---

## DS-04 — Incoming Inspection, Fit, Datum, Dock, And Visibility

Execute three distinct immutable snapshots when applicable: DS-04A records
HP-2 pre-hardware incoming condition; DS-04B records post-hardware incoming
condition, fit, datum, dock, and visibility; DS-04C records the later HP-3
assembled state. DS-04B must cite the DS-04A hash and DS-04C must cite DS-04B.
Never add later measurements to, replace, or re-hash an earlier signed snapshot.
Mark sections outside the selected snapshot with a prospectively approved N/A
and basis rather than leaving required fields blank.

| Snapshot-control field | Entry |
| --- | --- |
| Stage: DS-04A HP-2 pre-hardware / DS-04B HP-2 post-hardware and incoming fit / DS-04C HP-3 assembled state | |
| Snapshot record ID/revision | |
| Configuration and hardware-installation state | |
| Predecessor DS-04 record ID/revision/SHA-256 | |
| Executed snapshot immutable location | |
| Detached sidecar-manifest ID/location assigned before signing; sidecar records the final file hash after signing | |

### Lane fit census

| Slot | Surrogate ID | Seats/removes | Binding/rocking/damage | Released clearance or force result | Optical clear | Label clear | Drain/witness clear | Photo ID | Status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| S01 | | | | | | | | | |
| S02 | | | | | | | | | |
| S03 | | | | | | | | | |
| S04 | | | | | | | | | |
| S05 | | | | | | | | | |
| S06 | | | | | | | | | |
| S07 | | | | | | | | | |
| S08 | | | | | | | | | |
| S09 | | | | | | | | | |
| S10 | | | | | | | | | |
| S11 | | | | | | | | | |
| S12 | | | | | | | | | |
| S13 | | | | | | | | | |
| S14 | | | | | | | | | |
| S15 | | | | | | | | | |
| S16 | | | | | | | | | |

### Datum and fixture features

| Feature | Released requirement | Measured/observed result | Uncertainty | Photo/raw file | Status |
| --- | --- | --- | --- | --- | --- |
| Datum A carrier-bottom support | | | | | |
| Rear datum edge/rail contact | | | | | |
| Left datum edge/rail contact | | | | | |
| Front retention/lip contact | | | | | |
| D1 round locator | | | | | |
| D2 relieved locator | | | | | |
| D3 non-locating witness | | | | | |
| D4 non-locating witness | | | | | |
| Free carrier flatness/twist | | | | | |
| Docked carrier flatness/twist | | | | | |
| Nine internal stops | | | | | |
| Perimeter stops | | | | | |
| Sixteen fasteners/receivers | | | | | |
| Segmented dock openings/bridges | | | | | |
| Gutter and 8 mm drain visibility | | | | | |
| Global/per-slot label lands | | | | | |
| Closed-stack clearance, 41.35 mm reference | | | | | |

### Incoming visual and cleanliness checklist

| Region/condition | Lighting/method ID | Observation/photo | Acceptance decision | Status |
| --- | --- | --- | --- | --- |
| Seal lands clean and undamaged | | | | |
| No burr or coating buildup | | | | |
| No clogged port or unused-port cap conflict | | | | |
| No residue, corrosion, crack, or window defect | | | | |
| Connector, label, handling, optical, gutter, and drain regions clear | | | | |
| All identities/lots/certificates/calibration states reconciled | | | | |

### DS-04A pre-hardware disposition

Pre-hardware incoming inspection disposition: __________
Design authority/date: __________
Metrology lead/date: __________
Independent quality reviewer/date: __________
Records custodian/date: __________

### DS-04B post-hardware/incoming-fit disposition

Predecessor DS-04A identity/hash verified unchanged: __________
Post-hardware/incoming-fit disposition: __________
Design authority/date: __________
Metrology lead/date: __________
Independent quality reviewer/date: __________
Records custodian/date: __________

HP2-E05 is a detached downstream gate manifest that may reference the signed
DS-04A and DS-04B sidecar hashes one way; it is not written into either file.

### DS-04C HP-3 assembled-state disposition

Predecessor DS-04B identity/hash verified unchanged: __________
HP-3 assembled-state disposition: __________
Design authority/date: __________
Metrology lead/date: __________
Fluidics validation lead/date: __________
Independent quality reviewer/date: __________
Records custodian/date: __________

---

## DS-05 — Closure, Fastener, Compression, And Settling

Execute DS-05A as a separate immutable HP-1 squeeze-ladder-only copy, marking
the closure/HP-3 sections prospectively N/A. Execute DS-05B later for the HP-3
closure/checkpoint-1 dry stack and cite DS-05A without modifying it. Duplicate
DS-05B for checkpoints 5, 10, and 25; do not create a second cycle-1 record and
never overwrite an earlier checkpoint.

| Snapshot-control field | Entry |
| --- | --- |
| Stage: DS-05A HP-1 squeeze ladder or DS-05B HP-3/endurance closure | |
| Snapshot record ID/revision | |
| Gasket/closure/configuration identity | |
| Predecessor DS-05A record ID/revision/SHA-256; required for DS-05B | |
| Executed snapshot immutable location | |
| Detached sidecar-manifest ID/location assigned before signing; sidecar records the final file hash after signing | |

### Gasket free-height lot map

| Measurement ID/location | Free height | Standard u | Expanded U | k/coverage | Lot band | Status |
| --- | ---: | ---: | ---: | --- | --- | --- |
| | | | | | | |
| | | | | | | |
| | | | | | | |
| | | | | | | |
| | | | | | | |

### Fastener sequence

Enter both fastener IDs/readings in each paired cell. The eight rows cover all
sixteen fasteners. Complete finger, low, and initial-final passes first. Enter
the repeat-final pass only after the qualifying pre-repeat stability window.

| Pair order | Fastener IDs | Finger-tight readings/time | Low-pass setting/readings/time | Final-pass setting/readings/time | Repeat-final setting/readings/time | Driver ID | Hardware observation | Status |
| ---: | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | | | | | | | | |
| 2 | | | | | | | | |
| 3 | | | | | | | | |
| 4 | | | | | | | | |
| 5 | | | | | | | | |
| 6 | | | | | | | | |
| 7 | | | | | | | | |
| 8 | | | | | | | | |

### Compression witnesses

The start of the accepted five-minute window may occur any time after the
minimum ten-minute settle, but no later than minute 25. Record a failed window
and repeat without overwriting it; stability not achieved by minute 30 is FAIL.

#### Pre-repeat-final qualifying stability window

| Cycle | Witness ID/location | Free height | Initial-final height/time | Stability start time/height | Stability end time/height | 5-min delta | U height | U delta | k/coverage | Stable by 30 min? | Status |
| --- | --- | ---: | --- | --- | --- | ---: | ---: | ---: | --- | --- | --- |
| | S01 / front-left | | | | | | | | | | |
| | S04 / front-right | | | | | | | | | | |
| | S06 / interior | | | | | | | | | | |
| | S07 / interior | | | | | | | | | | |
| | S10 / interior | | | | | | | | | | |
| | S11 / interior | | | | | | | | | | |
| | S13 / rear-left | | | | | | | | | | |
| | S16 / rear-right | | | | | | | | | | |
| | Perimeter front-left | | | | | | | | | | |
| | Perimeter front-right | | | | | | | | | | |
| | Perimeter rear-left | | | | | | | | | | |
| | Perimeter rear-right | | | | | | | | | | |
| | Perimeter reference 5 | | | | | | | | | | |

Pre-repeat window approval/time: __________

#### Post-repeat-final witness and second stability window

After the pre-repeat window passes, apply the recorded repeat-final torque pass,
then verify height/squeeze and a second five-minute stability window.

| Cycle | Witness ID/location | Repeat-final pass/time ref | Post-repeat height | Second-window start time/height | Second-window end time/height | Delta | U height | U delta | Squeeze % | U squeeze | k/coverage | 1.68-1.92 mm guard | Hard-stop | Status |
| --- | --- | --- | ---: | --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | --- | --- |
| | S01 / front-left | | | | | | | | | | | | | |
| | S04 / front-right | | | | | | | | | | | | | |
| | S06 / interior | | | | | | | | | | | | | |
| | S07 / interior | | | | | | | | | | | | | |
| | S10 / interior | | | | | | | | | | | | | |
| | S11 / interior | | | | | | | | | | | | | |
| | S13 / rear-left | | | | | | | | | | | | | |
| | S16 / rear-right | | | | | | | | | | | | | |
| | Perimeter front-left | | | | | | | | | | | | | |
| | Perimeter front-right | | | | | | | | | | | | | |
| | Perimeter rear-left | | | | | | | | | | | | | |
| | Perimeter rear-right | | | | | | | | | | | | | |
| | Perimeter reference 5 | | | | | | | | | | | | | |

Post-repeat-final window approval/time: __________

### Production-intent 20/25/30% squeeze ladder

| Reference | Article/material/lot | Target squeeze | Target compressed height | Measured height | U height | Calculated squeeze | U squeeze | k/coverage | Visual/functional result | Status |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- | --- |
| 20% | | 20% | 1.92 mm nominal | | | | | | | |
| 25% | | 25% | 1.80 mm nominal | | | | | | | |
| 30% | | 30% | 1.68 mm nominal | | | | | | | |

### HP-1 squeeze-ladder subsection release

This subsection, the linked D7 record, and its immutable attachments may close
the squeeze-ladder part of HP-1 without completing the later HP-3 dry-stack
fields on the separate DS-05B copy.

DS-05A HP-1 squeeze-ladder snapshot ID/revision: __________
Detached sidecar-manifest ID/location; sidecar records the final snapshot hash after signing: __________
D7 actual-lot record ID/revision/SHA-256: __________
HP-1 squeeze-ladder disposition: __________
Design authority/date: __________
Metrology lead/date: __________
Independent quality reviewer/date: __________
Records custodian/date: __________

Closure/dock/optical/drain status: __________
DS-05B HP-3 dry-stack disposition: __________
DS-05B detached sidecar-manifest ID/location: __________
Operator/date: __________
Independent verifier/date: __________
Quality reviewer/date: __________

---

## DS-06 — Harness Topology And Keepout Reconciliation

Execute DS-06A as the immutable HP-1 functional-fixture/manifold topology
snapshot. Execute DS-06B later as the immutable HP-3 cassette/harness topology
snapshot. Each copy must identify its exact P&ID, BOM, port map, and predecessor;
do not append HP-3 observations to DS-06A.

| Snapshot-control field | Entry |
| --- | --- |
| Stage: DS-06A HP-1 fixture/manifold or DS-06B HP-3 cassette/harness | |
| Snapshot record ID/revision | |
| P&ID/BOM/port-map/configuration identity | |
| Predecessor DS-06 record ID/revision/SHA-256, when applicable | |
| Executed snapshot immutable location | |
| Detached sidecar-manifest ID/location assigned before signing; sidecar records the final file hash after signing | |

### Common-source, sensing, safety, and containment topology

| Element | Physical ID | Source/from | Destination/to | Flow/signal direction | Valve/cap/key state | Scan/photo | First verifier | Independent verifier | Status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Reservoir | | | | | | | | | |
| Pump/controller | | | | | | | | | |
| Filter | | | | | | | | | |
| Bubble window/injector | | | | | | | | | |
| Common supply | | | | | | | | | |
| Upstream pressure sensor | | | | | | | | | |
| R1 pressure sensor | | | | | | | | | |
| R2 pressure sensor | | | | | | | | | |
| R3 pressure sensor | | | | | | | | | |
| R4 pressure sensor | | | | | | | | | |
| Waste/backpressure sensor | | | | | | | | | |
| Relief device | | | | | | | | | |
| Pressure-source hard cap/interlock | | | | | | | | | |
| Isolation/no-makeup boundary | | | | | | | | | |
| Safe vent/drain | | | | | | | | | |
| Secondary containment | | | | | | | | | |

### G/M/W service ports

| Port | Planned role | Physical destination | Connector/scan ID | Cap state | Keying/segregation | Keepouts and bend radius clear | Verifier | Status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| G0 | | | | | | | | |
| G1 | | | | | | | | |
| G2 | | | | | | | | |
| G3 | | | | | | | | |
| M0 | | | | | | | | |
| M1 | | | | | | | | |
| M2 | | | | | | | | |
| M3 | | | | | | | | |
| M4 | | | | | | | | |
| M5 | | | | | | | | |
| M6 | | | | | | | | |
| W0 | | | | | | | | |
| W1 | | | | | | | | |
| W2 | | | | | | | | |
| W3 | | | | | | | | |
| W4 | | | | | | | | |

### Row and slot routes

| Route | Feed source/branch | Inlet ID | Outlet ID | Waste destination | Scan/label match | Strain relief | Keepouts clear | Status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| R1 | | | | | | | | |
| R2 | | | | | | | | |
| R3 | | | | | | | | |
| R4 | | | | | | | | |
| S01 | | | | | | | | |
| S02 | | | | | | | | |
| S03 | | | | | | | | |
| S04 | | | | | | | | |
| S05 | | | | | | | | |
| S06 | | | | | | | | |
| S07 | | | | | | | | |
| S08 | | | | | | | | |
| S09 | | | | | | | | |
| S10 | | | | | | | | |
| S11 | | | | | | | | |
| S12 | | | | | | | | |
| S13 | | | | | | | | |
| S14 | | | | | | | | |
| S15 | | | | | | | | |
| S16 | | | | | | | | |

First verifier/date: __________
Independent verifier/date: __________
Stage-specific topology disposition: __________
Records custodian/date: __________

---

## DS-07 — Prime And Bubble Challenge

Execute immutable DS-07A before P1 using only optical-method checks 1-3; mark
the prime, bubble-trial, and checks 4-6 sections prospectively N/A with the stage
basis. After DS-07A is signed and its detached sidecar hash exists, execute a
new DS-07B for P1-P3 prime/bubble work and checks 4-6. DS-07B cites DS-07A but
never copies results back into it.

| Snapshot-control field | Entry |
| --- | --- |
| Stage: DS-07A pre-P1 optical method or DS-07B P1-P3/post-P3 campaign | |
| Snapshot record ID/revision | |
| Camera/lighting/optical/stimulus configuration identity | |
| Predecessor DS-07A record ID/revision/SHA-256; required for DS-07B | |
| Executed snapshot immutable location | |
| Detached sidecar-manifest ID/location assigned before signing; sidecar records the final file hash after signing | |

### Prime record

Use one sheet per independent wet preparation P1/P2/P3 or later endurance
checkpoint. At closure/checkpoint 1, P1-P3 use the same still-connected initial
harness. Record fresh harness IDs at checkpoints 5, 10, and 25 when connectors
are single-use; a reconnect-rated plan may retain its designated harness.

| Preparation-level field | Entry |
| --- | --- |
| Fluid batch/recipe | |
| Fluid-preparation raw record/operator | |
| Actual fluid temperature/density/viscosity | |
| Actual dye concentration, when present | |
| Pressure zero/span precheck record/status | |
| Balance/check-weight precheck record/status | |
| Temperature precheck record/status | |
| Fluid-preparation check record/status | |
| Nominal command and stabilization record/status | |
| Frozen prime maximum volume/time/pressure/recovery bands | |
| Waste collection ID | |
| Waste tare/final/net recovered mass or volume and U/k | |
| Pressure trace file/hash | |
| Preparation-level prime disposition | |

| Run/preparation/cycle | Slot/route | Prime volume | U volume | Prime time | U time | Pressure result/U and trace | Recovered waste | Prime limits guard-banded | Inlet bubble-free | Outlet bubble-free | Unplanned intervention | Raw file/video | Status |
| --- | --- | ---: | ---: | ---: | ---: | --- | ---: | --- | --- | --- | --- | --- | --- |
| | Common | | | | | | | | | | | | |
| | R1 | | | | | | | | | | | | |
| | R2 | | | | | | | | | | | | |
| | R3 | | | | | | | | | | | | |
| | R4 | | | | | | | | | | | | |
| | S01 | | | | | | | | | | | | |
| | S02 | | | | | | | | | | | | |
| | S03 | | | | | | | | | | | | |
| | S04 | | | | | | | | | | | | |
| | S05 | | | | | | | | | | | | |
| | S06 | | | | | | | | | | | | |
| | S07 | | | | | | | | | | | | |
| | S08 | | | | | | | | | | | | |
| | S09 | | | | | | | | | | | | |
| | S10 | | | | | | | | | | | | |
| | S11 | | | | | | | | | | | | |
| | S12 | | | | | | | | | | | | |
| | S13 | | | | | | | | | | | | |
| | S14 | | | | | | | | | | | | |
| | S15 | | | | | | | | | | | | |
| | S16 | | | | | | | | | | | | |

### Bubble challenge

| Trial | Preparation | Verified-prime record | Location | Injector reading / reference P,T | Delivered volume | U volume / k | Flow/pressure condition | Clear time | U time | Clearing volume | U clearing volume | Optical decision limit/result | Chip inlet reached? | All downstream/high-point witness result | W1/W3 capture | Pressure/video/raw file | Status |
| ---: | --- | --- | --- | --- | ---: | --- | --- | ---: | ---: | ---: | ---: | --- | --- | --- | --- | --- | --- |
| 1 | P1 | | Common | | | | | | | | | | | | | | |
| 2 | P1 | | R1 | | | | | | | | | | | | | | |
| 3 | P1 | | R2 | | | | | | | | | | | | | | |
| 4 | P1 | | R3 | | | | | | | | | | | | | | |
| 5 | P1 | | R4 | | | | | | | | | | | | | | |
| 6 | P2 | | Common | | | | | | | | | | | | | | |
| 7 | P2 | | R1 | | | | | | | | | | | | | | |
| 8 | P2 | | R2 | | | | | | | | | | | | | | |
| 9 | P2 | | R3 | | | | | | | | | | | | | | |
| 10 | P2 | | R4 | | | | | | | | | | | | | | |
| 11 | P3 | | Common | | | | | | | | | | | | | | |
| 12 | P3 | | R1 | | | | | | | | | | | | | | |
| 13 | P3 | | R2 | | | | | | | | | | | | | | |
| 14 | P3 | | R3 | | | | | | | | | | | | | | |
| 15 | P3 | | R4 | | | | | | | | | | | | | | |

### Smallest-prohibited-bubble optical method checks

Run the three locations before P1 and again after P3. All 6/6 must detect the
verified stimulus under the frozen camera, lighting, and optical configuration.

| Check | Timing | Location | Verified stimulus volume/P/T/U | Optical threshold | Lighting/camera/config | Observed result | Video/hash | Status |
| ---: | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | Before P1 | Upstream witness | | | | | | |
| 2 | Before P1 | Row high point | | | | | | |
| 3 | Before P1 | Chip-inlet witness | | | | | | |
| 4 | After P3 | Upstream witness | | | | | | |
| 5 | After P3 | Row high point | | | | | | |
| 6 | After P3 | Chip-inlet witness | | | | | | |

DS-07A pre-P1 checks 1-3 three-of-three disposition/reviewer/date: __________
DS-07A detached sidecar-manifest ID/location: __________
DS-07B post-P3 checks 4-6 three-of-three disposition/reviewer/date: __________
Combined six-of-six disposition from immutable DS-07A and DS-07B records/reviewer/date: __________

Prime disposition: __________
Bubble disposition: __________
Operator/date: __________
Reviewer/date: __________

---

## DS-08 — Liquid Integrity, Pressure Decay, And Burst

### DS-08A — One source-isolated pressure-decay record

Duplicate DS-08A for every matched blank, isolated leak coupon/loop, installed
hold, endurance checkpoint, and applicable gasket-reassembly-coupon leak hold. Do not
use active pressure makeup. Acceptance uses differential/gauge pressure above
ambient. If absolute sensors are used, record synchronized ambient and calculate
Delta P = Psystem,abs - Pambient,abs with its uncertainty and covariance.

| Field | Entry |
| --- | --- |
| Hold/test ID and article/control ID | |
| Loop/system and P1/P2/P3 or closure checkpoint | |
| Harness/gasket/configuration identity and revision | |
| Isolation boundary, pump state, valve state, no-makeup verification | |
| Effective isolated liquid volume/method/U | |
| Residual gas estimate/limit/method | |
| Compliance characterization record/result | |
| Medium and initial/final fluid temperature | |
| T0 / T10 / Delta T / U temperature / Delta-T status | |
| Pressure basis and ambient-reference sensor/synchronization | |
| Target/tolerance / maximum permitted peak / actual peak | |
| Relief setpoint/tolerance and hard-cap status | |
| Stabilization rule met/time/reference | |
| Acquisition start and exact P0 window timestamps, t = 0-60 s | |
| Exact P10 window timestamps, t = 600-660 s | |
| P0 system absolute / ambient absolute / converted differential | |
| P10 system absolute / ambient absolute / converted differential | |
| P0 standard u / expanded U / k / coverage | |
| P10 standard u / expanded U / k / coverage | |
| P0-P10 covariance/correlation and budget record | |
| Raw DUT decay % = 100 x (P0 - P10) / P0 | |
| Standard u / expanded U / k for decay | |
| Pressure-rise result versus matched-blank thermal/compliance band | |
| Matched sealed-blank ID/band/result | |
| Artificial-leak control ID/rate/expected band/observed result | |
| Control-normalized diagnostic, if reported; never acceptance-subtracted | |
| Dye/visual witness locations and result | |
| Hardware/relief/containment result | |
| Native trace/photo/video path and SHA-256 | |
| Guard-banded disposition | |
| Operator/date and independent reviewer/date | |

### Coupon and method qualification

| Test | Article/control ID | Calibrated stimulus/rate | Required response/band | DS-08A or burst-record ID | Quantified result/U/k | Raw file/photo/hash | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Sealed blank before | | | | | | | |
| Artificial-leak positive control before | | | | | | | |
| Isolated gasket leak coupon | | | | | | | |
| Destructive burst coupon | | | | | | | |
| Sealed blank after | | | | | | | |
| Artificial-leak positive control after | | | | | | | |

### Destructive burst record

| Article ID | Burst-rig ID/weakest rating | Target survival/lower bound | Rig relief lower/upper/hard cap | Optional authorized failure pressure or N/A | Authorization/rated-ramp record | U/k | Maximum peak/overshoot | Abort/relief/shield result | Failure mode | Raw file/hash | Status |
| --- | --- | ---: | --- | ---: | --- | --- | ---: | --- | --- | --- | --- |
| | | | | | | | | | | | |

### Per-loop and installed-system pressure holds

Per-slot loops, nonpressurized perimeter work, and bracketing controls complete
before P1. P1-P3 are the three checkpoint-1 installed holds; do not add another
cycle-1 hold. Later rows are checkpoints 5, 10, and 25.

| Test ID | Loop/system | Checkpoint/preparation | Harness ID | DS-08A record | Target/tolerance | Max permitted / actual peak | Relief/hard-cap status | Differential P0/P10 | Raw decay % | U decay/k | Dye/hardware result | Trace/hash | Status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | ---: | --- | --- | --- | --- |
| | S01 | Pre-P1 | | | | | | | | | | | |
| | S02 | Pre-P1 | | | | | | | | | | | |
| | S03 | Pre-P1 | | | | | | | | | | | |
| | S04 | Pre-P1 | | | | | | | | | | | |
| | S05 | Pre-P1 | | | | | | | | | | | |
| | S06 | Pre-P1 | | | | | | | | | | | |
| | S07 | Pre-P1 | | | | | | | | | | | |
| | S08 | Pre-P1 | | | | | | | | | | | |
| | S09 | Pre-P1 | | | | | | | | | | | |
| | S10 | Pre-P1 | | | | | | | | | | | |
| | S11 | Pre-P1 | | | | | | | | | | | |
| | S12 | Pre-P1 | | | | | | | | | | | |
| | S13 | Pre-P1 | | | | | | | | | | | |
| | S14 | Pre-P1 | | | | | | | | | | | |
| | S15 | Pre-P1 | | | | | | | | | | | |
| | S16 | Pre-P1 | | | | | | | | | | | |
| | Installed hold | P1 / checkpoint 1 | | | | | | | | | | | |
| | Installed hold | P2 / checkpoint 1 | | | | | | | | | | | |
| | Installed hold | P3 / checkpoint 1 | | | | | | | | | | | |
| | Endurance hold | Checkpoint 5 | | | | | | | | | | | |
| | Endurance hold | Checkpoint 10 | | | | | | | | | | | |
| | Endurance hold | Checkpoint 25 | | | | | | | | | | | |

### Nonpressurized perimeter containment, migration, and drainage

This is not a pressure-decay or burst record. Do not enter P0, P10, or decay.

| Test ID | Stimulus liquid/volume/U | Liquid-head/elevation/U | Fixture orientation | Fill point(s) | Dwell | Tracer detection threshold | Capture/migration acceptance criteria | Captured volume/U | Migration/witness result | Gutter/drain route | Dry regions | Photo/raw file/hash | Status |
| --- | --- | --- | --- | --- | ---: | --- | --- | --- | --- | --- | --- | --- | --- |
| | | | | | | | | | | | | | |

### Gasket-reassembly-coupon checkpoints

Use the same serialized gasket reassembly coupon through closure/reassembly
cycles 1, 5, 10, and 25. Each leak result references a complete DS-08A record.
This table is separate from connector mate/demate counting and full-harness use.

| Coupon cycle | Gasket-reassembly coupon ID | Closure/reassembly count | Visual/compression result | DS-08A leak record | Decay/U/status | Damage or change | Reviewer/date | Status |
| ---: | --- | ---: | --- | --- | --- | --- | --- | --- |
| 1 | | | | | | | | |
| 5 | | | | | | | | |
| 10 | | | | | | | | |
| 25 | | | | | | | | |

Integrity disposition: __________
Safety reviewer/date: __________
Quality reviewer/date: __________

---

## DS-09 — Gravimetric Flow Raw Data And Summary

Duplicate DS-09A for every collection interval. Do not pool intervals.

### DS-09A — One collection interval

Run ID: ______  Harness: ______  Cycle/preparation: ______  Interval: ______
Start/end time: ______  Collection duration/U time: ______
Fluid density/U/k/temp: ______  Balance ID/check: ______
Measured-input method/instrument/calibration: ______
Raw input initial/final mass, volume, or calibrated displacement: ______
Measured input/U/k: ______

| Slot | Row | Vessel ID | Tare mass | Gross mass | Assigned evaporation correction/U | Corrected mass | U mass | Net-mass/flow LOQ | Collected volume/U | Calculated flow | U flow/k | Per-lane band | LOQ/band status | Raw file | Status |
| --- | --- | --- | ---: | ---: | --- | ---: | ---: | --- | --- | ---: | --- | --- | --- | --- | --- |
| S01 | R1 | | | | | | | | | | | | | | |
| S02 | R1 | | | | | | | | | | | | | | |
| S03 | R1 | | | | | | | | | | | | | | |
| S04 | R1 | | | | | | | | | | | | | | |
| S05 | R2 | | | | | | | | | | | | | | |
| S06 | R2 | | | | | | | | | | | | | | |
| S07 | R2 | | | | | | | | | | | | | | |
| S08 | R2 | | | | | | | | | | | | | | |
| S09 | R3 | | | | | | | | | | | | | | |
| S10 | R3 | | | | | | | | | | | | | | |
| S11 | R3 | | | | | | | | | | | | | | |
| S12 | R3 | | | | | | | | | | | | | | |
| S13 | R4 | | | | | | | | | | | | | | |
| S14 | R4 | | | | | | | | | | | | | | |
| S15 | R4 | | | | | | | | | | | | | | |
| S16 | R4 | | | | | | | | | | | | | | |

### Simultaneous evaporation blanks for this interval

Use at least three matched covered blanks. Signed delta is final minus initial
mass; positive loss is max(0, -signed delta). Operator initials attest that
start/end readings and lane mapping are contemporaneous.

| Blank ID/location | Matched fill/cover | Assigned lanes | Initial mass/time/temp | Final mass/time/temp | Signed delta | Positive loss | Correction model/result | U/k | Operator initials | Raw file | Status |
| --- | --- | --- | --- | --- | ---: | ---: | --- | --- | --- | --- | --- |
| Blank 1 | | | | | | | | | | | |
| Blank 2 | | | | | | | | | | | |
| Blank 3 | | | | | | | | | | | |

| Summary | Result | Standard u | Expanded U / k / coverage | Target/limit | Covariance or Monte Carlo method ref | Guard-banded pass? |
| --- | ---: | ---: | --- | --- | --- | --- |
| Slot mean flow | | | | | | |
| Expected mean/per-lane nominal-flow band | | | | | | |
| Slot population standard deviation | | | | | | |
| Slot CV | | | | 10% | | |
| R1 total / frozen row band | | | | | | |
| R2 total / frozen row band | | | | | | |
| R3 total / frozen row band | | | | | | |
| R4 total / frozen row band | | | | | | |
| Total recovered output | | | | | | |
| Traceably measured input | | | | | | |
| Collected-versus-measured-input recovery | | | | Frozen band | | |
| Row population standard deviation | | | | | | |
| Row CV | | | | 10% | | |
| Evaporation-blank model result | | | | | | |

### Pressure-drift formula inputs

If absolute value of the signed reference is below the frozen near-zero
threshold, use absolute-drift mode and do not divide by the reference.

| Channel | Analysis window | Signed reference mean | Near-zero threshold | Mode: relative / absolute | Positive normalization scale | Max abs deviation | Relative drift % or N/A | Absolute limit | u/U/k | Guard-banded status | Raw trace |
| --- | --- | ---: | ---: | --- | ---: | ---: | ---: | ---: | --- | --- | --- |
| Common upstream | | | | | | | | | | | |
| R1 | | | | | | | | | | | |
| R2 | | | | | | | | | | | |
| R3 | | | | | | | | | | | |
| R4 | | | | | | | | | | | |
| Waste/backpressure | | | | | | | | | | | |

Persistent position-bias assessment: __________
Interval disposition: __________
Calculation version/reviewer: __________

### DS-09B — Interval gate matrix

| Preparation/cycle | Interval 1 | Interval 2 | Interval 3 | All three independently pass? | Reviewer |
| --- | --- | --- | --- | --- | --- |
| FA-1 preparation 1 | | | | | |
| FA-1 preparation 2 | | | | | |
| FA-1 preparation 3 | | | | | |
| Checkpoint 5 | | | | | |
| Checkpoint 10 | | | | | |
| Checkpoint 25 | | | | | |

### DS-09C — Ordered checkpoint-1 preparation matrix

P1-P3 together are closure/checkpoint 1; there is no duplicate cycle-1 run.
The initial harness remains connected through P1-P3. Fault localization occurs
only in P2. Each preparation ends with a documented drain/reset.

| Preparation | Prechecks | Prime | Five bubble sites | Installed hold | Three nominal intervals | Independent A7-4/5/6/7 approval / reviewer timestamp before downstream | 16 faults | Dye recovery | Post-dye clear | Waste: nominal plus low/high/reverse/pump-off/W4-last | Drain/reset | Harness ID | Status |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| P1 | | | | | | | N/A | | | | | | |
| P2 | | | | | | | | | | | | | |
| P3 | | | | | | | N/A | | | | | | |

HP-4 disposition: __________
Quality reviewer/date: __________

---

## DS-10 — Coupon Characterization And Fault Localization

### DS-10A — One-coupon independent characterization

Duplicate DS-10A for every physical coupon: all sixteen nominal surrogates and
all four low, four high, four blocked, and four bypass coupons.

Coupon ID: ______  Type: ______  Revision/lot: ______

| Reading | Flow condition | Fluid temperature | Pin | Pout | Delta P | Measured flow | Calculated resistance | Expanded uncertainty | Raw file | Status |
| ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- | --- |
| 1 | | | | | | | | | | |
| 2 | | | | | | | | | | |
| 3 | | | | | | | | | | |

| Characterization summary | Entry |
| --- | --- |
| Mean resistance | |
| Standard u / expanded U / coverage factor | |
| Frozen resistance band | |
| Characterization artifact/record | |
| Approved by/date | |
| Final status | |

For a blocked coupon with flow at or below LOQ, do not report infinite or a
spurious finite mean resistance. Report a one-sided maximum-flow/leakage
decision, a resistance lower bound, and the pressure-signature band instead.

| Blocked-coupon decision field | Entry |
| --- | --- |
| Flow LOQ and one-sided upper decision limit | |
| Observed maximum flow/leakage and U/k | |
| Resistance lower bound and method | |
| Required/observed pressure-signature band | |
| Guard-banded blocked-coupon disposition | |

### DS-10B — Characterized-coupon inventory

| Required article | Coupon ID | Characterization record | Frozen band | Status |
| --- | --- | --- | --- | --- |
| Nominal S01 | | | | |
| Nominal S02 | | | | |
| Nominal S03 | | | | |
| Nominal S04 | | | | |
| Nominal S05 | | | | |
| Nominal S06 | | | | |
| Nominal S07 | | | | |
| Nominal S08 | | | | |
| Nominal S09 | | | | |
| Nominal S10 | | | | |
| Nominal S11 | | | | |
| Nominal S12 | | | | |
| Nominal S13 | | | | |
| Nominal S14 | | | | |
| Nominal S15 | | | | |
| Nominal S16 | | | | |
| Low 1 | | | | |
| Low 2 | | | | |
| Low 3 | | | | |
| Low 4 | | | | |
| High 1 | | | | |
| High 2 | | | | |
| High 3 | | | | |
| High 4 | | | | |
| Blocked 1 | | | | |
| Blocked 2 | | | | |
| Blocked 3 | | | | |
| Blocked 4 | | | | |
| Bypass 1 | | | | |
| Bypass 2 | | | | |
| Bypass 3 | | | | |
| Bypass 4 | | | | |

### Concealed randomized one-fault-at-a-time matrix

External manifold ID/revision: __________
Hydraulic-equivalence record/reference nodes/U: __________
Qualification proving all sixteen selections without opening or retorquing the
cassette, disturbing the gasket, mating/demating the harness, changing the
source condition, or revealing the key: __________

Missing equivalence or non-disturbing selection evidence is NOT READY.

#### Required coverage — not execution order and not analyst-facing

| Requirement ID | Required slot | Required class | Assigned coupon ID | Characterization record | Custodian coverage status |
| --- | --- | --- | --- | --- | --- |
| L-S01 | S01 | Low | | | |
| L-S06 | S06 | Low | | | |
| L-S11 | S11 | Low | | | |
| L-S16 | S16 | Low | | | |
| H-S02 | S02 | High | | | |
| H-S05 | S05 | High | | | |
| H-S12 | S12 | High | | | |
| H-S15 | S15 | High | | | |
| O-S03 | S03 | Blocked | | | |
| O-S08 | S08 | Blocked | | | |
| O-S09 | S09 | Blocked | | | |
| O-S14 | S14 | Blocked | | | |
| B-S04 | S04 | Bypass | | | |
| B-S07 | S07 | Bypass | | | |
| B-S10 | S10 | Bypass | | | |
| B-S13 | S13 | Bypass | | | |

Frozen discriminant/signature definition and calculation record: __________
Predictive-classifier instrument, within-coupon, between-coupon/class, and drift
dispersion model/version: __________
Concealed randomized run key/immutable record ID: __________
Independent key custodian: __________

Class separation uses combined standard uncertainty at k = 1:
u_comb = sqrt(u_a^2 + u_b^2 - 2 cov_ab),
z = abs(mu_a - mu_b) / u_comb, with z at least 3.

#### Analyst-facing blinded execution record

This record contains no actual slot, class, coupon, or requirement ID. Hash each
prediction before the next challenge and before any reveal.

| Run order | Blinded challenge ID | Nominal baseline restored | Manifold config/equivalence ID | Closure/gasket/harness untouched and source unchanged | Predicted class/slot | Prediction confidence/signature | Prediction hash/time | No leak/overshoot/invalid state | Trace file | Status |
| ---: | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | | | | | | | | | | |
| 2 | | | | | | | | | | |
| 3 | | | | | | | | | | |
| 4 | | | | | | | | | | |
| 5 | | | | | | | | | | |
| 6 | | | | | | | | | | |
| 7 | | | | | | | | | | |
| 8 | | | | | | | | | | |
| 9 | | | | | | | | | | |
| 10 | | | | | | | | | | |
| 11 | | | | | | | | | | |
| 12 | | | | | | | | | | |
| 13 | | | | | | | | | | |
| 14 | | | | | | | | | | |
| 15 | | | | | | | | | | |
| 16 | | | | | | | | | | |

#### Custodian-only post-reveal reconciliation

Populate only after all analyst predictions are immutable and the reveal time
is recorded.

| Run order | Blinded challenge ID | Revealed requirement/class/slot/coupon | Prediction correct? | Coupon band/post-check | Separation numerator | u_comb/covariance | z | Manifold/non-disturb confirmation | Raw record | Status |
| ---: | --- | --- | --- | --- | ---: | --- | ---: | --- | --- | --- |
| 1 | | | | | | | | | | |
| 2 | | | | | | | | | | |
| 3 | | | | | | | | | | |
| 4 | | | | | | | | | | |
| 5 | | | | | | | | | | |
| 6 | | | | | | | | | | |
| 7 | | | | | | | | | | |
| 8 | | | | | | | | | | |
| 9 | | | | | | | | | | |
| 10 | | | | | | | | | | |
| 11 | | | | | | | | | | |
| 12 | | | | | | | | | | |
| 13 | | | | | | | | | | |
| 14 | | | | | | | | | | |
| 15 | | | | | | | | | | |
| 16 | | | | | | | | | | |

Prediction record finalized before reveal / analyst/date: __________
Key reveal time / custodian/date: __________

Fault-localization disposition: __________
Fluidics lead/date: __________
Independent reviewer/date: __________

---

## DS-11 — Dye Recovery, Physical Hold-Up, And Residual

Duplicate for P1, P2, and P3. Each fraction must be physically and temporally
mutually exclusive. A volume collected at S01-S16 and later routed to W0/W3 is
owned by only one terminal ledger bucket; do not count it twice. Report a
nondetect as less than the reporting limit with a one-sided upper bound, never
as zero.

Preparation/trial: ______  Harness/configuration: ______
Dye batch: ______  Method/instrument/calibration/version: ______
Fluid temperature/density: ______

### Pre-injection system/carryover blank

This is a system blank across the frozen physical sampling map, not merely an
analytical cuvette blank. P2 and P3 must also demonstrate that the preceding
preparation's waste tracer has cleared.

| Preparation | Prior-preparation link or initial baseline | Frozen sampling map/method | Recovery check | Result | Standard u / U / k | Reporting limit / one-sided upper bound | Residual limit | Raw file/hash | Status |
| --- | --- | --- | --- | ---: | --- | --- | ---: | --- | --- |
| | | S01-S16, W0/W3, connector/filter/bubble element/dead legs | | | | | | | |

### Input tracer ledger

| Input field | Value | Standard u | Expanded U/k | Raw record | Status |
| --- | ---: | ---: | --- | --- | --- |
| Input concentration Cin | | | | | |
| Input volume Vin | | | | | |
| Input tracer mass Min = Cin x Vin | | | | | |
| Dilution/preparation factor | | | | | |

### Trial-local analytical controls

| Control ID/type | Expected result/band | Observed result | Recovery or bias | U/k | Raw file/hash | Status |
| --- | --- | ---: | ---: | --- | --- | --- |
| Analytical blank | | | | | | |
| Independent check standard | | | | | | |
| Recovery spike | | | | | | |
| Calibration verification | | | | | | |

### Mutually exclusive recovery fractions

| Fraction/location | Unique ledger bucket / route | Collection ID | Start/end time | Volume | Concentration/result | Dilution | Blank/recovery correction | Corrected tracer mass | U/k | ND reporting limit / upper bound | Residual/visual | Raw file/photo/hash | Status |
| --- | --- | --- | --- | ---: | ---: | ---: | --- | ---: | --- | --- | --- | --- | --- |
| S01 | | | | | | | | | | | | | |
| S02 | | | | | | | | | | | | | |
| S03 | | | | | | | | | | | | | |
| S04 | | | | | | | | | | | | | |
| S05 | | | | | | | | | | | | | |
| S06 | | | | | | | | | | | | | |
| S07 | | | | | | | | | | | | | |
| S08 | | | | | | | | | | | | | |
| S09 | | | | | | | | | | | | | |
| S10 | | | | | | | | | | | | | |
| S11 | | | | | | | | | | | | | |
| S12 | | | | | | | | | | | | | |
| S13 | | | | | | | | | | | | | |
| S14 | | | | | | | | | | | | | |
| S15 | | | | | | | | | | | | | |
| S16 | | | | | | | | | | | | | |
| W0, only if not already counted upstream | | | | | | | | | | | | | |
| W3, only if not already counted upstream | | | | | | | | | | | | | |
| Rinse fraction 1 | | | | | | | | | | | | | |
| Rinse fraction 2 | | | | | | | | | | | | | |
| Drain/leak capture | | | | | | | | | | | | | |
| Residual extract/estimate | | | | | | | | | | | | | |

### Physical volumetric hold-up ledger

This ledger bounds actual retained liquid volume independently of tracer
nonclosure. Do not convert all analytical error into physical hold-up.

| Physical location | Method/inspection | Recovered or directly measured volume | One-sided upper bound | U/k | Visual/analytical threshold/result | Raw record | Status |
| --- | --- | ---: | ---: | --- | --- | --- | --- |
| Connectors | | | | | | | |
| Filter | | | | | | | |
| Bubble element | | | | | | | |
| Dead legs | | | | | | | |
| Gasket interface | | | | | | | |
| Dry structural regions | | | | | | | |
| Unidentified balance | | | | | | | |

| Calculation | Result | Standard u | Expanded U/k | Limit | Status |
| --- | ---: | ---: | --- | --- | --- |
| Quantified input mass Min | | | | | |
| Total recovered mass Mrec = sum unique fractions plus residual | | | | | |
| Signed recovery bias = 100 x (Mrec - Min) / Min | | | | Frozen signed-bias band | |
| Absolute closure error | | | | 10% | |
| Tracer-equivalent unresolved mass/volume | | | | Reporting limit | |
| Physical volumetric hold-up upper bound | | | | min(one-chip-dose volume, 10% condition volume) | |
| Persistent retention result/upper bound | | | | Validated visual/analytical threshold | |

### Post-dye tracer-clear baseline before waste tests

| Method/flush record | Required locations | Decision/reporting limit | Results and ND upper bounds | Raw file/hash | Independent status |
| --- | --- | --- | --- | --- | --- |
| | S01-S16, W0/W3, connector/filter/bubble element/dead legs | | | | |

Trial disposition: __________
Analytical reviewer/date: __________

### Three-trial gate

| Trial 1 | Trial 2 | Trial 3 | All independently pass? | Reviewer/date |
| --- | --- | --- | --- | --- |
| P1 | P2 | P3 | | |

---

## DS-12 — Waste Backflow, Siphon, Relief, And Overflow

Each preparation contributes exactly one nominal reference plus one replicate
of every frozen challenge state. Execute within each preparation in this order:
nominal reference, low, high, reverse, pump-off, then W4 last. Waste work starts
only after that preparation's signed post-dye tracer-clear gate passes. Retain
the W4 row and enter prospectively approved N/A when W4 is not installed.

| Prep | Actual intra-prep state order, with W4 last | State-local reset/nominal-baseline/tracer-clear record before all six states | Unique waste/tracer batch mapping by state | Reviewer/date | Status |
| --- | --- | --- | --- | --- | --- |
| P1 | | | | | |
| P2 | | | | | |
| P3 | | | | | |

| Trial | Prep | State | State-local reset/baseline/tracer-clear | Harness / waste / tracer IDs and route | Elevation/head and U | Reverse pressure and U | Dwell | Pump | Initial/final level | Reverse-tracer decision limit / result / upper bound | Siphon volume / U / reporting limit / status | W4 released / captured / escaped upper bound, each U | Relief/containment route and recovery | Dry structures/electrics | Raw file/photo/hash | Status |
| ---: | --- | --- | --- | --- | --- | --- | ---: | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | P1 | Nominal reference | | | | | | | | | | N/A | | | | |
| 2 | P1 | Low/siphon | | | | | | | | | | N/A | | | | |
| 3 | P1 | High/backpressure | | | | | | | | | | N/A | | | | |
| 4 | P1 | Reverse head/pressure | | | | | | | | | | N/A | | | | |
| 5 | P1 | Pump-off worst state | | | | | | Off | | | | N/A | | | | |
| 6 | P1 | W4 relief/overflow, last | | | | | | | | | | | | | | |
| 7 | P2 | Nominal reference | | | | | | | | | | N/A | | | | |
| 8 | P2 | Low/siphon | | | | | | | | | | N/A | | | | |
| 9 | P2 | High/backpressure | | | | | | | | | | N/A | | | | |
| 10 | P2 | Reverse head/pressure | | | | | | | | | | N/A | | | | |
| 11 | P2 | Pump-off worst state | | | | | | Off | | | | N/A | | | | |
| 12 | P2 | W4 relief/overflow, last | | | | | | | | | | | | | | |
| 13 | P3 | Nominal reference | | | | | | | | | | N/A | | | | |
| 14 | P3 | Low/siphon | | | | | | | | | | N/A | | | | |
| 15 | P3 | High/backpressure | | | | | | | | | | N/A | | | | |
| 16 | P3 | Reverse head/pressure | | | | | | | | | | N/A | | | | |
| 17 | P3 | Pump-off worst state | | | | | | Off | | | | N/A | | | | |
| 18 | P3 | W4 relief/overflow, last | | | | | | | | | | | | | | |

Waste/container ID reconciliation: __________
Containment capacity/result: __________
HP-5 disposition: __________
Safety reviewer/date: __________
Quality reviewer/date: __________

---

## DS-13 — E1 Environmental And E2 Cleaning/Carryover Extensions

### DS-13A — E1 environmental map

Use all sixteen fixed slot-plane probes plus the chamber reference and ambient
probe. Duplicate the map for a separately accepted unloaded confirmation and
the loaded 24-hour exposure; an unloaded result cannot substitute for loaded
evidence.

Configuration record type — enter exactly UNLOADED or LOADED: ______
Power-cycle run: ______
Setpoint/RH/duration/log interval: ______  Probe fixture/revision: ______
RH sensor ID/calibration: ______  Analysis window: ______
Raw native trace/hash: ______

| Position | Probe ID/calibration | Min | Max | Mean | Temporal fluctuation | Spatial contribution | RH result | Evaporation numerator/denominator/result | Condensation/wetting | Standard u / U / k | Status |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- | --- | --- | --- | --- |
| S01 | | | | | | | | | | | |
| S02 | | | | | | | | | | | |
| S03 | | | | | | | | | | | |
| S04 | | | | | | | | | | | |
| S05 | | | | | | | | | | | |
| S06 | | | | | | | | | | | |
| S07 | | | | | | | | | | | |
| S08 | | | | | | | | | | | |
| S09 | | | | | | | | | | | |
| S10 | | | | | | | | | | | |
| S11 | | | | | | | | | | | |
| S12 | | | | | | | | | | | |
| S13 | | | | | | | | | | | |
| S14 | | | | | | | | | | | |
| S15 | | | | | | | | | | | |
| S16 | | | | | | | | | | | |
| Chamber reference | | | | | | | | | | | |
| Ambient | | | | | | | | | | | |

| E1 metric | Frozen definition and analysis window | Result | Limit | Standard u / U / k | Raw record | Status |
| --- | --- | ---: | ---: | --- | --- | --- |
| Stabilization time | | | | | | |
| Temporal fluctuation | | | | | | |
| Spatial range | | | | | | |
| Overshoot | | | | | | |
| Recovery after disturbance/power cycle | | | | | | |
| RH range | | | | | | |
| Evaporation numerator/denominator/result | | | | | | |

Unloaded map gate/reviewer/date: __________
Loaded map gate/reviewer/date: __________

### E1 humid-soak coupon — one loaded exposure

| Coupon ID/material/lot | Exposure record | Pre height/mass/condition | Post height/mass/condition | Delta/U/k | Post-exposure DS-08A leak result | Visual/material result | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| | | | | | | | |

### Exact E1 post-exposure repeats

| Required repeat | Record reference | Result | Independent reviewer/date |
| --- | --- | --- | --- |
| Compression and post-repeat stability, DS-05B | | | |
| Installed liquid integrity, DS-08A | | | |
| Three-interval nominal flow, DS-09 | | | |
| Label scan | | | |
| Window inspection | | | |
| Corrosion/residue/condensation inspection | | | |

E1 disposition: __________

### DS-13B — E2 reusable-surface soil-clean-blank cycle

Duplicate for each of three repeated soil-clean-blank process executions on the
same declared reusable article. These are process repetitions, not three
independent hardware articles.

The A6 wetted harness is disposable and is not cleaned for reuse. S01-S16 in
this sheet identify reusable dry surface regions associated with those
positions.

| Field | Entry |
| --- | --- |
| Cycle/run ID | |
| Reusable article/configuration and material list | |
| Nonbiological tracer and loading | |
| Dwell/dry time | |
| Cleaning agent/lot/concentration/pH/temp | |
| Contact time/agitation/flow | |
| Flush volume | |
| Direct/rinse sampling method | |
| Analytical recovery, LOD/LOQ, reporting/decision limit, carryover limit | |
| Surface sample map/area method | |

| Control ID/type | Expected band/behavior | Result | Recovery/bias | U/k | Reporting limit / upper bound | Raw file/hash | Status |
| --- | --- | ---: | ---: | --- | --- | --- | --- |
| Clean blank | | | | | | | |
| Process blank | | | | | | | |
| Deliberately dirty positive control; expected high response is a method PASS | | | | | | | |
| Recovery spike | | | | | | | |

The disposable wetted harness is out of E2 scope. “Connector exterior” below
means only a declared reusable dry exterior. Gasket/land sampling applies only
to the declared reusable dry gasket/land scope in the frozen material list.

| Reusable surface sampling unit | Material / sampled area | Applied tracer load | Extraction/direct-rinse method | Result | Recovery-corrected result | U/k | Reporting limit / one-sided upper bound | Carryover limit | Visual/material condition | Raw file/hash | Status |
| --- | --- | ---: | --- | ---: | ---: | --- | --- | ---: | --- | --- | --- |
| S01 | | | | | | | | | | | |
| S02 | | | | | | | | | | | |
| S03 | | | | | | | | | | | |
| S04 | | | | | | | | | | | |
| S05 | | | | | | | | | | | |
| S06 | | | | | | | | | | | |
| S07 | | | | | | | | | | | |
| S08 | | | | | | | | | | | |
| S09 | | | | | | | | | | | |
| S10 | | | | | | | | | | | |
| S11 | | | | | | | | | | | |
| S12 | | | | | | | | | | | |
| S13 | | | | | | | | | | | |
| S14 | | | | | | | | | | | |
| S15 | | | | | | | | | | | |
| S16 | | | | | | | | | | | |
| Declared reusable dry gasket/lands | | | | | | | | | | | |
| Declared reusable dry connector exteriors | | | | | | | | | | | |
| Window | | | | | | | | | | | |
| Labels | | | | | | | | | | | |

### Exact E2 post-third-cycle repeats

Fresh disposable harness ID/revision/lot: __________
Fresh-harness topology record/disposition: __________

| Required repeat | Record reference | Result | Independent reviewer/date |
| --- | --- | --- | --- |
| Full closure/compression and post-repeat stability, DS-05B | | | |
| Visual and dimensional inspection | | | |
| Label scan | | | |
| Declared gasket/land inspection | | | |
| Installed liquid integrity with fresh harness, DS-08A | | | |
| Three-interval nominal flow with fresh harness, DS-09 | | | |
| Dye recovery with fresh harness, DS-11 | | | |

E2 cycle disposition: __________
E2 final disposition: __________

---

## DS-14 — Closure And Connector Endurance

Closure 1 is the initial dry/closure checkpoint and P1-P3 campaign; do not
repeat it as a second cycle-1 checkpoint. For single-use connectors, identify
H1 at checkpoint 1 and fresh H5/H10/H25 checkpoint-only harnesses. The separate
gasket reassembly coupon is recorded on DS-08 and does not authorize reconnecting a
single-use full harness.

Standalone gasket-reassembly-coupon ID / DS-08 checkpoint record: __________

| Closure cycle | Gasket ID | Harness ID/role/connector class | Connector count | Open/inspect/reseat/close | Actual torque / driver / time | DS-05B pre/post-repeat stability | Damage/wear/label observation | Part changed? | Gasket-reassembly-coupon ref/result | Required checkpoint records | Status |
| ---: | --- | --- | ---: | --- | --- | --- | --- | --- | --- | --- | --- |
| 1 | | H1 full-core / | | | | | | | | DS-05B/06B/07B/08/09 P1-P3 | |
| 2 | | | | | | | | | | | |
| 3 | | | | | | | | | | | |
| 4 | | | | | | | | | | | |
| 5 | | H5 checkpoint-only or designated reconnect harness / | | | | | | | | DS-05B/06B/07B/08/09 | |
| 6 | | | | | | | | | | | |
| 7 | | | | | | | | | | | |
| 8 | | | | | | | | | | | |
| 9 | | | | | | | | | | | |
| 10 | | H10 checkpoint-only or designated reconnect harness / | | | | | | | | DS-05B/06B/07B/08/09 | |
| 11 | | | | | | | | | | | |
| 12 | | | | | | | | | | | |
| 13 | | | | | | | | | | | |
| 14 | | | | | | | | | | | |
| 15 | | | | | | | | | | | |
| 16 | | | | | | | | | | | |
| 17 | | | | | | | | | | | |
| 18 | | | | | | | | | | | |
| 19 | | | | | | | | | | | |
| 20 | | | | | | | | | | | |
| 21 | | | | | | | | | | | |
| 22 | | | | | | | | | | | |
| 23 | | | | | | | | | | | |
| 24 | | | | | | | | | | | |
| 25 | | H25 checkpoint-only or designated reconnect harness / | | | | | | | | DS-05B/06B/07B/08/09 | |

Any replacement and counter reset: __________
HP-6 endurance disposition: __________
Operator/date: __________
Independent reviewer/date: __________

---

## DS-15 — Deviation, Failure, Invalidation, And Retest

Use one sheet per deviation.

| Field | Entry |
| --- | --- |
| Deviation ID | |
| Run ID/test stage/timestamp | |
| Status: FAIL / INVALID / HOLD | |
| Failure class: S / I / M / C / D | |
| Objective observed facts | |
| Stop trigger | |
| Immediate safe response | |
| Affected article/component/sample IDs | |
| Affected raw files and downstream data | |
| Last verified-good hold point | |
| Quarantine location/status | |
| Evidence preserved | |
| Control/calibration/environment review | |
| Assignable method cause demonstrated? | |
| Root cause | |
| Correction/corrective action | |
| Drawing/material/software/configuration change | |
| Impact assessment | |
| Minimum restart scope from protocol table | |
| Approved one-time confirmation retest | |
| Retest run ID/result | |
| Recurrence? | |
| Final disposition | |
| Original failure retained in final report? | |

Operator/date: __________
Engineering owner/date: __________
Metrology lead/date: __________
Safety reviewer/date: __________
Quality approval/date: __________

---

## DS-16 — Gate Matrix And Final Release

| Gate | Required evidence | Result | Deviation/retest links | Independent reviewer/date |
| --- | --- | --- | --- | --- |
| HP-0 controlled inputs | DS-00/01, immutable DS-02A1 downselection and DS-02A2 final-capability snapshots, and exact HP0-E12 manifest with configuration ID, revision, and SHA-256 | | | |
| SAF-E01 limited empty-bench commissioning authorization | HP0-E08/HP0-E12, exact as-designed P&ID/BOM/fluid/pressure boundary, authorized limits, expiry triggers, and independent SR/QR signatures | | | |
| SAF-E02 as-built functional/gasket-coupon authorization | SAF-E01 plus DS-02B as-built dry inspection and bounded commissioning results; exact rig/configuration, permitted limits, expiry triggers, and independent SR/QR signatures | | | |
| HP-1 functional bench/coupons/controls | HP0-E12 and SAF-E01/02; immutable DS-02B/DS-02C/DS-03A/DS-05A/DS-06A snapshots; immutable DS-07A pre-P1 checks 1-3 only; DS-08 sealed blank/artificial leak/isolated gasket/burst/reassembly qualification only; DS-10A/B characterization only; exact HP1-E09 manifest with predecessor IDs/hashes and configuration identity | | | |
| HP-2 incoming metrology | HP0-E12; immutable DS-03B, DS-04A pre-hardware, and DS-04B post-hardware/incoming-fit snapshots; vendor FAI; independent D8 report; exact HP2-E05 manifest with predecessor IDs/hashes and configuration identity | | | |
| HP-3 fit/compression/topology | Separate immutable DS-04C, DS-05B, and DS-06B snapshots linked to their HP-2/HP-1 predecessors | | | |
| P1 local A7-4/5/6/7 approval before downstream work | DS-07/08/09/09C | | | |
| P2 local A7-4/5/6/7 approval before downstream work | DS-07/08/09/09C | | | |
| P3 local A7-4/5/6/7 approval before downstream work | DS-07/08/09/09C | | | |
| A7-4 prime | DS-07 | | | |
| A7-5 bubble and six optical method checks | DS-07 | | | |
| A7-6 integrity | DS-08 | | | |
| A7-7 nominal flow / HP-4 | DS-09 | | | |
| A7-8 fault localization | DS-10 | | | |
| A7-9 pre-injection blank, dye recovery/physical hold-up, and post-dye clear | DS-11 | | | |
| A7-10 waste / HP-5 | DS-12 | | | |
| A7-11 endurance / HP-6 | DS-14 | | | |
| E1 environmental, if in scope | DS-13A | | | |
| E2 cleaning/carryover, if in scope | DS-13B | | | |
| All deviations closed | DS-15 index | | | |
| Raw file/hash/photo/video manifest complete | Final manifest | | | |
| Correct claim level and limitations stated | Final report | | | |
| HP-7 complete record, closed evidence, and final release | DS-16 | | | |

### Detached prerequisite-gate manifests

Duplicate exactly one subsection below into a separate executed gate-manifest
page when that gate closes. Do not fill all three on one evolving DS-16 page.
Each manifest references predecessor sidecar hashes one way and receives its
own detached sidecar hash after signing. Signed source forms and gate manifests
must not be reopened to add downstream hashes.

#### HP0-E12

Manifest ID/revision: __________
Detached sidecar-manifest ID/location assigned before signing: __________
Configuration identity and predecessor sidecar hashes reconciled: __________
HP-0 disposition: __________
Design authority/date: __________
Fluidics validation lead/date: __________
Metrology lead/date: __________
Safety reviewer/date: __________
Independent quality reviewer/date: __________
Records custodian/date: __________

#### HP1-E09

Manifest ID/revision: __________
Detached sidecar-manifest ID/location assigned before signing: __________
HP0-E12, SAF-E01/02, configuration identity, and scoped predecessor sidecar hashes reconciled: __________
HP-1 disposition: __________
Design authority/date: __________
Fluidics validation lead/date: __________
Metrology lead/date: __________
Safety reviewer/date: __________
Independent quality reviewer/date: __________
Records custodian/date: __________

#### HP2-E05

Manifest ID/revision: __________
Detached sidecar-manifest ID/location assigned before signing: __________
HP0-E12, configuration identity, vendor FAI, independent D8, and predecessor sidecar hashes reconciled: __________
HP-2 disposition: __________
Design authority/date: __________
Metrology lead/date: __________
Independent quality reviewer/date: __________
Records custodian/date: __________

### Final record manifest

Use one row per file or immutable record; never place multiple unhashed files in
one aggregate cell. Duplicate rows until the manifest is complete. After DS-16
is signed, create a detached final sidecar manifest that also records the hash
of the signed DS-16 file; never reopen DS-16 to add its own hash.

| Record class | Test/preparation/cycle linkage | File path or record ID | Capture/export timestamp | SHA-256 | Immutable storage ID | Reviewer/date |
| --- | --- | --- | --- | --- | --- | --- |
| | | | | | | |
| | | | | | | |
| | | | | | | |
| | | | | | | |
| | | | | | | |
| | | | | | | |
| | | | | | | |
| | | | | | | |
| | | | | | | |
| | | | | | | |
| | | | | | | |
| | | | | | | |

Detached final sidecar-manifest ID/location assigned before DS-16 signing: __________
Manifest completeness reviewer/date: __________

### HP-7 final record review

| HP-7 requirement | Evidence/result | Independent reviewer/date | Status |
| --- | --- | --- | --- |
| Every required gate and individual lane/trial/checkpoint/control passed | | | |
| Every raw/native file, immutable export, calculation, photo/video, and hash reconciled | | | |
| Every deviation/invalidation/retest closed with original result retained | | | |
| Claim-level article/harness counts match DS-00 | | | |
| No aggregate score or later result masks an earlier failure | | | |
| Residual claim limitations are accurately stated | | | |
| Unresolved evidence is NONE | | | |

HP-7 disposition: __________
Independent quality/release reviewer/date: __________

### Final disposition

Claim level: __________
Disposition: PASS — eligible for media-only planning / HOLD / REJECT
Residual claim limitations: __________
Unresolved evidence (must be NONE for PASS): __________
Design authority/date: __________
Fluidics validation lead/date: __________
Metrology lead/date: __________
Safety reviewer/date: __________
Independent quality/release reviewer/date: __________
