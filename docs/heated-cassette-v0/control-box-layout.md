# Control-box fabrication and layout - Rev A

Use Hammond 1554YA2GY with its 1554YPL factory panel. Exterior is 299.33 x 239.46 x 119.73 mm. Base height is 69.84 mm; lid height 49.89 mm. Interior plan envelope is 289.09 x 229.22, closed interior height 111.75. Do not put the controllers through the lid.

Box datum B is the front-left-bottom outside bounding corner, excluding edge radii: X right, Y rear, Z up. Panel datum P is its front-left corner, viewed from above. Panel is 285 x 225 mm; nominal mapping Bx=Px+7.165, By=Py+7.230. Keep its factory perimeter and mounting holes.

## Front base-wall openings

Coordinates are B(X,Z), holes normal to the front wall at Y=0.

| Device | Center X,Z | Cut |
|---|---|---|
| C1 roof PID | 50,35 | 45 x 45 square, +0.5/0, corner R <=1 |
| C2 drawer PID | 115,35 | Same |
| S1 roof RESET | 185,35 | Diameter 22.5 +0.2/0 |
| S2 drawer RESET | 225,35 | Same |
| S0 STOP | 265,35 | Same |

Displays upright; label all buttons. The square openings leave 12.5 mm bottom and 12.34 mm top base-wall lands. Reserve each PID's 48 mm-high body at Z=11..59, a 60 mm-wide installation space including side clips, 110 mm rearward body depth and another 15 mm for wiring. The two 60 mm-wide reservations leave 5 mm between them. Keep panel-mounted parts out from under the controllers. Buttons each have a 35 x 35 x 55 mm rear installation reservation.

## Rear base-wall openings

Coordinates B(X,Z), normal to rear wall Y=239.46.

| Device | Center X,Z | Cut |
|---|---|---|
| CG1 roof power | 45,35 | Diameter 12.2 +0.1/0 |
| CG2 roof signal | 90,35 | Same |
| CG3 drawer power | 135,35 | Same |
| CG4 drawer signal | 180,35 | Same |
| J0 power inlet | 255,35 | 14.8 x 14.8 square, +0.2/0, corner R <=0.5 |
| J0 screw holes | 255,25.5 and 255,44.5 | Diameter 2.7 through |

The M12 glands use their supplied locknuts/O-rings, not tapped plastic. Accept actual cable OD 3-6.5 mm. The power and signal cables have separate glands.

## Inlet mounting and polarity

Mount Kycon KPJX-PM-4S BEHIND the wall, front flange against the inside surface, bosses projecting inward. The square opening clears the approximately 13 mm plug body so it can enter the wall recess and fully mate. Inspect the actual supplied plug's passage and latch engagement.

Use the manufacturer-style M2.5 pan-head screws, approximately 4.9 mm head diameter, effective length 6.60 mm plus measured wall thickness. For nominal 3.99 mm wall, trim M2.5x12 screws to 10.6 mm, dress tips and restore thread leads. Install from outside into the jack's mounting bosses. Seat evenly without crushing plastic; no unsupported torque value is specified.

Brick plug numbering and jack numbering DIFFER. The drawing-based Kycon jack mapping, looking into its mating face with key up, is upper-left 2 negative, upper-right 4 negative, lower-left 1 positive, lower-right 3 positive. Thus J0 1/3 form the positive pair and 2/4 the negative pair. This mapping is inferred from the keyed mating-face drawings.

Before joining any pair, leave all four 18 AWG jack tails separate, insulated and disconnected from loads. Mate the actual brick and measure: 1-to-2 and 3-to-4 approximately +24 V; 1-to-3 and 2-to-4 approximately zero. Remove power before joining measured-positive tails to XRAW and measured-negative tails to X0. Do not copy the PSU plug's printed pin numbers into the jack schedule.

## Panel mounting and holes

Attach the factory panel to its four enclosure M3 inserts with Hammond 1591MM100 M3x8 screws. Do not drill every alternate factory panel hole into the enclosure. Panel supports are approximately 6 mm above the inside floor. Accept panel top no higher than Z=9, leaving at least 2 mm below the PID body reservation; inspect clips before tightening.

Cut one Omron PFP-50N DIN rail to 220 mm, spanning P X=10..230 on Y=170. Drill three diameter 4.5 holes at P(20,170), (120,170), (220,170); align/open matching rail slots. Use M4x10 pan heads, flat washers and locknuts, with <=4 mm projection below the panel.

DIN positions along X: left stop 10..20; DDR converter 25..42.5; relay socket K1 52.5..68; K2 78..93.5; six Wago carriers 103.5..208.5; right stop 215..225. Their 90 mm maximum plan-depth reservation occupies Y=125..215. Route cables above or around this strip without blocking releases.

SSR Q1 center P(190,90), Q2 P(250,90). Long 58.2 mm axis runs Y; width 44.8, height 29.5. Drill diameter 5.5 holes at (190,66.25), (190,113.75), (250,66.25), (250,113.75). Mount with M5x10 pan heads, washers and nuts, <=4 mm underside projection, and thin HTS compound. Control terminals face front, load terminals rear.

The SSR bodies end at Y=119.1, leaving 5.9 mm to the DIN reservation. The right SSR ends at X=272.4, leaving 12.6 mm to the panel edge. Front button reservations end before the SSR bodies. Route wires above these parts rather than stuffing them into the 5.9 mm gap. Preserve top access to terminals.

## Splices and fuse retention

Six Wago 221-500 carriers each hold two separate compact splice blocks: XRAW/X24; two bridged X0 blocks; XC/X13; XC0/XS; XL1/XA1; XL2/XA2. Use three 221-415 blocks for X24 and the two X0 blocks; nine 221-413 elsewhere. Distinct potentials remain separate even when sharing a carrier. Verify both blocks latch securely.

Retain four covered fuse holders on the inside lid. Each gets two Panduit ABM2S-A-D tie bases and cable ties. Base-center pairs, using box X/Y coordinates: (65,60)/(95,60), (205,60)/(235,60), (65,175)/(95,175), (205,175)/(235,175). Each holder reserves 70 x 30 x 20 mm. Keep 250 mm service slack at the right edge, secured so lid removal cannot pull terminals. Inspect adhesive retention; do not suspend holders by leads.

This layout clears catalog body envelopes. Delivered PID clips, actual wire bends, inlet engagement and the lid service bundle remain normal assembly acceptance checks. Reject a component outside the reserved envelope rather than force it into the box. Enclosure lid screw torque is 0.9-1.0 N m.

## Supplemental hardware

Four Hammond 1591MM100 panel screws; three M4x10 pan screws/nuts/washers; four M5x10 pan screws/nuts/washers; two M2.5x12 inlet screws trimmed as above; two DIN end stops; six Wago carriers, three 221-415 and nine 221-413 splices; eight tie bases and eight ties. These quantities supersede the earlier generic two-rail layout. Buy extra wire labels, ferrules matching each terminal and appropriately rated insulating sleeve/heat shrink.

## Sources

Box: https://www.hammfg.com/files/parts/pdf/1554YA2GY.pdf

Panel: https://www.hammfg.com/files/parts/pdf/1554YPL.pdf

PID: https://www.auberins.com/images/Manual/SYL-2352_manual.pdf

Inlet: https://cloud.kycon.com/Pub_Eng_Draw/KPJX-PM-4S.pdf

Plug: https://cloud.kycon.com/Catalog_PDF/KPPX.pdf

Brick: https://www.meanwell.com/Upload/PDF/GST160A/GST160A-SPEC.PDF

SSR dimensions: https://www.gavazzionline.com/pdf/SSR_RM1D.pdf

Converter: https://www.meanwell.com/Upload/PDF/DDR-15/DDR-15-SPEC.PDF

Relay/socket: https://assets.omron.eu/downloads/latest/datasheet/en/j226_g2r-_-s_(s)_general-purpose_relay_datasheet_en.pdf
