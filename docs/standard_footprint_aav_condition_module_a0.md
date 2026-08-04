# Standard-Footprint AAV Condition Module Scale-Down Study

Ticket: T-4380A1AA

This document records a compact-module and local-coupon study that supports the
active 16-slot, 4 x 4 first physical cassette. It does not replace or supersede
`docs/sixteen_slot_cassette_a0_interface_spec.md`, and its mockups do not satisfy
an integrated 16-slot first-article gate.

The controlling rule is:

```text
precision local
structure cheap
scale by repetition
```

This is an architecture and mechanical interface decision record. It is not a biological protocol, sterile-barrier claim, live-cell release gate, AAV handling procedure, clinical plan, or vendor drawing.

## Study Decision

LaminarForge should use compact coupons and standard-footprint module studies to
keep precision local and retire expensive interface risks before committing a
full-size first-article carrier/dock plate.

The companion scale-down concept is a small standard-footprint condition module:

- One physical module equals one AAV condition.
- AAV conditions must not be mixed inside one module or one disposable wetted path.
- Precision belongs in the local chip, seal, connector, datum, and imaging interface.
- Large trays, racks, hotels, or holders are cheap organizers only.
- Scale comes from repeating validated modules, not from machining one huge high-precision plate.

The 16-slot cassette remains the controlling first-build architecture. The
compact module is a risk-reduction and possible future scale architecture whose
lessons may be integrated only through an explicit A0 revision.

## Why This Study Exists

The 16-slot cassette first article combines many jobs in one large mechanical
package:

- chip pocket array,
- imaging alignment,
- gasket compression,
- lid/dock datums,
- leak/drain features,
- tubing organization,
- service bulkhead,
- condition identity.

That creates cost and adoption risk if precision is allowed to spread across the
active 699.04 x 541.92 mm carrier or 869.04 x 691.92 mm dock. Compact coupons let the team
validate local interfaces first while retaining the active full-cassette
contract.

The study therefore applies a pattern used by practical organ-chip systems:
keep precision near the chip/module and make outer organization replaceable,
inexpensive, and non-critical wherever the A0 cassette interfaces allow it.

## Experimental Semantics

The active cassette and compact study preserve the same core experimental intent.

| Item | Requirement |
| --- | --- |
| Condition identity | One module equals one AAV capsid/promoter/payload/dose/timing/media condition. |
| Condition isolation | No shared AAV routing between different condition modules. |
| Slot/readout role | Chips or lanes inside a module are same-condition cell/readout zones, not separate AAV candidates. The useful target is 16 same-condition zones per AAV condition after one-zone validation is proven. |
| Wetted path | Disposable or separately validated chip/fluid components only. Reusable structural parts stay dry unless explicitly validated. |
| Validation order | No-cell mechanical, leak, flow, bubble, and ID validation comes before media-only, live-cell, or AAV planning. |
| Environment boundary | Incubator or chamber hardware controls bulk temperature, CO2, humidity, and gas recovery. The module proves local equivalence, fluid behavior, sealing, imaging, and traceability inside that bulk environment. |
| Scale strategy | Validate one zone/module first, then scale the same local precision pattern toward 16 same-condition zones and repeated modules/trays. Do not enlarge one precision sealing plate as the default. |

## Compact Study Geometry Direction

The compact study uses a single-condition module in the SBS/SLAS microplate
footprint:

```text
one AAV condition
  -> one 127.76 x 85.48 mm SLAS-footprint module
  -> one local readout zone for first no-cell validation
  -> scalable target of 16 same-condition tissue-chip/readout zones
  -> local gasket/seal/connector precision
  -> optical bottom/window plus fiducials
  -> condition ID on the physical module
  -> cheap tray/rack/hotel for scale
```

The module should still support both:

- a microscope-stage adapter for imaging and inspection, and
- an SBS/SLAS-style tray or rack for handling, labeling, storage, and future automation.

Do not make the tray, rack, or adapter the precision sealing fixture. The SLAS
footprint is the module envelope and handling standard; the actual precision is
still local to the chip/seal/connector/readout zone.

### Preferred Study Concept

Start with a one-condition SLAS-footprint module:

- one local tissue-chip or Rev C-class readout zone for first mechanical/no-cell validation,
- a preserved path to 16 same-condition tissue-chip or Rev C-class readout zones for the useful screening module,
- one local lid/clamp,
- one local gasket/seal stack,
- one local connector face,
- one local optical window path,
- local datum features tied to the chip and imaging plane,
- one visible condition ID and module ID zone.

Then place multiple modules into a cheap tray, hotel, incubator shelf adapter, or
automation rack when scale is needed.

This keeps the hard features small while preserving a route toward lab-compatible handling.

The project should not optimize for a dead-end one-chip-only device. One-zone
testing is the learning sequence; 16 same-condition zones per AAV condition is
the scale target unless later evidence proves a different readout count is
better.

Microscope-slide geometry remains valuable for quick coupons, one-chip imaging
prototypes, and small seal tests. It is not the controlling first architecture
unless the SLAS-footprint module proves too material-heavy or optically awkward.

## Footprint Candidates

| Candidate | Use | Strength | Risk | A0 position |
| --- | --- | --- | --- | --- |
| Single SLAS-footprint condition module | One condition module in the 127.76 x 85.48 mm envelope | Lab-friendly handling, direct condition identity, avoids oversized custom carrier | Can become too material-heavy if precision spreads across the whole body | Preferred compact study direction |
| Slide-class cartridge | Imaging coupon and one-chip prototype | Cheap, small, stage-friendly, easier local precision | Needs external tray for high-throughput handling | Prototype/coupon path |
| SBS/SLAS tray adapter | Holds repeated modules in a lab-friendly envelope | Compatible with common plate handling and labeling expectations | Must not become a precision sealing plate | Preferred support format |
| 4-zone intermediate module | Intermediate same-condition readout density | Better within-condition replication while keeping validation manageable | More seal and flow complexity than a one-zone coupon | Second step after one-zone local stack |
| 16-zone same-condition module | Useful target for broad cell/readout coverage under one AAV condition | Captures the desired cell/readout panel without mixing AAV candidates | Must avoid becoming a large precision monolith | Target architecture after local zones validate |
| 16-chip 4 x 4 cassette | Active first physical condition unit | Preserves the committed readout count and integrated cassette interfaces | Large-plate cost and tolerance risk must be retired through coupons and DFM | Controlling first-build architecture |

## Local Precision Boundary

Only these interfaces are allowed to require tight tolerance or controlled surface finish:

- chip seating datum or local chip stop,
- gasket groove, gasket land, compression stop, and seal surface,
- lid/clamp alignment to the chip and gasket,
- connector face alignment and strain relief,
- optical window and fiducial relationship to the chip,
- module ID/readout registration needed for imaging and run records.

These features should fit within a small local zone, replaceable insert, or coupon inside the SLAS-footprint module. If a tolerance requirement extends across a multi-module tray, that is a design smell and should be redesigned back into the local module.

## Cheap Structure Boundary

These parts must stay non-critical:

- tray,
- rack,
- storage hotel,
- stage adapter base,
- transport frame,
- bulk organizer,
- incubator shelf adapter,
- future automation nest.

They may position modules coarsely, protect them, make them easy to label, or support imaging access. They must not create the seal, set gasket compression, correct chip flatness, or define AAV condition isolation.

## Fabrication Direction

Prototype sequence:

1. Freeze the module boundary: one module, one AAV condition, module-carried ID.
2. Print Bambu FDM interface coupons for chip fit, clamp envelope, connector retention, tray fit, handling, and ID placement.
3. Choose one bought connector family and one disposable tubing assumption before designing real fluid ports.
4. Use resin only for dry detail, mold masters, or non-contact feature checks where FDM cannot honestly represent geometry.
5. Use a small CNC or otherwise validated dry insert only where the chip, gasket, connector, or optical datum requires it.
6. Validate gasket compression on a local coupon before expanding chip count.
7. Build a one-condition one-chip module with a disposable non-biological fluid path.
8. Run no-cell validation for leaks, priming, dead volume, bubbles, connector handling, optical access, barcode scan, and repeatability.
9. Expand to a four-zone intermediate module only after the one-zone stack passes no-cell gates.
10. Expand toward the 16-zone same-condition module only after local flow/seal/imaging equivalence is proven.
11. Add a cheap SLAS tray, hotel, or microscope adapter after the module-level interfaces are stable.

Cost-control rules:

- Never buy or quote a large precision plate to learn a local seal or connector lesson.
- Keep machined metal, if needed, to a small insert or local module body.
- Keep recurring wetted-path cost visible from the first BOM.
- Avoid custom sterile connector assumptions until bought connector options and training burden are documented.
- Do not let the tray become the sealing fixture.
- Buy connectors instead of inventing ports.
- Use hard stops and witness features instead of operator torque feel.
- Put condition ID on the module, not only on the tray position.
- Reject shared gaskets, shared manifolds, and multi-condition tubing harnesses.

### Material And Process Notes

| Process or material | Allowed use | Boundary |
| --- | --- | --- |
| Bambu FDM | Dry coupons, outer shells, handles, covers, tray pockets, rack organizers, strain-relief mockups | Not a wetted path, primary optical datum, or final sealing surface. |
| Resin printing | Dry small-detail checks, mold masters, non-contact prototype inserts | Treat as biologically risky because of extractables, cure uncertainty, autofluorescence, brittleness, and sterilization uncertainty. |
| Small CNC insert | Local chip seat, gasket hard stop, connector datum, optical datum | Must stay local to one condition. If it grows into a tray-scale part, the architecture is failing. |
| Laser-cut acrylic/PETG | Dry clamp plates, clear covers, spacers, adapters | Do not rely on laser-cut stacks as real fluidic seals unless separately validated. |
| Cast/die-cut gasket | Early gasket coupons and local module seals | Must include hard stops and visible compression witness features. |
| Bought fluid connectors | Real fluid interfaces and tubing transitions | Standardize early; do not custom-print real fluid ports. |
| PCB/SLAS carrier layer | Dry identity, fiducials, DataMatrix/NFC/EEPROM, possible future heater/sensor pads | Carrier layer must not become the sealing architecture. |

### Red Flags

Stop and redesign the compact study if it starts to require:

- tray-scale precision instead of local chip/seal/connector precision,
- tray-required sealing,
- a large multi-condition gasket,
- a shared AAV manifold,
- one clamp frame compressing many conditions,
- optical datum features on the rack instead of the module,
- row/column-only condition identity,
- a multi-condition tubing harness,
- a local validation coupon that depends on the full high-tolerance carrier.

## No-Cell Validation Gates

A module is not eligible for media-only, cell, or AAV planning until it passes no-cell checks with water, dye, or another non-biological surrogate fluid.

Minimum gates:

- incoming dimensional inspection,
- dry assembly and compression witness,
- chip or surrogate-chip seating without force,
- connector topology check,
- prime path confirmation,
- visible bubble clearing,
- leak/pressure-decay or dye leak inspection,
- flow repeatability or collected output check,
- dead-volume and recovery/flush estimate,
- waste path and backflow check,
- imaging-window clearance check,
- module ID, condition ID, chip ID, and run-record linkage.

Failures must block escalation. Operator judgment must not override a failed bench gate.

Rack position is metadata only. If flow, sealing, imaging, or leak behavior
changes because of rack slot position, the module or rack interface has failed
the architecture rule.

Bulk environment is an incubator responsibility during this phase. The module
does not need to be its own incubator for the first validation path. It must,
however, make local equivalence measurable under incubator conditions:

- all zones fill and clear predictably,
- edge and center zones have comparable fluid exposure,
- bubbles are visible or cleared before a run proceeds,
- dead volume is bounded per zone,
- imaging position and focus references are repeatable,
- condition/module/zone identity is preserved.

Minimum run record fields:

- run ID,
- module ID,
- module design revision,
- chip or insert revision,
- gasket/seal revision,
- disposable wetted-path kit ID when separate,
- tray/rack ID and slot position,
- assembly timestamp,
- condition placeholder ID for no-cell testing,
- leak result,
- fill/flow/repeatability result,
- bubble-clearing result,
- dead-volume/recovery observation,
- imaging record ID,
- pass/fail status,
- failure mode and disposition.

## Relationship To The Active 16-Slot Work

The 16-slot documents remain controlling. The compact study reuses their local
interface questions without changing their role.

Any compact coupon that claims to represent the active cassette interface must
use the shared A0 contract rather than stale local constants: the carrier base
body is 699.04 x 541.92 x 24.00 mm with 31.35 mm true overall Z; chip
protrusion, both seal-land families, all hard stops, and the lid underside
closure plane are 7.35 mm above carrier top; lid grooves are 1.80 mm deep x
3.20 mm wide for the 2.40 mm gasket at 25% nominal squeeze. The active stop
pattern is nine 4.00 mm-diameter inter-slot stops plus 4.00 mm-wide perimeter
web stops, with no seal intersection. The 16 fasteners sit outside the gutter
at side X = +/-332.52 mm and front/rear Y = +/-247.96 mm, with matching 3.30 mm
carrier pilot receivers. Side service reliefs are 7.00 mm high and below
closure. Dock coupons seat the carrier directly on datum A and contact nominal
rear, left, and front carrier edges with the rear/left rail inner faces and
front lip. These values constrain cassette-representative coupons; they do not
force a future compact module to inherit the full cassette envelope.

| Existing work | Role in this study |
| --- | --- |
| `sixteen_slot_cassette_a0_interface_spec.md` | Controlling first-build contract. Compact work may not change its locked interfaces. |
| `sixteen_slot_cassette_a1_carrier_spec.md` | Source for datum, pocket, label, drain, and handling questions to validate locally. |
| `sixteen_slot_cassette_a2_lid_clamp_spec.md` | Source for compression and window questions to exercise with local clamp logic. |
| `sixteen_slot_cassette_a3_gasket_spec.md` | Source for gasket compression vocabulary and representative local coupons. |
| `sixteen_slot_cassette_a6_disposable_fluid_path_spec.md` | Source for one-condition routing and port naming; compact paths remain disposable. |
| `sixteen_slot_cassette_a7_no_cell_validation_fixture_spec.md` | Source for bench gates that compact coupons can retire before integrated testing. |
| `sixteen_slot_cassette_print_coupon_plan.md` | Active printer-learning precursor for the 16-slot first article. |

## Open A0 Questions

These questions should be resolved before compact CAD can support a proposed
future architecture revision or feed validated interface changes back into A0:

- What is the smallest one-zone coupon that honestly validates the local precision island?
- What geometry lets the first useful module scale to 16 same-condition zones without making the whole module a precision plate?
- What Rev C chip dimensions and tolerances should control the first module pocket?
- Which connector family is the lowest-cost credible starting assumption for a disposable path?
- Does the optical workflow prefer slide-first geometry strongly enough to make SLAS compatibility secondary?
- Should the first tray hold 4, 8, 12, or 16 condition modules?
- What part, if any, must be CNC machined instead of printed for honest seal validation?
- What is the minimum run record schema for condition ID, module ID, chip ID, and measurement traceability?

## Companion CAD Package

The compact study package should stay small:

- one-zone local module body coupon,
- local lid/clamp coupon,
- local gasket compression coupon,
- connector-face mockup,
- microscope-stage adapter,
- cheap multi-module SLAS tray/rack outline,
- 16-zone same-condition layout study that proves scale-up without committing to a giant precision carrier.

Any compact-study release should prove geometry and assembly logic only. It
does not replace the integrated 16-slot first article and must not claim sterile
use, live-cell readiness, AAV readiness, or biological validity.
