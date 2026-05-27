# Cell Culture Starter BOM Research

This is a practical sourcing snapshot for the starter cell-culture equipment matrix. It is not a purchasing approval list; final selections still need datasheet review, fit checks, and facility biosafety approval where applicable.

| Category | Practical starter examples | Rough cost | Decision | Notes |
| --- | --- | ---: | --- | --- |
| CO2 sensor/control | CO2Meter ExplorIR-M 20%, SprintIR/SprintIR-R 20%, Micro-Hybrid MicroSENS 180-HS 0-20% incubator sensor | $197-$550 starter/OEM; MicroSENS roughly EUR 834-EUR 1,276 | Build control module | Use 0-20%, not MH-Z19B. 5% CO2 is 50,000 ppm. MicroSENS is closer to real incubator OEM spec; ExplorIR-M is a more practical starter module. |
| Independent CO2 analyzer | Bacharach Fyrite 0-20% CO2 analyzer/test kit or borrowed/rented calibrated analyzer | About $760 when available | Buy/access | Use as independent validation, not the controller sensor. |
| Sample pump | CO2Meter micro-pump kit, Universal Analyzers Mini DiaVac, Hilintec C30/D27 gas sampling pumps | $30-$250+ | Build module | Keep outside humid chamber. Use low flow, condensate protection, and filters. |
| Hydrophobic/sterile filters | Cytiva Whatman ReZist 50 mm sterile 0.2 um PTFE, Sartorius Minisart PTFE 0.2 um, Merck/Sigma Millex-FG 50 mm PTFE, Corning 431227 sterile PTFE | $5-$25/filter depending pack | Buy | Use on sample path and vacuum/aspirator protection. 50 mm PTFE vent filters are better than small syringe filters for vacuum lines. |
| CO2 regulator/solenoid | AQUATEK paintball regulator with solenoid, FZONE aquarium regulator, Milwaukee MA957, CO2Art dual-stage regulator | $60-$250 | Buy | Aquarium/paintball parts are acceptable for prototype injection only with check valve, restrictor/needle valve, leak testing, and pressure relief. Move toward proper CGA-320 regulator for lab credibility. |
| Temp/RH logger | Lascar/DataQ EL-SIE, Elitech GSP-6G, Monarch Track-It with optional NIST certificate | $130-$300 plus calibration | Buy | Independent logger is mandatory. RH probes in incubators need droplet protection and may be consumable. |
| Water bath safety | Commercial Thermo/PolyScience/VWR/MSE Pro bath, or low-voltage DIY heater with GFCI, fuse, thermal cutoff, low-water switch, and independent thermometer | DIY $60-$200; commercial $300-$1,500+ | Buy or cautious build | Water plus mains power is high risk. Prefer low-voltage heating and an external GFCI. Independent over-temperature cutoff is not optional. |
| Aspirator/vacuum trap | Two-flask trap train with disinfectant collection flask, secondary overflow flask, inline hydrophobic HEPA/PTFE filter, Welch Biovac/Aspire class aspirator | DIY support $100-$300; commercial $500-$1,500+ | Buy/build holder only | Build brackets/tray, not the biosafety logic. Protect the pump or house vacuum and follow disinfectant contact-time rules. |
| PPE/waste basics | Lab coat/gown, nitrile gloves, eye protection, biohazard bags, sharps container, disinfectant, spill kit, liquid waste bottle/trap | $150-$500 starter | Buy | Facility biosafety rules override this list. No DIY disposal assumptions for human, iPSC, or viral workflows. |
| Microscope | Accu-Scope EXI-310 inverted phase contrast, Globe/Oxion Inverso, AmScope IN330TC class | $2,000-$5,000+ | Buy/access | Must be inverted phase contrast for routine mammalian culture. Brightfield upright scope is not sufficient for flasks/plates. |
| Centrifuge | Drucker BOOST 4, Eppendorf 5702/5810 class, clinical/bench centrifuge with 15/50 mL support | $2,000-$8,000+ | Buy | Do not DIY centrifuges. Use correct RCF, rotor/bucket compatibility, imbalance safety, and cleanable carriers. |
| Pipettes | Gilson PIPETMAN P20/P200/P1000, Rainin Classic, Eppendorf Research plus, pipette aid | Refurb $300-$700; new $900-$1,500+ | Buy | Use calibrated micropipettes, filtered sterile tips, and preferably a separate culture set. |

## CAD Impact

- `co2_sensor_service_module` stays dimensionally provisional until the exact 0-20% NDIR sensor, pump, filters, and fittings are selected.
- `aspirator_waste_trap_holder` is a mechanical support part only. It does not validate disinfectant chemistry, contact time, biosafety containment, or disposal.
- `cell_culture_logger_enclosure` should house independent validation logging, not the incubator control loop.
- Do not put the DIY incubator, still-air enclosure, aspirator holder, or printed centrifuge adapters on the website as culture-ready equipment.
