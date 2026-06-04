# Standard-Footprint AAV Condition Module A0

Ticket: T-4380A1AA

This document replaces the giant 16-slot cassette as the first physical architecture direction for LaminarForge AAV condition screening hardware.

The controlling rule is:

```text
precision local
structure cheap
scale by repetition
```

This is an architecture and mechanical interface decision record. It is not a biological protocol, sterile-barrier claim, live-cell release gate, AAV handling procedure, clinical plan, or vendor drawing.

## Decision

LaminarForge should not make a large custom precision carrier/dock plate the core dependency of the AAV condition screening platform.

The first scalable architecture is a small standard-footprint condition module:

- One physical module equals one AAV condition.
- AAV conditions must not be mixed inside one module or one disposable wetted path.
- Precision belongs in the local chip, seal, connector, datum, and imaging interface.
- Large trays, racks, hotels, or holders are cheap organizers only.
- Scale comes from repeating validated modules, not from machining one huge high-precision plate.

The prior 16-slot cassette remains useful as a reference for slot semantics, labels, validation questions, and future automation rack planning. It is no longer the first lab-dependency architecture.

## Why This Pivot Exists

The 16-slot cassette first article combined too many jobs in one large mechanical part:

- chip pocket array,
- imaging alignment,
- gasket compression,
- lid/dock datums,
- leak/drain features,
- tubing organization,
- service bulkhead,
- condition identity.

That creates an avoidable cost and adoption risk. A 648 x 467 mm class carrier and larger dock can become a large precision-machined fixture. That is the wrong dependency if downstream labs must be able to adopt, repeat, repair, and scale the system without a high-dollar custom plate.

The better architecture follows the pattern used by many practical organ-chip systems: keep precision near the chip/module and let the outer holder be replaceable, inexpensive, and non-critical.

## Experimental Semantics

The old and new architectures preserve the same core experimental intent.

| Item | Requirement |
| --- | --- |
| Condition identity | One module equals one AAV capsid/promoter/payload/dose/timing/media condition. |
| Condition isolation | No shared AAV routing between different condition modules. |
| Slot/readout role | Chips or lanes inside a module are readouts or technical replicates under the same condition, not separate AAV candidates. |
| Wetted path | Disposable or separately validated chip/fluid components only. Reusable structural parts stay dry unless explicitly validated. |
| Validation order | No-cell mechanical, leak, flow, bubble, and ID validation comes before media-only, live-cell, or AAV planning. |
| Scale strategy | Run more modules or cheap trays of modules; do not enlarge one precision sealing plate as the default. |

## First Geometry Direction

The first replacement A0 direction is a single-condition module using the
SBS/SLAS microplate footprint:

```text
one AAV condition
  -> one 127.76 x 85.48 mm SLAS-footprint module
  -> 1-4 local tissue-chip/readout zones
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

### Preferred Module Concept

Start with a one-condition SLAS-footprint module:

- one to four local tissue-chip or Rev C-class readout zones,
- one local lid/clamp,
- one local gasket/seal stack,
- one local connector face,
- one local optical window path,
- local datum features tied to the chip and imaging plane,
- one visible condition ID and module ID zone.

Then place multiple modules into a cheap tray, hotel, incubator shelf adapter, or
automation rack when scale is needed.

This keeps the hard features small while preserving a route toward lab-compatible handling.

Microscope-slide geometry remains valuable for quick coupons, one-chip imaging
prototypes, and small seal tests. It is not the controlling first architecture
unless the SLAS-footprint module proves too material-heavy or optically awkward.

## Footprint Candidates

| Candidate | Use | Strength | Risk | A0 position |
| --- | --- | --- | --- | --- |
| Single SLAS-footprint condition module | One condition module in the 127.76 x 85.48 mm envelope | Lab-friendly handling, direct condition identity, avoids oversized custom carrier | Can become too material-heavy if precision spreads across the whole body | Preferred first geometry direction |
| Slide-class cartridge | Imaging coupon and one-chip prototype | Cheap, small, stage-friendly, easier local precision | Needs external tray for high-throughput handling | Prototype/coupon path |
| SBS/SLAS tray adapter | Holds repeated modules in a lab-friendly envelope | Compatible with common plate handling and labeling expectations | Must not become a precision sealing plate | Preferred support format |
| 4-chip mini-module | Higher readout density under one condition | Better within-condition replication | More seal and flow complexity than a one-chip coupon | Second step after one-chip local stack |
| 16-chip monolithic cassette | Legacy high-density condition unit | Captures old readout count in one object | Large precision plate, high cost, poor downstream adoption | Reference only for now |

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
9. Expand to a two-chip or four-chip module only after the one-chip stack passes no-cell gates.
10. Add a cheap SLAS tray, hotel, or microscope adapter after the module-level interfaces are stable.

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

Stop and redesign if the module direction starts to require:

- a precision-machined 16-slot carrier,
- tray-required sealing,
- a large multi-condition gasket,
- a shared AAV manifold,
- one clamp frame compressing many conditions,
- optical datum features on the rack instead of the module,
- row/column-only condition identity,
- a multi-condition tubing harness,
- a first validation build that depends on a large high-tolerance custom plate.

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

## Relationship To The 16-Slot Work

Existing 16-slot docs and CAD remain useful, but their role changes.

| Existing work | New role |
| --- | --- |
| `sixteen_slot_cassette_a0_interface_spec.md` | Legacy high-density reference and requirement source. Not the first physical architecture. |
| `sixteen_slot_cassette_a1_carrier_spec.md` | Reference for datum, pocket, label, drain, and handling questions. Do not inherit large-plate assumptions. |
| `sixteen_slot_cassette_a2_lid_clamp_spec.md` | Reference for compression and window questions. Convert to local clamp logic. |
| `sixteen_slot_cassette_a3_gasket_spec.md` | Reference for gasket compression vocabulary. Convert to local gasket coupon. |
| `sixteen_slot_cassette_a6_disposable_fluid_path_spec.md` | Reference for one-condition routing and port naming. Convert to local disposable path. |
| `sixteen_slot_cassette_a7_no_cell_validation_fixture_spec.md` | Reference for bench validation gates. Convert to module-level gates. |
| `sixteen_slot_cassette_print_coupon_plan.md` | Still useful for printer learning, but not proof that the giant cassette architecture should proceed. |

## Open A0 Questions

These questions should be resolved before new CAD replaces the 16-slot first article:

- Should the first module hold one chip only, or one chip plus space for a second/fourth replicate slot?
- What Rev C chip dimensions and tolerances should control the first module pocket?
- Which connector family is the lowest-cost credible starting assumption for a disposable path?
- Does the optical workflow prefer slide-first geometry strongly enough to make SLAS compatibility secondary?
- Should the first tray hold 4, 8, 12, or 16 condition modules?
- What part, if any, must be CNC machined instead of printed for honest seal validation?
- What is the minimum run record schema for condition ID, module ID, chip ID, and measurement traceability?

## Immediate Next CAD Package

The replacement A0 CAD package should be small:

- one-chip local module body coupon,
- local lid/clamp coupon,
- local gasket compression coupon,
- connector-face mockup,
- microscope-stage adapter,
- cheap multi-module SLAS tray/rack outline.

The first release should prove geometry and assembly logic only. It should not claim sterile use, live-cell readiness, AAV readiness, or biological validity.
