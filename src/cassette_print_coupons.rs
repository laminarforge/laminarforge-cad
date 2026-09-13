//! Dry fit variants of the fixed A0 assembly interface. These adjustments do not revise the A0 assembly itself.
use crate::{sixteen_slot_cassette_a0::*, REVC_CHIP_LENGTH, REVC_CHIP_WIDTH};
use serde::{Deserialize, Serialize};
use vcad::{centered_cube, centered_cylinder, Part};
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Parameters {
    pub chip_clearance_mm: f64,
    pub gasket_groove_width_mm: f64,
    pub feature_anchor_overlap_mm: f64,
    pub stop_clearance_mm: f64,
}
pub const OUTPUTS: [&str; 5] = [
    "sixteen_slot_chip_pocket_fit_coupon.stl",
    "sixteen_slot_gasket_compression_coupon.stl",
    "sixteen_slot_dock_datum_rail_coupon.stl",
    "sixteen_slot_carrier_corner_surrogate.stl",
    "sixteen_slot_bulkhead_connector_mockup.stl",
];
impl Parameters {
    pub fn validate(&self) -> Result<(), String> {
        for (name, value, low, high) in [
            ("chip_clearance_mm", self.chip_clearance_mm, 0.8, 1.6),
            (
                "gasket_groove_width_mm",
                self.gasket_groove_width_mm,
                3.0,
                3.6,
            ),
            (
                "feature_anchor_overlap_mm",
                self.feature_anchor_overlap_mm,
                0.2,
                0.6,
            ),
            ("stop_clearance_mm", self.stop_clearance_mm, 0.5, 1.5),
        ] {
            if !value.is_finite() || !(low..=high).contains(&value) {
                return Err(format!(
                    "{name} must be finite and within {low}..={high} mm"
                ));
            }
        }
        Ok(())
    }
    pub fn build(&self, index: usize) -> Result<Part, String> {
        self.validate()?;
        let part = match index {
            0 => self.chip_pocket_fit_coupon(),
            1 => self.gasket_compression_coupon(),
            2 => self.dock_datum_rail_coupon(),
            3 => self.carrier_corner_surrogate(),
            4 => self.bulkhead_connector_mockup(),
            _ => return Err("unknown coupon index".into()),
        };
        if index == 0 {
            let stats = crate::runtime_cad::mesh(&part.to_stl().map_err(|e| e.to_string())?)?;
            for (axis, expected) in [
                CHIP_FIT_COUPON_X,
                CHIP_FIT_COUPON_Y,
                CHIP_FIT_COUPON_OVERALL_Z,
            ]
            .iter()
            .enumerate()
            {
                if (stats["size_mm"][axis].as_f64().ok_or("invalid mesh size")? - expected).abs()
                    > 0.05
                {
                    return Err("chip coupon violates the shared A0 envelope".into());
                }
            }
        }
        Ok(part)
    }
    fn chip_pocket_fit_coupon(&self) -> Part {
        let body = centered_cube(
            "chip_pocket_coupon_body",
            CHIP_FIT_COUPON_X,
            CHIP_FIT_COUPON_Y,
            CARRIER_Z,
        );

        let pocket = centered_cube(
            "chip_pocket_coupon_rev_c_clearance_pocket",
            REVC_CHIP_LENGTH + self.chip_clearance_mm * 2.0,
            REVC_CHIP_WIDTH + self.chip_clearance_mm * 2.0,
            top_face_cut_height(CHIP_POCKET_DEPTH),
        )
        .translate(0.0, 0.0, top_face_cut_z(CARRIER_Z, CHIP_POCKET_DEPTH));

        let optical_window = centered_cube(
            "chip_pocket_coupon_optical_window_cut",
            REVC_CHIP_LENGTH - OPTICAL_WINDOW_MARGIN,
            REVC_CHIP_WIDTH - OPTICAL_WINDOW_MARGIN,
            CARRIER_Z + 2.0,
        );

        let gasket_land = rectangular_frame(
            "chip_pocket_coupon_gasket_land",
            PER_SLOT_GASKET_OUTER_X,
            PER_SLOT_GASKET_OUTER_Y,
            GASKET_LAND_Z + self.feature_anchor_overlap_mm,
            GASKET_LAND_W,
        )
        .translate(
            0.0,
            0.0,
            CARRIER_Z / 2.0 + GASKET_LAND_Z / 2.0 - self.feature_anchor_overlap_mm / 2.0,
        );

        let stop_z = CLOSURE_PLANE_ABOVE_CARRIER;
        let mut stops = Part::empty("chip_pocket_coupon_25pct_hard_stops");
        for (i, (x, y)) in [
            (
                -(PER_SLOT_GASKET_OUTER_X / 2.0
                    + INTERNAL_STOP_DIAMETER / 2.0
                    + self.stop_clearance_mm),
                -(PER_SLOT_GASKET_OUTER_Y / 2.0
                    + INTERNAL_STOP_DIAMETER / 2.0
                    + self.stop_clearance_mm),
            ),
            (
                (PER_SLOT_GASKET_OUTER_X / 2.0
                    + INTERNAL_STOP_DIAMETER / 2.0
                    + self.stop_clearance_mm),
                -(PER_SLOT_GASKET_OUTER_Y / 2.0
                    + INTERNAL_STOP_DIAMETER / 2.0
                    + self.stop_clearance_mm),
            ),
            (
                -(PER_SLOT_GASKET_OUTER_X / 2.0
                    + INTERNAL_STOP_DIAMETER / 2.0
                    + self.stop_clearance_mm),
                (PER_SLOT_GASKET_OUTER_Y / 2.0
                    + INTERNAL_STOP_DIAMETER / 2.0
                    + self.stop_clearance_mm),
            ),
            (
                (PER_SLOT_GASKET_OUTER_X / 2.0
                    + INTERNAL_STOP_DIAMETER / 2.0
                    + self.stop_clearance_mm),
                (PER_SLOT_GASKET_OUTER_Y / 2.0
                    + INTERNAL_STOP_DIAMETER / 2.0
                    + self.stop_clearance_mm),
            ),
        ]
        .into_iter()
        .enumerate()
        {
            stops = stops
                + centered_cylinder(
                    format!("chip_pocket_coupon_hard_stop_{i}"),
                    INTERNAL_STOP_DIAMETER / 2.0,
                    stop_z + self.feature_anchor_overlap_mm,
                    24,
                )
                .translate(
                    x,
                    y,
                    CARRIER_Z / 2.0 + stop_z / 2.0 - self.feature_anchor_overlap_mm / 2.0,
                );
        }

        body - pocket - optical_window + gasket_land + stops
    }

    fn gasket_compression_coupon(&self) -> Part {
        let coupon_x = 190.0;
        let coupon_y = 88.0;
        let coupon_z = 12.0;
        let mut coupon = centered_cube(
            "gasket_compression_coupon_body",
            coupon_x,
            coupon_y,
            coupon_z,
        );

        for (i, (name, y, compressed_height)) in [
            (
                "max_squeeze_30pct",
                26.0,
                GASKET_GUARD_MIN_COMPRESSED_HEIGHT,
            ),
            ("target_squeeze_25pct", 0.0, GASKET_COMPRESSED_HEIGHT),
            (
                "min_squeeze_20pct",
                -26.0,
                GASKET_GUARD_MAX_COMPRESSED_HEIGHT,
            ),
        ]
        .into_iter()
        .enumerate()
        {
            let groove = centered_cube(
                format!("gasket_compression_coupon_{name}_groove_cut"),
                138.0,
                self.gasket_groove_width_mm,
                GASKET_GROOVE_CUT_HEIGHT,
            )
            .translate(0.0, y, top_face_gasket_groove_cut_z(coupon_z));

            let left_stop = centered_cube(
                format!("gasket_compression_coupon_{name}_left_stop"),
                14.0,
                18.0,
                compressed_height,
            )
            .translate(-78.0, y, coupon_z / 2.0 + compressed_height / 2.0);
            let right_stop = centered_cube(
                format!("gasket_compression_coupon_{name}_right_stop"),
                14.0,
                18.0,
                compressed_height,
            )
            .translate(78.0, y, coupon_z / 2.0 + compressed_height / 2.0);
            let witness_pad = centered_cube(
                format!("gasket_compression_coupon_{name}_witness_pad_{i}"),
                34.0,
                18.0,
                compressed_height,
            )
            .translate(0.0, y, coupon_z / 2.0 + compressed_height / 2.0);

            coupon = coupon - groove + left_stop + right_stop + witness_pad;
        }

        let mounting_holes =
            centered_cylinder("gasket_coupon_mount_hole_left", 2.7, coupon_z + 2.0, 24)
                .translate(-86.0, -38.0, 0.0)
                + centered_cylinder("gasket_coupon_mount_hole_right", 2.7, coupon_z + 2.0, 24)
                    .translate(86.0, -38.0, 0.0);

        coupon - mounting_holes
    }

    fn dock_datum_rail_coupon(&self) -> Part {
        let base_x = 198.0;
        let base_y = 178.0;
        let base = centered_cube("dock_datum_rail_coupon_base", base_x, base_y, DOCK_Z);

        let rear_rail = centered_cube(
            "dock_datum_rail_coupon_rear_primary_rail",
            base_x - 26.0,
            DOCK_RAIL_W,
            DOCK_RAIL_Z,
        )
        .translate(0.0, base_y / 2.0 - 24.0, DOCK_Z / 2.0 + DOCK_RAIL_Z / 2.0);

        let left_rail = centered_cube(
            "dock_datum_rail_coupon_left_secondary_rail",
            DOCK_RAIL_W,
            base_y - 30.0,
            DOCK_RAIL_Z,
        )
        .translate(-base_x / 2.0 + 24.0, -2.0, DOCK_Z / 2.0 + DOCK_RAIL_Z / 2.0);

        let front_lip = centered_cube(
            "dock_datum_rail_coupon_front_low_retention_lip",
            base_x - 26.0,
            10.0,
            10.0,
        )
        .translate(0.0, -base_y / 2.0 + 20.0, DOCK_Z / 2.0 + 5.0);

        let gutter = centered_cube(
            "dock_datum_rail_coupon_visible_leak_gutter",
            base_x - 52.0,
            10.0,
            DOCK_Z + 2.0,
        )
        .translate(10.0, -base_y / 2.0 + 44.0, 0.0);

        let mounting_holes =
            centered_cylinder("dock_datum_rail_coupon_m5_left", 2.7, DOCK_Z + 2.0, 24).translate(
                -base_x / 2.0 + 22.0,
                -base_y / 2.0 + 22.0,
                0.0,
            ) + centered_cylinder("dock_datum_rail_coupon_m5_right", 2.7, DOCK_Z + 2.0, 24)
                .translate(base_x / 2.0 - 22.0, base_y / 2.0 - 22.0, 0.0);

        base + rear_rail + left_rail + front_lip - gutter - mounting_holes
    }

    fn carrier_corner_surrogate(&self) -> Part {
        let plate_x = 156.0;
        let plate_y = 132.0;
        let plate_z = 24.0;
        let plate = centered_cube("carrier_corner_surrogate_plate", plate_x, plate_y, plate_z);

        let rear_contact = centered_cube(
            "carrier_corner_surrogate_rear_datum_face_witness",
            plate_x - 24.0,
            6.0,
            12.0,
        )
        .translate(6.0, plate_y / 2.0 - 3.0, plate_z / 2.0 + 6.0);

        let left_contact = centered_cube(
            "carrier_corner_surrogate_left_datum_face_witness",
            6.0,
            plate_y - 24.0,
            12.0,
        )
        .translate(-plate_x / 2.0 + 3.0, -6.0, plate_z / 2.0 + 6.0);

        let handling_land =
            centered_cube("carrier_corner_surrogate_handling_land", 58.0, 16.0, 1.5).translate(
                10.0,
                -plate_y / 2.0 + 28.0,
                plate_z / 2.0 + 0.75,
            );

        let orientation_marker = centered_cube(
            "carrier_corner_surrogate_slot_one_orientation_marker_x",
            34.0,
            5.0,
            3.0,
        )
        .translate(
            -plate_x / 2.0 + 34.0,
            -plate_y / 2.0 + 28.0,
            plate_z / 2.0 + 1.5,
        ) + centered_cube(
            "carrier_corner_surrogate_slot_one_orientation_marker_y",
            5.0,
            34.0,
            3.0,
        )
        .translate(
            -plate_x / 2.0 + 20.0,
            -plate_y / 2.0 + 42.0,
            plate_z / 2.0 + 1.5,
        );

        let relief = centered_cube(
            "carrier_corner_surrogate_leak_relief_channel",
            plate_x - 34.0,
            8.0,
            plate_z + 2.0,
        )
        .translate(10.0, -plate_y / 2.0 + 12.0, 0.0);

        plate + rear_contact + left_contact + handling_land + orientation_marker - relief
    }

    fn bulkhead_connector_mockup(&self) -> Part {
        let block_x = 232.0;
        let block_y = 52.0;
        let block_z = 76.0;
        let block = centered_cube("bulkhead_connector_mockup_body", block_x, block_y, block_z);

        let mut media_ports = Part::empty("bulkhead_connector_mockup_media_port_cuts");
        for (i, x) in [-78.0, -52.0, -26.0, 0.0, 26.0, 52.0, 78.0]
            .into_iter()
            .enumerate()
        {
            media_ports = media_ports
                + centered_cylinder(
                    format!("bulkhead_connector_mockup_m{i}"),
                    3.2,
                    block_y + 4.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x, 0.0, 0.0);
        }

        let mut waste_ports = Part::empty("bulkhead_connector_mockup_waste_port_cuts");
        for (i, x) in [124.0, 150.0, 176.0].into_iter().enumerate() {
            waste_ports = waste_ports
                + centered_cylinder(
                    format!("bulkhead_connector_mockup_w{i}"),
                    3.2,
                    block_y + 4.0,
                    24,
                )
                .rotate(90.0, 0.0, 0.0)
                .translate(x - 52.0, 0.0, 0.0);
        }

        let label_strip = centered_cube(
            "bulkhead_connector_mockup_label_strip",
            block_x - 24.0,
            2.0,
            8.0,
        )
        .translate(0.0, -block_y / 2.0 - 1.0, block_z / 2.0 - 14.0);

        let mut comb = Part::empty("bulkhead_connector_mockup_strain_relief_comb");
        for i in 0..10 {
            comb = comb
                + centered_cube(
                    format!("bulkhead_connector_mockup_comb_tooth_{i}"),
                    5.0,
                    24.0,
                    18.0,
                )
                .translate(
                    centered_index(i, 10, 20.0),
                    -block_y / 2.0 - 12.0,
                    -20.0,
                );
        }

        let mounting_holes = centered_cylinder(
            "bulkhead_connector_mockup_mount_left",
            2.7,
            block_y + 4.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(-block_x / 2.0 + 18.0, 0.0, -block_z / 2.0 + 16.0)
            + centered_cylinder(
                "bulkhead_connector_mockup_mount_right",
                2.7,
                block_y + 4.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(block_x / 2.0 - 18.0, 0.0, -block_z / 2.0 + 16.0);

        block - media_ports - waste_ports - mounting_holes + label_strip + comb
    }
}
fn rectangular_frame(name: &str, x: f64, y: f64, z: f64, wall: f64) -> Part {
    let outer = centered_cube(format!("{name}_outer"), x, y, z);
    let inner = centered_cube(
        format!("{name}_inner"),
        x - wall * 2.0,
        y - wall * 2.0,
        z + 2.0,
    );
    outer - inner
}
