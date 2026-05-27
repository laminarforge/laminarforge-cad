# Cell Culture Equipment Boundary Breakdown

The boundary is not "do not design it." The boundary is the difference between a designed prototype, a validated engineering instrument, a biological-use instrument, and a certified/compliant instrument. LaminarForge should design as much of the stack as possible, but each equipment class needs a clear path through those gates.

## Readiness Levels

| Level | Meaning | Evidence required |
| --- | --- | --- |
| CAD prototype | Geometry exists and can be fabricated or printed | CAD export, STL verifier, fit-check dimensions |
| Engineering prototype | Hardware can run safely without cells | Electrical safety, leak/spill checks, thermal/flow/CO2 logs, failure-mode tests |
| Biological pilot | Hardware can support low-risk non-clinical culture trials | Sterile workflow SOP, contamination monitoring, cell morphology/growth logs, independent measurement |
| Production-ready internal tool | Hardware can be used repeatedly by trained operators | Written SOPs, maintenance schedule, calibration records, batch logs, change control |
| Certified/compliant equipment | Equipment can make biosafety/regulatory claims | Third-party certification or qualified facility review where required |

Do not skip levels. A good design should make the next level cheaper to prove.

## Equipment Boundary Map

| Equipment | What we can design | Engineering validation | Biological validation | Certification/compliance gap |
| --- | --- | --- | --- | --- |
| CO2 incubator | Chamber, door, shelves, gas manifold, external CO2 service module, logger enclosure, control architecture | 24-72 h temperature/CO2/RH logs, door recovery, leak/decay, overtemp cutoff, power-loss behavior | Empty-chamber contamination challenge, then low-risk cell-line growth/morphology compared to commercial incubator | Cannot claim incubator equivalence until calibration, cleaning, reliability, and SOP evidence are mature |
| Biosafety cabinet / sterile hood | Practice enclosure, airflow prototype, work-surface ergonomics, pass-through tray, CFD model | Smoke visualization, face velocity mapping, particle/flow checks | Practice-only until product/personnel/environment protection is proven elsewhere | Class II BSC claims require NSF/ANSI 49-style certification; DIY hood is not a BSC |
| Water bath / warming block | Low-voltage bath, rack, lid, probe clamp, bottle holder, cable strain relief, dry block option | Independent 37 C hold, leak test, low-water cutoff, thermal cutoff, GFCI/low-voltage safety, splash test | Warm media in sealed containers, compare cell response to commercial warming workflow | Mains-water safety and lab approval remain external review items |
| Aspirator / waste train | Bottle cradle, secondary trap holder, filter clip, tubing strain relief, spill tray | Vacuum leak test, overflow simulation, tip/tubing routing, disinfectant compatibility, filter placement | Use only under approved SOP with low-risk cultures first | Biohazard disposal, disinfectant contact time, and facility vacuum protection are SOP/facility requirements |
| Centrifugation | Balance racks, tube adapters for organization, rotor gauges, cleaning fixtures | Fit checks only; no load-bearing high-speed printed rotor parts | None for printed rotor substitutes | Buy centrifuge and rated rotors; do not certify printed spin hardware |
| Microscope support | Chip-stage adapter, holder, repeatable alignment fixtures, camera/illumination mounts | Stage fit, field-of-view alignment, repeatability, focus clearance | Compare confluence/morphology reads against normal culture vessels | The microscope itself should be bought/accessed; support fixtures can be designed |
| Pipetting / consumables | Pipette stand, tip-box organizer, conical rack, sterile workflow layout fixtures | Ergonomic fit, cleanability, tip-box compatibility | Workflow timing/error reduction checks | Pipette calibration and sterile consumables are bought controls |
| Microfluidic priming/perfusion | Chip pocket, tubing combs, luer clips, bubble-view fixture, syringe pump holders, reservoir supports | Flow-rate calibration, leak test, bubble/dead-volume measurement, pressure limit | Sterile disposable fluid path, media compatibility, cell viability under flow | Aseptic assembly and disposable-path validation are required before real culture |
| Cold storage support | Cold blocks, Peltier bench blocks, rack organization, temperature logger fixtures | Temperature hold curves, condensation management, thermal mass tests | Short bench handling only; storage remains in validated fridge/freezer/cryostorage | 4 C, -20 C, -80 C, and LN2 storage must be bought/accessed and logged |
| Shaker / rocker | Orbital shaker, rack rocker, chip-rocking fixture, spill containment | RPM/tilt calibration, tip-over/spill test, heat load, corrosion/high-RH check | Cell response under rocking/perfusion compared to static control | Incubator-compatible commercial shakers may be required for real CO2/high-RH operation |
| Environmental logging | Independent logger enclosure, probe clamp, CO2 service module, power logging | Calibration comparison, sensor drift checks, logger/controller disagreement alarms | Required alongside any biological trial | Calibration traceability may require bought/NIST-calibrated instruments |

## Design Implications

Design every module with these features unless there is a specific reason not to:

- external serviceability for electronics and sensors
- independent validation ports or fixtures
- leak/spill containment
- cleanable surfaces and removable wet parts
- cable and tubing strain relief
- failure states that are visible and recoverable
- labels or geometry that prevent reversed connections
- published acceptance gates before website release

## Website Rule

Website visibility should follow evidence level:

| Evidence level | Website treatment |
| --- | --- |
| CAD prototype only | Do not publish as equipment; internal roadmap only |
| Engineering prototype passing logs | Publish as research prototype with validation data and limitations |
| Biological pilot passing | Publish as experimental open-source equipment with SOPs and disclaimers |
| Production-ready internal tool | Publish build docs, BOM, validation procedure, maintenance procedure |
| Certified/compliant | Only claim certification if an appropriate third party or facility process actually granted it |

## Current Priority

The immediate work is not to stop at "buy/access." It is to convert each buy/access item into one of three things:

1. a LaminarForge-designed support fixture around bought core equipment,
2. a LaminarForge prototype with explicit engineering and biological validation gates, or
3. a compliance boundary that remains external but is documented and designed around.
