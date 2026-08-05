# 16-Slot Cassette HP-0 Through HP-2 Phased Purchase Plan

Purchase plan ID: LF-CAS-A7-PR002
Revision: A0
Ticket: T-4D2B2A36
Source procurement register: [LF-CAS-A7-PR001 revision A0](sixteen_slot_cassette_hp0_hp2_procurement_list.md)
Governing readiness checklist: [LF-CAS-A7-RC001 revision A0](sixteen_slot_cassette_hp0_hp2_readiness_checklist.md)
Pricing basis date: 2026-08-04
Currency: 2026 USD, before sales tax except through the controlled reserve
Disposition: BUDGETARY PHASING RECOMMENDATION — NOT PURCHASE AUTHORIZATION

## Decision

Use one opening authorization, IA-1, with a maximum exposure of **$97,168**.
It contains $85,168 of item-level high-end commitments and a separately
controlled $12,000 reserve. The plan therefore remains **$2,832 below the
$100,000 ceiling** even if every IA-1 line commits at its current high estimate
and the entire reserve is used.

IA-1 is deliberately split into two release groups. Phase 0A addresses the
earliest HP-0 input and metrology blockers. Phase 1A buys or rents only the
core control, measurement, preparation, closure, and safety equipment after
the applicable HP-0 envelopes are frozen. Inclusion in IA-1 is not permission
to order before a row's release trigger.

Do not use IA-1 for the full-size cassette, functional/fault articles,
single-use harnesses, custom validation stations, destructive testing, or
the EQ-17 independent large-part first-article inspection campaign. Those
items remain in later, independently approved phases.

## Budget Architecture

| Authorization or phase | Purpose | Direct estimate | Authorization treatment |
| --- | --- | ---: | --- |
| IA-1 / Phase 0A | HP-0 controlled inputs and preliminary metrology | $17,600-$39,800 | Included; release now only as each R0 condition closes. |
| IA-1 / Phase 1A | Core selected bench, measurement, and safety equipment | $33,418-$45,368 | Included; conditional R1 releases only. |
| IA-1 direct line-item subtotal | Thirteen controlled rows | **$51,018-$85,168** | Sum of the source-register ranges; $85,168 is the direct commitment cap. |
| IA-1 central reserve | Tax, freight, calibration shipping, and approved within-row quote variance | **$12,000** | Held centrally; not preallocated and not available for new scope. |
| IA-1 maximum exposure | Direct high cap plus full reserve | **$97,168** | Hard stop; includes open POs and non-cancelable deposits. |
| Unallocated ceiling headroom | Difference from $100,000 | **$2,832** | Cannot be assigned without a controlled plan revision. |
| Phase 1B | HP-1 measurement-method and physical-interface closeout | $17,149-$21,700 | Separate later authorization. |
| Phase 2A | HP-1 functional, safety, and fault-system build | $52,000-$122,500 | Separate later authorization after R2 release. |
| Phase 2B | HP-1 gasket, leak, dye, and destructive controls | $25,300-$57,800 | Separate later authorization after R2 release. |
| Phase 3 | Production mechanical first article plus independent inspection | $30,025-$73,075 | Separate later authorization after R3 and the commercial risk gate. |
| Full direct baseline | All 20 EQ and 21 HW rows, without duplicates | **$175,492-$360,243** | Reconciles exactly to LF-CAS-A7-PR001; later phases do not imply approval. |

The $12,000 IA-1 reserve is not the full-program contingency. Every later
authorization requires refreshed quotes and its own contingency decision. The
full-program working envelope remains the $210,590-$432,292 range in
LF-CAS-A7-PR001 until controlled quotes replace it.

Committed spend means the maximum non-cancelable obligation, not only cash
already invoiced. It includes open purchase orders, rental minimums,
nonrefundable deposits, service cancellation fees, tax, freight, calibration
shipping, and supplier-held material that LaminarForge is obligated to buy.

## Phase 0A — Controlled Inputs And Preliminary Metrology

This phase is first because it produces HP0-E04 inputs needed to correct the
release candidate, close D7, and freeze the ranges used to select later
equipment. Appoint the responsible and independent roles before committing a
row.

| ID | Preferred route and exact quantity | Earliest blocked gate/evidence | Release trigger | Estimate | Lead after release |
| --- | --- | --- | --- | ---: | --- |
| EQ-18 | **Rent or contract**, do not buy, one qualified D8 tool set/campaign. Structure one capped campaign with preliminary input metrology and later D8 support; any second mobilization requires a revised quote. | HP-0 HP0-E04; later HP-2 HP2-E02 | ML/GD approve method, calibration identity, range, U, and access; use R0 reservation only before D8. | $7,500-$13,500 | 3-8 wk |
| HW-07 | Inventory first, then **buy** 16 intended Rev C chips plus every declared spare; four spares and 20 total are the planning case. | HP-0 HP0-04/HP0-E04 | PR/DA declare spare count and controlled chip identity; credit only conforming on-hand articles. | $8,000-$20,000 for 20 | 6-10 wk |
| HW-02 | **Buy** representative M4 closure, receiver, and 6 mm datum samples; place the exact controlled-lot balance only after D7 selection. | HP-0 HP0-E04/05; later HP-2 | Candidate identity approved for preliminary work; final lot waits for D7 and DS-02A1. | $100-$300 | 1-2 wk |
| HW-05 | **Outsource** gasket applications review and buy one identified prototype/first production lot only; no multi-material campaign. | HP-0 D2/D4, HP0-E04/05; later HP-1/2 | GA selected; compound, geometry, tolerance, splice, compatibility, and lot evidence included in the quote. | $2,000-$6,000 | 6-10 wk |

Phase 0A cap: **$39,800**. Start no-cost, revocable capability RFIs for EQ-17,
HW-01, EQ-01/02/03, and Phase 2 service providers in parallel, but do not pay a
deposit or release an orderable production file. Any reservation fee counts
against IA-1 when it becomes non-cancelable.

## Phase 1A — Core Bench And Measurement Backbone

Phase 1A remains inside IA-1 but is not an immediate shopping list. Release
each row only after its range, rating, interface, calibration, raw-data, and
projected-uncertainty requirements are recorded in the applicable HP0-08
through HP0-14 evidence and DS-02A1 downselection.

| ID | Preferred route and exact quantity | Earliest blocked gate/evidence | Release trigger | Estimate | Lead after release |
| --- | --- | --- | --- | ---: | --- |
| EQ-02 | **Buy** six synchronized, liquid-compatible gauge-pressure channel assemblies. | HP-0 HP0-15; HP-1 HP1-05 | HP0-08 freezes all six ranges, basis, common clock, and U target. | $6,600 | 1-19 wk; carry 19 until range is frozen |
| EQ-01 | **Buy** one pressure-limited controller system and rated source/relief/isolation package. | HP-0 HP0-15; HP-1 HP1-03/04 | HP0-08/09 freeze MOP, hard cap, source, relief, reservoir, and logging interfaces; SR reviews. | $8,000-$12,000 | 6-10 wk |
| EQ-04 | **Buy** one synchronized DAQ/logger system. | HP-0 HP0-15; HP-1 HP1-03/05 | EQ-01/02/08 signals and clock/raw-export requirements frozen. | $5,300-$5,600 | 1-2 wk if distributor stock is secured; direct supply may be 12-28 wk |
| EQ-03 | **Rent or contract**, do not buy for FA-1, one controlled pressure-reference set spanning all selected ranges. | HP-0 HP0-15; HP-1 HP1-05 | HP0-08 freezes installed, Pgasket, Pburst, ceiling, accuracy, and module ranges. | $2,500-$5,000 | 1-3 wk |
| EQ-06 | **Buy** one analytical balance and one low/mid/high check-weight set. | HP-0 HP0-15; HP-1 HP1-05 | HP0-09/13 freeze the smallest net mass, capacity, readability, and U bands. | $4,100-$4,500 | 2-6 wk |
| EQ-08 | **Buy** two calibrated four-wire Pt100 probes. | HP-0 HP0-15; HP-1 HP1-05 | HP0-07 freezes fluid/ambient conditions and EQ-04 input compatibility. | $468-$568 | about 4 wk plus calibration handling |
| EQ-12 | **Buy** one base calibrated preparation/QC set; exclude the optional viscosity method. | HP-0 HP0-15; HP-1 HP1-06 | HP0-07/13 freeze recipe, density method, volume tools, and check ranges. | $3,200-$4,200 | 1-3 wk |
| EQ-19 | **Buy** one calibrated torque driver. | HP-0 HP0-15/D7; HP-1/2 | HW-02 closure choice and D7 torque range/tolerance frozen away from endpoints. | $850-$1,200 | 1-3 wk |
| EQ-20 | **Buy** one core safety-station set, excluding HW-18. | HP-0 HP0-07/15; HP-1 | HP0-07/08 freeze chemistry, inventory, pressure, PPE, spill, electrical, and depressurization needs; SR approves. | $2,400-$5,700 | 1-3 wk materials; 4-6 wk if facility work is required |

Phase 1A cap: **$45,368**. The preferred retained assets are items that form
the repeatable bench backbone or require configuration-specific integration.
The pressure reference stays rented because FA-1 needs campaign-limited,
range-specific accredited capability and purchase would add approximately
$7,900-$10,400 above the rental allowance.

## Phase 1B — Measurement Methods And Short-Lead Interfaces

Authorize Phase 1B separately after DS-02A1 records the exact methods and
interfaces. Selection and written quotes may support HP-0; physical receipt,
installation, and commissioning must occur before the applicable HP-1 rows.

| ID | Preferred route and exact quantity | Earliest blocked gate/evidence | Release trigger | Estimate | Lead after release |
| --- | --- | --- | --- | ---: | --- |
| EQ-10 | **Buy/build**, do not rent, one fixed four-view optical system; its validated geometry and timebase must remain stable through the campaign. | HP-0 selection; HP-1 HP1-E05 | HP0-06/10/11 freeze witness zones, smallest prohibited bubble/leak, FOV, resolution, lighting, and clock proof. | $7,500-$9,000 | 4-6 wk |
| EQ-05 | **Outsource/contract first**, one independent measured-input reference method; buy a dedicated source balance only if the contracted method cannot meet U, range, schedule, or raw-data requirements. | HP-0 selection; HP-1 HP1-05 | HP0-07/09 freeze input range, interval, independence, and U. | $3,600-$5,000 | 2-6 wk |
| EQ-11 | **Outsource dye analysis first** if a qualified service returns raw absorbance, blanks, standards, checks, LOD/LOQ, recovery, identity, and turnaround; otherwise buy one visible spectrophotometer within the same cap. | HP-0 selection; HP-1 HP1-E07 | HP0-13 freezes dye, wavelength, range, controls, sample count, hold time, and analytical decision rule. | $4,700-$4,950 | 4-6 wk |
| HW-11A | **Buy** four matched reservoirs/bags. | HP-0 selection; HP-1/2 | Capacity, material, lot, headspace/vent, rating, connector, and maximum inventory frozen. | $184-$600 | 3-6 wk |
| HW-13 | **Buy** 16 service-position connectors/fittings or controlled caps. | HP-0 selection; HP-1/2 | G0-G3, M0-M6, and W0-W4 states, panel geometry, tubing, rating, valving, and dead volume frozen. | $400-$1,200 | 1-4 wk |
| HW-18 | **Buy/build** one segregated W0/W1/W3/W4 destination set. | HP-0 selection; HP-1 | HP0-12 freezes waste map, anti-backflow/anti-siphon method, capacity, and containment; SR reviews. | $615-$800 | 1-2 wk |
| HW-19 | **Outsource printing** for the first controlled label set unless compatible in-house equipment and validated stock already exist. | HP-0 selection; HP-2 identity | Identity scheme, cleaning chemistry, humidity exposure, substrate, adhesive, and print durability frozen. | $150 | 1-2 wk |

Phase 1B cap before its own contingency: **$21,700**.

## Phase 2A — Functional, Safety, And Fault-System Build

Release no Phase 2A order until the exact HP0-E06 drawings, P&ID, BOM,
materials, reference nodes, and methods reach R2 at HP0-15A. Dry receipt and
commissioning do not authorize wet work; SAF-E01 and SAF-E02 remain separate.

| ID | Preferred route and exact quantity | Gate blocked | Release trigger | Estimate | Lead after release |
| --- | --- | --- | --- | ---: | --- |
| EQ-09 | **Buy/build** one calibrated injector serving exactly five selectable locations; contract calibration where useful. | HP-0/1 | R2 HP0-E06 plus frozen gas volume, P/T correction, selector, and U. | $2,500-$4,000 | 4-8 wk |
| EQ-13 | **Outsource custom fabrication/integration** of one measured waste/backpressure station; retain the accepted station. | HP-0/1 | R2 P&ID/BOM, positions, head/backpressure ranges, W4 stimulus, drain, and containment interfaces released; SR reviews. | $4,600-$9,500 | 4-8 wk |
| EQ-14 | **Outsource fabrication** of one installed-system safety rig and retain it for repeat commissioning. | HP-0/1 | R2 pressure boundary, hard cap, interlock, relief, isolation, containment, and derating package released; SR approves. | $7,600-$16,000 | 6-10 wk |
| EQ-07 | **Buy/fabricate** 16 covered vessels, 16 keyed positions, and at least three matched blanks/positions. | HP-1 | R2 volume/material/cover/visibility/tare/evaporation method released. | $900-$2,000 | 1-4 wk |
| HW-10A | **Outsource fabrication** of one fixture mechanical set; retain the accepted assembly. | HP-1 | R2 datum nest, pressure-bar, deck, mounts, tray, and FAI requirements released. | $8,000-$20,000 | 6-10 wk |
| HW-10 | **Buy/integrate or outsource** one externally accessible 16-selection fault manifold. | HP-0/1 | R2 blind key, reference nodes, wetted volume/material/rating, and non-disturbing switching proof released. | $6,000-$15,000 | 8-12 wk |
| HW-11 | **Outsource assembly** of four complete controlled single-use harnesses; do not reduce quantity without prospective reconnect qualification. | HP-0/1/2 | R2 topology, materials, lots, ratings, proof/leak record, and four checkpoint identities released. | $16,000-$40,000 | 6-10 wk |
| HW-12 | Receive four complete topologies **included in HW-11**; no separate PO. | HP-0/1/2 | Same release and receipt control as HW-11. | $0 incremental | 6-10 wk |
| HW-08 | **Outsource fabrication and characterization** of 16 serialized nominal functional surrogates. | HP-1/2 | R2 footprint, flow-through path, finite resistance band, materials, ratings, U, and identity released. | $4,000-$9,600 | 4-8 wk |
| HW-09 | **Outsource fabrication and characterization** of 16 serialized fault coupons: four each low, high, blocked, and bypass. | HP-1 | R2 class bands, row/column map, one-sided blocked limit, materials, ratings, and U released. | $2,400-$6,400 | 4-8 wk |
| HW-17 | Receive the vessel/blank inventory **included in EQ-07**; no separate PO. | HP-1 | Same release and receipt control as EQ-07. | $0 incremental | 1-4 wk |

Phase 2A cap before its own contingency: **$122,500**.

## Phase 2B — Gasket, Leak, Dye, And Destructive Controls

Phase 2B follows the same R2 control but is commercially separated so the
highest-risk destructive and analytical work cannot consume functional-rig
funds. Release the physical coupon/control work before the destructive service
slot so the service quote is tied to accepted article identities.

| ID | Preferred route and exact quantity | Gate blocked | Release trigger | Estimate | Lead after release |
| --- | --- | --- | --- | ---: | --- |
| EQ-15 | **Outsource as a physically separate destructive-test service** for FA-1; do not buy a dedicated rig unless no qualified service meets ratings, raw capture, safety, independence, and schedule. | HP-0/1 | R2 destructive method, pressure hierarchy, hard ceiling, source volume, capture rate, shielding, containment, and abort package released; SR approves. | $12,600-$27,000 | 8-12 wk |
| EQ-16 | **Outsource** one leak-method characterization/control service set. | HP-0/1 | Leak medium, threshold, trapped volume, dye tray/witness method, controls, and raw-data requirements frozen. | $1,700-$3,300 | 5-8 wk |
| HW-06 | **Outsource fabrication** of one production-intent coupon set for the single selected gasket material/lot. | HP-1 | D4, D7, exact gasket lot, groove/finish/stop/fastener geometry, FAI, and serialization released. | $2,000-$6,000 for 1 set | 3-6 wk |
| HW-14 | **Outsource fabrication** of one matched sealed-fixture blank. | HP-1 | Boundary, fittings, materials, temperature, trapped volume, and no-leak verification released. | $500-$1,500 | 3-6 wk |
| HW-15 | **Outsource fabrication and independent calibration** of one permanent artificial-leak control. | HP-1 | Smallest rejectable leak, medium, pressure, temperature, stability, and calibration method frozen. | $500-$2,000 | 4-8 wk |
| HW-16 | **Outsource build/integration** of one quantitative dye/mass-closure and hold-up station; retain the accepted station. | HP-1 | R2 fraction map, switching/reset, mass ledger, materials, ratings, and no-hidden-mixing proof released. | $8,000-$18,000 | 8-12 wk |

Phase 2B cap before its own contingency: **$57,800**. Phase 2A plus 2B
reconciles to **$77,300-$180,300**.

## Phase 3 — Production First Article And Independent Inspection

The formal R3 prerequisite is HP0-15B: matching released D0-D9 drawings, STEP
files, final D7, D8, BOM, and manifests. This plan adds a stricter commercial
risk hold: do not release HW-01 until the selected-lot production-intent
gasket/coupon evidence has no unresolved critical failure. Removing this hold
requires a prospective controlled revision; it cannot be waived ad hoc. The
hold changes procurement timing, not a technical acceptance requirement.

| ID | Preferred route and exact quantity | Gate blocked | Release trigger | Estimate | Lead after release |
| --- | --- | --- | --- | ---: | --- |
| HW-01 | **Outsource machining/finishing** of one serialized matched carrier/lid/window/bulkhead/dock set with vendor FAI. | HP-2 | R3 HP0-15B, no unresolved selected-lot gasket/coupon critical, controlled RFQ/PO, and independent inspection slot confirmed. | $25,000-$60,000 | 6-10 wk after released package |
| EQ-17 | **Outsource**, do not buy, one independent large-part CMM/vision inspection campaign separate from vendor FAI. | HP-2 HP2-E02 | D8 released; accepted machine envelope, access, U, programming, raw data, calibration identity, and chain of custody. | $4,000-$9,000 | 2-6 wk |
| HW-04 | **Outsource fabrication/buy** one retained-window hardware set. | HP-2 | D3 retention, load path, seal, cleaning, visibility, installation, and inspection requirements released. | $1,000-$4,000 | 4-8 wk |
| HW-03 | **Buy** six M5 dock fastener stacks only if D5 retains them; otherwise obtain prospective QR-approved N/A. | HP-0/2 | D5 exact length/head/material/retention/engagement/torque/finish released. | $25-$75 | 1-2 wk |

Phase 3 cap before its own contingency: **$73,075**.

## Buy, Rent, Outsource, And Defer Decisions

| Route | Controlled rows | Decision |
| --- | --- | --- |
| Buy and retain in IA-1 | EQ-01, EQ-02, EQ-04, EQ-06, EQ-08, EQ-12, EQ-19, EQ-20; HW-02 and HW-07 inputs | These are repeatable bench-backbone assets or controlled physical inputs. Buy only after exact selection; an on-hand credit requires the same identity, calibration, rating, and evidence as a purchase. |
| Rent/contract in IA-1 | EQ-03 and EQ-18 | Campaign-specific calibrated capability is cheaper and avoids owning range-specific or specialized metrology. Purchase is deferred. |
| Outsource specialized service | EQ-05, EQ-11 when qualified, EQ-15, EQ-16, EQ-17; HW-05 engineering and HW-15 calibration | Use only providers that return required raw/native data, calibration identity, methods, chain of custody, and change control. Supplier FAI never replaces EQ-17 independence. |
| Outsource controlled fabrication; retain deliverable | EQ-13, EQ-14; HW-01, HW-04, HW-06, HW-08, HW-09, HW-10A, HW-11, HW-14, HW-16 | Released drawings/BOMs and controlled POs govern these builds. Outsourcing does not relax incoming inspection or acceptance. |
| Buy/build after initial authorization | EQ-07, EQ-09, EQ-10; HW-10, HW-11A, HW-13, HW-18, HW-19 | Needed only after their methods and interfaces freeze; short leads or configuration dependence make early ordering poor risk. |
| Included, no separate spend | HW-12 with HW-11; HW-17 with EQ-07 | Maintain separate identity and acceptance rows but prohibit duplicate POs. |
| Explicitly deferred outside IA-1 | EQ-05, EQ-07, EQ-09, EQ-10, EQ-11, EQ-13, EQ-14, EQ-15, EQ-16, EQ-17; HW-01, HW-03, HW-04, HW-06, HW-08, HW-09, HW-10, HW-10A, HW-11, HW-11A, HW-12, HW-13, HW-14, HW-15, HW-16, HW-17, HW-18, HW-19 | Deferred means not initially funded, not removed from LF-CAS-A7-RC001. Each returns only at the phase trigger above. |

The following remain deferred outside the 41-row core baseline:

- the $5,500-$6,000 viscosity bath/viscometer method unless HP0-07 retains
  viscosity-adjusted fluid;
- purchasing EQ-03 instead of renting it, which would add approximately
  $7,900-$10,400 for FA-1;
- owned large-volume CMM or broad D8 metrology capacity in place of EQ-17/18
  services;
- additional gasket-material coupon sets at $2,000-$6,000 per material/lot;
- a second full cassette set, confirmation lots, production scale, and extra
  run consumables; and
- all E1 environmental-extension and E2 hazardous-chemistry equipment.

Reconsider an owned pressure reference, metrology asset, spectrophotometer, or
destructive rig only after at least two later approved campaigns have defined
the same range/method and a documented 24-month total-cost comparison includes
purchase, calibration, maintenance, storage, software, operator competency,
downtime, and independence. Lower sticker price alone is not a justification.

## IA-1 Commitment Ledger And Release Rules

SL must maintain a live IA-1 ledger with, at minimum, row ID, supplier,
quote/version, route, committed amount, tax, freight, calibration shipping,
deposit/cancellation exposure, expected receipt, gate trigger, receipt state,
and remaining direct/reserve/headroom balances.

Apply these rules to every commitment:

1. Record the full non-cancelable exposure before issuing the PO, rental, or
   service authorization. Stop if direct commitments would exceed $85,168 or
   total exposure including approved reserve use would exceed $97,168.
2. Use the $12,000 reserve only for tax, freight, calibration shipping, or a
   documented within-row quote variance. It cannot fund a later-phase row,
   quantity increase, optional method, facility expansion, or acceptance-limit
   change.
3. Savings on one row remain uncommitted. They may cover another IA-1 row only
   within that row's existing high cap; they do not advance a deferred row.
4. A quote above a row's high estimate requires an identical-scope rebid,
   route comparison, and prospective PR/QR plan revision. Do not silently use
   reserve to hide a scope or specification mismatch.
5. Inventory is a $0 procurement credit only after ML/FV/GD, as applicable,
   confirms exact identity, quantity, condition, range, rating, calibration,
   uncertainty, software/firmware, compatibility, and raw-data capability.
6. A rental or service deposit counts at its maximum nonrefundable amount. A
   no-cost capacity hold stays at $0 only while it is fully revocable.
7. Every custom PO requires no unapproved substitution, material/process and
   sub-supplier change notification, raw data, calibration IDs, lot/serial
   traceability, FAI where applicable, NCR control, and chain of custody.
8. A provider that will not return required raw evidence or preserve the
   assigned independence is disqualified. Missing capability leaves the gate
   NOT READY; it does not authorize an unqualified fallback.
9. Lead-time pressure never permits ordering HW-01 or an R2 article from an
   unreleased package. Rebid, reserve a later slot, or accept the schedule slip.
10. Any selected range, material, interface, method, software, or supplier
    change reopens the affected checklist rows and invalidates downstream
    commitments until prospective review closes.

## Authorization Checkpoints

| Checkpoint | Minimum evidence before commitment | Approval record |
| --- | --- | --- |
| IA-1 opening | Named GD, ML, FV, TO, SR, QR, RC, and GA roles in addition to the current PR/DA/SL assignments; independence record complete; ledger opened; row caps loaded; Phase 0A quotes match the controlled scope. | PR authorizes envelope; SL accepts ledger duty; QR confirms release logic. |
| Each Phase 0A order | Row-specific R0 trigger, exact quantity/identity, traceability, and preliminary method acceptance. | Technical owner plus SL; QR release check; SR where safety-relevant. |
| Each Phase 1A order | Applicable HP0-08 through HP0-14 envelope and DS-02A1 exact downselection; projected U and calibration plan. | Technical owner plus SL; independent QR; SR for pressure/safety rows. |
| Phase 1B | HP0-15 downselection and method/interface freeze; refreshed written quotes and separate phase contingency. | New PR authorization; QR and applicable SR review. |
| Phase 2A/2B | R2 HP0-E06/HP0-15A package and dry-proof evidence; exact custom RFQs and incoming plans. | Separate authorization per subphase; QR and SR approval. |
| Phase 3 | R3 HP0-15B plus commercial gasket/coupon risk gate; vendor FAI and independent inspection POs both ready. | Separate PR authorization; DA/GD/GA/ML evidence; independent QR release. |

This plan changes only procurement timing and commercial route. It does not
change any quantity, specification, sample size, hold point, acceptance limit,
independence requirement, or failure-response rule in LF-CAS-A7-RC001,
LF-CAS-A7-PR001, or LF-CAS-A7-P001.
