//! Parametric fit prototype within the fixed cartridge/reader footprint.
use crate::*;
use serde::{Deserialize, Serialize};
use vcad::{centered_cube, centered_cylinder, Part};
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Parameters {
    pub body_height_mm: f64,
    pub swab_port_diameter_mm: f64,
    pub channel_width_mm: f64,
    pub channel_depth_mm: f64,
    pub alignment_hole_diameter_mm: f64,
}
pub const OUTPUTS: [&str; 3] = [
    "diagnostic_cartridge_body.stl",
    "diagnostic_cartridge_window_frame.stl",
    "diagnostic_cartridge_assembly.stl",
];
impl Parameters {
    pub fn validate(&self) -> Result<(), String> {
        for (name, value, nominal) in [
            ("body_height_mm", self.body_height_mm, CARTRIDGE_BODY_HEIGHT),
            (
                "swab_port_diameter_mm",
                self.swab_port_diameter_mm,
                CARTRIDGE_SWAB_PORT_DIAMETER,
            ),
            (
                "channel_width_mm",
                self.channel_width_mm,
                CARTRIDGE_CHANNEL_WIDTH,
            ),
            (
                "channel_depth_mm",
                self.channel_depth_mm,
                CARTRIDGE_CHANNEL_DEPTH,
            ),
            (
                "alignment_hole_diameter_mm",
                self.alignment_hole_diameter_mm,
                CARTRIDGE_ALIGNMENT_HOLE_DIAMETER,
            ),
        ] {
            if !value.is_finite() || !(nominal..=nominal * 1.1).contains(&value) {
                return Err(format!(
                    "{name} must be finite and within {nominal}..={} mm for this fixed footprint",
                    nominal * 1.1
                ));
            }
        }
        Ok(())
    }
    pub fn build(&self, index: usize) -> Result<Part, String> {
        self.validate()?;
        Ok(match index {
            0 => self.cartridge_body(),
            1 => self.optical_window_frame(),
            2 => {
                let body = self.cartridge_body();
                let frame = self.optical_window_frame();
                // Preserve the original component-export evaluation before union.
                body.to_mesh();
                frame.to_mesh();
                body + frame.translate(0.0, 0.0, self.body_height_mm / 2.0 + 0.8)
            }
            _ => return Err("invalid component".into()),
        })
    }
    fn cartridge_body(&self) -> Part {
        let body = centered_cube(
            "diagnostic_cartridge_body",
            CARTRIDGE_LENGTH,
            CARTRIDGE_WIDTH,
            self.body_height_mm,
        );

        let cutouts = self.swab_dock_cutouts()
            + self.reaction_chamber_cutouts()
            + self.macrofluidic_channel_cutouts()
            + self.waste_chamber_cutout()
            + self.alignment_holes()
            + self.gasket_groove();

        body - cutouts + self.cartridge_latch_tabs() + self.handling_ribs() + self.film_lands()
    }

    fn swab_dock_cutouts(&self) -> Part {
        let bore_len = CARTRIDGE_SWAB_CHAMBER_LENGTH + 6.0;
        let bore_center_x = -CARTRIDGE_LENGTH / 2.0 + bore_len / 2.0 - 1.0;
        let swab_bore = centered_cylinder(
            "swab_insert_bore",
            self.swab_port_diameter_mm / 2.0,
            bore_len,
            48,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(bore_center_x, CARTRIDGE_SWAB_CENTER_Y, 0.0);

        let elution_chamber = centered_cube(
            "swab_elution_chamber",
            CARTRIDGE_SWAB_CHAMBER_LENGTH,
            CARTRIDGE_SWAB_CHAMBER_WIDTH,
            CARTRIDGE_SWAB_CHAMBER_DEPTH,
        )
        .translate(
            -CARTRIDGE_LENGTH / 2.0 + CARTRIDGE_SWAB_CHAMBER_LENGTH / 2.0 + 4.0,
            CARTRIDGE_SWAB_CENTER_Y,
            0.6,
        );

        let inlet_mouth = centered_cube(
            "swab_port_keyed_mouth",
            6.0,
            self.swab_port_diameter_mm + 4.0,
            self.swab_port_diameter_mm + 2.0,
        )
        .translate(-CARTRIDGE_LENGTH / 2.0 - 0.5, CARTRIDGE_SWAB_CENTER_Y, 0.0);

        swab_bore + elution_chamber + inlet_mouth
    }

    fn reaction_chamber_cutouts(&self) -> Part {
        let mut cutouts = Part::empty("reaction_chamber_cutouts");
        let top_z = self.body_height_mm / 2.0;

        for i in 0..NUM_SLOTS {
            let x = reaction_lane_x(i);
            let chamber = centered_cube(
                format!("reaction_chamber_{i}"),
                REACTION_CHAMBER_LENGTH,
                REACTION_CHAMBER_WIDTH,
                REACTION_CHAMBER_DEPTH + 0.2,
            )
            .translate(
                x,
                REACTION_CHAMBER_CENTER_Y,
                top_z - REACTION_CHAMBER_DEPTH / 2.0 + 0.1,
            );

            let optical_window_recess = centered_cube(
                format!("reaction_window_recess_{i}"),
                REACTION_WINDOW_LENGTH,
                REACTION_WINDOW_WIDTH,
                CARTRIDGE_TOP_FILM_THICKNESS + 0.2,
            )
            .translate(
                x,
                REACTION_CHAMBER_CENTER_Y,
                top_z - CARTRIDGE_TOP_FILM_THICKNESS / 2.0 + 0.1,
            );

            cutouts = cutouts + chamber + optical_window_recess;
        }

        cutouts
    }

    fn macrofluidic_channel_cutouts(&self) -> Part {
        let mut cuts = Part::empty("macrofluidic_channel_cutouts");
        let top_z = self.body_height_mm / 2.0;
        let channel_z = top_z - self.channel_depth_mm / 2.0 + 0.1;

        let common_bus = centered_cube(
            "sample_distribution_bus",
            HEATER_ZONE_LENGTH,
            self.channel_width_mm,
            self.channel_depth_mm + 0.2,
        )
        .translate(0.0, 0.5, channel_z);
        cuts = cuts + common_bus;

        let swab_to_bus = centered_cube(
            "swab_to_distribution_bus",
            34.0,
            self.channel_width_mm,
            self.channel_depth_mm + 0.2,
        )
        .rotate(0.0, 0.0, 25.0)
        .translate(-CARTRIDGE_LENGTH / 2.0 + 35.0, -5.5, channel_z);
        cuts = cuts + swab_to_bus;

        for i in 0..NUM_SLOTS {
            let x = reaction_lane_x(i);
            let inlet = centered_cube(
                format!("lane_{i}_reaction_inlet"),
                self.channel_width_mm,
                REACTION_CHAMBER_CENTER_Y - 0.5,
                self.channel_depth_mm + 0.2,
            )
            .translate(x - REACTION_CHAMBER_LENGTH / 4.0, 4.2, channel_z);
            let outlet = centered_cube(
                format!("lane_{i}_reaction_to_waste"),
                self.channel_width_mm,
                CARTRIDGE_WASTE_CENTER_Y - REACTION_CHAMBER_CENTER_Y,
                self.channel_depth_mm + 0.2,
            )
            .translate(x + REACTION_CHAMBER_LENGTH / 4.0, 13.0, channel_z);
            cuts = cuts + inlet + outlet;
        }

        cuts
    }

    fn waste_chamber_cutout(&self) -> Part {
        centered_cube(
            "sealed_waste_chamber",
            CARTRIDGE_WASTE_CHAMBER_LENGTH,
            CARTRIDGE_WASTE_CHAMBER_WIDTH,
            REACTION_CHAMBER_DEPTH + 0.4,
        )
        .translate(
            CARTRIDGE_LENGTH / 2.0 - CARTRIDGE_WASTE_CHAMBER_LENGTH / 2.0 - 8.0,
            CARTRIDGE_WASTE_CENTER_Y,
            self.body_height_mm / 2.0 - REACTION_CHAMBER_DEPTH / 2.0,
        )
    }

    fn alignment_holes(&self) -> Part {
        let mut holes = Part::empty("cartridge_alignment_holes");
        for (i, (x, y)) in self.cartridge_alignment_positions().iter().enumerate() {
            let hole = centered_cylinder(
                format!("cartridge_alignment_hole_{i}"),
                self.alignment_hole_diameter_mm / 2.0,
                self.body_height_mm + 2.0,
                24,
            )
            .translate(*x, *y, 0.0);
            holes = holes + hole;
        }
        holes
    }

    fn gasket_groove(&self) -> Part {
        let top_z = self.body_height_mm / 2.0;
        let groove_z = top_z - 0.25;
        let upper = centered_cube(
            "top_film_gasket_groove_rear",
            CARTRIDGE_LENGTH - 12.0,
            0.9,
            0.4,
        )
        .translate(0.0, CARTRIDGE_WIDTH / 2.0 - 5.0, groove_z);
        let lower = centered_cube(
            "top_film_gasket_groove_front",
            CARTRIDGE_LENGTH - 12.0,
            0.9,
            0.4,
        )
        .translate(0.0, -CARTRIDGE_WIDTH / 2.0 + 5.0, groove_z);
        let left = centered_cube(
            "top_film_gasket_groove_left",
            0.9,
            CARTRIDGE_WIDTH - 10.0,
            0.4,
        )
        .translate(-CARTRIDGE_LENGTH / 2.0 + 6.0, 0.0, groove_z);
        let right = centered_cube(
            "top_film_gasket_groove_right",
            0.9,
            CARTRIDGE_WIDTH - 10.0,
            0.4,
        )
        .translate(CARTRIDGE_LENGTH / 2.0 - 6.0, 0.0, groove_z);
        upper + lower + left + right
    }

    fn cartridge_latch_tabs(&self) -> Part {
        let tab_z = 0.0;
        let left = centered_cube("left_side_latch_tab", 10.0, 3.0, 3.0).translate(
            -CARTRIDGE_LENGTH / 2.0 + 22.0,
            -CARTRIDGE_WIDTH / 2.0 - 1.5,
            tab_z,
        );
        let right = centered_cube("right_side_latch_tab", 10.0, 3.0, 3.0).translate(
            -CARTRIDGE_LENGTH / 2.0 + 22.0,
            CARTRIDGE_WIDTH / 2.0 + 1.5,
            tab_z,
        );
        left + right
    }

    fn handling_ribs(&self) -> Part {
        let top_z = self.body_height_mm / 2.0 + 0.45;
        let mut ribs = Part::empty("cartridge_handling_ribs");
        for (i, y) in [-15.0, 15.0].iter().enumerate() {
            let rib = centered_cube(format!("cartridge_handling_rib_{i}"), 72.0, 1.2, 0.9)
                .translate(8.0, *y, top_z);
            ribs = ribs + rib;
        }
        ribs
    }

    fn film_lands(&self) -> Part {
        let top_z = self.body_height_mm / 2.0 + 0.25;
        let mut lands = Part::empty("reaction_window_film_lands");
        for i in 0..NUM_SLOTS {
            let land = centered_cube(
                format!("reaction_window_film_land_{i}"),
                REACTION_WINDOW_LENGTH + 2.0,
                REACTION_WINDOW_WIDTH + 2.0,
                0.5,
            )
            .translate(reaction_lane_x(i), REACTION_CHAMBER_CENTER_Y, top_z);
            let opening = centered_cube(
                format!("reaction_window_film_opening_{i}"),
                REACTION_WINDOW_LENGTH,
                REACTION_WINDOW_WIDTH,
                0.8,
            )
            .translate(reaction_lane_x(i), REACTION_CHAMBER_CENTER_Y, top_z);
            lands = lands + (land - opening);
        }
        lands
    }

    fn optical_window_frame(&self) -> Part {
        let frame = centered_cube(
            "diagnostic_cartridge_optical_film_frame",
            HEATER_ZONE_LENGTH + 8.0,
            REACTION_WINDOW_WIDTH + 8.0,
            1.0,
        );

        let mut windows = Part::empty("diagnostic_cartridge_film_frame_windows");
        for i in 0..NUM_SLOTS {
            let window = centered_cube(
                format!("film_frame_window_{i}"),
                REACTION_WINDOW_LENGTH,
                REACTION_WINDOW_WIDTH,
                1.4,
            )
            .translate(reaction_lane_x(i), 0.0, 0.0);
            windows = windows + window;
        }

        frame - windows
    }

    fn cartridge_alignment_positions(&self) -> [(f64, f64); 4] {
        [
            (-CARTRIDGE_LENGTH / 2.0 + 8.0, -CARTRIDGE_WIDTH / 2.0 + 7.0),
            (CARTRIDGE_LENGTH / 2.0 - 8.0, -CARTRIDGE_WIDTH / 2.0 + 7.0),
            (-CARTRIDGE_LENGTH / 2.0 + 8.0, CARTRIDGE_WIDTH / 2.0 - 7.0),
            (CARTRIDGE_LENGTH / 2.0 - 8.0, CARTRIDGE_WIDTH / 2.0 - 7.0),
        ]
    }
}
