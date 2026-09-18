# Assembly handbook - LF-CAS-V0 Rev C

## Mechanical hardware to order

All ordinary machine screws below are A2-70 stainless socket caps, ISO 4762, coarse metric threads. Buy several spares. Thread lengths are under the head. Do not substitute longer screws without checking bottoming and interference.

| Quantity | Item | Use |
|---:|---|---|
| 6 | M3x40 | Risers and guide rails into housing |
| 4 | M3x12 | Rear cover |
| 3 | M3x14 | Front flange to drawer |
| 8 | M3x14 | Heater guards through 8 mm spacers |
| 4 | M2x5 | Plate nest |
| 2 | M2x4 | Gasket shim ears |
| 4 | M3x8 | Moving and fixed cable bracket bases |
| 4 | M2x12 | Cable clamp jaws |
| 2 | M3x6 with M3 flat washer | Thermostat tabs, inspect assembled engagement |
| 1 | M5x8 socket set screw | Close temporary probe port when unused |
| 1 | PTFE tube, 4 mm OD / 2 mm ID, 10 mm length | Removable probe-wire sleeve through front flange |
| 1 | Accu SKH-M2-10-A2 | Diameter 3 x 10 shoulder, M2x0.4 x 3.8 thread, diameter 5 x 2 head |
| 2 | Elesa BT.12 p-M4x16, code 6342 | Manual closure knobs |
| 2 | Keenovo 12073, variant 33194097803404 | 24 V, 60 W, 100 x 100 mm heater |
| 2 | Omega SA1-RTD | 3-wire Pt100, 1 m stripped leads |
| 2 | Sensata 67L050 | NC temperature cutoffs; electrical BOM includes these |
| 1 set | BF-1000 gasket, 9731 PSA, measured backing stack | Front seal |
| 1 set | A160-T-010-0500-G tape profiles | Six guide strips |
| 1 each | Greiner 655101 and 656101 lid | Water-test plate and lid |

Heaters, RTDs and cutoffs also appear in the electrical BOM: order two total of each, not four. Guard spacers are eight aluminum sleeves, diameter 6 / bore 3.4 / length 8.0 +/-0.10; plain uncoated tube cut and faced to length is acceptable. Use the exact CAD/stock kit for other custom parts.

Reference links: https://www.accu.co.uk/knurled-socket-shoulder-screws/49839-SKH-M2-10-A2 ; https://www.elesa.com/en/elesab2bstoreus/Clamping-knobs--Knurled-knobs--BT-p . Confirm Elesa M4x16 variant at order time.

## Incoming checks

Check the shop's finished-dimension report, threaded holes, flat stop faces and burr-free cable edges. Dry-fit the delivered plate and lid inside the 26 mm object-height allowance. Check actual pad/PSA thickness and lead bump: the space beneath each guard is 8 mm. Accept no protrusion above 6 mm in that space, leaving at least 2 mm clearance. Route tall joints out through the rear notch, not under a guard screw.

Check each heater resistance at room temperature: at least 9.0 ohms, no conductor-to-metal short. Confirm the three-wire Pt100 lead pair by resistance, and confirm the 67L050 contact is closed at room temperature. The heater's embedded NTC pair is unused: insulate the two wires separately.

## 1. Bond the two heaters and sensors

The pad land is centered at X=0, Y=59.99: X=-50..50, Y=9.99..109.99. Housing heater bonds to Z=53.225; drawer heater bonds to its underside Z=7.225. Leads point rearward. Clean the bare masked lands as the adhesive manufacturer specifies, apply without air bubbles, and allow its required bond dwell before cycling heat.

Each RTD carrier bonds directly to its aluminum zone in the 30 x 25 mm land X=-20..10, Y=116.73..141.73. It does not sit on the heater. Locate the thermostat tab at X=25, Y=136.73 on the same zone; use M3x6 plus a flat washer. The tab must contact metal. Check screw length against the supplied tab/washer stack and that it neither bottoms nor projects above the plate-support face. Use a shorter screw if measured engagement would violate that envelope; minimum engagement 3 mm.

Dress RTD and thermostat leads separately from heater power joints. Use heat-rated sleeving on exposed joints. Strain-relieve the leads on the heated assembly; no solder splice belongs in the moving bend. Keep all components below the guard's 6 mm accepted stack limit. Record sensor and heater serial/lot identifiers.

## 2. Install guide tape, rails and risers

Apply the six cut tape pieces to clean dry guide edges; plate contact remains the aluminum drawer surface, with its specified anodized finish. Left lower has stop access; left upper begins Y=22. No folded edge, adhesive squeeze-out or overlapping tape. Measure installed thickness and assemble within the guide acceptance ranges.

Fit rails and risers with six M3x40 screws, heads in the underside counterbores. Starting assembly torque is 0.3 N m, an engineering bench limit rather than a supplier fastener rating. Seat evenly and verify a flat base. Reduce torque if thin parts distort; do not use tightening to pull a crooked rail into alignment. All M3 structural joints use this initial limit unless a component manufacturer specifies otherwise.

## 3. Fit drawer, flange, stop and nest

Attach the front flange with three M3x14 screws. Slide the drawer in without the gasket, confirming free travel and the closed front stop. Through the left rail's diameter 6 access, install the SKH-M2-10-A2 shoulder screw into the housing receiver. Its 10 mm shoulder seats against the fixed guide roof and passes through the drawer slot. It must not clamp the drawer. Do not substitute a fully threaded screw.

Fit the nest with four M2x5 screws. Initial M2 seating torque is at most 0.08 N m, subject to free movement and no distortion. Check the plate drops into and lifts from the nest without binding. Check nominal full travel of 107.73 mm and stop retention. Never pull violently against the stop.

## 4. Fit the face gasket

Select and measure the foam/adhesive/shim stack using the manufacturing-note formula. Bond foam to the shim ring with 9731, leaving ears clear. Fit the stack into the groove and retain the ears with M2x4 screws. Verify the heads sit inside the mating flange relief pockets.

Install the two closure knobs. Close by hand, alternating knobs until the flange seats on its metal stop. Use fingers only; do not use pliers or a lever. Verify continuous light contact around the full ring and free release during opening. Uneven marks mean a stack/flatness problem, not a reason to apply more torque. A paper-strip contact check is sufficient for this draft seal; no pressure-tight test.

## 5. Fit guards and cable brackets

Install each guard on four 8 mm spacers with M3x14 screws. Keep leads clear of the spacer faces and exit through the notch. The moving bracket attaches beneath the drawer at X=-42/-28, Y=146.73 with two M3x8 screws. The fixed bracket attaches beneath the rear cover at X=28/42, Y=165.73 with two M3x8 screws. Fit the rear cover with four M3x12 screws. Confirm no fastener tip reaches a plate/contact surface.

Clamp the 6 mm power jacket in the lower groove centered Z=-18 and the 5.5 mm signal jacket in the upper groove Z=-9. The M2x12 jaw screws tighten evenly. Nominal groove diameters are 5.8 and 5.3 mm; accept only if the delivered jackets fit without cutting, flattening conductors or losing grip. The final clamp is a prototype strain relief, not a cable-maker crush certification. Mark the jacket at each clamp to reveal slipping.

## 6. Route the moving harness

Use igus CF9-10-03 power and CF9-02-06 signal cable. Both exit the REAR faces of their clamps toward +Y, in separate horizontal planes. Moving center X=-35, fixed X=+35. Cut 250 mm free centerline length between rear clamp faces, excluding the 20 mm gripped portions at each end. Add the required static tails; buy at least 2 m of each cable for prototype tails and service loops, then cut to the actual controller-box position.

Shape each loop into two straight legs joined by a 35 mm-radius semicircle bowing rearward. At closed position, the arc ends are at Y=226.752 and its rear centerline reaches Y=261.752. At full extension these become 172.887 and 207.887. The fixed straight leg remains 16.157 mm. The largest cable outside surface reaches approximately Y=264.752.

Reserve X=-45..45, Z=-24..-3 and Y=49..270 as the underbody/rear moving-harness corridor. Keep tools, shelves, other cables and the controller box out. The two loop planes are 9 mm apart; keep them separate. Actual cables can sag/twist: cycle the full stroke by hand and inspect this space before applying power. This free-loop V0 has no production flex-life rating.

## 7. Assemble and wire the controller box

Use the selected enclosure and the final control-box drilling sheet. Mount PIDs in the FRONT BASE wall, not on the lid. Keep their rear bodies/clips clear of the panel and DIN devices. Mount SSRs to the metal panel; apply manufacturer torque values from the electrical schedule. The external enclosed brick carries all mains connections.

Wire exactly to the terminal schedule and schematic. The PIDs receive regulated 13.0 V from DDR-15G-12, not the nominal 24 V brick. Use DC-output SSRs. The relay coils have polarity-sensitive diodes. Label every terminal and connector before powering. Make heater-side connectors dead before disconnecting them.

Micro-Fit contacts accept 20-24 AWG; use the specified transition pigtails for 1 mm2 power cable and the RTD's finer factory wires. Do not cut away strands to fit a contact. Keep all transitions outside flexible bends. Use equal three-wire RTD extension lengths and gauges. Roof and drawer connectors must carry durable zone labels.

## 8. Commission before loading water

Follow the electrical inspection and fault-test sequence, then the water-temperature protocol. Keep prototype operation attended until those tests pass. Record actual controller values, sensor calibration corrections, gasket shim stack, currents and temperatures in the commissioning log.

This cassette can later be mounted to a rocking shelf, but the V0 bench feet are not a rocker. A later shelf must support both risers, secure the cassette, retain the plate/lid and preserve cable clearances through tilt. No angle is assumed for an unselected tissue plate.
