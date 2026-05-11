use vcad::centered_cylinder;

// ─── Centrifuge Adapters ───
//
// 3D-printed (PETG) adapter rings for a standard mini centrifuge rotor.
// Allows the rotor to hold different tube sizes by providing a stepped
// bore: the outer diameter fits the rotor slot, the inner bore is sized
// for the specific tube type.
//
// Exports:
//   - centrifuge_adapter_02ml.stl (for 0.2mL PCR tubes, 3.5mm ID)
//   - centrifuge_adapter_15ml.stl (for 1.5mL microcentrifuge tubes, 11mm ID)

fn main() {
    // ══════════════════════════════════════════════════════════════
    // 1. 0.2mL PCR TUBE ADAPTER
    // ══════════════════════════════════════════════════════════════

    let adapter_02_od = 6.5;
    let adapter_02_id = 3.5;
    let adapter_02_height = 20.0;

    let adapter_02_outer = centered_cylinder(
        "adapter_02_outer",
        adapter_02_od / 2.0,
        adapter_02_height,
        32,
    );
    let adapter_02_bore = centered_cylinder(
        "adapter_02_bore",
        adapter_02_id / 2.0,
        adapter_02_height + 2.0,
        24,
    );

    let adapter_02 = adapter_02_outer - adapter_02_bore;

    adapter_02
        .write_stl("output/centrifuge_adapter_02ml.stl")
        .unwrap();

    println!("Exported: output/centrifuge_adapter_02ml.stl");

    // ══════════════════════════════════════════════════════════════
    // 2. 1.5mL MICROCENTRIFUGE TUBE ADAPTER
    // ══════════════════════════════════════════════════════════════

    let adapter_15_od = 6.5;
    let adapter_15_id = 11.0;
    let adapter_15_height = 25.0;

    // For this adapter the ID is larger than OD — this means the adapter
    // needs a wider outer shell. The OD here refers to the rotor slot it
    // fits INTO, but since a 1.5mL tube is 11mm, the adapter outer must
    // be at least 11mm + wall. So the outer actually needs to be larger.
    let adapter_15_actual_od = adapter_15_id + 3.0; // 14mm outer for 11mm bore
    let adapter_15_rotor_stem_d = adapter_15_od; // 6.5mm stem at bottom for rotor

    // Two-part design: wider top section for tube, narrow stem for rotor
    let stem_height = 8.0;
    let cup_height = adapter_15_height - stem_height;

    // Cup section (holds the tube)
    let cup_outer = centered_cylinder("cup_outer", adapter_15_actual_od / 2.0, cup_height, 32)
        .translate(0.0, 0.0, stem_height / 2.0);

    let cup_bore = centered_cylinder("cup_bore", adapter_15_id / 2.0, cup_height + 1.0, 24)
        .translate(0.0, 0.0, stem_height / 2.0);

    // Stem section (fits into rotor)
    let stem_outer =
        centered_cylinder("stem_outer", adapter_15_rotor_stem_d / 2.0, stem_height, 32).translate(
            0.0,
            0.0,
            -(cup_height / 2.0),
        );

    // Through-bore in stem (same ID or slightly smaller for tube tip clearance)
    let stem_bore = centered_cylinder(
        "stem_bore",
        (adapter_15_id - 2.0) / 2.0,
        stem_height + 1.0,
        24,
    )
    .translate(0.0, 0.0, -(cup_height / 2.0));

    let adapter_15 = (cup_outer + stem_outer) - cup_bore - stem_bore;

    adapter_15
        .write_stl("output/centrifuge_adapter_15ml.stl")
        .unwrap();

    println!("Exported: output/centrifuge_adapter_15ml.stl");

    // ── Print specs ──

    println!();
    println!("── Centrifuge Adapter Specs ──");
    println!("  0.2mL adapter:");
    println!("    Outer:  {adapter_02_od:.1}mm OD x {adapter_02_height:.0}mm tall");
    println!("    Bore:   {adapter_02_id:.1}mm ID (for 0.2mL PCR tubes)");
    println!("  1.5mL adapter:");
    println!("    Cup:    {adapter_15_actual_od:.0}mm OD x {cup_height:.0}mm");
    println!("    Bore:   {adapter_15_id:.0}mm ID (for 1.5mL tubes)");
    println!(
        "    Stem:   {adapter_15_rotor_stem_d:.1}mm OD x {stem_height:.0}mm (fits rotor slot)"
    );
    println!("    Total:  {adapter_15_height:.0}mm tall");
    println!("  Material: PETG");
}
