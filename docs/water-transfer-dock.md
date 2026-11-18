# Water-transfer dock — first CAD candidate

Ticket: T-19E61FA5. Concept decision: A-1E21CAD0.

This model implements the dry parts from the approved single-lane water/dye
concept: a removable coupon cradle, two screw-mounted tubing guides and a
closed-floor catch tray. It complements the existing syringe-pump workstream.
The pump, clear fluidic coupon, fittings, tubing and collection balance are
separate components. No biological operation is included.

## Candidate interfaces

`models/water_transfer_dock.toml` contains required runtime dimensions in
millimeters. The nominal coupon envelope is 75 × 25 × 4 mm. Length and width
come from the proposed Rev D0 first article; **4 mm total thickness is a fit
assumption**, comprising the proposed 3 mm body plus an unselected bottom.
Neither that envelope nor the tube diameter is an approved vendor interface.
Measure the selected parts and revise the TOML before releasing a print.

The cradle supports the coupon around its edges and leaves the middle open for
inspection. Lift it out to inspect the underside; the tray below is opaque.
Gravity-seated feet locate the cradle in blind sockets. There is no claim of
positive retention during transport or of a validated snap fit.

The tube guides attach to pads outside the catch basin with purchased screws
and nuts. Their open saddles and cable-tie passages organize the tubing; they
are not validated strain-relief clamps. Check tube slip and bore deformation
with the actual tubing. No mounting hole penetrates the basin floor.

The nominal basin body is 160 × 100 × 10.4 mm; external mounting pads make the
tray's overall length 195.2 mm. The cradle is 81.6 × 31.6 × 12 mm with a
71 × 21 mm viewing opening. These are candidate CAD dimensions, not measured
or printed results. The guide is 20 × 32 × 14.8 mm; print two copies.

The tray is secondary spill capture. An FDM print is not assumed liquid-tight.
Check its floor, walls and sockets physically before relying on it to contain a
spill. Printed parts remain outside the normal wetted path.

## Model and verification contract

The `water_transfer_dock` binary requires `--config PATH --output-dir PATH`.
Missing, unknown, nonfinite or mechanically invalid parameters fail before
publication. Its `--verify` mode uses the same configuration to regenerate
geometry and compare the publication, including configuration and executable
identities. The publication includes individual print parts, a dry assembly,
a separately named coupon reference, a bill of materials, assembly transforms
and interface evidence. Reference and assembly meshes are not print parts.

| File | Intended use |
| --- | --- |
| `water_transfer_dock_tray_PRINT.stl` | One tray |
| `water_transfer_dock_coupon_cradle_PRINT.stl` | One removable cradle |
| `water_transfer_dock_tube_guide_PRINT.stl` | Two identical guides |
| `water_transfer_dock_assembly_DRY_with_REF_coupon.stl` | Assembly viewing aid |
| `water_transfer_dock_REF_coupon_NOT_FOR_MANUFACTURE.stl` | Blank coupon fit envelope only |
| `water_transfer_dock.interfaces.json` | BOM, assembly transforms, dimensions and checked mesh points |
| `water_transfer_dock.manifest.json` | Publication identity and output hashes |

Each STL has a sibling `.preview.svg`. The reference coupon deliberately has no
channels, ports or bond definition; it must not be fabricated as the fluidic
article. Use the printed-part transforms in the interface report to reconstruct
the assembly. Review supports and orientation in the selected slicer before
printing; no support-free or printer-ready claim is made.

SVG previews project the actual STL vertices into a dark isometric view. They
are viewing aids; the renderer is not a clearance or fabrication verifier.
No STEP conversion or analytic machining definition is claimed.

Compilation is requested through `submit_work` to canonical `main`. The
registered `cad-priming-dev` recipe compiles this target and retains the existing
priming fixture. Do not invoke Cargo, Make or MCP build/check/run actions.

After a matching supported executable receipt is available, the intended MCP
execution request is:

```json
{
  "repo": "laminarforge-cad",
  "action": "execute",
  "bin": "water_transfer_dock",
  "profile": "dev",
  "args": [
    "--config", "models/water_transfer_dock.toml",
    "--output-dir", "/Volumes/ORICO/MacMiniBuilds/projects-output/water-transfer-dock-T-19E61FA5/candidate-01"
  ]
}
```

Verify with the same request plus `--verify`, then open the actual assembly and
component previews and inspect the meshes. A pipeline `compiled-binary`
receipt alone does not authorize this execution. At implementation time, the
public MCP has no pipeline-artifact import/receipt-registration action; geometry
execution and all generated-output inspection remain pending that adapter.
Keep task outputs on the mounted ORICO filesystem and use a new directory for
each changed candidate.

## Physical checkpoint

First establish the coupon bond and bought fitting/tubing dimensions. Fit the
dry parts without forcing the coupon, verify that tubing loads do not move it,
and check that the outlet can enter the independent collection vial without
touching the vial or balance. Transfer water through the complete inlet,
chamber and outlet; record elapsed time, collected mass, visible leakage and
trapped bubbles. A successful transfer does not establish flow accuracy,
pressure rating, sterility or biological readiness.

Source implementation, compilation, generated geometry, printed fit and actual
water transfer are separate checkpoints. The ticket records which were
observed; this document does not claim that any physical part was made.
