# Connected water-bench modules — interface revision W0

Ticket T-21BC0885. Scope: one water/dye engineering lane. No cell, pathogen,
vector, culture, assay or incubation procedure is specified here.

This is the working design map for the modules discussed with Alex. A module
can be a purchased assembly, a dry printed fixture or software; it need not be
a separate box or controller. Dimensions of unselected purchased parts remain
provisional. CAD completion, physical assembly and measured performance are
separate states.

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
