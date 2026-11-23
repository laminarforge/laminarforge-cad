# Connected water-bench modules — interface revision W6

Ticket T-21BC0885. Scope: one water/dye engineering lane. No cell, pathogen,
vector, culture, assay or incubation procedure is specified here.

This is the working design map for the modules discussed with Alex. A module
can be a purchased assembly, a dry printed fixture or software; it need not be
a separate box or controller. Dimensions of unselected purchased parts remain
provisional. CAD completion, physical assembly and measured performance are
separate states.


## Relationship to the intended research instrument

Alex's objective is usable research equipment. The fixtures in this document are
engineering and measurement assets serving that objective; they are not the
completed research box. The engineering recommendation is delegated. Alex need
not choose among old architectures or invent pump limits and other technical
specifications. Supplier evidence and qualified laboratory input, where needed,
must establish the real interfaces and acceptance requirements.

The reconciliation index is `models/research_instrument_completion.json`.
It distinguishes the existing 16-slot cassette contract, heated-water prototype,
and single-lane fluid bench. The A0 cassette contract is preserved; recent bench
work neither supersedes A0 nor establishes an adopted final pump/BOM. The
November 3 liquid-service requirements record (A-12A0063A, T-9F64B1F6) supports
a finite useful liquid-handling operation with an appropriate carrier/vessel
interface, manual initial loading and honest delivery evidence. It leaves the
complete hardware architecture unselected.

The next engineering priority is an actual compatible vessel/connector/actuator
combination and its complete support-equipment costs, grounded in the intended
measurement/environment requirements. A 75×25 mm water coupon, the Rev C
127.76×85.48 mm payload contract, and a heated microplate surrogate are different
interfaces; their evidence must not be substituted for one another.

F1 now recommends a commercial NE-501 integration candidate with an independent
mass-observation path; see [cost and compatibility gates](water-bench-low-cost.md#f1--path-toward-the-actual-instrument-2026-11-22)
and `models/research_fluid_service_selection.json`. This is not an adopted
research vessel, rated complete fluid assembly or purchase release.

A new CAD task must close an identified instrument interface or verification gap.
More provisional fixtures alone do not retire component selection, manufacturing,
physical commissioning or intended-use requirements. The earlier seven-block
bench estimate is not an estimate of all remaining work for the research box.
No reliable total remaining-part count is established yet.

## Connected system

Liquid: source syringe → pump → replaceable tubing/fittings → transparent
single-channel coupon → outlet suspended above collection vial.

Measurement: vial and its nest → independent scale → timestamped run record.
The outlet support is carried by the bench, never by the scale. Tubing must not
touch the vial, nest or pan. The scale supports only its own weighing assembly,
the vial, the nest and collected liquid; tare the empty vial and nest together.

Control: operator → bench controller → selected pump interface. Scale and any
additional instruments report independent observations. A command acknowledgement
does not prove motor motion, stopped motion or delivered flow.

The wet zone contains the coupon tray and collection area. Place the controller,
motor driver and power connections outside the actual spill/splash envelope.
This revision does not establish an ingress rating or electrical protection.

## Module contracts and design queue

| ID | Module | Current design basis | Interfaces still to establish | Next observable checkpoint |
| --- | --- | --- | --- | --- |
| M01 | Pump/source | Historical `syringe_pump_standalone` is frozen concept geometry; PHIL/open-well research is another earlier path, not an adopted closed-flow controller | Actual pump, syringe barrel/flange/plunger dimensions, travel, driver, stop/limit behavior, connector | Select and measure the actual assembly; observe commanded displacement and independent delivered mass |
| M02 | Tubing/fittings | Dry guides use provisional 4 mm OD; wetted parts are purchased components | Tube ID/OD, bend radius, lengths, fitting interfaces, dead volume and material compatibility | Assemble and photograph a fully identified water-only harness; inspect bends, connections and free outlet |
| M03 | Flow coupon | Adjustable water-only review geometry based on Rev D0, 75×25×4 mm nominal stack | Bottom/bond stack, bought fittings, ports, actual thickness, fabrication/inspection definition | Advisor/vendor review followed by measured water-only first article; no fabrication release in this revision |
| M04 | Dry coupon dock | Candidate 01 already generated and verified; tray, cradle, two guides | Actual coupon/tube fit and print/material choice | Physical dry fit without forcing coupon or pinching tube; inspect floor and sockets |
| M05 | Receiver/measurement | New adjustable on-pan vial nest plus independently supported outlet frame | Actual scale body/pan, accuracy/resolution/interface, vial dimensions, tube anchoring and support stability | Fit selected scale/vial; demonstrate no mechanical contact or load path from tubing/frame into weighing assembly |
| M06 | Controller/electrical | New adjustable generic dry board carrier; not an electronics design | Board/driver, actual hole pattern and connectors, power ratings, limit inputs, stop circuit and cable restraint | Review actual wiring/power/stop design; verify clearances and observed motor stop before commanded water run |
| M07 | Instrumentation/observation | Independent scale and visible coupon/outlet are first observations | Camera/light mount; pressure, bubble or wetness instrument selection and interfaces | Record real readings/images with instrument identity and calibration status; introduce no fictional telemetry |
| M08 | Control/run records | State and observation contract below; no bench hardware driver implemented by this CAD work | Selected device protocols, time synchronization, persistent record location and operator interface | One real command/readback/stop sequence with timestamped mass readings and recorded faults |

M05 and M06 are dry fixture candidates, not a claim that the scale, controller
board, pump or firmware exists physically. `models/water_bench_modules.json`
is the machine-readable queue and interface index for this revision.

## Receiver CAD contract

Use the existing enrolled target `water_transfer_dock --module receiver`, with
`models/water_bench_receiver.toml`. Its nominal scale envelope is 120×120 mm,
pan 100×100 mm at 40 mm above the bench, and vial Ø24×50 mm. These are generic
layout values, not manufacturer dimensions or a shopping selection.

Derived default geometry:

- Vial nest: Ø32.8 mm base, 11 mm overall height; pocket Ø24.8 mm.
- Independent frame: 168×168×121 mm; 12 mm nominal gap around the scale body.
- Vial bottom at Z43 mm, vial top at Z93 mm, intended tube tip at Z101 mm.
- Crossbar underside at Z113 mm; route the tube 12 mm below it to preserve the
  8 mm nominal air gap above the vial. Tube itself is not modeled.

The frame rests on the bench and surrounds the scale without attachment.
The nest rests on the pan only. The crossbar passage organizes the tube but is
not validated strain relief; an off-scale anchor, slack route and actual slip,
kink, flex and stability checks remain necessary. Scale cable routing must
also avoid loading the weighing mechanism. The integral overhead bridge needs
slicer/orientation/support review and physical stability assessment. No
support-free print or anti-tip claim is made.

Outputs separate the nest and frame print candidates, a dry assembly and
explicit NOT_FOR_MANUFACTURE scale/vial references. Reports retain transformations,
clearances, configuration/executable hashes, actual-STL sampled checkpoints
and regenerated-byte verification.

## Controller carrier CAD contract

Use `water_transfer_dock --module controller`, with
`models/water_bench_controller.toml`. Nominal generic board: 80×50×1.6 mm,
mount pitch 64×34 mm, four 3.4 mm clearance holes. Carrier: 90×60×15 mm.
Board underside is at Z13 mm, top at Z14.6 mm; base underside is at Z4 mm.
No board, driver, connector, supply, pin assignment or wire gauge is selected.

The four standoffs have through-clearance holes, not printed threads. Select
fastener/nut/washer and insulating hardware after measuring the actual board
and underside component envelope. Hardware is not modeled. Verify that screws
cannot touch the table or conductive features. The two end windows are generic
connector clearances, not strain relief or proof of USB/power connector fit.
The carrier is open, has no wet-area or electrical rating, and is not a protective
enclosure. Do not energize hardware based on this dry model.

## Water-only coupon review contract

Use `water_transfer_dock --module coupon`, with
`models/water_bench_coupon.toml`. This makes review meshes for the proposed
Rev D0 topology: one inlet, one chamber and one outlet. The 75×25×4 mm outer
interface is asserted to match the current dock. Internal channel/chamber/port
dimensions can vary within checked bounds without changing Rust.

Default body 3 mm and bottom 1 mm are provisional; the zero-thickness bond
interface is a geometry assumption. Defaults retain the earlier proposed
0.50×0.20 mm channel and 14×6×0.20 mm chamber. Ports at X±27 mm have a 2 mm
review bore, which is **not matched to a selected fitting**. These dimensions
do not establish manufacturability, acceptable pressure, bond integrity or flow.

The generator checks port/channel/chamber intersection requirements, nominal
roof and edge/bond lands, actual mesh bounds and sampled openings/solid lands.
It also exports an intended fluid-space mesh for connectivity inspection.
The underside-view mesh is the same body turned over, not another part.
Every coupon mesh is named REVIEW or VIEW_ONLY. Neither these STL files nor a
faceted conversion to STEP supplies a fabricator-approved analytic solid or a
controlled drawing. Keep the earlier advisor/vendor review gate in
`microfluidic_chip_revd_first_article_review_rfq.md`; do not send this as a
released fabrication package or print it as the functional fluidic article.

## Controller and record design

Proposed states: `idle → armed → running → stopped`; an explicit operator pause
or any detected communication/instrument/limit fault leaves `running`.
`fault` remains latched until acknowledged and prerequisites are rechecked.
Reconnect must not restart motion automatically. An independent hardware stop
and appropriate limit protection must be designed with the selected drive.
These are requirements; this revision does not implement or verify them.

Each run needs:

- Run ID; configuration revision/hash; actual module/part IDs; operator; UTC time.
- Pump/syringe identity, calibration identity, commanded settings with units,
  command time and acknowledgement; observed motor state if available.
- Actual scale readings with mass units, instrument identity, monotonic sample
  time, UTC mapping and calibration/settling/overload flags. Preserve raw readings,
  including drift, instead of silently replacing them with commanded flow.
- Empty vial/nest tare, collection start/stop times and independently observed
  collected mass. Record the liquid/temperature basis before any mass-to-volume
  conversion; mass alone is not a measured volume.
- Images/observations of bubbles, outlet position, leaks and physical changes.
- Stop/fault/reconnect events, operator interventions and final disposition.

Do not assign acceptance numbers from a convenient CAD default. Agree the
actual flow range, uncertainty, repeatability, run duration, pressure ceiling
and stop criteria after selecting hardware and reviewing the coupon/fittings.

## Integration gates

1. Identify the actual syringe/pump, tubing/fittings, coupon, vial, scale and board.
   Replace provisional envelopes with measurements or controlled vendor drawings.
2. Review the two new dry fixtures in the selected slicer and physically fit
   them. Check tube, scale, board, fastener and cable clearances.
3. Review pump/electrical/stop design and observe actual stop and reconnect
   behavior. Do not infer hardware safety from a UI acknowledgement.
4. Assemble one complete water-only path with independent collection measurement.
   Record leakage, bubbles, collected mass, elapsed time and operator actions.
5. Repeat real runs against agreed criteria; retain failures and drift evidence.
   Geometry verification alone does not complete this gate.

Once that lane is understood, design enclosure/environment interfaces,
multi-position carrier connections and automated handling around measured
requirements. Existing larger cassette/environment concepts remain references;
they are not required purchases or validated assemblies for this first lane.
Partner-led model/assay and facility work is a separate track.

## Execute and verify

Publish Rust changes to canonical main through `submit_work`. Select its exact
accepted CAD pipeline job, keep the checkout at that SHA, then use the supported
receipt-authorized tool (no direct compiler or manual build):

```json
{
  "repo": "laminarforge-cad",
  "action": "execute",
  "bin": "water_transfer_dock",
  "profile": "dev",
  "pipeline_job_id": "<accepted CAD job UUID>",
  "args": ["--module", "receiver", "--config", "models/water_bench_receiver.toml", "--output-dir", "<new ORICO candidate directory>"]
}
```

Repeat with `--verify` and the same directory. Use `controller` with its matching
config for M06, or `coupon` and `water_bench_coupon.toml` for review geometry.
Omitting `--module` preserves the existing dock behavior and config.
Keep all generated files and diagnostics on the mounted ORICO filesystem. Inspect
the actual mesh/preview outputs and preserve native tool receipts with each candidate.

Pump redesign findings and required measurements: [pump review](water-bench-pump-review.md).

Hardware ownership is now confirmed: none selected. See the [sourced hardware basis](water-bench-hardware-basis.md) for the proposed direct-serial first lane, alternatives, measurement budget and commercial-fit changes still required. The generic fixtures remain provisional.

Alex chose lowest practical cost. The [W2 cost basis](water-bench-low-cost.md) prioritizes manual water-path bring-up, a lower-cost balance candidate and a separate 4.7 mm dry guide config. Motorized operation and full coupon fabrication costs remain open.

## Independent adjustable outlet support

`water_transfer_dock --module outlet` uses
`models/water_bench_outlet_support.toml`. This is a separate support path that
does not wrap a guessed balance body. A 112×100×8 mm base carries a slotted
40×6 mm vertical rail, total height 220 mm. The separate arm reaches 180 mm
from its clamp-face center and uses two nominal M4 clearance paths 20 mm apart.
The configured bolt center is Z150 mm; arm underside Z135 mm; intended 20 mm
tube projection gives a nominal tip at Z115 mm. Position the actual balance
and vial independently, then measure the real gap. No instrument fit is implied.

The base must be clamped or anchored to a suitable independent bench/board.
The footprint is not qualified against cantilever tipping. Select actual
fasteners, washers/nuts, anchors or clamps and inspect arm deflection, slip,
retention and tube bend radius before use. Fasteners/tubing are omitted from
geometry. Slot endpoints are not usable bolt-center endpoints: retain at least
half the clearance diameter; bracket/base/rail clearance also constrains travel.
The two separate printed candidates need slicer and material review; no load
capacity or support-free print is established. This does not modify a balance
or require holes drilled in Alex's bench.

## Non-actuated source saddle pair

`water_transfer_dock --module source` uses
`models/water_bench_source_saddle.toml`. Two separate 24×58×22 mm saddle
candidates support a generic Ø20×100 mm barrel envelope on a common X axis,
with their centers 60 mm apart. Each has four nominal M3 base holes and a
transverse strap passage. The two STL files A/B represent two physical copies
of the same saddle. A reference barrel and configured assembly are review-only.

The model checks clearance for the entered washer envelope around the raised
saddle, base edge lands, separation between tie and bolt passages, floor
material below the saddle, support spacing and reference length. Unknown fields,
missing values and invalid dimensions fail before publication. These checks do
not establish an actual syringe fit: its flange, plunger and outlet are absent.
There is no axial restraint and no actuator-load qualification. Independently
anchor the bases and choose straps against measured source dimensions; do not
use a loose barrel as a pump drive assembly. The historical pump redesign stays
open. This support is useful for organizing a manual water-only source while
that drive decision remains unresolved.

The [engineering-plan recorder](water-bench-records.md) implements configuration/evidence storage only. The [observation plan](water-bench-observation-plan.md) defines M07 image/readout interfaces; physical acquisition and M08 device control remain pending.

## Combined dry layout

`water_transfer_dock --module bench` uses `models/water_bench_layout.toml`.
It embeds the existing source, dock, outlet and generic collection parameters
under explicit sections, with a shared layout section. The collection center
is derived from the outlet tube axis. The independent stand base, source and
dock have minimum plan-envelope gaps; the nominal tube-tip/vial air gap is
checked separately. Parameter validation precedes geometry publication.

The initial planning envelope is 610×284×220 mm. The nominal air gap is 22 mm;
source-to-dock gap 102.4 mm, dock-to-collection gap 92.4 mm, and stand-base to
collection gap 64 mm. These values are configured envelope relationships, not
measured installed clearances. The source support and collection references
have no actual syringe/flange/outlet or commercial scale/draft shield detail.
The dock contains its reference coupon blank; no fluidic fabrication is implied.

The five VIEW_ONLY exports provide the combined assembly and its four posed
subassemblies. They are not print orientations or a fabrication release. Use
the individual module PRINT candidates for slicer/physical review. Tubing,
fittings, screws, clamps, ties, cables and actual liquid connections remain
unmodeled. Full collision/access review, anchoring, purchased-part fit and
water performance remain open. The layout establishes neither a pump nor a
camera/electronics purchase requirement.
