# Integrated Sealed Swab Diagnostic Cartridge

Ticket: `T-588F689A`  
Revision: `P1-INT-R0`  
Publication stem: `swab_integrated_sealed_diagnostic_cartridge`

## Design boundary

This is one proposed parametric engineering architecture, not a manufacturing release, wet-lab recipe, validated assay, clinical claim, shelf-life claim, disposal classification, or vendor/material qualification. Dimensions and pressure values are design envelopes to make interfaces visible and testable; they require physical and assay-owner validation before use.

The model implements one irreversible disposable wet path:

`wash pouch → permanently closed swab/elution cup → heated lysis pocket → settling/debris barrier → paired meters → isolated target/control dry-reagent vestibules → clear amplification/optical chambers → high-point bubble outlets → one-way terminal waste`

Excess metering liquid goes to that same terminal waste sink. The two target/control lanes are required isolated branches of the single architecture, not alternate cartridge architectures. A physically separate gas-only route leaves captive waste headspace, traverses splash baffles and a vent plenum, and terminates at the hydrophobic aerosol-barrier membrane. No amplification chamber vents directly outside.

## Visible parametric features

- A rigid swab cup with hard stop, cartridge-owned compression grommet, retained shaft, seated permanent cap, and tamper-evident irreversible latch bridge.
- A separately sealed 600 µL wash-pouch envelope under dry reader plunger A.
- Two independently sealed 22.4 µL reaction-fill pouch lobes under the equalizing platen of dry reader plunger B.
- A laminated planar path with five represented pressure stages, forward flap witnesses, reverse stops, a 1.2 mm channel envelope, and no gravity-dependent or reusable wet interface.
- A heater-coupled lysis region, supported debris barrier, two 2.5 µL-class overflow meters, two peripheral dry-deposit vestibules, and two 22.4 µL clear optical chambers.
- Bottom-up fill ramps and separate high-point outlets that keep the optical regions free of intentional blind pockets.
- Dedicated reusable-reader contact geometry: separate lysis/amplification thermal pads, paired optical alignment frames, round-hole/relief-slot 3-2-1 registration, asymmetric keying, clamp datums, and two dry plunger guides.
- One terminal waste chamber with an irreversible inlet, mechanically captured low-shedding pad representation, noncompressive retainers, at least 20% reserved headspace, two splash baffles, and a physically offset gas-only hydrophobic aerosol-barrier vent.

The CAD uses raised internal witness geometry so the complete route is visible in the opaque STL/STP/USDZ review model. Those relief heights are visualization devices; the manifest's fluidic dimensions are the proposed engineering envelopes.

## Parametric gates

The generator fails immediately unless all gates pass:

- exactly two dry reader plungers and two isolated optical chambers;
- idealized wash-pouch volume of 600 µL and each reaction-fill lobe/optical chamber at 22.4 µL;
- minimum represented perimeter seal land of 3.0 mm;
- monotonically increasing one-way pressure stages, with the highest proposed operating stage no greater than 50% of the proposed minimum destructive-pressure screen;
- every liquid source reaches the one terminal waste node through an acyclic directed graph;
- no liquid edge reaches the exterior barrier;
- proposed confined-pad retained capacity at least 2× maximum possible delivered liquid;
- at least 20% reserved waste-cell headspace;
- zero reusable reader wet interfaces and zero open liquid outlets.

The selected architecture contains no evaporation/desiccation route, superabsorbent gel, waste heating, capture/wash alternate, reusable pump, reusable wet plumbing, or alternate fluid route.

## Outputs and deterministic verification

The publication generator writes root-level:

- `output/swab_integrated_sealed_diagnostic_cartridge.stl`
- `output/swab_integrated_sealed_diagnostic_cartridge.stp`
- `output/swab_integrated_sealed_diagnostic_cartridge.manifest.json`

The LaminarForge build workflow converts the same-stem STP into the root-level USDZ used by the authenticated Agentic Flowstate CAD API/tab. The verification bin checks the architecture gates, tracked manifest, generated output sizes/hashes, and a byte-identical second STL export.

## Authority

The design is constrained by artifacts `A-A1A77D11`, `A-345211E9`, `A-696CE730`, `A-CE59D39F`, `A-BC05D7E3`, `A-EE7B1198`, and `A-7C487534`. Where earlier research described optional or fallback concepts, this integrated model follows the later ticket-specific decision: two dry actuators, sealed pouch cells, confined passive absorbent waste, and no alternate architecture.
