# Single AAV Condition Module Mockup

Ticket: T-AC301220

This document describes the first printable visualization model for the new
LaminarForge standard-footprint AAV condition module architecture.

This is a dry visualization and no-cell mechanical planning model. It is not a
wetted path, sterile part, live-cell fixture, AAV-contact part, vendor drawing,
or biological validation article.

## Generator

Run:

```text
mcp__agentic_mcp.laminarforge_build action=run repo=laminarforge-cad bin=single_aav_condition_module_mockup
```

Output directory:

```text
output/single_condition_module/
```

## Outputs

| Output | Purpose |
| --- | --- |
| `single_aav_condition_module_mockup.stl` | Main single-condition module visualization: SLAS-size base, one local readout zone, gasket witness frame, optical window, connector placeholders, ID land, fiducials, and tray-contact witness. |
| `single_aav_condition_tray_reference.stl` | Cheap non-critical tray reference. It locates and protects the module but does not seal, align precision features, or route fluid. |
| `single_aav_connector_face_coupon.stl` | Connector-face comparison coupon with three placeholder regions for commercial microfluidic fitting, magnetic/gasketed connector, and luer bench-adapter scale checks. |
| `single_aav_16_zone_scale_ghost.stl` | Scale study showing how a later 16 same-condition zone layout might fit inside the SLAS footprint without committing to final fluidics. |

## Current Dimensions

| Feature | Value |
| --- | ---: |
| Module footprint | 127.76 x 85.48 mm |
| Approx footprint in inches | 5.03 x 3.36 in |
| Visualization service height | 17.0 mm / 0.67 in |
| Local readout placeholder | 58.0 x 36.0 mm |
| Optical window placeholder | 40.0 x 22.0 mm |
| Tray reference envelope | 147.76 x 105.48 mm |

These dimensions are for visualization only. The final module height, connector
family, optical stack, and local readout geometry remain open design decisions.

## What This Model Shows

- One module equals one AAV condition.
- First validation can start with one local readout zone.
- The useful scale target remains 16 same-condition zones after local mechanics
  validate.
- Precision stays local to the chip/readout, gasket, connector, optical datum,
  and ID features.
- The tray is deliberately non-critical.

## What To Inspect On The Bambu Print

- Whether the 5.03 x 3.36 in footprint feels practical in hand.
- Whether the local readout zone has enough working room.
- Whether the connector placeholders crowd the optical/readout region.
- Whether the ID land and fiducials are visible and reachable.
- Whether the tray concept reads as a holder rather than a sealing fixture.
- Whether the 16-zone ghost makes the scale target feel plausible or too dense.

## Hard Boundaries

- Do not put cells, media, AAV, or sterile tubing through these printed parts.
- Do not treat connector placeholders as selected connector SKUs.
- Do not treat the 16-zone ghost as final fluid routing.
- Do not let the tray become a sealing or precision alignment dependency.
