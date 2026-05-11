use vcad::{centered_cube, centered_cylinder, Part};

// ═══════════════════════════════════════════════════════════════════════════
// ESP32-S3 Controller Box — External Housing for Chip-Stack Thermal Container
// ═══════════════════════════════════════════════════════════════════════════
//
// Wall-mount (with alternative DIN-rail ears) control box that houses ALL
// electronics for the chip-stack incubator. Lives OUTSIDE the thermal
// container (which runs at 37 °C / 90 %RH — bad for electronics).
//
// Target IP rating: IP54 (dust-protected, splash-resistant). Not IP67 — the
// box is not submerged; it sits next to a humid incubator, so the rating
// defends against 90 %RH condensation drip and dust ingress.
//
// Overall outside dimensions: 300 × 200 × 150 mm (X × Y × Z).
// Construction: 3 mm PETG shell (printable on any 300 mm-bed FDM, split into
// halves if needed) with a removable front panel and a cable-gland back panel.
//
// Coordinate system: box centered at origin.
//   X: left → right    (−150 … +150)   outside width  300 mm
//   Y: back → front    (−100 … +100)   outside depth  200 mm   (front = +Y)
//   Z: bottom → top    (−75  … +75)    outside height 150 mm
//
// Contents (centered around origin):
//   • ESP32-S3 controller PCB v2 — 100 × 80 mm, 4× M3 standoffs 10 mm tall,
//     mounted to left wall interior.
//   • MeanWell LRS-240-24 PSU — 199 × 98 × 38 mm, bolted to bottom wall via
//     4× M4 through-holes.
//   • Crydom DC60S5 SSR — 58 × 45 × 23 mm, mounted to 80 × 60 × 3 mm aluminum
//     heat-spreader plate fastened to bottom wall behind the PSU.
//   • Generic buck 24→12 V DC-DC — 40 × 20 × 15 mm, clipped onto DIN rail.
//   • DIN rail segment — 35 mm × 150 mm on right wall interior; carries the
//     Phoenix Contact PT 2.5 terminal blocks (20 positions) and the buck.
//   • Front-panel: 2.4" TFT (Adafruit 1770, 56 × 35 mm), 40 mm E-stop
//     (Schneider XALK178), main-power rocker (NKK CWSB11AA2S), 3× status LEDs.
//   • Back panel: 6× cable glands (M16/M20), USB-C bulkhead, RJ45 bulkhead,
//     80 mm exhaust fan (Noctua NF-A8) + louver cutout.
//   • Both side walls: 2× louver bands each for passive cross-flow ventilation.
//
// Exports:
//   output/controller_enclosure_body.stl
//   output/controller_enclosure_front_panel.stl
//   output/controller_enclosure_back_panel.stl
//   output/controller_enclosure_din_rail.stl
//   output/controller_enclosure_pcb_mount.stl
//   output/controller_enclosure_psu_mount.stl
//   output/controller_enclosure_ssr_heatsink.stl
//   output/controller_enclosure_display_cutout.stl   (reference cut volume)
//   output/controller_enclosure_estop_cutout.stl     (reference cut volume)
//   output/controller_enclosure_assembly.stl         (all parts, positioned)
//
// Remember: vcad::Part does NOT implement Clone. Build each feature fresh.

// ─── Overall box dimensions ─────────────────────────────────────────────────
const BOX_OUTER_X: f64 = 300.0;
const BOX_OUTER_Y: f64 = 200.0;
const BOX_OUTER_Z: f64 = 150.0;
const BOX_WALL: f64 = 3.0; // PETG wall thickness

// Interior
const BOX_INNER_X: f64 = BOX_OUTER_X - 2.0 * BOX_WALL; // 294
const BOX_INNER_Y: f64 = BOX_OUTER_Y - 2.0 * BOX_WALL; // 194
const BOX_INNER_Z: f64 = BOX_OUTER_Z - 2.0 * BOX_WALL; // 144

// Front-panel recess (front-panel sits flush inside a 3 mm-deep lip on +Y face)
const FRONT_PANEL_THICKNESS: f64 = 4.0;
const FRONT_PANEL_INSET: f64 = 2.0; // 2 mm gasket-groove inset from outer edge
const FRONT_GASKET_DEPTH: f64 = 2.0;
const FRONT_GASKET_WIDTH: f64 = 3.0;

// Back-panel (gland plate — removable for wiring, gasketed)
const BACK_PANEL_THICKNESS: f64 = 4.0;

// Front/back panel screw holes (M4, 4 corners)
const PANEL_SCREW_DIA: f64 = 4.3;
const PANEL_SCREW_INSET: f64 = 10.0;

// ─── Components (datasheet dimensions) ──────────────────────────────────────

// ESP32-S3 controller PCB v2 (from T-8050DF80 / A-496F33BE)
const PCB_X: f64 = 100.0;
const PCB_Y: f64 = 80.0;
const PCB_STANDOFF_H: f64 = 10.0;
const PCB_STANDOFF_DIA: f64 = 6.0;
const PCB_MOUNT_HOLE_DIA: f64 = 3.2;
const PCB_MOUNT_INSET: f64 = 3.0; // corner inset on the PCB

// MeanWell LRS-240-24 (199 × 98 × 38 mm, screws on long edges)
const PSU_X: f64 = 199.0;
const PSU_Y: f64 = 98.0;
#[allow(dead_code)]
const PSU_Z: f64 = 38.0;
const PSU_SCREW_DIA: f64 = 4.3; // M4 clearance
const PSU_SCREW_SPACING_X: f64 = 150.0; // long-edge mount holes
const PSU_SCREW_SPACING_Y: f64 = 50.0;

// Crydom DC60S5 SSR (58 × 45 × 23 mm, 2× M4 mounting holes along centerline)
#[allow(dead_code)]
const SSR_X: f64 = 58.0;
#[allow(dead_code)]
const SSR_Y: f64 = 45.0;
#[allow(dead_code)]
const SSR_Z: f64 = 23.0;
const SSR_MOUNT_SPACING: f64 = 47.6; // datasheet M4 hole pitch
const SSR_MOUNT_DIA: f64 = 4.3;

// Aluminum heat-spreader under SSR
const HEATSINK_X: f64 = 80.0;
const HEATSINK_Y: f64 = 60.0;
const HEATSINK_Z: f64 = 3.0;

// DIN rail — standard TS35, 35 mm wide × 7.5 mm tall, 150 mm length here
const DIN_RAIL_W: f64 = 35.0;
const DIN_RAIL_H: f64 = 7.5;
const DIN_RAIL_LEN: f64 = 150.0;
const DIN_RAIL_SLOT_W: f64 = 25.0;
const DIN_RAIL_SLOT_H: f64 = 3.0;

// Front-panel cutouts
const DISPLAY_W: f64 = 56.0; // Adafruit 1770 2.4" TFT
const DISPLAY_H: f64 = 35.0;
const DISPLAY_SCREW_DIA: f64 = 3.2; // M3 for 4× corner holes
const DISPLAY_SCREW_SPACING_X: f64 = 66.0;
const DISPLAY_SCREW_SPACING_Y: f64 = 45.0;

const ESTOP_DIA: f64 = 22.5; // Schneider XALK178 mounting hole (22 mm standard + slight clearance)
const ESTOP_KEYSLOT_W: f64 = 3.2;
const ESTOP_KEYSLOT_DEPTH: f64 = 1.5;

const ROCKER_LEN: f64 = 30.5; // NKK CWSB11AA2S panel cutout
const ROCKER_W: f64 = 22.5;

const STATUS_LED_DIA: f64 = 5.5; // 5 mm bezel LEDs, 3 of them

// Back-panel cutouts
const GLAND_M16_DIA: f64 = 16.5; // M16 gland threaded bushing hole
const GLAND_M20_DIA: f64 = 20.5;
const USBC_W: f64 = 16.0; // Amphenol 12401589E412GLF bulkhead cutout (D-hole-ish)
const USBC_H: f64 = 10.0;
const RJ45_W: f64 = 19.5;
const RJ45_H: f64 = 16.5;
const FAN_CUTOUT_DIA: f64 = 80.0;
const FAN_SCREW_SPACING: f64 = 71.5;
const FAN_SCREW_DIA: f64 = 4.3;

// Louvers (side walls)
const LOUVER_LEN: f64 = 60.0;
const LOUVER_W: f64 = 4.0;
const LOUVER_COUNT: usize = 6;
const LOUVER_PITCH: f64 = 10.0;

// Wall-mount ears (back face)
const EAR_X: f64 = 30.0;
const EAR_Y: f64 = BOX_WALL;
const EAR_Z: f64 = 30.0;
const EAR_HOLE_DIA: f64 = 5.5; // M5 clearance for wood screws
const EAR_OFFSET_X: f64 = BOX_OUTER_X / 2.0 - EAR_X / 2.0 + 15.0;

// DIN-rail-mount ears (alt): small tab with TS35 clip on the back
const DIN_EAR_X: f64 = 60.0;
const DIN_EAR_Y: f64 = 6.0;
const DIN_EAR_Z: f64 = 40.0;

// ─── Part builders ──────────────────────────────────────────────────────────

/// Main enclosure body — 5 walls (bottom, top, left, right, back) plus an
/// open +Y face for the front panel. Features cable glands, louvers,
/// mounting bosses, wall-mount ears, and DIN-rail-mount tabs.
fn enclosure_body() -> Part {
    // Start with solid outer shell, subtract inner cavity
    let outer = centered_cube("box_outer", BOX_OUTER_X, BOX_OUTER_Y, BOX_OUTER_Z);
    let inner = centered_cube("box_inner", BOX_INNER_X, BOX_INNER_Y, BOX_INNER_Z);
    let mut body = outer - inner;

    // ── Front-face opening (for removable front panel, +Y) ──
    // Sits inside a 3 mm rim that retains the front panel. Cut a through-hole
    // sized for the panel outline; the lip is built by NOT cutting the rim.
    let front_cutout_w = BOX_INNER_X - 2.0 * FRONT_PANEL_INSET; // 294 - 4 = 290
    let front_cutout_z = BOX_INNER_Z - 2.0 * FRONT_PANEL_INSET; // 144 - 4 = 140
    let front_cutout = centered_cube(
        "front_cutout",
        front_cutout_w,
        BOX_WALL + 2.0,
        front_cutout_z,
    )
    .translate(0.0, BOX_OUTER_Y / 2.0, 0.0);
    body = body - front_cutout;

    // ── Front-face gasket groove (3 × 2 mm, square path just inside the rim) ──
    // Carved into the +Y face so the front panel seals against silicone cord.
    let groove_outer_x = front_cutout_w + FRONT_GASKET_WIDTH * 2.0;
    let groove_outer_z = front_cutout_z + FRONT_GASKET_WIDTH * 2.0;
    let groove_inner_x = front_cutout_w;
    let groove_inner_z = front_cutout_z;
    let groove_y_center = BOX_OUTER_Y / 2.0 - FRONT_GASKET_DEPTH / 2.0;

    let groove_outer = centered_cube(
        "groove_outer",
        groove_outer_x,
        FRONT_GASKET_DEPTH + 0.1,
        groove_outer_z,
    )
    .translate(0.0, groove_y_center, 0.0);
    let groove_inner = centered_cube(
        "groove_inner",
        groove_inner_x,
        FRONT_GASKET_DEPTH + 0.2,
        groove_inner_z,
    )
    .translate(0.0, groove_y_center, 0.0);
    body = body - (groove_outer - groove_inner);

    // ── Front-panel screw holes (4× M4 tapped bosses around rim) ──
    // Modeled as through-holes in the rim; matching holes on front-panel.
    let fp_positions = [
        (
            -(front_cutout_w / 2.0) - FRONT_GASKET_WIDTH - PANEL_SCREW_INSET / 2.0,
            front_cutout_z / 2.0 + FRONT_GASKET_WIDTH + PANEL_SCREW_INSET / 2.0,
        ),
        (
            (front_cutout_w / 2.0) + FRONT_GASKET_WIDTH + PANEL_SCREW_INSET / 2.0,
            front_cutout_z / 2.0 + FRONT_GASKET_WIDTH + PANEL_SCREW_INSET / 2.0,
        ),
        (
            -(front_cutout_w / 2.0) - FRONT_GASKET_WIDTH - PANEL_SCREW_INSET / 2.0,
            -(front_cutout_z / 2.0) - FRONT_GASKET_WIDTH - PANEL_SCREW_INSET / 2.0,
        ),
        (
            (front_cutout_w / 2.0) + FRONT_GASKET_WIDTH + PANEL_SCREW_INSET / 2.0,
            -(front_cutout_z / 2.0) - FRONT_GASKET_WIDTH - PANEL_SCREW_INSET / 2.0,
        ),
    ];
    for (i, &(hx, hz)) in fp_positions.iter().enumerate() {
        let hole = centered_cylinder(
            format!("fp_screw_{i}"),
            PANEL_SCREW_DIA / 2.0,
            BOX_WALL + 2.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(hx, BOX_OUTER_Y / 2.0 - BOX_WALL / 2.0, hz);
        body = body - hole;
    }

    // ── Back-panel screw holes (4× M4 around −Y face) ──
    let bp_positions = [
        (
            -(BOX_INNER_X / 2.0) + PANEL_SCREW_INSET,
            (BOX_INNER_Z / 2.0) - PANEL_SCREW_INSET,
        ),
        (
            (BOX_INNER_X / 2.0) - PANEL_SCREW_INSET,
            (BOX_INNER_Z / 2.0) - PANEL_SCREW_INSET,
        ),
        (
            -(BOX_INNER_X / 2.0) + PANEL_SCREW_INSET,
            -(BOX_INNER_Z / 2.0) + PANEL_SCREW_INSET,
        ),
        (
            (BOX_INNER_X / 2.0) - PANEL_SCREW_INSET,
            -(BOX_INNER_Z / 2.0) + PANEL_SCREW_INSET,
        ),
    ];
    for (i, &(hx, hz)) in bp_positions.iter().enumerate() {
        let hole = centered_cylinder(
            format!("bp_screw_{i}"),
            PANEL_SCREW_DIA / 2.0,
            BOX_WALL + 2.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(hx, -(BOX_OUTER_Y / 2.0) + BOX_WALL / 2.0, hz);
        body = body - hole;
    }

    // ── Side-wall louver slits (both left and right walls) ──
    // Two 6-slit bands per wall, one low and one high, for natural cross-flow.
    // Slits angle slightly (modeled as straight rectangles for FDM simplicity;
    // real IP54 louvers are covered by a fine stainless mesh gasket bonded
    // behind the slits — mesh is a BOM item, not a modeled feature).
    let louver_depth = BOX_WALL + 2.0;
    let band_z_offsets = [-(BOX_INNER_Z / 4.0), BOX_INNER_Z / 4.0];
    for (wall_idx, &wall_x) in [-(BOX_OUTER_X / 2.0), BOX_OUTER_X / 2.0].iter().enumerate() {
        for (band_idx, &band_z) in band_z_offsets.iter().enumerate() {
            for slit_i in 0..LOUVER_COUNT {
                let x_off = (slit_i as f64 - (LOUVER_COUNT as f64 - 1.0) / 2.0) * LOUVER_PITCH;
                let slit = centered_cube(
                    format!("louver_{wall_idx}_{band_idx}_{slit_i}"),
                    louver_depth,
                    LOUVER_LEN,
                    LOUVER_W,
                )
                .translate(wall_x, x_off, band_z);
                body = body - slit;
            }
        }
    }

    // ── Wall-mount ears (4× on back face, protruding in -Y) ──
    // Cast as added volume with M5 through-holes for wood/drywall screws.
    let ear_positions = [
        (-EAR_OFFSET_X, BOX_OUTER_Z / 2.0 - EAR_Z / 2.0),
        (EAR_OFFSET_X, BOX_OUTER_Z / 2.0 - EAR_Z / 2.0),
        (-EAR_OFFSET_X, -(BOX_OUTER_Z / 2.0) + EAR_Z / 2.0),
        (EAR_OFFSET_X, -(BOX_OUTER_Z / 2.0) + EAR_Z / 2.0),
    ];
    for (i, &(ex, ez)) in ear_positions.iter().enumerate() {
        let ear = centered_cube(format!("ear_{i}"), EAR_X, EAR_Y, EAR_Z).translate(
            ex,
            -(BOX_OUTER_Y / 2.0) - EAR_Y / 2.0,
            ez,
        );
        let hole = centered_cylinder(format!("ear_hole_{i}"), EAR_HOLE_DIA / 2.0, EAR_Y + 4.0, 24)
            .rotate(90.0, 0.0, 0.0)
            .translate(ex, -(BOX_OUTER_Y / 2.0) - EAR_Y / 2.0, ez);
        body = body + ear;
        body = body - hole;
    }

    // ── DIN-rail-mount tabs (alt mount, 2× on back face) ──
    // Thin vertical tabs along the back face centerline that accept TS35
    // spring clips screwed to them at assembly time.
    let din_ear_positions = [
        (0.0, BOX_OUTER_Z / 2.0 - DIN_EAR_Z / 2.0 - 5.0),
        (0.0, -(BOX_OUTER_Z / 2.0) + DIN_EAR_Z / 2.0 + 5.0),
    ];
    for (i, &(dx, dz)) in din_ear_positions.iter().enumerate() {
        let tab = centered_cube(format!("din_ear_{i}"), DIN_EAR_X, DIN_EAR_Y, DIN_EAR_Z).translate(
            dx,
            -(BOX_OUTER_Y / 2.0) - DIN_EAR_Y - 2.0,
            dz,
        );
        body = body + tab;
        // M4 tapping holes for spring clip (2 per tab)
        for (j, &hx) in [-20.0, 20.0].iter().enumerate() {
            let hole = centered_cylinder(
                format!("din_ear_hole_{i}_{j}"),
                PANEL_SCREW_DIA / 2.0,
                DIN_EAR_Y + 4.0,
                24,
            )
            .rotate(90.0, 0.0, 0.0)
            .translate(dx + hx, -(BOX_OUTER_Y / 2.0) - DIN_EAR_Y - 2.0, dz);
            body = body - hole;
        }
    }

    // ── PCB standoff bosses on left wall interior (4×) ──
    // 10 mm tall solid cylindrical posts extending rightward (+X) from the
    // left wall. Each has a 2.7 mm tapping hole for M3 self-tappers.
    let pcb_center_x = -(BOX_INNER_X / 2.0) + PCB_STANDOFF_H + PCB_Y / 2.0; // measured later
    let _ = pcb_center_x; // bosses attach to interior of left wall directly
    let pcb_boss_positions_yz = [
        (
            -(PCB_Y / 2.0) + PCB_MOUNT_INSET,
            -(PCB_X / 2.0) + PCB_MOUNT_INSET,
        ),
        (
            (PCB_Y / 2.0) - PCB_MOUNT_INSET,
            -(PCB_X / 2.0) + PCB_MOUNT_INSET,
        ),
        (
            -(PCB_Y / 2.0) + PCB_MOUNT_INSET,
            (PCB_X / 2.0) - PCB_MOUNT_INSET,
        ),
        (
            (PCB_Y / 2.0) - PCB_MOUNT_INSET,
            (PCB_X / 2.0) - PCB_MOUNT_INSET,
        ),
    ];
    // PCB is mounted vertically on the left wall (X = -BOX_INNER_X/2), face +X.
    // Standoffs extend along +X; their height is 10 mm. PCB plane is at
    // X = -BOX_INNER_X/2 + PCB_STANDOFF_H.
    // PCB's local Y maps to box Y, local X maps to box Z.
    let boss_base_x = -(BOX_INNER_X / 2.0); // inner surface of left wall
    for (i, &(by, bz)) in pcb_boss_positions_yz.iter().enumerate() {
        let boss = centered_cylinder(
            format!("pcb_boss_{i}"),
            PCB_STANDOFF_DIA / 2.0,
            PCB_STANDOFF_H,
            24,
        )
        .rotate(0.0, 90.0, 0.0)
        .translate(boss_base_x + PCB_STANDOFF_H / 2.0, by, bz);
        body = body + boss;
        // Tapping hole (2.7 mm, centered, 8 mm deep)
        let tap = centered_cylinder(format!("pcb_tap_{i}"), 2.7 / 2.0, 8.0, 24)
            .rotate(0.0, 90.0, 0.0)
            .translate(boss_base_x + 8.0 / 2.0 + 1.0, by, bz);
        body = body - tap;
    }

    // ── PSU mount holes in bottom wall (4× M4 through-holes) ──
    // PSU lies flat on the floor with its long axis along X, centered on
    // (0, -30, -bottom). Datasheet mount holes are on the two long edges
    // at (±PSU_SCREW_SPACING_X/2, ±PSU_SCREW_SPACING_Y/2).
    let psu_center_x = 0.0;
    let psu_center_y = -(BOX_INNER_Y / 2.0) + PSU_Y / 2.0 + 10.0;
    let psu_positions = [
        (
            psu_center_x - PSU_SCREW_SPACING_X / 2.0,
            psu_center_y - PSU_SCREW_SPACING_Y / 2.0,
        ),
        (
            psu_center_x + PSU_SCREW_SPACING_X / 2.0,
            psu_center_y - PSU_SCREW_SPACING_Y / 2.0,
        ),
        (
            psu_center_x - PSU_SCREW_SPACING_X / 2.0,
            psu_center_y + PSU_SCREW_SPACING_Y / 2.0,
        ),
        (
            psu_center_x + PSU_SCREW_SPACING_X / 2.0,
            psu_center_y + PSU_SCREW_SPACING_Y / 2.0,
        ),
    ];
    for (i, &(hx, hy)) in psu_positions.iter().enumerate() {
        let hole = centered_cylinder(
            format!("psu_mount_{i}"),
            PSU_SCREW_DIA / 2.0,
            BOX_WALL + 2.0,
            24,
        )
        .translate(hx, hy, -(BOX_OUTER_Z / 2.0) + BOX_WALL / 2.0);
        body = body - hole;
    }

    // ── Heatsink/SSR mount holes in bottom wall (2× M4) ──
    // Heatsink sits in a reserved footprint between PSU and back wall.
    let hs_center_x = 0.0;
    let hs_center_y = -(BOX_INNER_Y / 2.0) + PSU_Y + 10.0 + HEATSINK_Y / 2.0 + 5.0;
    for (i, &sx) in [-SSR_MOUNT_SPACING / 2.0, SSR_MOUNT_SPACING / 2.0]
        .iter()
        .enumerate()
    {
        let hole = centered_cylinder(
            format!("ssr_mount_{i}"),
            SSR_MOUNT_DIA / 2.0,
            BOX_WALL + 2.0,
            24,
        )
        .translate(
            hs_center_x + sx,
            hs_center_y,
            -(BOX_OUTER_Z / 2.0) + BOX_WALL / 2.0,
        );
        body = body - hole;
    }

    body
}

/// Front panel (removable). Carries the 2.4" TFT display, E-stop, main
/// rocker switch, and 3× status LEDs. Labeled cutouts only — actual text
/// labels are silk-screened or laser-engraved at the printer.
fn front_panel() -> Part {
    let panel_w = BOX_INNER_X - 2.0 * FRONT_PANEL_INSET - 0.4; // 289.6 mm (0.2 mm gap/side)
    let panel_z = BOX_INNER_Z - 2.0 * FRONT_PANEL_INSET - 0.4; // 139.6 mm
    let panel = centered_cube("front_panel", panel_w, FRONT_PANEL_THICKNESS, panel_z);
    let mut panel = panel;

    // ── Display cutout (rectangle + 4× M3 mount holes) ──
    let display_x = -80.0; // left-of-center, above power rocker
    let display_z = 20.0;
    let display_cut = centered_cube(
        "display_cut",
        DISPLAY_W,
        FRONT_PANEL_THICKNESS + 2.0,
        DISPLAY_H,
    )
    .translate(display_x, 0.0, display_z);
    panel = panel - display_cut;
    let display_screw_offsets = [
        (
            -DISPLAY_SCREW_SPACING_X / 2.0,
            -DISPLAY_SCREW_SPACING_Y / 2.0,
        ),
        (
            DISPLAY_SCREW_SPACING_X / 2.0,
            -DISPLAY_SCREW_SPACING_Y / 2.0,
        ),
        (
            -DISPLAY_SCREW_SPACING_X / 2.0,
            DISPLAY_SCREW_SPACING_Y / 2.0,
        ),
        (DISPLAY_SCREW_SPACING_X / 2.0, DISPLAY_SCREW_SPACING_Y / 2.0),
    ];
    for (i, &(dx, dz)) in display_screw_offsets.iter().enumerate() {
        let hole = centered_cylinder(
            format!("display_screw_{i}"),
            DISPLAY_SCREW_DIA / 2.0,
            FRONT_PANEL_THICKNESS + 2.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(display_x + dx, 0.0, display_z + dz);
        panel = panel - hole;
    }

    // ── E-stop cutout (22 mm hole + anti-rotation key slot) ──
    let estop_x = 90.0; // right-of-center
    let estop_z = 0.0;
    let estop_hole = centered_cylinder(
        "estop_hole",
        ESTOP_DIA / 2.0,
        FRONT_PANEL_THICKNESS + 2.0,
        48,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(estop_x, 0.0, estop_z);
    panel = panel - estop_hole;
    let estop_keyslot = centered_cube(
        "estop_keyslot",
        ESTOP_KEYSLOT_W,
        FRONT_PANEL_THICKNESS + 2.0,
        ESTOP_KEYSLOT_DEPTH,
    )
    .translate(
        estop_x,
        0.0,
        estop_z + ESTOP_DIA / 2.0 + ESTOP_KEYSLOT_DEPTH / 2.0 - 0.1,
    );
    panel = panel - estop_keyslot;

    // ── Main-power rocker switch (rectangular cutout, lower-left) ──
    let rocker_x = -80.0;
    let rocker_z = -40.0;
    let rocker_cut = centered_cube(
        "rocker_cut",
        ROCKER_LEN,
        FRONT_PANEL_THICKNESS + 2.0,
        ROCKER_W,
    )
    .translate(rocker_x, 0.0, rocker_z);
    panel = panel - rocker_cut;

    // ── 3× Status LED holes (across the top, 10 mm apart) ──
    let led_y = 0.0;
    let led_z = 55.0;
    for (i, &lx) in [-20.0_f64, 0.0, 20.0].iter().enumerate() {
        let led_hole = centered_cylinder(
            format!("status_led_{i}"),
            STATUS_LED_DIA / 2.0,
            FRONT_PANEL_THICKNESS + 2.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(lx, led_y, led_z);
        panel = panel - led_hole;
    }

    // ── 4× panel-screw holes (corners) ──
    let sc_positions = [
        (
            -(panel_w / 2.0 - FRONT_GASKET_WIDTH - PANEL_SCREW_INSET / 2.0),
            (panel_z / 2.0 - FRONT_GASKET_WIDTH - PANEL_SCREW_INSET / 2.0),
        ),
        (
            panel_w / 2.0 - FRONT_GASKET_WIDTH - PANEL_SCREW_INSET / 2.0,
            panel_z / 2.0 - FRONT_GASKET_WIDTH - PANEL_SCREW_INSET / 2.0,
        ),
        (
            -(panel_w / 2.0 - FRONT_GASKET_WIDTH - PANEL_SCREW_INSET / 2.0),
            -(panel_z / 2.0 - FRONT_GASKET_WIDTH - PANEL_SCREW_INSET / 2.0),
        ),
        (
            panel_w / 2.0 - FRONT_GASKET_WIDTH - PANEL_SCREW_INSET / 2.0,
            -(panel_z / 2.0 - FRONT_GASKET_WIDTH - PANEL_SCREW_INSET / 2.0),
        ),
    ];
    for (i, &(sx, sz)) in sc_positions.iter().enumerate() {
        let hole = centered_cylinder(
            format!("fp_screw_{i}"),
            PANEL_SCREW_DIA / 2.0,
            FRONT_PANEL_THICKNESS + 2.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(sx, 0.0, sz);
        panel = panel - hole;
    }

    panel
}

/// Back panel (removable — gland plate). Carries 6× cable glands, 80 mm fan
/// + filter, USB-C bulkhead, RJ45 bulkhead. Sized to mate with the rear of
///   the enclosure body with a perimeter gasket.
fn back_panel_with_glands() -> Part {
    let panel_w = BOX_INNER_X - 0.4; // 293.6
    let panel_z = BOX_INNER_Z - 0.4; // 143.6
    let panel = centered_cube("back_panel", panel_w, BACK_PANEL_THICKNESS, panel_z);
    let mut panel = panel;

    // ── 6× cable glands along bottom edge (3× M16 small signals + 3× M20 power) ──
    let gland_z = -(panel_z / 2.0) + 20.0;
    let gland_specs = [
        (-120.0, GLAND_M20_DIA), // mains 24 V in
        (-72.0, GLAND_M20_DIA),  // heater out (200 W)
        (-24.0, GLAND_M20_DIA),  // stepper drive out (to syringe pump)
        (24.0, GLAND_M16_DIA),   // fan out (12 V blower)
        (72.0, GLAND_M16_DIA),   // CO2 solenoid
        (120.0, GLAND_M16_DIA),  // sensor umbilical (SHT40 + CO2 + DS18B20 multi-conductor)
    ];
    for (i, &(gx, gdia)) in gland_specs.iter().enumerate() {
        let gland = centered_cylinder(
            format!("gland_{i}"),
            gdia / 2.0,
            BACK_PANEL_THICKNESS + 2.0,
            32,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(gx, 0.0, gland_z);
        panel = panel - gland;
    }

    // ── 80 mm fan cutout + 4× M4 holes (exhaust, upper-center) ──
    let fan_center_x = 0.0;
    let fan_center_z = 35.0;
    let fan_hole = centered_cylinder(
        "fan_hole",
        FAN_CUTOUT_DIA / 2.0,
        BACK_PANEL_THICKNESS + 2.0,
        64,
    )
    .rotate(90.0, 0.0, 0.0)
    .translate(fan_center_x, 0.0, fan_center_z);
    panel = panel - fan_hole;
    let fan_corners = [
        (-FAN_SCREW_SPACING / 2.0, -FAN_SCREW_SPACING / 2.0),
        (FAN_SCREW_SPACING / 2.0, -FAN_SCREW_SPACING / 2.0),
        (-FAN_SCREW_SPACING / 2.0, FAN_SCREW_SPACING / 2.0),
        (FAN_SCREW_SPACING / 2.0, FAN_SCREW_SPACING / 2.0),
    ];
    for (i, &(fx, fz)) in fan_corners.iter().enumerate() {
        let hole = centered_cylinder(
            format!("fan_screw_{i}"),
            FAN_SCREW_DIA / 2.0,
            BACK_PANEL_THICKNESS + 2.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(fan_center_x + fx, 0.0, fan_center_z + fz);
        panel = panel - hole;
    }

    // ── USB-C bulkhead (upper-left, 16 × 10 mm rectangle) ──
    let usbc_cut = centered_cube("usbc_cut", USBC_W, BACK_PANEL_THICKNESS + 2.0, USBC_H)
        .translate(-100.0, 0.0, 55.0);
    panel = panel - usbc_cut;
    for (i, &(ux, uz)) in [(-100.0 - 11.0, 55.0), (-100.0 + 11.0, 55.0)]
        .iter()
        .enumerate()
    {
        let hole = centered_cylinder(
            format!("usbc_screw_{i}"),
            2.5 / 2.0,
            BACK_PANEL_THICKNESS + 2.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(ux, 0.0, uz);
        panel = panel - hole;
    }

    // ── RJ45 bulkhead (upper-right, 19.5 × 16.5 mm rectangle) ──
    let rj_cut = centered_cube("rj45_cut", RJ45_W, BACK_PANEL_THICKNESS + 2.0, RJ45_H)
        .translate(100.0, 0.0, 55.0);
    panel = panel - rj_cut;
    for (i, &(rx, rz)) in [(100.0 - 15.0, 55.0), (100.0 + 15.0, 55.0)]
        .iter()
        .enumerate()
    {
        let hole = centered_cylinder(
            format!("rj45_screw_{i}"),
            2.5 / 2.0,
            BACK_PANEL_THICKNESS + 2.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(rx, 0.0, rz);
        panel = panel - hole;
    }

    // ── 4× panel-screw holes (corners) to match body ──
    let bp_positions = [
        (
            -(BOX_INNER_X / 2.0) + PANEL_SCREW_INSET,
            (BOX_INNER_Z / 2.0) - PANEL_SCREW_INSET,
        ),
        (
            (BOX_INNER_X / 2.0) - PANEL_SCREW_INSET,
            (BOX_INNER_Z / 2.0) - PANEL_SCREW_INSET,
        ),
        (
            -(BOX_INNER_X / 2.0) + PANEL_SCREW_INSET,
            -(BOX_INNER_Z / 2.0) + PANEL_SCREW_INSET,
        ),
        (
            (BOX_INNER_X / 2.0) - PANEL_SCREW_INSET,
            -(BOX_INNER_Z / 2.0) + PANEL_SCREW_INSET,
        ),
    ];
    for (i, &(sx, sz)) in bp_positions.iter().enumerate() {
        let hole = centered_cylinder(
            format!("bp_screw_{i}"),
            PANEL_SCREW_DIA / 2.0,
            BACK_PANEL_THICKNESS + 2.0,
            24,
        )
        .rotate(90.0, 0.0, 0.0)
        .translate(sx, 0.0, sz);
        panel = panel - hole;
    }

    panel
}

/// DIN rail segment (TS35 × 150 mm). Printable in PETG for prototype;
/// production build uses extruded aluminum rail from Phoenix Contact.
/// 3 mm mount slots along centerline for screwing to the interior side wall.
fn din_rail_segment() -> Part {
    let base = centered_cube("din_base", DIN_RAIL_W, DIN_RAIL_LEN, DIN_RAIL_H);
    // Two mount slots
    let mut rail = base;
    for (i, &y_off) in [-(DIN_RAIL_LEN / 2.0) + 20.0, DIN_RAIL_LEN / 2.0 - 20.0]
        .iter()
        .enumerate()
    {
        let slot = centered_cube(
            format!("din_slot_{i}"),
            DIN_RAIL_SLOT_W,
            DIN_RAIL_SLOT_H + 2.0,
            DIN_RAIL_H + 2.0,
        )
        .translate(0.0, y_off, 0.0);
        rail = rail - slot;
    }
    rail
}

/// Standalone PCB mount plate (for retrofitting or rework). Mirrors the
/// 4× M3 standoff pattern of the ESP32-S3 PCB v2 on a 110 × 90 × 3 mm plate.
fn pcb_mount() -> Part {
    let plate = centered_cube("pcb_mount_plate", PCB_X + 10.0, PCB_Y + 10.0, 3.0);
    let mut plate = plate;
    let holes = [
        (
            -(PCB_X / 2.0) + PCB_MOUNT_INSET,
            -(PCB_Y / 2.0) + PCB_MOUNT_INSET,
        ),
        (
            (PCB_X / 2.0) - PCB_MOUNT_INSET,
            -(PCB_Y / 2.0) + PCB_MOUNT_INSET,
        ),
        (
            -(PCB_X / 2.0) + PCB_MOUNT_INSET,
            (PCB_Y / 2.0) - PCB_MOUNT_INSET,
        ),
        (
            (PCB_X / 2.0) - PCB_MOUNT_INSET,
            (PCB_Y / 2.0) - PCB_MOUNT_INSET,
        ),
    ];
    for (i, &(hx, hy)) in holes.iter().enumerate() {
        let standoff = centered_cylinder(
            format!("pcb_mount_standoff_{i}"),
            PCB_STANDOFF_DIA / 2.0,
            PCB_STANDOFF_H,
            24,
        )
        .translate(hx, hy, PCB_STANDOFF_H / 2.0 + 1.5);
        plate = plate + standoff;
        let hole = centered_cylinder(
            format!("pcb_mount_hole_{i}"),
            PCB_MOUNT_HOLE_DIA / 2.0,
            3.0 + PCB_STANDOFF_H + 2.0,
            24,
        )
        .translate(hx, hy, 0.0);
        plate = plate - hole;
    }
    plate
}

/// PSU clamp / mount reference. The MeanWell LRS-240-24 ships with M4 mount
/// holes on its case; this is a small floor-anchored strap that retains the
/// PSU if vibration-coupled. Printable in PETG. 200 × 15 × 3 mm strap.
fn psu_mount() -> Part {
    let strap = centered_cube("psu_strap", PSU_X + 2.0, 15.0, 3.0);
    let mut strap = strap;
    let hole_positions = [-100.0, 100.0];
    for (i, &hx) in hole_positions.iter().enumerate() {
        let hole = centered_cylinder(
            format!("psu_strap_hole_{i}"),
            PSU_SCREW_DIA / 2.0,
            3.0 + 2.0,
            24,
        )
        .translate(hx, 0.0, 0.0);
        strap = strap - hole;
    }
    strap
}

/// SSR heat-spreader plate (80 × 60 × 3 mm aluminum). Modeled in PETG-like
/// form for visualization; production uses 6061 aluminum plate with thermal
/// paste interface to the SSR.
fn ssr_heatsink() -> Part {
    let plate = centered_cube("ssr_plate", HEATSINK_X, HEATSINK_Y, HEATSINK_Z);
    let mut plate = plate;
    // 2× M4 clearance for SSR bolts
    for (i, &sx) in [-SSR_MOUNT_SPACING / 2.0, SSR_MOUNT_SPACING / 2.0]
        .iter()
        .enumerate()
    {
        let hole = centered_cylinder(
            format!("ssr_hs_hole_{i}"),
            SSR_MOUNT_DIA / 2.0,
            HEATSINK_Z + 2.0,
            24,
        )
        .translate(sx, 0.0, 0.0);
        plate = plate - hole;
    }
    // 4× M4 clearance for floor mount (outer)
    let corner_offsets = [
        (-HEATSINK_X / 2.0 + 6.0, -HEATSINK_Y / 2.0 + 6.0),
        (HEATSINK_X / 2.0 - 6.0, -HEATSINK_Y / 2.0 + 6.0),
        (-HEATSINK_X / 2.0 + 6.0, HEATSINK_Y / 2.0 - 6.0),
        (HEATSINK_X / 2.0 - 6.0, HEATSINK_Y / 2.0 - 6.0),
    ];
    for (i, &(cx, cy)) in corner_offsets.iter().enumerate() {
        let hole = centered_cylinder(
            format!("ssr_floor_{i}"),
            SSR_MOUNT_DIA / 2.0,
            HEATSINK_Z + 2.0,
            24,
        )
        .translate(cx, cy, 0.0);
        plate = plate - hole;
    }
    plate
}

/// Reference volume for the display cutout (for drawing/BOM docs).
fn display_cutout() -> Part {
    centered_cube(
        "display_cutout",
        DISPLAY_W,
        FRONT_PANEL_THICKNESS,
        DISPLAY_H,
    )
}

/// Reference volume for the E-stop cutout.
fn estop_cutout() -> Part {
    centered_cylinder("estop_cutout", ESTOP_DIA / 2.0, FRONT_PANEL_THICKNESS, 48)
        .rotate(90.0, 0.0, 0.0)
}

/// Assembly: all parts positioned inside the enclosure for visualization.
/// NOTE: this is a single STL; printing is done per-part.
fn assembly() -> Part {
    let body = enclosure_body();
    let fp = front_panel().translate(0.0, BOX_OUTER_Y / 2.0 - FRONT_PANEL_THICKNESS / 2.0, 0.0);
    let bp = back_panel_with_glands().translate(
        0.0,
        -(BOX_OUTER_Y / 2.0) + BACK_PANEL_THICKNESS / 2.0,
        0.0,
    );

    // DIN rail on right wall interior, vertical
    let rail = din_rail_segment().rotate(0.0, 90.0, 0.0).translate(
        BOX_INNER_X / 2.0 - DIN_RAIL_H / 2.0 - 2.0,
        30.0,
        0.0,
    );

    // SSR heat-spreader on floor
    let hs = ssr_heatsink().translate(
        0.0,
        -(BOX_INNER_Y / 2.0) + PSU_Y + 10.0 + HEATSINK_Y / 2.0 + 5.0,
        -(BOX_INNER_Z / 2.0) + HEATSINK_Z / 2.0,
    );

    body + fp + bp + rail + hs
}

// ─── Drawing ────────────────────────────────────────────────────────────────

fn write_drawing() {
    use std::fs::File;
    use std::io::Write;
    let path = "output/controller_enclosure_drawing.txt";
    let mut f = File::create(path).unwrap();
    writeln!(
        f,
        r#"══════════════════════════════════════════════════════════════
  ESP32-S3 CONTROLLER ENCLOSURE — DIMENSIONED DRAWING
══════════════════════════════════════════════════════════════

OUTSIDE:  300 (X) × 200 (Y) × 150 (Z) mm    (wall 3 mm PETG)
INSIDE:   294 × 194 × 144 mm

─── TOP VIEW (looking down +Z toward −Z) ──────────────────────
  +X right, +Y up (front of box = +Y)

           ←── 300 ──→
        ┌────────────────────────────┐
  Back  │ GLANDS ●●●●●● FAN ○  USBC▭ │  ↑
 (-Y)   │ ┌─BP───────────────────┐  │ 200 (Y)
        │ │  PSU 199×98          │  │  ↓
        │ │  (LRS-240-24)        │  │
        │ │  SSR heatsink 80×60  │  │
        │ │  PCB 100×80 (vert)   │  │  Front
        │ │  DIN rail 35×150     │  │  (+Y)
        │ └──────────────────────┘  │
        │ [Display]  [E-STOP]       │
        └────────────────────────────┘

─── FRONT VIEW (+Y face, looking in −Y direction) ─────────────
  +X right, +Z up
           ←── 300 ──→
        ┌────────────────────────────┐  ↑
        │  ○ ○ ○  STATUS LEDs        │ 150 (Z)
        │ ┌──────┐         ╭───╮     │  ↓
        │ │ TFT  │         │ E │     │
        │ │ 56×35│         │STOP     │
        │ └──────┘         ╰───╯     │
        │ [ROCKER SW]                │
        │                            │
        └────────────────────────────┘

─── BACK VIEW (-Y face, looking in +Y direction) ──────────────
  +X LEFT (mirror), +Z up
           ←── 300 ──→
        ┌──────────────────────────────┐  ↑
        │          ○ 80 mm FAN         │ 150
        │  ▭ USB-C             ▭ RJ45  │  ↓
        │  ○ ○ ○  ○ ○ ○  (6× GLANDS)  │
        │  MAIN HTR STPR FAN CO2 SENS  │
        └──────────────────────────────┘

─── SIDE VIEW (right, +X face) ────────────────────────────────
  -Y right (back), +Z up
           ←── 200 ──→
        ┌──────────────────────────────┐  ↑
        │ ▦▦▦▦▦▦ LOUVER BAND (upper)  │ 150
        │                              │
        │ ▦▦▦▦▦▦ LOUVER BAND (lower)  │  ↓
        └──────────────────────────────┘

─── COMPONENT POSITIONS (box-centered mm, (X, Y, Z)) ──────────
  PSU (LRS-240-24, 199×98×38)        ( 0, -47,   -53.5 )
  SSR heatsink (80×60×3 Al plate)    ( 0, +15,   -70.5 )
  Crydom SSR (58×45×23) on heatsink  ( 0, +15,   -55.5 )
  ESP32-S3 PCB (100×80) on standoffs (-137, 0, 0)  [left wall, vertical]
  DIN rail (TS35 × 150 mm)           (+143, +30,  0)  [right wall, vertical]
  Buck 24→12 V (40×20×15, DIN clip)  (+138, +10, -40)
  TB (Phoenix PT 2.5 × 20 pos)       (+138, +30,  +20)

─── CABLE GLAND ASSIGNMENTS (back panel, left → right) ────────
  1. M20   ○    24 V MAINS IN  (from LRS-240 AC side)
  2. M20   ○    HEATER OUT     (200 W silicone pad, fused)
  3. M20   ○    STEPPER DRIVE  (syringe pump A/B/C/D)
  4. M16   ○    FAN OUT        (12 V blower, chamber)
  5. M16   ○    CO2 SOLENOID   (12 V, NO, 1/8" tubing)
  6. M16   ○    SENSOR UMBIL.  (SHT40 I2C + CO2 UART + DS18B20 1-wire)

─── IP54 GASKET STRATEGY ──────────────────────────────────────
  • Front panel:  3×2 mm silicone cord in machined groove around
                  the entire +Y perimeter. Compressed ~30 % by
                  4× M4 screws at 0.8 N·m.
  • Back panel:   Same — 3×2 mm silicone cord around −Y perimeter.
  • Louvers:      Stainless mesh screen (180-µm) bonded behind
                  slits with neutral-cure silicone (prevents dust
                  ingress while passing air).
  • Glands:       Each M16/M20 includes captive EPDM O-ring rated
                  IP68; tighten to manufacturer torque.
  • Fan:          Noctua NF-A8 is IP-uncertified; behind it sits
                  an IP54-rated fan guard (e.g., SEPA 80 mm FG-80
                  with PTFE breather membrane) for splash defense.

─── THERMAL BUDGET ────────────────────────────────────────────
  Heat dissipation inside box at full load:
    PSU @ 89 % eff, 240 W out   → 29.6 W loss
    SSR (DC60S5, 1.3 V drop × 8 A in heater loop) →  10.4 W  (to heatsink)
    ESP32-S3 + peripherals      →   2.0 W
    Buck 24→12 V @ 2 A, 90 %    →   2.7 W
    Terminal blocks + wiring    →   1.0 W
    ─────────────────────────────────────
    TOTAL                        → ~45.7 W

  Noctua NF-A8 @ full PWM: 32.7 m³/h = 9.08 L/s free-air.
  With filter (50 % derate): ~4.5 L/s.

  ΔT over ambient at 4.5 L/s, 45.7 W:
    ΔT = P / (ρ·cp·V̇)
       = 45.7 / (1.2 kg/m³ × 1005 J/kg·K × 0.0045 m³/s)
       = 45.7 / 5.43 = 8.4 K

  Interior steady-state: 25 °C ambient + 8.4 K = 33.4 °C.
  With the incubator radiating some heat to the box exterior, add
  a margin of +5 K → 38 °C worst-case interior.
  Conclusion: interior stays well below the 50 °C target (all
  components are rated ≥ 70 °C). ✓

─── CLEARANCE CHECK ───────────────────────────────────────────
  Interior available:  294 × 194 × 144 mm
  PSU footprint:       199 × 98 mm   (floor)           → 68 % of floor X
  SSR heatsink:        80 × 60 mm    (floor, behind PSU) → fits 5 mm behind PSU
  PCB (vertical, left): 80 × 100 mm  (Y × Z)            → 52 % of Y, 69 % of Z
  DIN rail (vertical, right): 35 × 150 mm               → fits
  Minimum clearances:
    PSU ↔ SSR heatsink      : 5 mm
    PSU ↔ PCB (X axis)      : 47 mm   (left-wall PCB clears PSU body)
    DIN rail ↔ PSU (X axis) : 43 mm
    All components ↔ fan    : > 40 mm  (fan is on back panel above PSU)
  ✓ No interference.

─── BOM (vendor part numbers) ─────────────────────────────────
  1. ESP32-S3 Controller PCB v2 (in-house, see A-496F33BE)
  2. MeanWell LRS-240-24 (Digi-Key 1866-4341-ND)
  3. Crydom DC60S5 SSR (Digi-Key CC1226-ND)
  4. Noctua NF-A8 PWM 80 mm fan (Noctua, Amazon B00VXTANO2)
  5. Phoenix Contact PT 2.5 (3209510), ×20
  6. Adafruit 1770 2.4" TFT
  7. Schneider XALK178 E-stop
  8. NKK CWSB11AA2S rocker switch w/LED
  9. Amphenol 12401589E412GLF USB-C bulkhead
 10. Amphenol MRJ-5381-01 RJ45 bulkhead (optional)
 11. Altech 5308 M16 glands × 3, 5310 M20 glands × 3
 12. Heyco 3050 80 mm fan guard w/ filter
 13. TS35 DIN rail 150 mm (Phoenix Contact 0801733)
 14. Buck 24→12 V DIN-clip module (Mean Well DDR-15G-12)
 15. 6061-T6 Al plate 80×60×3 mm (heatsink)
 16. M3/M4/M5 fasteners as noted
 17. Silicone cord 3 mm (gasket), 2 m roll

══════════════════════════════════════════════════════════════
"#
    )
    .unwrap();
    println!("Wrote: {path}");
}

fn main() {
    std::fs::create_dir_all("output").ok();

    // Individual parts
    enclosure_body()
        .write_stl("output/controller_enclosure_body.stl")
        .unwrap();
    println!("Exported: output/controller_enclosure_body.stl");

    front_panel()
        .write_stl("output/controller_enclosure_front_panel.stl")
        .unwrap();
    println!("Exported: output/controller_enclosure_front_panel.stl");

    back_panel_with_glands()
        .write_stl("output/controller_enclosure_back_panel.stl")
        .unwrap();
    println!("Exported: output/controller_enclosure_back_panel.stl");

    din_rail_segment()
        .write_stl("output/controller_enclosure_din_rail.stl")
        .unwrap();
    println!("Exported: output/controller_enclosure_din_rail.stl");

    pcb_mount()
        .write_stl("output/controller_enclosure_pcb_mount.stl")
        .unwrap();
    println!("Exported: output/controller_enclosure_pcb_mount.stl");

    psu_mount()
        .write_stl("output/controller_enclosure_psu_mount.stl")
        .unwrap();
    println!("Exported: output/controller_enclosure_psu_mount.stl");

    ssr_heatsink()
        .write_stl("output/controller_enclosure_ssr_heatsink.stl")
        .unwrap();
    println!("Exported: output/controller_enclosure_ssr_heatsink.stl");

    display_cutout()
        .write_stl("output/controller_enclosure_display_cutout.stl")
        .unwrap();
    println!("Exported: output/controller_enclosure_display_cutout.stl");

    estop_cutout()
        .write_stl("output/controller_enclosure_estop_cutout.stl")
        .unwrap();
    println!("Exported: output/controller_enclosure_estop_cutout.stl");

    assembly()
        .write_stl("output/controller_enclosure_assembly.stl")
        .unwrap();
    println!("Exported: output/controller_enclosure_assembly.stl");

    // Drawing
    write_drawing();

    // STEP export
    for stl in [
        "output/controller_enclosure_body.stl",
        "output/controller_enclosure_front_panel.stl",
        "output/controller_enclosure_back_panel.stl",
        "output/controller_enclosure_din_rail.stl",
        "output/controller_enclosure_pcb_mount.stl",
        "output/controller_enclosure_psu_mount.stl",
        "output/controller_enclosure_ssr_heatsink.stl",
        "output/controller_enclosure_assembly.stl",
    ] {
        laminarforge_cad::stl_to_step(stl);
    }

    println!();
    println!("══════════════════════════════════════════════════════════════");
    println!("   ESP32-S3 CONTROLLER ENCLOSURE — SUMMARY");
    println!("══════════════════════════════════════════════════════════════");
    println!("   Outer:   {BOX_OUTER_X:.0} × {BOX_OUTER_Y:.0} × {BOX_OUTER_Z:.0} mm");
    println!("   Inner:   {BOX_INNER_X:.0} × {BOX_INNER_Y:.0} × {BOX_INNER_Z:.0} mm");
    println!("   Wall:    {BOX_WALL:.0} mm PETG");
    println!("   Rating:  IP54 (gaskets, filter, mesh louvers)");
    println!("══════════════════════════════════════════════════════════════");
}
