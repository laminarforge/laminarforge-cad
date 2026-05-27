use vcad::{centered_cube, centered_cylinder, Part};

// External CO2 sensor service module for incubator-range 0-20% NDIR sensors.
// The incubator chamber only gets gas/sample/relief ports; the humid-sensitive
// sensor, sample pump, filters, and tubing service points stay outside.
//
// Exports:
//   output/co2_sensor_service_module_base.stl
//   output/co2_sensor_service_module_cover.stl
//   output/co2_sensor_service_module_filter_clip.stl
//   output/co2_sensor_service_module_assembly.stl

fn main() {
    let base_x = 180.0;
    let base_y = 100.0;
    let base_z = 8.0;

    let base_plate = centered_cube("co2_service_base", base_x, base_y, base_z);

    let mut mount_holes = Part::empty("co2_service_mount_holes");
    for x in [-75.0, 75.0] {
        for y in [-35.0, 35.0] {
            mount_holes = mount_holes
                + centered_cylinder("co2_service_mount", 4.3 / 2.0, base_z + 2.0, 24)
                    .translate(x, y, 0.0);
        }
    }

    let sensor_footprint = centered_cube("sensor_footprint", 62.0, 38.0, 2.0).translate(
        -42.0,
        8.0,
        base_z / 2.0 + 1.0,
    );
    let pump_footprint =
        centered_cube("pump_footprint", 38.0, 28.0, 2.0).translate(42.0, 10.0, base_z / 2.0 + 1.0);
    let manifold_block = centered_cube("sample_manifold", 120.0, 18.0, 14.0).translate(
        0.0,
        -32.0,
        base_z / 2.0 + 7.0,
    );

    let mut barb_bores = Part::empty("barb_bores");
    for (i, x) in [-45.0, 0.0, 45.0].iter().enumerate() {
        let bore = centered_cylinder(format!("barb_bore_{i}"), 4.0 / 2.0, 22.0, 24)
            .rotate(90.0, 0.0, 0.0)
            .translate(*x, -32.0, base_z / 2.0 + 7.0);
        barb_bores = barb_bores + bore;
    }

    let base = base_plate - mount_holes
        + sensor_footprint
        + pump_footprint
        + (manifold_block - barb_bores);
    base.write_stl("output/co2_sensor_service_module_base.stl")
        .unwrap();

    let cover_outer =
        centered_cube("co2_service_cover_outer", base_x, base_y, 54.0).translate(0.0, 0.0, 31.0);
    let cover_inner = centered_cube("co2_service_cover_inner", base_x - 6.0, base_y - 6.0, 50.0)
        .translate(0.0, 0.0, 30.0);
    let tube_window =
        centered_cube("tube_window", 130.0, 8.0, 18.0).translate(0.0, -base_y / 2.0, 25.0);
    let cable_gland = centered_cylinder("cable_gland", 12.0 / 2.0, 8.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(70.0, base_y / 2.0, 22.0);
    let cover = cover_outer - cover_inner - tube_window - cable_gland;
    cover
        .write_stl("output/co2_sensor_service_module_cover.stl")
        .unwrap();

    let clip_body = centered_cube("filter_clip_body", 60.0, 20.0, 20.0);
    let filter_channel =
        centered_cylinder("filter_channel", 7.0 / 2.0, 62.0, 24).rotate(0.0, 90.0, 0.0);
    let slot = centered_cube("filter_clip_slot", 62.0, 6.0, 12.0).translate(0.0, 6.0, 0.0);
    let filter_clip = clip_body - filter_channel - slot;
    filter_clip
        .write_stl("output/co2_sensor_service_module_filter_clip.stl")
        .unwrap();

    let assembly = base
        + cover
        + filter_clip.translate(-45.0, -62.0, 18.0)
        + filter_clip.translate(45.0, -62.0, 18.0);
    assembly
        .write_stl("output/co2_sensor_service_module_assembly.stl")
        .unwrap();

    println!("Exported: output/co2_sensor_service_module_base.stl");
    println!("Exported: output/co2_sensor_service_module_cover.stl");
    println!("Exported: output/co2_sensor_service_module_filter_clip.stl");
    println!("Exported: output/co2_sensor_service_module_assembly.stl");
    println!("External module: sample in, filtered sensor path, pump bay, relief/return manifold.");
}
