//! Rev D water-test prototype: analytic fabrication solids, assembly sheets and sizing evidence.
use base64::Engine;
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
use vcad::{centered_cube, centered_cylinder};
#[path = "../cassette/assembly_drawings.rs"]
mod assembly_drawings;
#[path = "../cassette/drawings.rs"]
mod drawings;
#[path = "../cassette/feature_drawings.rs"]
mod feature_drawings;
#[path = "../cassette/joints.rs"]
mod joints;
#[path = "../cassette/package.rs"]
mod package;
#[path = "../cassette/solid.rs"]
mod solid;
#[path = "../cassette/thermal.rs"]
mod thermal;
use solid::Part;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    config: PathBuf,
    #[arg(long)]
    output_dir: PathBuf,
    /// Validate all dimensions without producing geometry.
    #[arg(long)]
    validate_only: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Config {
    plate_x: f64,
    plate_y: f64,
    surrogate_plate_z: f64,
    plate_clearance: f64,
    nest_wall: f64,
    nest_height: f64,
    nest_side_margin: f64,
    front_margin: f64,
    top_clearance: f64,
    side_wall: f64,
    roof_thickness: f64,
    drawer_thickness: f64,
    guide_overlap: f64,
    guide_inreach: f64,
    guide_rail_thickness: f64,
    shoe_thickness: f64,
    guide_clearance: f64,
    lateral_clearance: f64,
    rear_retention: f64,
    access_margin: f64,
    rear_clearance: f64,
    rear_thickness: f64,
    bezel_thickness: f64,
    flange_thickness: f64,
    bezel_side_extension: f64,
    bezel_bottom_extension: f64,
    bezel_top_extension: f64,
    opening_clearance: f64,
    gasket_land: f64,
    gasket_width: f64,
    gasket_free_thickness: f64,
    gasket_groove_depth: f64,
    gasket_backing_thickness: f64,
    gasket_outer_radius: f64,
    heater_x: f64,
    heater_y: f64,
    heater_recess_depth: f64,
    stop_pin_diameter: f64,
    stop_slot_width: f64,
    stop_y: f64,
    mesh_segments: u32,
    thermal: thermal::Parameters,
    joints: joints::Parameters,
}

#[derive(Debug, Serialize)]
struct Layout {
    nest_x: f64,
    nest_y: f64,
    cavity_x: f64,
    outer_x: f64,
    drawer_x: f64,
    drawer_y: f64,
    drawer_bottom: f64,
    drawer_top: f64,
    upper_shoe_bottom: f64,
    guide_ceiling: f64,
    guide_width: f64,
    plate_center_y: f64,
    plate_rear_y: f64,
    stroke: f64,
    roof_bottom: f64,
    roof_top: f64,
    rear_y: f64,
    overall_y: f64,
    bezel_x: f64,
    bezel_min_z: f64,
    bezel_max_z: f64,
    opening_x: f64,
    opening_min_z: f64,
    opening_max_z: f64,
    gasket_outer_x: f64,
    gasket_outer_z: f64,
    gasket_center_z: f64,
    stop_x: f64,
    rail_screw_x: f64,
}

impl Config {
    fn layout(&self) -> Layout {
        let nest_x = self.plate_x + 2.0 * (self.plate_clearance + self.nest_wall);
        let nest_y = self.plate_y + 2.0 * (self.plate_clearance + self.nest_wall);
        let cavity_x = nest_x + 2.0 * self.nest_side_margin;
        let outer_x = cavity_x + 2.0 * self.side_wall;
        let drawer_x = cavity_x + 2.0 * self.guide_overlap;
        let drawer_bottom = self.guide_rail_thickness + self.shoe_thickness;
        let drawer_top = drawer_bottom + self.drawer_thickness;
        let upper_shoe_bottom = drawer_top + self.guide_clearance;
        let guide_ceiling = upper_shoe_bottom + self.shoe_thickness;
        let guide_width = drawer_x + 2.0 * (self.shoe_thickness + self.lateral_clearance);
        let plate_center_y = self.bezel_thickness + self.front_margin + nest_y / 2.0;
        let plate_rear_y = plate_center_y + self.plate_y / 2.0;
        let stroke = plate_rear_y + self.access_margin;
        let drawer_y = stroke + self.bezel_thickness + self.rear_retention;
        let rear_y = drawer_y + self.rear_clearance;
        let roof_bottom = drawer_top + self.surrogate_plate_z + self.top_clearance;
        let roof_top = roof_bottom + self.roof_thickness;
        // Opening encloses the moving heater guard and screw heads as well as plate.
        let opening_min_z = drawer_bottom - 14.0;
        let opening_max_z = roof_bottom;
        let opening_x = drawer_x + 2.0 * self.opening_clearance;
        Layout {
            nest_x,
            nest_y,
            cavity_x,
            outer_x,
            drawer_x,
            drawer_y,
            drawer_bottom,
            drawer_top,
            upper_shoe_bottom,
            guide_ceiling,
            guide_width,
            plate_center_y,
            plate_rear_y,
            stroke,
            roof_bottom,
            roof_top,
            rear_y,
            overall_y: self.flange_thickness + rear_y + self.rear_thickness,
            bezel_x: outer_x + 2.0 * self.bezel_side_extension,
            bezel_min_z: -self.bezel_bottom_extension,
            bezel_max_z: roof_top + self.bezel_top_extension,
            opening_x,
            opening_min_z,
            opening_max_z,
            gasket_outer_x: opening_x + 2.0 * (self.gasket_land + self.gasket_width),
            gasket_outer_z: opening_max_z - opening_min_z
                + 2.0 * (self.gasket_land + self.gasket_width),
            gasket_center_z: (opening_max_z + opening_min_z) / 2.0,
            stop_x: -(cavity_x / 2.0 + self.guide_overlap / 2.0),
            rail_screw_x: (outer_x + guide_width) / 4.0,
        }
    }

    fn validate(&self) -> Result<Layout, String> {
        let values = serde_json::to_value(self).map_err(|e| e.to_string())?;
        for (k, v) in values.as_object().ok_or("configuration is not an object")? {
            if k == "mesh_segments" || k == "thermal" || k == "joints" {
                continue;
            }
            let n = v.as_f64().ok_or_else(|| format!("{k}: must be finite"))?;
            if !n.is_finite() || n < 0.0 || (n == 0.0 && k != "heater_recess_depth") {
                return Err(format!("{k}: must be positive and finite"));
            }
        }
        let d = self.layout();
        thermal::validate(&self.thermal)?;
        joints::validate(self, &d)?;
        let squeeze = 1.0
            - (self.gasket_groove_depth - self.gasket_backing_thickness)
                / self.gasket_free_thickness;
        let conditions = [
            (
                self.mesh_segments >= 16 && self.mesh_segments <= 128,
                "mesh_segments outside 16..128",
            ),
            (
                self.plate_x >= 100.0
                    && self.plate_x <= 160.0
                    && self.plate_y >= 60.0
                    && self.plate_y <= 110.0,
                "outside single-microplate design envelope",
            ),
            (
                self.surrogate_plate_z >= 10.0 && self.surrogate_plate_z <= 50.0,
                "surrogate height outside 10..50 mm envelope",
            ),
            (
                self.nest_height < self.surrogate_plate_z && self.nest_height >= 1.5,
                "nest height invalid",
            ),
            (
                self.plate_clearance >= 0.3 && self.plate_clearance <= 2.0,
                "plate clearance outside prototype range",
            ),
            (
                self.drawer_thickness >= 6.0 && self.roof_thickness >= 4.0,
                "spreader too thin for proposed attachment scheme",
            ),
            (
                d.roof_bottom > d.guide_ceiling + 5.0,
                "guide intersects roof",
            ),
            (
                (d.outer_x - d.guide_width) / 2.0 >= 5.5,
                "outer guide cheek too thin for M3 rail fastening",
            ),
            (
                self.guide_overlap >= 5.0 && self.guide_inreach >= 4.0,
                "insufficient guide capture/support",
            ),
            (
                self.guide_clearance >= 0.2
                    && self.guide_clearance <= 0.8
                    && self.lateral_clearance >= 0.2
                    && self.lateral_clearance <= 0.8,
                "guide clearance outside bench design range",
            ),
            (
                self.rear_retention >= 40.0 && self.access_margin >= 4.0,
                "insufficient extension retention or access clearance",
            ),
            (
                self.heater_x <= d.cavity_x - 12.0 && self.heater_y <= d.drawer_y - 20.0,
                "heater intersects guide/nest reservations",
            ),
            (
                self.heater_recess_depth < self.drawer_thickness - 3.0
                    && self.heater_recess_depth < self.roof_thickness - 3.0,
                "heater recess leaves insufficient spreader",
            ),
            (
                squeeze >= 0.1 && squeeze <= 0.35,
                "gasket squeeze outside provisional design window",
            ),
            (
                self.gasket_outer_radius > self.gasket_width && self.gasket_outer_radius < 10.0,
                "gasket corner radius invalid",
            ),
            (
                self.gasket_groove_depth < self.bezel_thickness - 3.0,
                "gasket groove breaches bezel",
            ),
            (
                (d.bezel_x - d.gasket_outer_x) / 2.0 >= 9.0,
                "insufficient space for closure screws outside seal",
            ),
            (
                d.gasket_center_z - d.gasket_outer_z / 2.0 > d.bezel_min_z + 2.0
                    && d.gasket_center_z + d.gasket_outer_z / 2.0 < d.bezel_max_z - 2.0,
                "gasket exceeds bezel lands",
            ),
            (
                self.stop_pin_diameter >= 3.0
                    && self.stop_slot_width > self.stop_pin_diameter
                    && self.stop_slot_width <= self.guide_overlap - 1.0,
                "stop slot/pin width invalid",
            ),
            (
                self.stop_y > self.bezel_thickness + 5.0
                    && self.stop_y + d.stroke + self.stop_slot_width / 2.0 < d.drawer_y - 5.0,
                "stop slot outside supported drawer",
            ),
            (
                self.front_margin >= 4.0
                    && self.opening_clearance >= 0.5
                    && self.opening_clearance <= 1.0,
                "front access clearance invalid",
            ),
        ];
        for (ok, msg) in conditions {
            if !ok {
                return Err(msg.into());
            }
        }
        Ok(d)
    }
}

fn block(name: &str, min: [f64; 3], max: [f64; 3]) -> Part {
    Part::primitive(
        name,
        centered_cube(name, max[0] - min[0], max[1] - min[1], max[2] - min[2]).translate(
            (min[0] + max[0]) / 2.0,
            (min[1] + max[1]) / 2.0,
            (min[2] + max[2]) / 2.0,
        ),
        solid::Primitive::Box(min, max),
    )
}
fn cy(name: &str, x: f64, y: f64, z: f64, diameter: f64, length: f64, n: u32) -> Part {
    Part::primitive(
        name,
        centered_cylinder(name, diameter / 2.0, length, n)
            .rotate(90.0, 0.0, 0.0)
            .translate(x, y, z),
        solid::Primitive::Cylinder([x, y, z], diameter / 2.0, length, 1),
    )
}
fn cx(name: &str, x: f64, y: f64, z: f64, diameter: f64, length: f64, n: u32) -> Part {
    Part::primitive(
        name,
        centered_cylinder(name, diameter / 2.0, length, n)
            .rotate(0.0, 90.0, 0.0)
            .translate(x, y, z),
        solid::Primitive::Cylinder([x, y, z], diameter / 2.0, length, 0),
    )
}
fn cz(name: &str, x: f64, y: f64, z: f64, diameter: f64, length: f64, n: u32) -> Part {
    Part::primitive(
        name,
        centered_cylinder(name, diameter / 2.0, length, n).translate(x, y, z),
        solid::Primitive::Cylinder([x, y, z], diameter / 2.0, length, 2),
    )
}
fn rounded_face(w: f64, h: f64, r: f64, z: f64, ymin: f64, ymax: f64, n: u32) -> Part {
    let mut s = block(
        "rounded_rect",
        [-w / 2.0 + r, ymin, z - h / 2.0],
        [w / 2.0 - r, ymax, z + h / 2.0],
    ) + block(
        "rounded_rect",
        [-w / 2.0, ymin, z - h / 2.0 + r],
        [w / 2.0, ymax, z + h / 2.0 - r],
    );
    for x in [-w / 2.0 + r, w / 2.0 - r] {
        for zz in [z - h / 2.0 + r, z + h / 2.0 - r] {
            s = s + cy(
                "corner",
                x,
                (ymin + ymax) / 2.0,
                zz,
                2.0 * r,
                ymax - ymin,
                n,
            );
        }
    }
    s
}

struct Item {
    name: String,
    part: Part,
    moving: bool,
    reference: bool,
    color: [u8; 3],
}
fn item(name: &str, part: Part, moving: bool, reference: bool, color: [u8; 3]) -> Item {
    Item {
        name: name.into(),
        part,
        moving,
        reference,
        color,
    }
}

fn model(p: &Config, d: &Layout) -> Vec<Item> {
    let n = p.mesh_segments;
    let alum = [161, 179, 195];
    let dark = [72, 89, 104];
    let copper = [214, 128, 52];
    let mut out = Vec::new();
    let mut housing = block(
        "roof_plate",
        [-d.outer_x / 2.0, p.bezel_thickness, d.roof_bottom],
        [d.outer_x / 2.0, d.rear_y, d.roof_top],
    );
    // Surface-bonded pads: no recess weakens the 6 mm spreaders.
    // Four guard holes per zone, outside the heater and plate envelope.
    let guard_y = [p.bezel_thickness + 6.0, d.drawer_y - 6.0];
    for x in [-61.0, 61.0] {
        for y in guard_y {
            housing = housing
                - cz(
                    "guard_M3_tap",
                    x,
                    y,
                    (d.roof_bottom + d.roof_top) / 2.0,
                    2.5,
                    p.roof_thickness + 2.0,
                    n,
                );
        }
    }
    // Two dedicated thermostat mounting holes; sensor carrier bonds beside them.
    housing = housing
        - cz(
            "cutoff_M3_tap",
            25.0,
            d.drawer_y - 22.0,
            (d.roof_bottom + d.roof_top) / 2.0,
            2.5,
            p.roof_thickness + 2.0,
            n,
        );
    housing = housing
        - block(
            "top_heater_recess",
            [
                -p.heater_x / 2.0,
                d.plate_center_y - p.heater_y / 2.0,
                d.roof_top - p.heater_recess_depth,
            ],
            [
                p.heater_x / 2.0,
                d.plate_center_y + p.heater_y / 2.0,
                d.roof_top + 1.0,
            ],
        );
    let mount_points = [
        (
            -d.cavity_x / 2.0 - p.side_wall / 2.0,
            (d.guide_ceiling + d.roof_bottom) / 2.0,
        ),
        (
            d.cavity_x / 2.0 + p.side_wall / 2.0,
            (d.guide_ceiling + d.roof_bottom) / 2.0,
        ),
        (-d.cavity_x / 3.0, (d.roof_bottom + d.roof_top) / 2.0),
        (d.cavity_x / 3.0, (d.roof_bottom + d.roof_top) / 2.0),
    ];
    for (x, z) in mount_points.into_iter().skip(2) {
        housing = housing - cy("rear_M3_tap", x, d.rear_y - 5.0, z, 2.5, 12.0, n);
    }
    let rail_positions = [p.bezel_thickness + 20.0, d.rear_y / 2.0, d.rear_y - 16.0];
    housing = joints::roof_holes(housing, p, d);
    let left_side = joints::side(p, d, -1.0, rail_positions);
    let right_side = joints::side(p, d, 1.0, rail_positions);
    let mut bezel = block(
        "bezel",
        [-d.bezel_x / 2.0, 0.0, d.bezel_min_z],
        [d.bezel_x / 2.0, p.bezel_thickness, d.bezel_max_z],
    );
    let mut opening = block(
        "front_opening",
        [-d.opening_x / 2.0, -1.0, d.opening_min_z],
        [d.opening_x / 2.0, p.bezel_thickness + 1.0, d.opening_max_z],
    );
    // Cutter reliefs retain clearance for a square-ended drawer in the opening.
    for x in [-d.opening_x / 2.0, d.opening_x / 2.0] {
        for z in [d.opening_min_z, d.opening_max_z] {
            opening = opening
                + cy(
                    "opening_corner_relief",
                    x,
                    p.bezel_thickness / 2.0,
                    z,
                    2.0,
                    p.bezel_thickness + 2.0,
                    n,
                );
        }
    }
    let groove = rounded_face(
        d.gasket_outer_x + 0.5,
        d.gasket_outer_z + 0.5,
        p.gasket_outer_radius + 0.25,
        d.gasket_center_z,
        -0.1,
        p.gasket_groove_depth,
        n,
    ) - rounded_face(
        d.gasket_outer_x - 2.0 * p.gasket_width - 0.5,
        d.gasket_outer_z - 2.0 * p.gasket_width - 0.5,
        p.gasket_outer_radius - p.gasket_width - 0.25,
        d.gasket_center_z,
        -1.0,
        p.gasket_groove_depth + 1.0,
        n,
    );
    bezel = bezel - opening - groove;
    let mut flange = block(
        "flange",
        [-d.bezel_x / 2.0, -p.flange_thickness, d.bezel_min_z],
        [d.bezel_x / 2.0, 0.0, d.bezel_max_z],
    );
    // Temporary water-probe cable feedthrough; moves with the plate during access.
    flange = flange
        - cy(
            "validation_M5_port",
            0.0,
            -p.flange_thickness / 2.0,
            25.0,
            4.2,
            p.flange_thickness + 2.0,
            n,
        );
    // One-piece replaceable shim frame; PSA is on ring only, not retention ears.
    let shim_top = d.gasket_center_z + d.gasket_outer_z / 2.0;
    let metal_shim = p.gasket_backing_thickness - 0.14;
    let shim_front = p.gasket_groove_depth - metal_shim;
    let mut shim = rounded_face(
        d.gasket_outer_x,
        d.gasket_outer_z,
        p.gasket_outer_radius,
        d.gasket_center_z,
        shim_front,
        p.gasket_groove_depth,
        n,
    ) - rounded_face(
        d.gasket_outer_x - 2.0 * p.gasket_width,
        d.gasket_outer_z - 2.0 * p.gasket_width,
        p.gasket_outer_radius - p.gasket_width,
        d.gasket_center_z,
        shim_front - 1.0,
        p.gasket_groove_depth + 1.0,
        n,
    );
    for x in [-50.0, 50.0] {
        let z = shim_top + 3.5;
        shim = shim
            + rounded_face(
                6.0,
                7.0,
                1.0,
                shim_top + 3.0,
                shim_front,
                p.gasket_groove_depth,
                n,
            )
            .translate(x, 0.0, 0.0);
        shim = shim - cy("shim_M2_clearance", x, 2.0, z, 2.2, 4.0, n);
        bezel = bezel
            - rounded_face(
                6.4,
                7.4,
                1.0,
                shim_top + 3.0,
                -0.1,
                p.gasket_groove_depth,
                n,
            )
            .translate(x, 0.0, 0.0)
            - cy("shim_M2_tap", x, 3.0, z, 1.6, 8.0, n);
        flange = flange - cy("shim_head_relief", x, -0.25, z, 4.6, 2.5, n);
    }
    out.push(item(
        "13_gasket_shim_nominal_0p60",
        shim,
        false,
        false,
        [200, 200, 180],
    ));
    let closure_x = (d.bezel_x + d.gasket_outer_x) / 4.0;
    for x in [-closure_x, closure_x] {
        out.push(item(
            &format!("REF_closure_knob_{x:.2}"),
            cy(
                "knob",
                x,
                -p.flange_thickness - 5.75,
                d.gasket_center_z,
                12.0,
                11.5,
                n,
            ),
            true,
            true,
            dark,
        ));
        bezel = bezel
            - cy(
                "closure_M4_tap",
                x,
                p.bezel_thickness / 2.0,
                d.gasket_center_z,
                3.3,
                p.bezel_thickness + 2.0,
                n,
            );
        flange = flange
            - cy(
                "closure_M4_clearance",
                x,
                -p.flange_thickness / 2.0,
                d.gasket_center_z,
                6.0,
                p.flange_thickness + 2.0,
                n,
            );
    }
    let mut rear = block(
        "rear",
        [-d.outer_x / 2.0, d.rear_y, 0.0],
        [d.outer_x / 2.0, d.rear_y + p.rear_thickness, d.roof_top],
    );
    for x in [28.0, 42.0] {
        rear = rear - cz("fixed_harness_M3_tap", x, d.rear_y + 3.0, 4.0, 2.5, 10.0, n);
    }
    for (x, z) in mount_points {
        rear = rear
            - cy(
                "rear_M3_clearance",
                x,
                d.rear_y + p.rear_thickness / 2.0,
                z,
                3.4,
                p.rear_thickness + 2.0,
                n,
            );
    }
    out.push(item("01_roof_plate", housing, false, false, alum));
    out.push(item("01_left_side_plate", left_side, false, false, alum));
    out.push(item("01_right_side_plate", right_side, false, false, alum));
    out.push(item(
        "01_front_bezel",
        joints::bezel_holes(bezel, p, d),
        false,
        false,
        alum,
    ));
    out.push(item("03_rear_cover", rear, false, false, alum));
    let mut drawer = block(
        "drawer",
        [-d.drawer_x / 2.0, 0.0, d.drawer_bottom],
        [d.drawer_x / 2.0, d.drawer_y, d.drawer_top],
    ) - block(
        "drawer_heater_recess",
        [
            -p.heater_x / 2.0,
            d.plate_center_y - p.heater_y / 2.0,
            d.drawer_bottom - 1.0,
        ],
        [
            p.heater_x / 2.0,
            d.plate_center_y + p.heater_y / 2.0,
            d.drawer_bottom + p.heater_recess_depth,
        ],
    );
    for x in [-61.0, 61.0] {
        for y in guard_y {
            drawer = drawer
                - cz(
                    "guard_M3_tap",
                    x,
                    y,
                    (d.drawer_bottom + d.drawer_top) / 2.0,
                    2.5,
                    p.drawer_thickness + 2.0,
                    n,
                );
        }
    }
    drawer = drawer
        - cz(
            "cutoff_M3_tap",
            25.0,
            d.drawer_y - 22.0,
            (d.drawer_bottom + d.drawer_top) / 2.0,
            2.5,
            p.drawer_thickness + 2.0,
            n,
        );
    // Moving cable anchor: two M3 holes outside the test-plate footprint.
    for x in [-42.0, -28.0] {
        drawer = drawer
            - cz(
                "harness_anchor_M3",
                x,
                d.drawer_y - 12.0,
                (d.drawer_bottom + d.drawer_top) / 2.0,
                2.5,
                p.drawer_thickness + 2.0,
                n,
            );
    }
    let slot = block(
        "stop_slot",
        [
            d.stop_x - p.stop_slot_width / 2.0,
            p.stop_y,
            d.drawer_bottom - 1.0,
        ],
        [
            d.stop_x + p.stop_slot_width / 2.0,
            p.stop_y + d.stroke,
            d.drawer_top + 1.0,
        ],
    ) + cz(
        "stop_slot_end",
        d.stop_x,
        p.stop_y,
        (d.drawer_bottom + d.drawer_top) / 2.0,
        p.stop_slot_width,
        p.drawer_thickness + 2.0,
        n,
    ) + cz(
        "stop_slot_end",
        d.stop_x,
        p.stop_y + d.stroke,
        (d.drawer_bottom + d.drawer_top) / 2.0,
        p.stop_slot_width,
        p.drawer_thickness + 2.0,
        n,
    );
    drawer = drawer - slot;
    for x in [-p.plate_x * 0.36, 0.0, p.plate_x * 0.36] {
        let z = (d.drawer_bottom + d.drawer_top) / 2.0;
        out.push(item(
            &format!("REF_flange_M3_head_{x:.2}"),
            cy("head", x, -p.flange_thickness - 1.5, z, 5.5, 3.0, n),
            true,
            true,
            dark,
        ));
        drawer = drawer - cy("flange_M3_tap", x, 5.0, z, 2.5, 12.0, n);
        flange = flange
            - cy(
                "flange_M3_clearance",
                x,
                -p.flange_thickness / 2.0,
                z,
                3.4,
                p.flange_thickness + 2.0,
                n,
            );
    }
    let mut nest = block(
        "nest",
        [
            -d.nest_x / 2.0,
            d.plate_center_y - d.nest_y / 2.0,
            d.drawer_top,
        ],
        [
            d.nest_x / 2.0,
            d.plate_center_y + d.nest_y / 2.0,
            d.drawer_top + p.nest_height,
        ],
    ) - block(
        "nest_opening",
        [
            -p.plate_x / 2.0 - p.plate_clearance,
            d.plate_center_y - p.plate_y / 2.0 - p.plate_clearance,
            d.drawer_top - 1.0,
        ],
        [
            p.plate_x / 2.0 + p.plate_clearance,
            d.plate_center_y + p.plate_y / 2.0 + p.plate_clearance,
            d.drawer_top + p.nest_height + 1.0,
        ],
    );
    // End-mill relief at the four inner corners; no hand-filing a plate to fit.
    for x in [
        -p.plate_x / 2.0 - p.plate_clearance,
        p.plate_x / 2.0 + p.plate_clearance,
    ] {
        for y in [
            d.plate_center_y - p.plate_y / 2.0 - p.plate_clearance,
            d.plate_center_y + p.plate_y / 2.0 + p.plate_clearance,
        ] {
            nest = nest
                - cz(
                    "nest_corner_relief",
                    x,
                    y,
                    d.drawer_top + p.nest_height / 2.0,
                    2.0,
                    p.nest_height + 2.0,
                    n,
                );
        }
    }
    // Four M2 screws in reinforced front/rear nest webs.
    for y in [
        d.plate_center_y - d.nest_y / 2.0 + p.nest_wall / 2.0,
        d.plate_center_y + d.nest_y / 2.0 - p.nest_wall / 2.0,
    ] {
        for x in [-45.0, 45.0] {
            out.push(item(
                &format!("REF_nest_M2_head_{x:.0}_{y:.2}"),
                cz(
                    "head",
                    x,
                    y,
                    d.drawer_top + p.nest_height + 1.0,
                    3.8,
                    2.0,
                    n,
                ),
                true,
                true,
                dark,
            ));
            nest = nest
                - cz(
                    "nest_M2_clearance",
                    x,
                    y,
                    d.drawer_top + p.nest_height / 2.0,
                    2.2,
                    p.nest_height + 2.0,
                    n,
                );
            drawer = drawer - cz("nest_M2_tap", x, y, d.drawer_top - 2.0, 1.6, 5.0, n);
        }
    }
    out.push(item("04_heated_drawer", drawer, true, false, alum));
    out.push(item("05_drawer_front_flange", flange, true, false, dark));
    out.push(item(
        "06_replaceable_nest",
        nest,
        true,
        false,
        [74, 157, 143],
    ));
    for (label, sign) in [("left", -1.0), ("right", 1.0)] {
        let mut rail = block(
            "rail",
            [d.cavity_x / 2.0 - p.guide_inreach, p.bezel_thickness, 0.0],
            [d.outer_x / 2.0, d.rear_y, p.guide_rail_thickness],
        );
        if sign < 0.0 {
            rail = rail.mirror_x();
        }
        for y in rail_positions {
            rail = rail
                - cz(
                    "rail_clearance",
                    sign * d.rail_screw_x,
                    y,
                    p.guide_rail_thickness / 2.0,
                    3.4,
                    p.guide_rail_thickness + 2.0,
                    n,
                );
        }
        if sign < 0.0 {
            rail = rail
                - cz(
                    "stop_install_access",
                    d.stop_x,
                    p.stop_y,
                    p.guide_rail_thickness / 2.0,
                    6.0,
                    p.guide_rail_thickness + 2.0,
                    n,
                );
        }
        out.push(item(
            &format!("07_guide_rail_{label}"),
            rail,
            false,
            false,
            dark,
        ));
        // Bench risers provide a protected 30 mm underbody harness space.
        let x = sign * d.rail_screw_x;
        let mut foot = block(
            "riser",
            [x - 5.0, p.bezel_thickness, -30.0],
            [x + 5.0, d.rear_y, 0.0],
        );
        for y in rail_positions {
            foot = foot
                - cz("riser_M3_clearance", x, y, -15.0, 3.4, 32.0, n)
                - cz("riser_head_counterbore", x, y, -28.9, 6.0, 4.2, n);
        }
        out.push(item(
            &format!("10_bench_riser_{label}"),
            foot,
            false,
            false,
            dark,
        ));
        let shoes = [
            (
                "lower",
                [
                    d.cavity_x / 2.0 - p.guide_inreach + 2.0,
                    p.bezel_thickness,
                    p.guide_rail_thickness,
                ],
                [d.drawer_x / 2.0, d.rear_y, d.drawer_bottom],
            ),
            (
                "upper",
                [d.cavity_x / 2.0, p.bezel_thickness, d.upper_shoe_bottom],
                [d.drawer_x / 2.0, d.rear_y, d.guide_ceiling],
            ),
            (
                "side",
                [
                    d.drawer_x / 2.0 + p.lateral_clearance,
                    p.bezel_thickness,
                    p.guide_rail_thickness,
                ],
                [d.guide_width / 2.0, d.rear_y, d.guide_ceiling],
            ),
        ];
        for (location, mut min, max) in shoes {
            if sign < 0.0 && location == "upper" {
                min[1] = p.stop_y + 6.0;
            }
            let mut shoe = block("shoe", min, max);
            if sign < 0.0 {
                shoe = shoe.mirror_x();
            }
            if sign < 0.0 && location == "lower" {
                shoe = shoe
                    - cz(
                        "stop_access",
                        d.stop_x,
                        p.stop_y,
                        (min[2] + max[2]) / 2.0,
                        6.0,
                        max[2] - min[2] + 2.0,
                        n,
                    );
            }
            out.push(item(
                &format!("08_shoe_{label}_{location}"),
                shoe,
                false,
                false,
                [48, 58, 65],
            ));
        }
    }
    let gasket = rounded_face(
        d.gasket_outer_x,
        d.gasket_outer_z,
        p.gasket_outer_radius,
        d.gasket_center_z,
        0.0,
        p.gasket_groove_depth - p.gasket_backing_thickness,
        n,
    ) - rounded_face(
        d.gasket_outer_x - 2.0 * p.gasket_width,
        d.gasket_outer_z - 2.0 * p.gasket_width,
        p.gasket_outer_radius - p.gasket_width,
        d.gasket_center_z,
        -1.0,
        p.gasket_groove_depth + 1.0,
        n,
    );
    out.push(item(
        "09_gasket_compressed_reference",
        gasket,
        false,
        true,
        [225, 177, 67],
    ));
    // Reference plate: external bounding box with a visible well-pattern only.
    // Holes are NOT a manufacturer's fluid-path model.
    let mut plate = block(
        "surrogate_plate",
        [
            -p.plate_x / 2.0,
            d.plate_center_y - p.plate_y / 2.0,
            d.drawer_top,
        ],
        [
            p.plate_x / 2.0,
            d.plate_center_y + p.plate_y / 2.0,
            d.drawer_top + p.surrogate_plate_z,
        ],
    );
    for row in 0..8 {
        for col in 0..12 {
            plate = plate
                - cz(
                    "illustrative_well",
                    (col as f64 - 5.5) * 9.0,
                    d.plate_center_y + (row as f64 - 3.5) * 9.0,
                    d.drawer_top + p.surrogate_plate_z - 3.0,
                    6.0,
                    7.0,
                    16,
                );
        }
    }
    out.push(item(
        "REF_surrogate_plate_NOT_VENDOR_CAD",
        plate,
        true,
        true,
        [109, 172, 202],
    ));
    for (name, bottom, moving) in [
        ("REF_top_heater", d.roof_top, false),
        ("REF_drawer_heater", d.drawer_bottom - 1.5, true),
    ] {
        out.push(item(
            name,
            block(
                name,
                [
                    -p.heater_x / 2.0,
                    d.plate_center_y - p.heater_y / 2.0,
                    bottom,
                ],
                [
                    p.heater_x / 2.0,
                    d.plate_center_y + p.heater_y / 2.0,
                    bottom + 1.5,
                ],
            ),
            moving,
            true,
            copper,
        ));
    }
    out.push(item(
        "REF_stop_shoulder_pin",
        cz(
            "pin",
            d.stop_x,
            p.stop_y,
            (d.drawer_bottom + d.guide_ceiling) / 2.0,
            p.stop_pin_diameter,
            d.guide_ceiling - d.drawer_bottom,
            n,
        ),
        false,
        true,
        [220, 193, 98],
    ));
    for (zone, surface, moving, sign) in [
        ("roof", d.roof_top, false, 1.0),
        ("drawer", d.drawer_bottom, true, -1.0),
    ] {
        let z0 = surface + sign * 8.0;
        let z1 = surface + sign * 9.0;
        let mut guard = block(
            "heater_guard",
            [-66.0, 0.0, z0.min(z1)],
            [66.0, d.drawer_y, z0.max(z1)],
        );
        for x in [-61.0, 61.0] {
            for y in guard_y {
                guard = guard - cz("guard_clearance", x, y, (z0 + z1) / 2.0, 3.4, 3.0, n);
                let spacer = cz("spacer", x, y, surface + sign * 4.0, 6.0, 8.0, n)
                    - cz("spacer_bore", x, y, surface + sign * 4.0, 3.4, 10.0, n);
                out.push(item(
                    &format!("11_{zone}_spacer_{x:.0}_{y:.2}"),
                    spacer,
                    moving,
                    false,
                    alum,
                ));
                out.push(item(
                    &format!("REF_{zone}_guard_screw_head_{x:.0}_{y:.2}"),
                    cz("M3_head", x, y, surface + sign * 10.5, 5.5, 3.0, n),
                    moving,
                    true,
                    dark,
                ));
            }
        }
        // Rear-edge lead exit; 20 x 6 mm opening permits routing without pinching.
        guard = guard
            - block(
                "lead_exit",
                [-48.0, d.drawer_y - 22.0, z0.min(z1) - 1.0],
                [-22.0, d.drawer_y + 1.0, z0.max(z1) + 1.0],
            );
        out.push(item(
            &format!("12_{zone}_heater_guard"),
            guard,
            moving,
            false,
            alum,
        ));
        out.push(item(
            &format!("REF_{zone}_RTD_land"),
            block(
                "rtd",
                [
                    -20.0,
                    d.drawer_y - 42.0,
                    (surface + sign * 2.0).min(surface),
                ],
                [10.0, d.drawer_y - 17.0, (surface + sign * 2.0).max(surface)],
            ),
            moving,
            true,
            [149, 102, 174],
        ));
    }
    let a = d.drawer_y - 12.0;
    for (zone, c, moving) in [("moving", -35.0, true), ("fixed", 35.0, false)] {
        let mut base = block(
            "clamp_base",
            [c - 8.0, a - 10.0, -24.0],
            [c, a + 10.0, -3.0],
        );
        let mut cap = block("clamp_cap", [c, a - 10.0, -24.0], [c + 8.0, a + 10.0, -3.0]);
        if moving {
            base =
                base + block(
                    "bracket_web",
                    [-47.0, a - 5.0, -24.0],
                    [-39.0, a + 5.0, d.drawer_bottom - 4.0],
                ) + block(
                    "bracket_mount",
                    [-47.0, a - 5.0, d.drawer_bottom - 4.0],
                    [-23.0, a + 5.0, d.drawer_bottom],
                );
            for x in [-42.0, -28.0] {
                base = base - cz("mount_clearance", x, a, d.drawer_bottom - 2.0, 3.4, 6.0, n);
            }
        } else {
            base =
                base + block(
                    "rear_mount",
                    [23.0, d.rear_y, -4.0],
                    [47.0, d.rear_y + 6.0, 0.0],
                ) + block(
                    "rear_web",
                    [23.0, d.rear_y, -8.0],
                    [31.0, d.rear_y + 6.0, 0.0],
                ) + block(
                    "cantilever",
                    [23.0, a - 5.0, -8.0],
                    [29.0, d.rear_y + 6.0, -4.0],
                );
            for x in [28.0, 42.0] {
                base = base - cz("mount_clearance", x, d.rear_y + 3.0, -2.0, 3.4, 6.0, n);
            }
        }
        for (z, diam) in [(-18.0, 5.8), (-9.0, 5.3)] {
            base = base - cy("jacket_groove", c, a, z, diam, 22.0, n);
            cap = cap - cy("jacket_groove", c, a, z, diam, 22.0, n);
        }
        for y in [a - 6.0, a + 6.0] {
            base = base - cx("jaw_M2_tap", c - 3.0, y, -13.5, 1.6, 8.0, n);
            cap = cap - cx("jaw_M2_clearance", c + 4.0, y, -13.5, 2.2, 10.0, n);
        }
        out.push(item(
            &format!("14_{zone}_harness_bracket"),
            base,
            moving,
            false,
            alum,
        ));
        out.push(item(
            &format!("15_{zone}_harness_jaw"),
            cap,
            moving,
            false,
            alum,
        ));
    }
    out.extend(joints::hardware(p, d));
    out
}

fn sha(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn git(args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let o = Command::new("git").args(args).output()?;
    if !o.status.success() {
        return Err("git identity query failed".into());
    }
    Ok(String::from_utf8(o.stdout)?.trim().into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let raw = fs::read_to_string(&args.config)?;
    let p: Config = toml::from_str(&raw)?;
    let d = p
        .validate()
        .map_err(|e| format!("invalid cassette geometry: {e}"))?;
    if args.validate_only {
        println!("{}", serde_json::to_string_pretty(&d)?);
        return Ok(());
    }
    if !cfg!(feature = "step") {
        return Err("fabrication export requires the step Cargo feature".into());
    }
    if args.output_dir.exists() && fs::read_dir(&args.output_dir)?.next().is_some() {
        return Err("output directory must be empty; preserve each candidate's evidence".into());
    }
    let parts = model(&p, &d);
    joints::verify_hardware(&parts, &d)?;
    let mut checks = Vec::new();
    for (index, a) in parts.iter().enumerate().filter(|(_, i)| !i.reference) {
        for b in parts
            .iter()
            .skip(index + 1)
            .filter(|i| !i.reference && i.moving == a.moving)
        {
            let volume = a.part.intersection(&b.part).volume();
            if volume > 0.03 {
                return Err(format!(
                    "assembled rigid-part overlap: {} / {}: {volume:.6}",
                    a.name, b.name
                )
                .into());
            }
        }
    }
    // Continuous, axis-aligned travel: sweep each moving part's solid by samples.
    // This is a sampled collision check, not certified continuous swept volume.
    for fraction in [0.0, 0.25, 0.5, 0.75, 1.0] {
        for a in parts.iter().filter(|i| i.moving) {
            for b in parts.iter().filter(|i| !i.moving && !i.reference) {
                let volume = a
                    .part
                    .translate(0.0, -d.stroke * fraction, 0.0)
                    .intersection(&b.part)
                    .volume();
                if volume > 0.03 {
                    return Err(format!(
                        "collision: {} / {} at {fraction}: {volume:.5} mm3",
                        a.name, b.name
                    )
                    .into());
                }
            }
        }
        checks.push(json!({"stroke_fraction":fraction,"moving_vs_fixed_collision_check":"pass","volume_tolerance_mm3":0.03}));
    }
    fs::create_dir_all(&args.output_dir)?;
    fs::write(args.output_dir.join("config.toml"), &raw)?;
    let mut reports = Vec::new();
    for i in &parts {
        let mesh = i.part.to_mesh();
        if i.part.is_empty() || i.part.volume() <= 0.01 || mesh.indices().is_empty() {
            return Err(format!("empty geometry: {}", i.name).into());
        }
        // Every undirected mesh edge must have exactly two incident triangles.
        let mut edges = BTreeMap::new();
        for t in mesh.indices().chunks_exact(3) {
            for (a, b) in [(t[0], t[1]), (t[1], t[2]), (t[2], t[0])] {
                *edges.entry((a.min(b), a.max(b))).or_insert(0usize) += 1;
            }
        }
        if edges.values().any(|n| *n != 2) {
            return Err(format!("non-closed mesh: {}", i.name).into());
        }
        let path = args.output_dir.join(format!("{}.stl", i.name));
        i.part.write_stl(&path)?;
        #[cfg(feature = "step")]
        if !i.reference {
            let path = args.output_dir.join(format!("{}.step", i.name));
            let mut shape = i.part.analytic();
            // Unify coplanar Boolean faces for portable STEP import. Geometry is verified below.
            shape.clean();
            shape.write_step(&path)?;
            let recovered = opencascade::primitives::Shape::read_step(&path)?;
            let mesh = recovered.mesh();
            let mut volume = 0.0f64;
            let mut min = [f64::INFINITY; 3];
            let mut max = [f64::NEG_INFINITY; 3];
            for v in &mesh.vertices {
                for k in 0..3 {
                    min[k] = min[k].min(v[k]);
                    max[k] = max[k].max(v[k]);
                }
            }
            for t in mesh.indices.chunks_exact(3) {
                volume +=
                    mesh.vertices[t[0]].dot(mesh.vertices[t[1]].cross(mesh.vertices[t[2]])) / 6.0;
            }
            let text = fs::read_to_string(&path)?;
            let solids = text.matches("MANIFOLD_SOLID_BREP(").count();
            let relative_volume_error = (volume.abs() - i.part.volume()).abs() / i.part.volume();
            let (mesh_min, mesh_max) = i.part.bounding_box();
            let bounds_error = (0..3)
                .flat_map(|k| [(min[k] - mesh_min[k]).abs(), (max[k] - mesh_max[k]).abs()])
                .fold(0.0, f64::max);
            if solids != 1
                || relative_volume_error > 0.015
                || bounds_error > 0.03
                || recovered.faces().count() == 0
            {
                return Err(format!("STEP round-trip failed: {} solids={solids} volume_error={relative_volume_error}",i.name).into());
            }
            reports.push(json!({"analytic_step":i.name,"sha256":sha(text.as_bytes()),"manifold_solid_brep_count":solids,"reimport_faces":recovered.faces().count(),"reimport_volume_mm3":volume.abs(),"relative_volume_error_vs_review_mesh":relative_volume_error,"bounds_error_mm":bounds_error,"bounds_min_mm":min,"bounds_max_mm":max}));
        }
        reports.push(json!({"part":i.name,"moving":i.moving,"reference_only":i.reference,"volume_mm3":i.part.volume(),"bounds_mm":i.part.bounding_box(),"triangles":i.part.num_triangles(),"closed_mesh_edges":true,"sha256":sha(&fs::read(path)?)}));
    }
    for (name, travel) in [("assembly_closed", 0.0), ("assembly_open", d.stroke)] {
        let mut assembly = Part::empty(name);
        for i in &parts {
            assembly = assembly
                + i.part
                    .translate(0.0, if i.moving { -travel } else { 0.0 }, 0.0);
        }
        assembly.write_stl(args.output_dir.join(format!("{name}.stl")))?;
    }
    render(&args.output_dir.join("design-review.svg"), &parts, &p, &d)?;
    drawings::sheets(&args.output_dir.join("drawing-pages"), &parts, &p, &d)?;
    assembly_drawings::sheets(&args.output_dir.join("assembly-pages"), &parts, &p, &d)?;
    thermal::run(&args.output_dir, &p, &d, &parts)?;
    package::templates(&args.output_dir, &p, &d)?;
    package::handbook(&args.output_dir)?;
    package::manufacturing_index(&args.output_dir, &parts)?;
    let source = fs::read("src/bin/heated_microplate_cassette_v0.rs")?;
    let mut dependencies = BTreeMap::new();
    for path in [
        "src/bin/heated_microplate_cassette_v0.rs",
        "src/cassette/solid.rs",
        "src/cassette/joints.rs",
        "src/cassette/drawings.rs",
        "src/cassette/assembly_drawings.rs",
        "src/cassette/feature_drawings.rs",
        "src/cassette/thermal.rs",
        "src/cassette/package.rs",
        "Cargo.toml",
        "Cargo.lock",
        "docs/heated-cassette-v0/README.md",
        "docs/heated-cassette-v0/revd-design-review.md",
        "docs/heated-cassette-v0/manufacturing-notes.md",
        "docs/heated-cassette-v0/assembly.md",
        "docs/heated-cassette-v0/electrical-assembly.md",
        "docs/heated-cassette-v0/control-box-layout.md",
        "docs/heated-cassette-v0/validation.md",
        "docs/heated-cassette-v0/RFQ.md",
        "docs/heated-cassette-v0/wiring.svg",
    ] {
        dependencies.insert(path, sha(&fs::read(path)?));
    }
    let report = json!({"status":"V0_WATER_TEST_PROTOTYPE_FABRICATION_PACKAGE","configuration_sha256":sha(raw.as_bytes()),"source_sha256":sha(&source),"source_dependencies_sha256":dependencies,"git_head":git(&["rev-parse","HEAD"])? ,"git_status":git(&["status","--porcelain"])? ,"binary_sha256":sha(&fs::read(std::env::current_exe()?)?),"config":p,"layout":d,"parts":reports,"rigid_parts_assembled_interference_check":"pass","plate_joint_hardware_clearance_check":"pass; threaded parent engagement excluded","travel_checks":checks,"limitations":["Greiner water-test surrogate; future tissue plate compatibility unqualified","Reduced aluminum sheet model only; no media-temperature or physical performance validation","Sampled rigid travel; some hardware simplified; flexible cable geometry requires assembly inspection","Threads represented by pilot bores: drawings specify thread and depth","Guide clearances remain draft paths; no hermetic or pressure rating","Electrical startup/fault response, cable retention and thermal maps require commissioning"]});
    fs::write(
        args.output_dir.join("verification.json"),
        serde_json::to_vec_pretty(&report)?,
    )?;
    println!(
        "{}",
        json!({"status":"water_test_fabrication_package_generated","output_dir":args.output_dir,"parts":parts.len(),"stroke_mm":d.stroke,"housing_envelope_mm":[d.bezel_x,d.overall_y,d.bezel_max_z-d.bezel_min_z],"travel_checks":5})
    );
    Ok(())
}

fn render(
    path: &Path,
    parts: &[Item],
    p: &Config,
    d: &Layout,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut svg=String::from("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"1600\" height=\"1100\" viewBox=\"0 0 1600 1100\"><rect width=\"1600\" height=\"1100\" fill=\"#eef2f5\"/><style>text{font-family:Helvetica,Arial,sans-serif;fill:#162b3b}.title{font-size:34px;font-weight:bold}.sub{font-size:18px;fill:#425d70}.label{font-size:22px;font-weight:bold}.note{font-size:17px}</style>");
    svg.push_str("<text x=\"45\" y=\"55\" class=\"title\">LaminarForge / heated microplate cassette V0</text><text x=\"45\" y=\"87\" class=\"sub\">Rev D water-test prototype / manual drawer / 6061 aluminum / two heater zones</text>");
    let panels = [
        ("01  CLOSED", 0.0, false, 40.0, 115.0),
        ("02  FULL ACCESS", d.stroke, false, 820.0, 115.0),
        (
            "03  INTERNAL LAYOUT / HOUSING HIDDEN",
            0.0,
            true,
            40.0,
            565.0,
        ),
        ("04  UNDERSIDE / GUARDS HIDDEN", 0.0, false, 820.0, 565.0),
    ];
    for (title, travel, cutaway, px, py) in panels {
        let underside = title.starts_with("04");
        svg.push_str(&format!("<rect x=\"{px}\" y=\"{py}\" width=\"740\" height=\"420\" rx=\"12\" fill=\"white\"/><text x=\"{}\" y=\"{}\" class=\"label\">{title}</text>",px+22.0,py+33.0));
        let mut tris: Vec<([[f64; 3]; 3], [u8; 3])> = Vec::new();
        let mut bounds = [
            f64::INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY,
        ];
        for i in parts {
            if cutaway
                && (i.name.starts_with("01_")
                    || i.name == "03_rear_cover"
                    || i.name == "REF_top_heater"
                    || i.name.starts_with("REF_joint_")
                    || i.name.starts_with("11_roof")
                    || i.name.starts_with("12_roof")
                    || i.name.starts_with("REF_roof"))
            {
                continue;
            }
            if underside
                && (i.name.starts_with("12_") || i.name.starts_with("REF_drawer_guard_screw"))
            {
                continue;
            }
            let mesh = i.part.to_mesh();
            let verts = mesh.vertices();
            for idx in mesh.indices().chunks_exact(3) {
                let mut points = [[0.0; 3]; 3];
                let mut xyz = [[0.0; 3]; 3];
                for j in 0..3 {
                    let k = idx[j] as usize * 3;
                    let x = verts[k] as f64;
                    let y = verts[k + 1] as f64 - if i.moving { travel } else { 0.0 };
                    let z = verts[k + 2] as f64;
                    xyz[j] = [x, y, z];
                    let z = if underside { -z } else { z };
                    points[j] = [
                        0.85 * x + 0.5 * y,
                        0.25 * x - 0.425 * y - 0.85 * z,
                        0.425 * x - 0.7225 * y + 0.48625 * z,
                    ];
                    bounds[0] = bounds[0].min(points[j][0]);
                    bounds[1] = bounds[1].min(points[j][1]);
                    bounds[2] = bounds[2].max(points[j][0]);
                    bounds[3] = bounds[3].max(points[j][1]);
                }
                let a = [
                    xyz[1][0] - xyz[0][0],
                    xyz[1][1] - xyz[0][1],
                    xyz[1][2] - xyz[0][2],
                ];
                let b = [
                    xyz[2][0] - xyz[0][0],
                    xyz[2][1] - xyz[0][1],
                    xyz[2][2] - xyz[0][2],
                ];
                let norm = [
                    a[1] * b[2] - a[2] * b[1],
                    a[2] * b[0] - a[0] * b[2],
                    a[0] * b[1] - a[1] * b[0],
                ];
                let length = (norm[0] * norm[0] + norm[1] * norm[1] + norm[2] * norm[2])
                    .sqrt()
                    .max(1e-9);
                let shade = (0.72 + 0.28 * (norm[2] / length).abs()).clamp(0.5, 1.0);
                tris.push((points, i.color.map(|c| (c as f64 * shade) as u8)));
            }
        }
        let scale = (680.0 / (bounds[2] - bounds[0])).min(300.0 / (bounds[3] - bounds[1]));
        let ox = 370.0 - scale * (bounds[0] + bounds[2]) / 2.0;
        let oy = 170.0 - scale * (bounds[1] + bounds[3]) / 2.0;
        let (w, h) = (1480u32, 660u32);
        let mut raster = image::RgbImage::from_pixel(w, h, image::Rgb([255, 255, 255]));
        let mut zbuffer = vec![f64::NEG_INFINITY; (w * h) as usize];
        for (mut t, c) in tris {
            for v in &mut t {
                v[0] = 2.0 * (ox + v[0] * scale);
                v[1] = 2.0 * (oy + v[1] * scale);
            }
            let area = (t[1][0] - t[0][0]) * (t[2][1] - t[0][1])
                - (t[1][1] - t[0][1]) * (t[2][0] - t[0][0]);
            if area.abs() < 1e-9 {
                continue;
            }
            let xmin = t
                .iter()
                .map(|v| v[0])
                .fold(f64::INFINITY, f64::min)
                .floor()
                .max(0.0) as u32;
            let xmax = t
                .iter()
                .map(|v| v[0])
                .fold(f64::NEG_INFINITY, f64::max)
                .ceil()
                .min(w as f64 - 1.0)
                .max(0.0) as u32;
            let ymin = t
                .iter()
                .map(|v| v[1])
                .fold(f64::INFINITY, f64::min)
                .floor()
                .max(0.0) as u32;
            let ymax = t
                .iter()
                .map(|v| v[1])
                .fold(f64::NEG_INFINITY, f64::max)
                .ceil()
                .min(h as f64 - 1.0)
                .max(0.0) as u32;
            for y in ymin..=ymax {
                for x in xmin..=xmax {
                    let (xx, yy) = (x as f64 + 0.5, y as f64 + 0.5);
                    let b = ((xx - t[0][0]) * (t[2][1] - t[0][1])
                        - (yy - t[0][1]) * (t[2][0] - t[0][0]))
                        / area;
                    let cc = ((t[1][0] - t[0][0]) * (yy - t[0][1])
                        - (t[1][1] - t[0][1]) * (xx - t[0][0]))
                        / area;
                    let a = 1.0 - b - cc;
                    if a >= -1e-7 && b >= -1e-7 && cc >= -1e-7 {
                        let z = a * t[0][2] + b * t[1][2] + cc * t[2][2];
                        let k = (y * w + x) as usize;
                        if z > zbuffer[k] {
                            zbuffer[k] = z;
                            raster.put_pixel(x, y, image::Rgb(c));
                        }
                    }
                }
            }
        }
        let mut png = std::io::Cursor::new(Vec::new());
        image::DynamicImage::ImageRgb8(raster).write_to(&mut png, image::ImageFormat::Png)?;
        let encoded = base64::engine::general_purpose::STANDARD.encode(png.into_inner());
        svg.push_str(&format!("<image x=\"{px}\" y=\"{}\" width=\"740\" height=\"330\" href=\"data:image/png;base64,{encoded}\"/>",py+50.0));
        let caption = if travel > 0.0 {
            format!(
                "{:.1} mm stroke / {:.1} mm rear guide engagement",
                d.stroke, p.rear_retention
            )
        } else {
            format!(
                "Body {:.1} W × {:.1} D × {:.1} H mm",
                d.bezel_x,
                d.overall_y,
                d.bezel_max_z - d.bezel_min_z
            )
        };
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" class=\"note\">{caption}</text>",
            px + 22.0,
            py + 399.0
        ));
    }
    svg.push_str("<text x=\"45\" y=\"1025\" class=\"note\">Blue: plate envelope / Green: removable nest / Orange: heaters / Yellow: face gasket</text><text x=\"45\" y=\"1060\" class=\"sub\">Assembly illustration / Some hardware simplified / See separate cable routing / Water validation required</text></svg>");
    fs::write(path, svg)?;
    Ok(())
}
