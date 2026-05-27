# Cell Culture CFD and Thermal Validation Plan

Use CFD as an engineering filter, not as evidence of sterility, biosafety, or cell readiness. For LaminarForge, the right sequence is:

1. CAD generation and deterministic geometry checks.
2. Lumped thermal/CO2 models for fast control and sizing gates.
3. CFD/CHT for airflow, mixing, gradients, and door-open recovery.
4. Physical logger data and biological validation before any culture claims.

## Recommended Stack

| Layer | Tool | Role |
| --- | --- | --- |
| Geometry | Rust CAD bins | Generate air volumes, walls, shelves, trays, diffuser, ports, and service manifolds |
| Meshing | Gmsh first; OpenFOAM `snappyHexMesh` later if needed | Scriptable mesh generation and refinement around shelves, fan, ports, and door opening |
| Mesh gate | OpenFOAM `checkMesh` | Fail poor meshes before solver time is spent |
| Air/heat CFD | OpenFOAM `buoyantPimpleFoam` | Transient buoyant airflow and temperature distribution in the incubator air volume |
| CO2 mixing | OpenFOAM passive scalar transport | CO2 source, mixing, leakage, relief, and door-open recovery |
| Conjugate heat transfer | OpenFOAM `chtMultiRegionFoam` | Later wall/door/insulation/heater conduction model |
| Secondary multiphysics | Elmer | Thermal and natural-convection checks when OpenFOAM is heavier than needed |
| Optimization option | SU2 | Later CHT/adjoint studies, not the first indoor-air validation path |
| Post-processing | Rust metrics parser plus ParaView/VTK | Shelf-zone min/max, gradients, recovery time, dead-zone volume, wall heat flux |

## First OpenFOAM Case

Start with a simplified `validation/openfoam/co2_incubator/` case generated from Rust templates:

- air region only
- fan/diffuser as prescribed velocity or momentum source
- heater diffuser as heat-flux or fixed-temperature boundary
- CO2 inlet as scalar source
- sample/relief ports as outlet/leak boundaries
- shelves as flow obstructions
- 60-second door-open event matching `co2_incubator_sim`

Acceptance metrics should match the lumped simulator where possible:

| Metric | Initial gate |
| --- | --- |
| Shelf-zone temperature spread | <= 0.5 C at steady state |
| CO2 steady-state spread near cultureware | <= 0.3 percentage points after mixing |
| Door-open recovery | <= 20 min to return near 37 C and 5% CO2 |
| Dead-zone volume | Report volume below low-velocity threshold; gate after first baseline |
| Heater-adjacent hot region | No cultureware-adjacent surface above the physical acceptance threshold |

## What CFD Can Validate

- Fan and diffuser placement.
- CO2 inlet placement and short-circuit risk to relief/sample ports.
- Shelf-to-shelf temperature and CO2 gradients.
- Door-open recovery sensitivity.
- Local hot/cold zones near window, door, wall seams, and water tray.
- Whether larger `chip_incubator_v3` geometry blocks airflow or scales poorly.
- Airflow uniformity for a clean-bench-style prototype.

## What CFD Cannot Validate

- Sterility.
- HEPA filter integrity.
- Biosafety containment.
- Microbial contamination outcomes.
- Sensor calibration.
- Humidity reliability without condensation/evaporation testing.
- Cell viability, morphology, growth rate, or protocol suitability.
- Regulatory or certification claims for a biosafety cabinet or incubator.

## Sterile Handling Boundary

For biosafety cabinets and sterile hoods, CFD can identify airflow problems, nonuniform face velocity, recirculation, or disturbance sensitivity. It cannot certify product/personnel/environment protection. Real mammalian culture still requires a certified Class II BSC or facility-approved sterile hood, plus physical certification and SOPs.

## Source Notes

- OpenFOAM `buoyantPimpleFoam`: https://doc.openfoam.com/2312/tools/processing/solvers/rtm/heat-transfer/buoyantPimpleFoam/
- OpenFOAM `chtMultiRegionFoam`: https://doc.openfoam.com/2306/tools/processing/solvers/rtm/heat-transfer/chtMultiRegionFoam/
- OpenFOAM scalar transport source reference: https://api.openfoam.com/2406/scalarTransportFoam_8C_source.html
- Incubator CFD door/opening recovery study: https://www.degruyterbrill.com/document/doi/10.1515/cdbme-2016-0073/html
- Biosafety cabinet CFD and physical validation context: https://pmc.ncbi.nlm.nih.gov/articles/PMC9402246/
- Elmer natural convection example: https://github.com/ElmerCSC/elmerfem/blob/devel/fem/tests/NaturalConvection2/case.sif
- SU2 CHT tutorial: https://su2code.github.io/tutorials/Static_CHT/
- SU2 multizone documentation: https://su2code.github.io/docs_v7/Multizone
- Gmsh/FreeCAD meshing reference: https://github.com/FreeCAD/FreeCAD-documentation/blob/main/wiki/FEM_MeshGmshFromShape.md
- OpenFOAM mesh conversion guide: https://www.openfoam.com/documentation/user-guide/4-mesh-generation-and-conversion/4.5-mesh-conversion
- FreeCAD CfdOF workbench: https://github.com/jaheyns/cfdof
