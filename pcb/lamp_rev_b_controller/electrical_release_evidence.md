# Electrical Release Evidence Reference

Artifact: `A-9F7C4851`  
Artifact content SHA-256: `115195d8b4461a74d18234f57c7f51d3dacb4eb2943bd202a2291956dc6957bc`  
Canonical evidence commit: `9cd44a524f9804889fc87a89cfe2b9c1c56ab40f`

The artifact records physical DRC 0, active unconnected 0 after three reviewed identical-self-zone artifacts, 154 accepted item-scoped ERC findings with 0 blocking electrical findings, and BOM/CPL/footprint issue count 0. The manufacturer-handoff generator does not trust ignored checkout caches; it regenerates DRC and ERC into each temporary build and re-evaluates those same item-scoped rules.

Any source, population, placement, schematic, stackup, or copper change invalidates the recorded evidence and requires fresh gates. The handoff generator binds its validation summary and package manifest to its own full source commit.
