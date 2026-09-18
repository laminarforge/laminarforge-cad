# Heated cassette V0: selected electrical assembly and terminal schedule
Rev C component selection and terminal schedule, 2026-09-13. Read with the final control-box drilling layout and mechanical assembly handbook. No purchases or physical commissioning have been performed.

## Selected architecture
Two independent 24 V resistive heaters, each controlled by a standalone Pt100 PID and DC-output SSR. Each heater also has a separate metal-mounted, normally-closed bimetal thermostat controlling a manually latched electromechanical power relay. The brick contains all mains wiring. A shared red STOP drops both heater relays; green RESET buttons arm zones separately. Controllers remain powered when heaters are stopped.

A 50°C nominal thermostat is not a precise 50°C maximum: its tolerance permits operation at 55°C, with additional mounted thermal lag. This cutoff protects against runaway, not sample-temperature excursion. Actual media temperature needs immersion-probe validation.

## Exact BOM
Prices are observed USD unit prices, excluding tax/shipping; unverified prices are explicitly not quotes. Counts are for two zones.

|Qty|Reference|Manufacturer/orderable part|Selection and sourcing|
|---:|---|---|---|
|2|H1,H2|Keenovo 12073; variant 33194097803404|24 V, 60 W, 100×100 mm silicone pad with 3M PSA and embedded NTC; $27.74 each from manufacturer public variant data.|
|2|T1,T2|Omega SA1-RTD|Three-wire Pt100 Class A, 1 m leads; carrier 25×19 mm. Price not verified.|
|2|C1,C2|Auber SYL-2L52|12–24 V AC/DC supply, Pt100 input, SSR drive; $46.50 each. Do not order triac version SYL-2L72.|
|2|Q1,Q2|Carlo Gavazzi RM1D060D10|10 A DC-output SSR, load 1–60 VDC, control 4–32 VDC; $57 each, 191 shown in stock.|
|2|TH1,TH2|Sensata/Airpax 67L050|NC, opens nominal 50°C; 519 stock shown, single-piece price not verified. Do not substitute 67F050, which closes on rise.|
|2|K1,K2|Omron G2R-2-SD DC24(S)|DPDT relay with integral coil diode; $16.28 each, 150 stock shown.|
|2|XK1,XK2|Omron P2RF-08-PU|Push-in DIN socket for selected relay; price not verified.|
|1|PS1|Mean Well GST160A24-R7B|Enclosed external 24 V, 160 W, 6.67 A brick, C14 inlet; $52.40.|
|1|AC1|Mean Well YP12_YC12|US NEMA5-15P to IEC C13 cord, 6 ft; $7.29 observed related listing.|
|1|J0|Kycon KPJX-PM-4S|Panel-mount mating 4-position power DIN socket, 48 VDC/7.5 A; $2.45 observed related listing.|
|1|DC1|Mean Well DDR-15G-12|DIN converter, 9–36 V input; adjust output to 13.0 V before attaching controllers. Price not verified.|
|2|S1,S2|Schneider XB5AA31|Green spring-return NO reset button.|
|1|S0|Schneider XB5AA42|Red spring-return NC stop button; this is a heater-stop function, not a certified emergency-stop system.|
|1|F0|Littelfuse 028707.5PXCN|7.5 A ATOF main positive fuse.|
|2|F1,F2|Littelfuse 0287004.PXCN|4 A ATOF heater branch fuses.|
|1|F3|Littelfuse 0287001.PXCN|1 A ATOF control branch fuse.|
|4|XF0–XF3|Littelfuse FHAC0001ZXJ|Inline covered ATO holder, 32 VDC/20 A, 16 AWG leads.|
|1|E1|Hammond 1554YA2GY|300×240×120 mm enclosure.|
|1|P1|Hammond 1554YPL|Matching inner mounting panel.|
|1|R1|Omron PFP-50N|500 mm stock; cut one 220 mm rail to the final layout.|
|2|ES1–2|Omron PFP-M|DIN end stops.|
|1 set|X nodes|Nine Wago 221-413, three 221-415, six carriers 221-500|Separate-potential splice groups per final layout.|
|4|CG1–4|Hammond 1427NCGM12B|M12 cable glands for four cable entries; individual PDF gives3–6.5mm cable range, locknut/o-ring included.|
|2 lengths|WP1,WP2|igus CF9-10-03|3×1mm² cable, approximate6mm OD; two numbered cores carry heater current, green/yellow spare insulated.|
|2 lengths|WS1,WS2|igus CF9-02-06|6×0.25mm² signal/cutoff cable, approximate5.5mm OD.|
|2 each|JP housings|Molex430250200 /430200201|Two-position power connector pair per zone.|
|2 each|JS housings|Molex430250600 /430200601|Six-position signal/cutoff pair per zone, five circuits populated.|
|14 each|Crimp contacts|Molex430300007 /430310007|Female/male20–24AWG; power uses20AWG transition pigtails, sensor24AWG.|
|1|TIM|Electrolube HTS02S|Thin thermal compound at SSR mounting interfaces, as permitted in SSR guidance.|

Heaters: [manufacturer selected variant](https://keenovo.store/products/keenovo-square-silicone-heater-3d-printer-build-plate-heatbed-heating-pad?variant=33194097803404). RTD geometry: [Omega drawing](https://assets.omega.com/pdf/test-and-measurement-equipment/temperature/sensors/rtds/SA1-RTD.pdf). Controller price: [Auber product](https://www.auberins.com/index.php?main_page=product_info&products_id=601). SSR price/stock: [DigiKey](https://www.digikey.com/en/products/detail/carlo-gavazzi-inc/RM1D060D10/13277977). Relay price/stock: [DigiKey](https://www.digikey.com/en/products/detail/omron-automation-and-safety/G2R-2-SD-DC24-S/1789761). Thermostat stock: [DigiKey](https://www.digikey.com/en/products/detail/sensata-airpax/67L050/1631970). Supply price: [DigiKey](https://www.digikey.com/en/products/detail/mean-well-usa-inc/GST160A24-R7B/7703602).

Verified priced subtotal for pads, PIDs, SSRs, relays, brick, cord and inlet is $357.18; this is not the total build cost. Sensors, sockets, converter, switches, protection, enclosure and harness add to it. Do not describe the full electrical package as a $55 heater system.

## Manufacturer pin references
These are physical terminal designations, not inferred wire colors. PDF page numbers below are one-based.

- C1/C2: 9/10 power; 3/4 same-end RTD leads, 5 opposite-end; 7 positive SSR drive, 8 negative. Alarm terminals 1/13/14 and unused terminals remain unwired. [Auber low-voltage supplement, p1 Fig2 and p2 RTD instructions](https://www.auberins.com/images/Manual/syl_24v_manual.pdf).
- Q1/Q2: load 1 positive, 2 negative; control 3/A1 positive, 4/A2 negative. Control demand below 16 mA is within controller drive capability. [RM1D, pp4,6,19](https://www.gavazziautomation.com/fileadmin/images/PIM/DATASHEET/ENG/SSR_RM1D.pdf).
- K1/K2 socket: A1 equals relay pin8 positive; A2 equals pin1 negative. Contact 11/pin3 common, 14/pin4 NO, 12/pin2 NC; second contact 21/pin6 common, 24/pin5 NO, 22/pin7 NC. Observe coil polarity because diode is integral. [Omron, p9 relay diagrams, p11 socket drawing](https://assets.omron.eu/downloads/latest/datasheet/en/j226_g2r-_-s_(s)_general-purpose_relay_datasheet_en.pdf).
- PS1 plug numbering: R7B1/4 positive,2/3 negative. KyconJ0 uses different numbering: inferred jack1/3 positive,2/4 negative. Before joining ANY pairs, measure the four separate jack tails with the actual brick. Wire measured-positive tails to XRAW and measured-negative tails to X0, using equal-length18AWG pigtails. [GST160A p3](https://www.meanwell.com/Upload/PDF/GST160A/GST160A-SPEC.PDF); [Kycon mating jack drawing](https://www.kycon.com/Catalog_PDF/KPJX-PM.pdf).
- DC1: terminal4 +Vin, 3 −Vin, 1 +Vout, 2 −Vout. Output adjustment range 9–13.2 V; use 13.0 V to avoid the controller's 12 V minimum and the brick's possible >24 V output. [DDR-15 pp2,5](https://www.meanwell.com/Upload/PDF/DDR-15/DDR-15-SPEC.PDF).
- S1/S2 use their NO block, marked13/14; S0 uses NC21/22. Confirm supplied contact-block markings and continuity before wiring. [Green button](https://www.se.com/us/en/product/XB5AA31/push-button-harmony-xb5-plastic-flush-green-22mm-spring-return-unmarked-1no/), [red button](https://www.se.com/us/en/product/XB5AA42/push-button-harmony-xb5-plastic-flush-red-22mm-spring-return-unmarked-1nc/).

## Complete internal terminal schedule
R=roof, D=drawer. Repeat zone rows for i=1/roof and i=2/drawer. Each named X node is one common-potential Wago splice; if more than five endpoints are needed, bridge two 5-way splices with the same gauge as the feeder. One conductor per port. Do not put distinct circuit potentials in the same splice.

|Net|From|To|Wire/termination|
|---|---|---|---|
|Raw +24|J0 measured-positive pair (drawing inference:1,3)|XRAW; XRAW→F0 input|Two equal18AWG inlet pigtails;16AWG main lead|
|Protected +24|F0 output|X24: F1 input, F2 input, F3 input|16AWG fixed distribution|
|24V return|J0 measured-negative pair (drawing inference:2,4)|X0: Q1/2 load terminal2, DC1 terminal3, K1/K2 A2|Equal18AWG inlet pigtails;16AWG distribution,1mm² SSR branches,.5mm² controls|
|Control +24|F3 output|XC: DC1 terminal4 and S0 terminal21|.5mm²|
|Control return|DC1 terminal3|X0|.5mm²|
|PID +13|DC1 terminal1|X13: C1 terminal9, C2 terminal9|.5mm²|
|PID return|DC1 terminal2|XC0: C1 terminal10, C2 terminal10|.5mm²; do not join XC0 to X0|
|Stop-permitted|S0 terminal22|XS: TH1 leadA and TH2 leadA through respective harnesses|.5mm²|
|Zone thermal return|THi leadB through harness|XLi: Si terminal13 and Ki socket21|.5mm²|
|Zone arm|Si terminal14|XAi: Ki socket24 and Ki A1|.5mm²|
|Zone coil return|Ki A2|X0|.5mm²|
|Heater branch|Fi output|Ki socket11|1mm²|
|Enabled heater feed|Ki socket14|Zone heater-positive harness→Hi power leadA|1mm² fixed; moving section per harness specification|
|Switched heater return|Hi power leadB→zone return harness|Qi load terminal1 positive|1mm² at SSR terminal|
|SSR low side|Qi load terminal2 negative|X0|1mm²|
|SSR command +|Ci terminal7|Qi control3/A1+|.5mm²|
|SSR command −|Ci terminal8|Qi control4/A2−|.5mm²|
|RTD red1|Ti first red lead via dedicated harness core|Ci terminal3|Equal-gauge three-wire RTD extension|
|RTD red2|Ti second red lead via dedicated harness core|Ci terminal4|Do not common with other red except through sensor's factory junction|
|RTD white|Ti white lead via dedicated harness core|Ci terminal5|No connection to either supply return|
|Unused|Ki12/22; Ci unused terminals; heaterNTC pair|Insulate and leave unconnected|NTC wires individually insulated, not shorted|

Inline holders are nonpolar; identify input/output labels in the built harness. Physical low-side SSR polarity is important even though the heater is nonpolar. Do not substitute an AC-output SSR.

The circuit is:
F3→STOP(NC)→THi(NC)→[RESETi(NO) parallel Ki21–24(NO)]→Ki coil→0V.
Separate power path is F0→Fi→Ki11–14(NO)→heater→Qi→0V.
A thermostat opening drops Ki even with a failed-short SSR. Cooling cannot relatch Ki with RESET released. A broken thermostat wire also drops Ki. On ordinary power removal/restoration, heaters remain disabled until each RESET is pressed. This economical circuit is not an anti-tie-down or monitored safety relay: holding RESET during restoration can rearm, and welded Ki contacts are not detected. Label RESET “release after pressing”; disconnect the brick for service. No claim of redundant fault tolerance or thermal/electrical certification is made.

## Harness boundary and terminations
Use separate Micro-Fit3.0 connectors at each zone's fixed service point, outside the moving bend. For two zones order2×Molex430250200 receptacle housings and2×430200201 plug housings for power;2×430250600 receptacles and2×430200601 plugs for signals. Use14×430300007 female contacts and14×430310007 male contacts for the seven populated circuits per zone; buy spares. Female-contact receptacles are on the control-box/supply side. Six-position cavity6 is unpopulated and insulated. Both housing pairs are polarized/latching, but roof and drawer use identical pairs: label them clearly.

|Interface cavity|Control-box endpoint|Cassette endpoint|
|---|---|---|
|JP1/JP2 cavity1|Ki socket14|Hi power leadA|
|JP1/JP2 cavity2|Qi load1+|Hi power leadB|
|JS1/JS2 cavity1|Ci terminal3|RTDred1|
|JS1/JS2 cavity2|Ci terminal4|RTDred2|
|JS1/JS2 cavity3|Ci terminal5|RTDwhite|
|JS1/JS2 cavity4|XS afterSTOP|THi leadA|
|JS1/JS2 cavity5|XLi beforeRESET/latch|THi leadB|
|JS1/JS2 cavity6|Unpopulated|Unpopulated|

Cavity numbering is the molded housing numbering; do not infer pin1 from a front-view sketch that may mirror the wire-entry view. Continuity-check each mated pair before power.

[Power receptacle](https://www.molex.com/en-us/products/part-detail/430250200), [power plug](https://www.molex.com/en-us/products/part-detail/430200201), [signal receptacle](https://www.molex.com/en-us/products/part-detail/430250600), [signal plug](https://www.molex.com/en-us/products/part-detail/430200601), [female contact](https://www.molex.com/en-us/products/part-detail/430300007), [male contact](https://www.molex.com/en-us/products/part-detail/430310007).

Selected contacts accept20/22/24AWG, insulationOD≤1.85mm, and are tin plated. They do NOT accept1mm² cable or Omega26AWG directly. At every power contact use a short≤100mm20AWG stranded copper pigtail with insulationOD≤1.85mm, rated≥80°C, transitioning by individually insulated, strain-relieved solder splice to the1mm² cable. At RTD contacts transition factory26AWG to equal24AWG pigtails; CF9-02-06 is catalogued24AWG and can enter the contact only if its actual core insulationOD fits. Keep every transition outside the flex bend. Do not remove conductor strands to fit a contact. Inspect/crimp with tooling qualified for these exact contacts; neither a universal plier nor soldering into the crimp barrel is specified. The7A per-contact catalog maximum is not a full-harness rating; verify≤2.75A normal heater operation and4A branch protection with actual20AWG pigtails, bundle and ambient.

Four Hammond1427NCGM12B glands enter E1, one power and one signal cable per zone. Use igusCF9-10-03 power cable's two numbered black cores for heater feed/return; green/yellow is insulated unused. Sleeve CF9-02-06 cores1–5 to match JS1–5 and leave core6 insulated. Record actual color mapping on both ends. Never merge RTD and heater returns. No live disconnection is specified; remove brick power before separating any connector.

Both selected cables support moving radius5×OD; R35mm covers6mm and5.5mm nominal OD. Keep actual cable minimum radius and full-stroke clearance verified. [igusCF9 pp1–2](https://www.igus.com/us/pdf/cf9.pdf). Gland individual drawing specifies3–6.5mm, narrower than the family table's3–7mm; use the individual limit. M12 sealing-nut guidance0.8–1.5Nm, locknut1–2Nm. [Gland drawing](https://www.hammfg.com/files/parts/pdf/1427NCGM12B.pdf), [manufacturer table](https://www.hammfg.com/electronics/small-case/accessories/1427ncg). Do not grip multiple separate wires in one gland. Fixed cassette-end jacket clamps/guards are mechanical parts in the parent assembly.

The selected1mm² power cable replaces the earlier0.5mm² proposal and meets the SSR load-terminal minimum. Signal extensions use equal0.25mm² cores; all RTD pigtail solder transitions must be symmetrical and thermally isolated from high-current joints. Seven active conductors are required; never merge RTD return with heater return.

Wago221 fine-stranded minimum is0.14mm², larger than Omega26AWG≈0.129mm². Do not insert bare stock RTD leads directly into221. Extend via strain-relieved solder joints to suitable larger wire or a connector contact explicitly rated26AWG. [Wago221 wire range](https://www.wago.com/us/splicing-connectors-221), [carrier](https://www.wago.com/us/wire-splicing-connectors/mounting-carrier/p/221-500).

## Mounting and assembly details
- Mount SA1-RTD directly on clean flat aluminum, not on the heater silicone or loose in a blind hole. Reserve30×25mm flat land. Keep a separate adjacent metal-contact site for each67L050 mounting tab. Do not put both sensors on one surface and infer the other.
- 67L050 heat enters primarily via terminals/bracket. Use its metal tab in firm contact with the local heated aluminum, mechanically fastened with strain-relieved leads; its TO220 mounting envelope comes from p3, not a generic transistor drawing. The thermostat switches only the roughly22mA relay coil. [Sensata pp1–4](https://www.sensata.com/sites/default/files/a/sensata-6700-series-subminiature-bimetal-disc-thermostats-datasheet.pdf).
- RM1D060D10 needs no separate heatsink at≤4A within its published no-heatsink table, but mount securely to the metal inner panel with thin compound. Load terminals accept flexible1–6mm²,12mm strip,2.4Nm; inputs with ferrule .5–2.5mm²,8mm strip,.5Nm. Mounting screws M5,1.5–2Nm. Maintain terminal guards/touch protection. [RM1D pp11,15,20–21](https://www.gavazziautomation.com/fileadmin/images/PIM/DATASHEET/ENG/SSR_RM1D.pdf).
- Mount PIDs in the front base wall using 45 x 45 mm cutouts. Fit converter, relay sockets and splice carriers on the single 220 mm rear rail; SSRs mount directly on the metal panel. Use the final control-box layout. [Hammond enclosure](https://www.hammfg.com/electronics/small-case/plastic/1554).
- Relay socket contact system is6A; relay double-pole contacts are5A at30VDC resistive, above the selected heater load. Rail/socket envelope is in [Omron pp10–13](https://assets.omron.eu/downloads/latest/datasheet/en/j226_g2r-_-s_(s)_general-purpose_relay_datasheet_en.pdf).
- Both 100 x 100 mm pads bond to flat spreader surfaces beneath removable guards. The family 1.5 mm body dimension does not bound PSA and lead bumps. The final mechanical design reserves an 8 mm guard gap and accepts a maximum 6 mm installed component/lead stack. Inspect delivered parts before bonding.

## Voltage, current and fuse design bounds
Heater nominal resistance is9.6Ω; two pads draw5A at24V. The160W brick leaves40W nominal for controls and tolerance. It has±3% output tolerance; at24.72V,9.6Ω produces63.65W/2.575A per pad. For this assembly, accept room-temperature resistance≥9.0Ω per pad and measured combined steady input≤6.2A, including controls; these are design acceptance criteria, not an asserted Keenovo tolerance. Verify warm operation too. Do not increase brick voltage or substitute higher-power pads.

The controller output limit changes time-averaged heat, not on-pulse current. Setpoints and duty caps cannot replace fuses. Main7.5A, branches4A and controls1A protect wiring in conjunction with the current-limited brick. TheATO family is32VDC/1000A interrupt. [ATOF datasheet pp1–3](https://www.littelfuse.com/assetdocs/littelfuse-datasheet-287-atof?assetguid=43dcdce8-8ca2-426f-8998-7e566f048d40); [holder drawing](https://www.littelfuse.com/assetdocs/ato-fhac-datasheet?assetguid=272e0b1a-a576-4173-8740-c1eb469efd79).

Brick overload protection hiccups and recovers; it can prevent a fuse clearing promptly. Thus neither branch-fuse selectivity nor semiconductor protection is guaranteed. Use the specified wire gauges and perform a controlled fault-protection assessment before unattended use. DC1 has15A typical input inrush; confirm1A F3 survives normal starts. If it does not, do not blindly upsize: coordinate an appropriately delayed DC-rated fuse against its conductor and button/cutoff paths. This is a qualification point, not proof of a nuisance-free fuse design.

PS1's negative output is bonded to its AC protective earth; preserve the three-wire mains cord. No separate mains conductor enters E1. [GST160A pp2–3](https://www.meanwell.com/Upload/PDF/GST160A/GST160A-SPEC.PDF). Never connect a PID supply directly across the brick merely because both labels say24V: the PID's published supply ceiling is24V and brick tolerance exceeds it. DC1 at13.0V removes that mismatch.

## Controller setup and commissioning
For eachSYL-2L52: Sn=21(Pt100), COOL=2(heating,Celsius), dP=1, At=3(PID), t=2s, OUTL=0, OUTH=50 initially, A-M=2(disable manual output), OP-A=0, FILt=0, Pb=0 initially. SetSV37.0°C initially; proposed editable limitsP-SL20/P-SH42°C. These are engineering starting settings, not validated water setpoints. Use supervisedAt=2autotune only after mechanical assembly, then retain resultingPID values. NormalOUTH can become100 after thermal verification. [Auber configuration manual pp3–6](https://www.auberins.com/images/Manual/SYL-2352_manual.pdf).

OUTL=0 is essential because the controller uses that output on detected sensor failure. Test removal of each RTD lead individually and verify output shuts off; an apparent valid low reading from some shorts is not guaranteed detectable. Independent67L050 cutoff remains necessary. Do not use alarm relay output as the independent cutoff.

1. With power removed, check every net against the table, coil-diode polarity, no RTD-to-power continuity, protective-earth cord continuity, and insulation between heater circuits and metal. Check every sleeved harness core at both ends.
2. First leave all fourJ0tails separate, insulated and disconnected from the box; mate the brick and measure each contact before joining the measured-positive pair and measured-negative pair. Remove power before joining. Then energize with heaters disconnected; adjustDC1 to13.0V and verify bothPIDsupplies within12–24V. VerifySTOP/RESET and power-cycle dropout.
3. Attach sensors, compare both metal readings around37°C with a calibrated reference. CorrectPb only from measured calibration; controller and RTD labels alone do not establish±0.5°C system accuracy.
4. Connect one heater at a time, then both; record voltage/current at startup and steady operation. Inspect terminal and cable temperatures; confirm F3 normal startup behavior.
5. Simulate thermostat opening electrically: each zone must latch off independently, remain off after loop reclosure, and requireRESET. TestSTOP and power-loss restoration.
6. On a controlled heated metal test, measure actual67L050 trip temperature and mounted lag; do not infer cutoff from its nominal marking. SimulateSSR stuck-on with a supervised jumper only in a controlled fixture to demonstrate relay removal of power.
7. Validate actual water temperatures at corners and center, with plate/lid, cold start and 107.73 mm drawer opening/recovery. A 37 C metal reading does not prove 37 C water. Save results and final PID settings.

## Release distinction
Electrical functional connections, exact major parts and separate2/6-position Micro-Fit interfaces are selected here. Remaining prototype qualification includes measured heater stack/lead bump, actual thermostat lag, control-supply fuse inrush compatibility, current/temperature limits and water uniformity. Those tests cannot be honestly replaced by catalog data. If the parent needs a fully released production package, they must remain explicit acceptance checks before unattended operation.


## Inlet correction — supersedes earlier mounting and J0 numbering
Reviewed Kycon drawing rev C2 and the Mean Well R7B drawing on 2026-09-13.

**Mount the jack behind the wall.** Its flat front flange bears against the inside face of the enclosure rear wall; its 6.1 mm mounting bosses extend into the enclosure. The previous outside-flange mounting with small clearance holes is withdrawn because those holes do not clear the bosses.

At box rear coordinates (X,Z)=(255,35), cut a **14.8 × 14.8 mm square opening**, +0.2/−0, internal corner radius≤0.5 mm. Drill two **Ø2.7 mm** screw clearance holes at (255,25.5) and (255,44.5). Deburr. Orient the jack's key at the top. Fit two manufacturer-style **M2.5 screws, Ø4.9 mm pan heads, effective under-head length 6.60+X**, where X is the measured wall thickness. With nominal3.99 mm wall, the length is10.59 mm: trim standardM2.5×12 screws to10.6 mm, dress the tips and restore the thread lead before installation. Screws enter from outside through the wall into the jack'sØ2.3 mm mounting holes/bosses. Do not use M2 clearance bolts with nuts or drive an untrimmed12 mm screw to an assumed stop. Seat both screws evenly without crushing the plastic; the source supplies geometry, not a verified torque value.

The square opening admits the complete nominal13.0 mm KPPX plug front-body envelope, rather than only itsØ9.6 mm projecting nose. Thus the plug body can enter the3.99 mm wall recess and does not lose mating travel by stopping against the wall. Its nominal clearance is0.9 mm per side. This is a designed panel opening, not a published Kycon cutout. Mean Well identifies R7B as KPPX-4P equivalent. The supplied plug must pass this opening freely and fully latch during normal assembly inspection; the box must not be used to force the plug home. Sources: [Kycon jack, p1](https://cloud.kycon.com/Pub_Eng_Draw/KPJX-PM-4S.pdf), [KPPX plug dimensional sheet,p1](https://cloud.kycon.com/Catalog_PDF/KPPX.pdf), [Mean Well GST160A,p3](https://www.meanwell.com/Upload/PDF/GST160A/GST160A-SPEC.PDF).

**J0 pin numbering correction:** Mean Well's R7B plug numbering is not the Kycon jack numbering. With key at the top, the Mean Well drawing shows upper contacts2/3=negative and lower1/4=positive. Kycon's jack mating-face drawing shows upper2/4 and lower1/3. Therefore the geometrically mapped jack connections are:
- **Kycon J0 pins1 and3 → XRAW positive**, separate equal18AWG pigtails.
- **Kycon J0 pins2 and4 → X0 negative**, separate equal18AWG pigtails.

Looking into the installed jack from outside, upper-left=2(−), upper-right=4(−), lower-left=1(+), lower-right=3(+). Looking from the solder side mirrors left/right. The prior instruction to put jack1/4 on positive is withdrawn. The PSU's own labeled assignment remains unchanged: R7B1/4 positive,2/3 negative. This jack mapping is an inference from the keyed mating-face drawings, not a manufacturer-issued combined harness schematic.

Mandatory assembly polarity test: with every J0 pigtail individually insulated and disconnected from the rest of the box, mate the supplied brick and measure all four jack tails against each other. Confirm1-to2 and3-to4 approximately+24 V;1-to3 and2-to4 approximately0 V. Then remove power and join the specified pairs. This verifies actual supplied connector numbering before any parallel connection can short the supply.
