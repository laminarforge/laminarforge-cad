# LaminarForge heated microplate cassette V0 - Rev C

This package defines one manual, two-zone heated aluminum cassette for water-filled microplate temperature testing. Manufacture to the analytic STEP files together with the shop drawings and manufacturing notes. STLs and assembly images are viewing aids. The physical prototype still needs commissioning and temperature mapping.

Greiner 655101 with lid 656101 is the selected inexpensive thermal surrogate. It is not a perfused tissue-on-chip plate. No live-cell suitability, sterile atmosphere, humidity control, CO2 control, hermetic seal, rocking endurance or media-temperature performance is certified here.

## Fabrication and assembly

Send the complete supplier RFQ ZIP, including the manufacturing BOM, shop-drawings PDF and assembly handbook, to a CNC shop. Request one set, 6061-T6, including clear Type II finishing and the limited fit-dimension report. Buy the commercial mechanical/electrical parts from the BOM. Assembly needs metric hex tools, a small torque driver, multimeter, suitable connector crimp tooling, solder/heat-shrink supplies and temperature-measurement equipment.

One representative spacer file specifies eight identical spacers. Both guards, both risers and both cable jaws are identical after translation. The left guide rail has an extra stop-access hole. The cable-bracket bases differ.

## Design definition

- Fixed U housing/front bezel, rear cover, heated sliding bottom, front flange and removable nest.
- Two manual M4 closure knobs; continuous silicone face gasket; metal face-to-face closing stop.
- Two 100 x 100 mm, 24 V, 60 W heaters with separate Pt100 sensors, PIDs and DC SSRs.
- Independent NC thermostats drop latched power relays. Shared STOP and separate zone RESET buttons.
- Guide tape on edges only. The plate sits directly on the aluminum drawer.
- Rearward R35 cable loops and 30 mm bench risers; keep the service bay clear.

Housing/flange envelope: 184.76 W x 176.73 D x 75.225 H mm. Complete bench assembly is approximately 95.225 mm high, Z=-30 to +65.225 including upper screw heads. Reserve 120 mm forward of the closed front for drawer access and space through Y=270 behind the assembly datum for cable travel. Closed outer front is Y=-8. Controller box and power brick are separate.

The closure knobs project another 11.5 mm forward and slightly beyond the flange sides; their total width is approximately 186.01 mm. With the reserved rear cable bay, allow approximately 290 mm closed bench depth, plus the forward drawer-access space. Aluminum mass is about 2.21 kg in this prototype. This is a bench arrangement, not a finalized rack pitch.

Nominal stroke is 107.73 mm. Rear aluminum capture is 45 mm; the shortened left upper tape retains 29 mm overlap. The flange defines closing position. The stop slot allows approximately 0.2 mm pin clearance beyond nominal travel.

## Plate fit

Greiner's 655101 drawing specifies 127.76 +/-0.20 x 85.48 +/-0.20 mm footprint and 14.6 +/-0.1 mm lidless height. The nest opening is 129.26 x 86.98 mm with corner reliefs. Minimum centered side clearance is 0.65 mm at the plate's upper tolerance, before nest manufacturing tolerance.

Lid 656101 is 127.5 +/-0.2 x 85.0 +/-0.2 mm. Its drawing shows 10 mm overall lid features but does not certify assembled plate/lid height. This design reserves a 26 mm lidded object plus 8 mm roof clearance. The 26 mm value is a design allowance, not the actual Greiner assembled height. Measure the delivered lidded plate. Future tissue plates need separate exterior, lid, port, orientation, access and rocking checks.

## Verification and file precedence

The generator uses one construction tree for analytic STEP and review meshes. It reimports every custom STEP, checks one manifold solid, compares volumes and records hashes. It checks closed meshes and sampled moving/fixed interference. These checks do not establish fatigue, deformable-part behavior, process capability or electrical certification.

The reduced thermal model estimates steady aluminum-sheet conduction over assumed heat-loss/conductivity cases. It does not resolve media, plastic wells, evaporation, airflow or PID transients. Water measurements decide thermal acceptance.

Use configuration, verification manifest, STEP and drawings from the same release. Explicit thread and manufacturing notes control where STEP represents pilot bores; paired STEP controls other geometry. Resolve conflicting instructions before cutting an affected part. Integrated mechanical instructions supersede preliminary research arrangements.

## Sources

Plate: https://hamptonresearch.com/uploads/support_materials/HR3-139_Greiner_655101_customer_drawing.pdf

Lid: https://shop.gbo.com/en/usa/products/bioscience/lids-sealers-capmats/lids/656101.html

Lid drawing: https://www.bunzlasiapacific.com/extension/IP/document_896_5bfca99a5b406.pdf

Component sources are linked in the BOM/electrical appendices. Prices are snapshots, not a complete landed cost or a CNC quote.
