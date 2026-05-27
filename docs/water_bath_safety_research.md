# Water Bath Safety Research Notes

The starter water bath should be treated as a low-voltage warming prototype, not as a generic mains immersion-heater project. The safer path is to buy commodity safety-critical components and design the mechanical retention, routing, and validation fixtures around them.

## Research Findings

| Topic | Design impact | Source |
| --- | --- | --- |
| Immersion heaters can be a fire hazard if unattended, run dry, or used outside manufacturer instructions | Avoid exposed mains immersion heaters. Use low-voltage heating, independent cutoff, and low-water detection. | UC Berkeley EHS fire hazard note: https://ehs.berkeley.edu/news/fire-hazard-immersion-water-heaters |
| Commercial lab baths emphasize over-temperature protection, bath covers, temperature uniformity, and user controls | Prototype should include independent probe placement, splash/drip control, and validation logging. | Thermo Precision water baths brochure: https://assets.thermofisher.com/TFS-Assets/LED/brochures/Precision-Water-Baths-Brochure-BRTCPRECISION.pdf |
| Lab bath manuals specify fluid level, cleaning, electrical safety, and operating limits | Add low-water bracket, cable strain relief, spill tray, and cleaning-friendly removable wet parts. | PolyScience operator manual: https://www.polyscience.com/media/tcdmnooj/110-349.pdf |
| Water baths can become contamination reservoirs in cell-culture workflows | Prefer sealed bottles, regular cleaning, and dry-block warming when compatible. | Sigma contamination troubleshooting: https://www.sigmaaldrich.com/US/en/technical-documents/technical-article/cell-culture-and-cell-culture-analysis/mammalian-cell-culture/cell-culture-troubleshooting-contamination |
| Dry baths avoid standing water but need good thermal contact and correct blocks | Keep `heating_block` as the lower-contamination alternative for small tubes. | Corning dry bath product reference: https://ecatalog.corning.com/life-sciences/b2c/US/en/Equipment/Constant-Temperature-Equipment/Dry-Bath-Heaters/Corning%C2%AE-LSE%E2%84%A2-Digital-Dry-Bath-Heater/p/6875-SB |

## CAD Response

`water_bath_safety_kit` adds:

- bottle/tube retention rack
- independent probe clamp
- cable guard / strain relief
- low-water float-switch bracket
- spill/splash tray
- visual assembly

## Validation Gates

- independent probe shows 37 C +/- 0.5 C at bottle contact region
- over-temperature cutoff removes heater power without firmware
- low-water switch trips before heater exposure
- no cable exits below the wet plane
- bottle rack prevents tip-over under normal bench bumps
- spill tray contains at least a minor splash or bottle drip event
- cleaning procedure does not require disassembling electronics
