use vcad::{centered_cube, centered_cylinder, Part};

// ─── Media Reservoir & Auto-Feeder for 100-Chip LaminarForge Rev C Stack ───
//
// Pumped auto-feeder that refills media in-place so the incubator door
// stays closed between 48h media changes. Sized for the chip_farm_assembly:
//   - 100 chips × 5 mL/chip × 48h media change = 500 mL/cycle
//   - 7-day uninterrupted reserve (3 cycles + overhead) = 1.8 L
//   - Expansion / priming / dead-leg / safety margin → 8 L PETG reservoir
//
// Topology: 5 pump channels (one per shelf of 20 chips). Each channel
// drips over the rack's front-to-back gutter; rocker agitation + shelf
// tilt distribute media into the 20 chips on that shelf through their
// open-top wells.
//
// Coordinate system: reservoir body centered at origin.
//   X: left/right    (reservoir width)
//   Y: front/back    (reservoir depth)
//   Z: bottom/top    (reservoir height; +Z up)
//
// Exports:
//   - media_reservoir_body.stl         (8 L PETG reservoir, autoclavable lid)
//   - media_reservoir_lid.stl          (screw-on lid w/ HEPA vent, fill, dip-tube, sensor)
//   - media_reservoir_pump_manifold.stl (5-channel Kamoer KCS-B peristaltic mount)
//   - media_reservoir_passthrough.stl  (gasketed thermal-container wall feedthrough)
//   - media_reservoir_drip_head.stl    (per-shelf manifold drip head, ×5)
//   - media_reservoir_drip_tray.stl    (shallow overflow tray beneath bottom shelf)
//   - media_reservoir_waste_bottle.stl (4 L external waste bottle w/ gravity drain)
//   - media_reservoir_assembly.stl     (all parts positioned for visualization)

fn main() {
    // ══════════════════════════════════════════════════════════════
    // 0. GLOBAL PARAMETERS
    // ══════════════════════════════════════════════════════════════

    // ── Reservoir (8 L usable — cylinder with flat ends) ──
    // V = π·r²·h. With OD 200mm → ID ~194mm (r=97), h_interior=270mm
    // V = π × 97² × 270 = 7.98 L (≈ 8 L).
    let reservoir_od: f64 = 200.0;
    let reservoir_wall: f64 = 3.0;
    let reservoir_id: f64 = reservoir_od - reservoir_wall * 2.0; // 194mm
    let reservoir_interior_h: f64 = 270.0;
    let reservoir_floor_thickness: f64 = 4.0; // thicker bottom for stability
    let reservoir_outer_h: f64 = reservoir_interior_h + reservoir_floor_thickness; // 274mm

    // Reservoir is open-top; the lid screws on via a threaded collar.
    let lid_collar_height: f64 = 20.0;
    let lid_thread_pitch: f64 = 4.0; // nominal, buttress-style 3D-printed thread
    let _ = lid_thread_pitch; // visual reference only

    // ── Lid ──
    let lid_od: f64 = reservoir_od + 8.0; // lid skirt overhangs reservoir by 4mm per side
    let lid_thickness: f64 = 8.0;
    let _lid_id: f64 = reservoir_id; // seals against reservoir ID via silicone O-ring

    // Lid penetrations:
    //   A. HEPA-vented fill port (luer-lock), 22mm boss on top
    //   B. Dip tube passthrough (10mm hole, fits 3/8" OD rigid PP dip tube)
    //   C. Low-level optical sensor port (M12×1 thread = 12.5mm hole)
    //   D. Quick-disconnect CPC AseptiQuik bulkhead port (25mm hole)
    //   E. Lift handle eyelet (8mm hole × 2)
    let fill_port_od: f64 = 22.0;
    let fill_port_id: f64 = 10.0;
    let dip_tube_od: f64 = 10.0;
    let sensor_port_od: f64 = 14.0; // M12 clearance + hex nut seat
    let aseptiquik_port_od: f64 = 25.0;
    let lift_eyelet_od: f64 = 8.0;

    // ── Peristaltic pump manifold (Kamoer KCS-B, 5 channels) ──
    // Kamoer KCS-B datasheet: body 76 × 60 × 45 mm, NEMA17 driven,
    // shaft boss Ø23, 4× M3 mounting holes on 38 mm square pattern,
    // inlet/outlet spigots 6 mm OD nylon barbs on ±22 mm from head center.
    // For 5 channels we stack 5 heads side-by-side on a common 42 mm
    // NEMA17 shaft (daisied) OR run 5 independent KCS-B pumps. The
    // independent option wins for failure isolation: one pump jam can't
    // take down the farm. Use 5× independent pump slots on a plate.
    let pump_channels: usize = 5;
    let pump_body_x: f64 = 76.0;
    let pump_body_y: f64 = 60.0;
    let pump_body_z: f64 = 45.0;
    let _pump_boss_dia: f64 = 23.0;
    let pump_mount_hole_dia: f64 = 3.2;
    let pump_mount_pattern: f64 = 38.0; // square bolt pattern
    let pump_barb_offset_y: f64 = 22.0; // inlet/outlet Y offset from head center
    let pump_pitch: f64 = 90.0; // center-to-center spacing between pumps (×5)

    // NEMA17 motor passthrough in plate (boss clearance)
    let nema17_boss_clr: f64 = 25.0; // 23 + 2 clearance

    // Pump manifold plate dimensions
    let manifold_plate_x: f64 = (pump_channels as f64) * pump_pitch + 40.0; // 490mm
    let manifold_plate_y: f64 = 160.0; // holds pump body (60) + barb routing clearance
    let manifold_plate_thickness: f64 = 8.0; // 8 mm PETG, stiff enough to carry 5 NEMA17s

    // ── Silicone tubing (platinum-cured, ID 1.6mm, OD 4.8mm) ──
    let tube_id: f64 = 1.6;
    let tube_od: f64 = 4.8;
    let _ = tube_id;

    // ── Thermal-container wall pass-through ──
    // Incubator wall cross-section: 3mm inner PETG + 25mm foam + 3mm outer PETG
    // + side-wall mount spacing. Use a single bulkhead gland 50 mm OD
    // with a 5-hole silicone grommet insert (one hole per channel).
    let passthrough_od: f64 = 60.0; // outer flange OD
    let passthrough_boss_od: f64 = 40.0; // threaded boss OD (goes through wall)
    let passthrough_boss_length: f64 = 45.0; // 3 + 25 + 3 + 14 backing nut engage
    let passthrough_center_bore: f64 = 28.0; // fits 5-hole grommet
    let passthrough_grommet_holes: f64 = 5.0; // 5 tube channels through grommet
    let _ = passthrough_grommet_holes;

    // Grommet tube hole (tube OD + 0.3mm interference)
    let grommet_hole_dia: f64 = tube_od + 0.3; // 5.1mm

    // ── Drip head (per-shelf manifold that sits above a shelf's 20-chip row) ──
    // Takes one incoming tube (4.8 mm OD) and splits to 4 drip nozzles that
    // span the 4 columns of chips on that shelf. Nozzle length kept short
    // so the rocker's ±15° tilt doesn't swing the head into adjacent shelves.
    let drip_head_x: f64 = 540.0; // spans all 4 columns of 127.76mm chips
    let drip_head_y: f64 = 25.0;
    let drip_head_z: f64 = 12.0;
    let drip_nozzles: usize = 4;
    let drip_nozzle_od: f64 = 3.0;
    let drip_nozzle_length: f64 = 10.0;
    let drip_nozzle_bore: f64 = 1.2;
    let drip_nozzle_pitch: f64 = 127.76 + 5.0; // one per column
    let drip_inlet_od: f64 = 6.0; // tube barb fitting OD

    // ── Drip tray (under bottom shelf; gravity-drained to waste bottle) ──
    let tray_x: f64 = 545.0; // fits inside incubator rack footprint
    let tray_y: f64 = 465.0;
    let tray_wall: f64 = 2.5;
    let tray_outer_h: f64 = 15.0; // shallow — 1 mm floor slope to the drain
    let tray_drain_od: f64 = 10.0;

    // ── Waste bottle (external, 4 L HDPE analog) ──
    let waste_od: f64 = 180.0;
    let waste_wall: f64 = 2.5;
    let waste_h: f64 = 190.0;
    let waste_neck_od: f64 = 40.0;
    let waste_neck_h: f64 = 15.0;

    // ══════════════════════════════════════════════════════════════
    // 1. RESERVOIR BODY (cylindrical, open-top, threaded lid interface)
    // ══════════════════════════════════════════════════════════════

    let outer = centered_cylinder("res_outer", reservoir_od / 2.0, reservoir_outer_h, 96);
    let inner = centered_cylinder(
        "res_inner",
        reservoir_id / 2.0,
        reservoir_interior_h + 0.2,
        96,
    )
    .translate(0.0, 0.0, reservoir_floor_thickness / 2.0 + 0.1);

    // Stiffening rings — 2× external raised bands for hand grip / stability
    let band1 = (centered_cylinder("band1_o", reservoir_od / 2.0 + 3.0, 6.0, 96)
        - centered_cylinder("band1_i", reservoir_od / 2.0, 6.2, 96))
    .translate(0.0, 0.0, -reservoir_outer_h / 2.0 + 30.0);
    let band2 = (centered_cylinder("band2_o", reservoir_od / 2.0 + 3.0, 6.0, 96)
        - centered_cylinder("band2_i", reservoir_od / 2.0, 6.2, 96))
    .translate(0.0, 0.0, reservoir_outer_h / 2.0 - 40.0);

    // Lid-seat shoulder (top rim, slightly recessed 2 mm to seat lid O-ring)
    let rim_relief = (centered_cylinder("rim_o", reservoir_od / 2.0 + 0.1, 3.0, 96)
        - centered_cylinder("rim_i", reservoir_id / 2.0 - 2.0, 3.2, 96))
    .translate(0.0, 0.0, reservoir_outer_h / 2.0 - 1.5);

    // Collar flange where lid skirt grips (knurl proxy — 12 shallow pockets)
    let mut knurl = Part::empty("knurl");
    for i in 0..12 {
        let theta = (i as f64) * std::f64::consts::PI / 6.0;
        let kx = (reservoir_od / 2.0 + 2.0) * theta.cos();
        let ky = (reservoir_od / 2.0 + 2.0) * theta.sin();
        let pocket = centered_cube(&format!("kn{i}"), 4.0, 3.0, lid_collar_height).translate(
            kx,
            ky,
            reservoir_outer_h / 2.0 - lid_collar_height / 2.0,
        );
        knurl = knurl + pocket;
    }

    let reservoir = (outer - inner + band1 + band2 - rim_relief) - knurl;
    reservoir
        .write_stl("output/media_reservoir_body.stl")
        .unwrap();
    println!("Exported: output/media_reservoir_body.stl");

    // ══════════════════════════════════════════════════════════════
    // 2. LID (screw-on, with HEPA vent + fill + dip-tube + sensor + QD)
    // ══════════════════════════════════════════════════════════════

    let lid_body = centered_cylinder("lid_body", lid_od / 2.0, lid_thickness, 96);

    // Skirt (hangs down around reservoir top, 15 mm tall)
    let skirt_h: f64 = 15.0;
    let skirt_outer = centered_cylinder("skirt_o", lid_od / 2.0, skirt_h, 96).translate(
        0.0,
        0.0,
        -(lid_thickness / 2.0) - skirt_h / 2.0,
    );
    let skirt_inner = centered_cylinder("skirt_i", (reservoir_od + 0.4) / 2.0, skirt_h + 0.2, 96)
        .translate(0.0, 0.0, -(lid_thickness / 2.0) - skirt_h / 2.0);

    // O-ring groove (on lid underside, seats onto reservoir ID rim)
    let oring_outer = centered_cylinder("or_o", (reservoir_id / 2.0) - 1.0, 2.2, 96).translate(
        0.0,
        0.0,
        -(lid_thickness / 2.0) + 1.1,
    );
    let oring_inner = centered_cylinder("or_i", (reservoir_id / 2.0) - 3.0, 2.4, 96).translate(
        0.0,
        0.0,
        -(lid_thickness / 2.0) + 1.1,
    );
    let oring_groove = oring_outer - oring_inner;

    // Port A: HEPA-vented fill port — 22mm boss, 10mm bore, luer-lock compatible
    let fill_x = 0.0;
    let fill_y = lid_od / 2.0 - 25.0; // toward one edge
    let fill_boss = centered_cylinder("fill_boss", fill_port_od / 2.0, 15.0, 48).translate(
        fill_x,
        fill_y,
        lid_thickness / 2.0 + 7.5,
    );
    let fill_bore = centered_cylinder("fill_bore", fill_port_id / 2.0, lid_thickness + 20.0, 48)
        .translate(fill_x, fill_y, 0.0);
    // Luer lip (6mm ID step for luer-lock taper)
    let luer_lip = centered_cylinder("luer_lip", 6.0 / 2.0, 6.0, 32).translate(
        fill_x,
        fill_y,
        lid_thickness / 2.0 + 15.0 + 3.0,
    );

    // Port B: Dip tube passthrough (center)
    let dip_bore = centered_cylinder("dip_bore", dip_tube_od / 2.0, lid_thickness + 2.0, 48);
    // Dip tube collar (prevents slide-through)
    let dip_collar = (centered_cylinder("dipc_o", 16.0 / 2.0, 6.0, 48)
        - centered_cylinder("dipc_i", dip_tube_od / 2.0, 6.2, 48))
    .translate(0.0, 0.0, lid_thickness / 2.0 + 3.0);

    // Port C: Low-level sensor port (M12 clearance, near opposite of fill port)
    let sensor_x = 0.0;
    let sensor_y = -(lid_od / 2.0 - 25.0);
    let sensor_bore = centered_cylinder("sens_bore", sensor_port_od / 2.0, lid_thickness + 2.0, 48)
        .translate(sensor_x, sensor_y, 0.0);
    let sensor_boss = (centered_cylinder("sens_boss_o", 22.0 / 2.0, 10.0, 48)
        - centered_cylinder("sens_boss_i", sensor_port_od / 2.0, 10.2, 48))
    .translate(sensor_x, sensor_y, lid_thickness / 2.0 + 5.0);

    // Port D: AseptiQuik QD bulkhead (25mm bore, with 40mm flange boss)
    let qd_x = lid_od / 2.0 - 35.0;
    let qd_y = 0.0;
    let qd_bore = centered_cylinder(
        "qd_bore",
        aseptiquik_port_od / 2.0,
        lid_thickness + 20.0,
        48,
    )
    .translate(qd_x, qd_y, 0.0);
    let qd_boss = (centered_cylinder("qd_boss_o", 40.0 / 2.0, 12.0, 48)
        - centered_cylinder("qd_boss_i", aseptiquik_port_od / 2.0, 12.2, 48))
    .translate(qd_x, qd_y, lid_thickness / 2.0 + 6.0);

    // Port E: Lift eyelets (×2, on either side of fill port axis)
    let lift1 = centered_cylinder("lift1", lift_eyelet_od / 2.0, lid_thickness + 2.0, 24)
        .translate(-(lid_od / 2.0) + 15.0, 0.0, 0.0);
    let lift2 = centered_cylinder("lift2", lift_eyelet_od / 2.0, lid_thickness + 2.0, 24)
        .translate((lid_od / 2.0) - 15.0, 0.0, 0.0);

    let lid = lid_body + skirt_outer - skirt_inner - oring_groove + fill_boss - fill_bore
        + luer_lip
        - dip_bore
        + dip_collar
        - sensor_bore
        + sensor_boss
        - qd_bore
        + qd_boss
        - lift1
        - lift2;
    lid.write_stl("output/media_reservoir_lid.stl").unwrap();
    println!("Exported: output/media_reservoir_lid.stl");

    // ══════════════════════════════════════════════════════════════
    // 3. PUMP MANIFOLD PLATE (holds 5 Kamoer KCS-B pumps)
    // ══════════════════════════════════════════════════════════════

    let mut plate = centered_cube(
        "pump_plate",
        manifold_plate_x,
        manifold_plate_y,
        manifold_plate_thickness,
    );

    let pump_x_start = -(manifold_plate_x / 2.0) + 40.0; // 20mm edge + 20mm to first pump center
    let _ = pump_body_x;
    let _ = pump_body_y;
    let _ = pump_body_z;

    let mut pump_cutouts = Part::empty("pump_cutouts");
    for i in 0..pump_channels {
        let cx = pump_x_start + (i as f64) * pump_pitch;

        // NEMA17 shaft/boss passthrough in plate
        let boss = centered_cylinder(
            &format!("boss_{i}"),
            nema17_boss_clr / 2.0,
            manifold_plate_thickness + 2.0,
            48,
        )
        .translate(cx, 0.0, 0.0);
        pump_cutouts = pump_cutouts + boss;

        // 4× M3 pump mounting holes on 38 mm square pattern (coplanar with
        // the pump's front face, centered on the shaft)
        for dx in [-0.5f64, 0.5] {
            for dy in [-0.5f64, 0.5] {
                let mx = cx + dx * pump_mount_pattern;
                let my = dy * pump_mount_pattern;
                let m3 = centered_cylinder(
                    &format!("m3_{i}_{}_{}", (dx * 10.0) as i32, (dy * 10.0) as i32),
                    pump_mount_hole_dia / 2.0,
                    manifold_plate_thickness + 2.0,
                    16,
                )
                .translate(mx, my, 0.0);
                pump_cutouts = pump_cutouts + m3;
            }
        }

        // Barb routing clearance slots — inlet toward +Y, outlet toward -Y
        // (routing tubes flow out the -Y edge toward the container wall)
        for bd in [-1.0f64, 1.0] {
            let barb = centered_cylinder(
                &format!("barb_{i}_{}", (bd * 10.0) as i32),
                5.0,
                manifold_plate_thickness + 2.0,
                16,
            )
            .translate(cx, bd * pump_barb_offset_y, 0.0);
            pump_cutouts = pump_cutouts + barb;
        }
    }

    // Channel ID labels (engraved front edge)
    for i in 0..pump_channels {
        let cx = pump_x_start + (i as f64) * pump_pitch;
        let label = centered_cube(&format!("lbl_{i}"), 20.0, 5.0, 1.0).translate(
            cx,
            -(manifold_plate_y / 2.0) + 8.0,
            manifold_plate_thickness / 2.0 - 0.4,
        );
        pump_cutouts = pump_cutouts + label;
    }

    // Baseplate mounting — 4 corner M5 holes for 20×20 extrusion T-nuts
    let mut base_mounts = Part::empty("base_mounts");
    for (bx, by) in [
        (
            -(manifold_plate_x / 2.0) + 15.0,
            -(manifold_plate_y / 2.0) + 15.0,
        ),
        (
            (manifold_plate_x / 2.0) - 15.0,
            -(manifold_plate_y / 2.0) + 15.0,
        ),
        (
            -(manifold_plate_x / 2.0) + 15.0,
            (manifold_plate_y / 2.0) - 15.0,
        ),
        (
            (manifold_plate_x / 2.0) - 15.0,
            (manifold_plate_y / 2.0) - 15.0,
        ),
    ] {
        let h = centered_cylinder("bm", 5.3 / 2.0, manifold_plate_thickness + 2.0, 24)
            .translate(bx, by, 0.0);
        base_mounts = base_mounts + h;
    }

    plate = plate - pump_cutouts - base_mounts;
    plate
        .write_stl("output/media_reservoir_pump_manifold.stl")
        .unwrap();
    println!("Exported: output/media_reservoir_pump_manifold.stl");

    // ══════════════════════════════════════════════════════════════
    // 4. WALL PASS-THROUGH (gasketed bulkhead, 5-hole silicone grommet)
    // ══════════════════════════════════════════════════════════════

    // Outer flange (inside container side)
    let inside_flange = centered_cylinder("pt_flange_in", passthrough_od / 2.0, 4.0, 64);
    // Threaded boss (through the wall)
    let boss = centered_cylinder(
        "pt_boss",
        passthrough_boss_od / 2.0,
        passthrough_boss_length,
        64,
    )
    .translate(0.0, 0.0, 4.0 / 2.0 + passthrough_boss_length / 2.0);
    // Outer flange (outside container side) — smaller, acts as backing nut face
    let outside_flange = centered_cylinder("pt_flange_out", 52.0 / 2.0, 5.0, 64).translate(
        0.0,
        0.0,
        4.0 / 2.0 + passthrough_boss_length + 5.0 / 2.0,
    );

    // Center bore (holds 5-hole grommet)
    let center_bore = centered_cylinder(
        "pt_center",
        passthrough_center_bore / 2.0,
        4.0 + passthrough_boss_length + 5.0 + 2.0,
        64,
    )
    .translate(0.0, 0.0, (4.0 + passthrough_boss_length + 5.0) / 2.0);

    // Mounting bolts through the inside flange (4× M4)
    let mut pt_bolts = Part::empty("pt_bolts");
    for i in 0..4 {
        let theta = (i as f64) * std::f64::consts::PI / 2.0 + std::f64::consts::PI / 4.0;
        let bx = ((passthrough_od - 10.0) / 2.0) * theta.cos();
        let by = ((passthrough_od - 10.0) / 2.0) * theta.sin();
        let bolt = centered_cylinder(&format!("ptb_{i}"), 4.2 / 2.0, 4.0 + 2.0, 16).translate(
            bx,
            by,
            4.0 / 2.0,
        );
        pt_bolts = pt_bolts + bolt;
    }

    let passthrough = inside_flange + boss + outside_flange - center_bore - pt_bolts;
    passthrough
        .write_stl("output/media_reservoir_passthrough.stl")
        .unwrap();
    println!("Exported: output/media_reservoir_passthrough.stl");

    // Grommet (separate print in TPU/silicone replacement) — 5 tube holes
    let grommet_od = passthrough_center_bore - 0.4; // press-fit
    let grommet_h = passthrough_boss_length;
    let mut grommet = centered_cylinder("grommet", grommet_od / 2.0, grommet_h, 48);
    // Central + 4 satellite holes (one per channel, ×5 total)
    let center_hole = centered_cylinder("gh_c", grommet_hole_dia / 2.0, grommet_h + 2.0, 24);
    grommet = grommet - center_hole;
    for i in 0..4 {
        let theta = (i as f64) * std::f64::consts::PI / 2.0;
        let hx = 8.0 * theta.cos();
        let hy = 8.0 * theta.sin();
        let h = centered_cylinder(
            &format!("gh_{i}"),
            grommet_hole_dia / 2.0,
            grommet_h + 2.0,
            24,
        )
        .translate(hx, hy, 0.0);
        grommet = grommet - h;
    }
    grommet
        .write_stl("output/media_reservoir_grommet.stl")
        .unwrap();
    println!("Exported: output/media_reservoir_grommet.stl");

    // ══════════════════════════════════════════════════════════════
    // 5. DRIP HEAD (×5 — one per shelf)
    // ══════════════════════════════════════════════════════════════

    let head_body = centered_cube("head_body", drip_head_x, drip_head_y, drip_head_z);

    // Internal distribution channel (hollow along X-axis inside the body)
    let dist_channel =
        centered_cube("dist_ch", drip_head_x - 20.0, 4.0, 4.0).translate(0.0, 0.0, 2.0);

    // Inlet barb (on +Y face center)
    let inlet_barb = centered_cylinder("inlet_barb", drip_inlet_od / 2.0, 12.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, drip_head_y / 2.0 + 6.0, 2.0);
    let inlet_bore = centered_cylinder("inlet_bore", 3.0 / 2.0, drip_head_y + 14.0, 24)
        .rotate(90.0, 0.0, 0.0)
        .translate(0.0, 0.0, 2.0);

    // 4 drip nozzles on underside, one per column
    let mut nozzles = Part::empty("nozzles");
    let mut nozzle_bores = Part::empty("nozzle_bores");
    let nozzle_x_start = -((drip_nozzles - 1) as f64) * drip_nozzle_pitch / 2.0;
    for i in 0..drip_nozzles {
        let nx = nozzle_x_start + (i as f64) * drip_nozzle_pitch;
        let nz = -(drip_head_z / 2.0) - drip_nozzle_length / 2.0;
        let body = centered_cylinder(
            &format!("noz_{i}"),
            drip_nozzle_od / 2.0,
            drip_nozzle_length,
            24,
        )
        .translate(nx, 0.0, nz);
        let bore = centered_cylinder(
            &format!("nozb_{i}"),
            drip_nozzle_bore / 2.0,
            drip_nozzle_length + drip_head_z,
            16,
        )
        .translate(nx, 0.0, nz + drip_nozzle_length / 2.0 - drip_head_z / 2.0);
        nozzles = nozzles + body;
        nozzle_bores = nozzle_bores + bore;
    }

    // Mounting feet — 2× M3 holes on each short end, for hanging from rack frame
    let mut feet_holes = Part::empty("feet_holes");
    for fx in [-(drip_head_x / 2.0) + 10.0, (drip_head_x / 2.0) - 10.0] {
        for fy in [-(drip_head_y / 2.0) + 6.0, (drip_head_y / 2.0) - 6.0] {
            let fh =
                centered_cylinder("fh", 3.2 / 2.0, drip_head_z + 2.0, 16).translate(fx, fy, 0.0);
            feet_holes = feet_holes + fh;
        }
    }

    let drip_head =
        head_body - dist_channel + inlet_barb - inlet_bore + nozzles - nozzle_bores - feet_holes;
    drip_head
        .write_stl("output/media_reservoir_drip_head.stl")
        .unwrap();
    println!("Exported: output/media_reservoir_drip_head.stl");

    // ══════════════════════════════════════════════════════════════
    // 6. DRIP TRAY (under bottom shelf, slopes to drain)
    // ══════════════════════════════════════════════════════════════

    let tray_outer_box = centered_cube("tray_outer", tray_x, tray_y, tray_outer_h);
    let tray_inner_box = centered_cube(
        "tray_inner",
        tray_x - tray_wall * 2.0,
        tray_y - tray_wall * 2.0,
        tray_outer_h - tray_wall + 0.5,
    )
    .translate(0.0, 0.0, tray_wall / 2.0 + 0.25);

    // Drain hole at back-left corner (matches incubator floor drain)
    let tray_drain = centered_cylinder("tdrain", tray_drain_od / 2.0, tray_outer_h + 2.0, 32)
        .translate(-(tray_x / 2.0) + 30.0, (tray_y / 2.0) - 30.0, 0.0);

    // Drain boss under the drain hole (barb for 9mm ID silicone drain tubing)
    let tray_drain_boss = (centered_cylinder("tdb_o", 14.0 / 2.0, 12.0, 32)
        - centered_cylinder("tdb_i", 9.0 / 2.0, 12.2, 32))
    .translate(
        -(tray_x / 2.0) + 30.0,
        (tray_y / 2.0) - 30.0,
        -(tray_outer_h / 2.0) - 6.0,
    );

    // Splash ridge along front edge so the rocker tilt doesn't slosh media out
    let splash_ridge = centered_cube("splash", tray_x - 20.0, 3.0, 10.0).translate(
        0.0,
        -(tray_y / 2.0) + 4.0,
        tray_outer_h / 2.0 + 5.0 - 0.5,
    );

    let drip_tray = tray_outer_box - tray_inner_box - tray_drain + tray_drain_boss + splash_ridge;
    drip_tray
        .write_stl("output/media_reservoir_drip_tray.stl")
        .unwrap();
    println!("Exported: output/media_reservoir_drip_tray.stl");

    // ══════════════════════════════════════════════════════════════
    // 7. WASTE BOTTLE (external, 4 L HDPE analog; printed PETG proto)
    // ══════════════════════════════════════════════════════════════

    let waste_body = centered_cylinder("waste_body", waste_od / 2.0, waste_h, 96);
    let waste_interior = centered_cylinder(
        "waste_int",
        (waste_od - waste_wall * 2.0) / 2.0,
        waste_h - waste_wall,
        96,
    )
    .translate(0.0, 0.0, waste_wall / 2.0);

    let waste_neck = centered_cylinder("waste_neck", waste_neck_od / 2.0, waste_neck_h, 48)
        .translate(0.0, 0.0, waste_h / 2.0 + waste_neck_h / 2.0);
    let waste_neck_bore = centered_cylinder(
        "waste_neckb",
        (waste_neck_od - 6.0) / 2.0,
        waste_neck_h + 2.0,
        48,
    )
    .translate(0.0, 0.0, waste_h / 2.0 + waste_neck_h / 2.0);

    // Drain-in barb (side, 15 mm from top rim, fits 9 mm ID tubing)
    let drain_in_barb = (centered_cylinder("din_o", 14.0 / 2.0, 18.0, 32)
        - centered_cylinder("din_i", 9.0 / 2.0, 20.0, 32))
    .rotate(0.0, 90.0, 0.0)
    .translate(waste_od / 2.0 + 9.0, 0.0, waste_h / 2.0 - 20.0);

    // Vent HEPA barb (back, top)
    let vent_barb = (centered_cylinder("vent_o", 10.0 / 2.0, 15.0, 24)
        - centered_cylinder("vent_i", 4.0 / 2.0, 17.0, 24))
    .translate(
        0.0,
        -(waste_od / 2.0) + 10.0,
        waste_h / 2.0 + waste_neck_h + 7.5,
    );

    let waste =
        waste_body - waste_interior + waste_neck - waste_neck_bore + drain_in_barb + vent_barb;
    waste
        .write_stl("output/media_reservoir_waste_bottle.stl")
        .unwrap();
    println!("Exported: output/media_reservoir_waste_bottle.stl");

    // ══════════════════════════════════════════════════════════════
    // 8. SPECS REPORT
    // ══════════════════════════════════════════════════════════════
    //
    // Assembly visualization is intentionally omitted — individual STLs
    // are positioned in their own coordinate systems. See
    // media_reservoir_drawing.txt for integration geometry relative to
    // the chip_farm_assembly envelope.

    let reservoir_volume_ml =
        std::f64::consts::PI * (reservoir_id / 2.0).powi(2) * reservoir_interior_h / 1000.0;
    let media_per_cycle_ml = 100.0 * 5.0; // 100 chips × 5 mL
    let cycles_per_fill = reservoir_volume_ml / media_per_cycle_ml;
    let days_per_fill = cycles_per_fill * 2.0; // 48h/cycle

    println!();
    println!("══════════════════════════════════════════════════════════════");
    println!("   MEDIA RESERVOIR & AUTO-FEEDER — SPECS");
    println!("══════════════════════════════════════════════════════════════");
    println!();
    println!(" RESERVOIR");
    println!(
        "   OD × ID × H:           {reservoir_od:.0} × {reservoir_id:.0} × {reservoir_interior_h:.0} mm"
    );
    println!(
        "   Usable volume:         {reservoir_volume_ml:.0} mL (~{:.1} L)",
        reservoir_volume_ml / 1000.0
    );
    println!("   Floor thickness:       {reservoir_floor_thickness:.1} mm");
    println!("   Wall thickness:        {reservoir_wall:.1} mm PETG (autoclave 121°C OK)");
    println!("   Lid seal:              Silicone O-ring, lid skirt over rim, 2×stiffener bands");
    println!();
    println!(" MEDIA BUDGET");
    println!("   Chips:                 100");
    println!("   Media per chip per 48h: 5 mL");
    println!("   Media per 48 h cycle:  {media_per_cycle_ml:.0} mL");
    println!(
        "   Cycles per full fill:  {cycles_per_fill:.1}  →  {days_per_fill:.1} days uninterrupted"
    );
    println!();
    println!(" LID PORTS");
    println!("   HEPA-vented fill:      Ø{fill_port_od:.0} mm boss, Ø{fill_port_id:.0} mm bore, luer-lock taper (0.2 μm inline PTFE HEPA, Millipore Millex-FG)");
    println!("   Dip tube:              Ø{dip_tube_od:.0} mm boss (3/8\" OD rigid PP stand-pipe)");
    println!("   Low-level sensor:      M12×1 optical level (OMRON E3F-DS30C4)");
    println!("   QD bulkhead:           Ø{aseptiquik_port_od:.0} mm (CPC AseptiQuik DC Ø1/4\" hose barb)");
    println!("   Lift eyelets:          2× Ø{lift_eyelet_od:.0} mm");
    println!();
    println!(" PUMP MANIFOLD");
    println!("   Channels:              {pump_channels} × Kamoer KCS-B (NEMA17, 0.002-170 mL/min)");
    println!("   Manifold plate:        {manifold_plate_x:.0} × {manifold_plate_y:.0} × {manifold_plate_thickness:.0} mm PETG");
    println!("   Pump pitch:            {pump_pitch:.0} mm");
    println!("   Bolt pattern/head:     4× M3 on {pump_mount_pattern:.0} mm square");
    println!("   Boss clearance:        Ø{nema17_boss_clr:.0} mm (NEMA17 shaft/boss)");
    println!();
    println!(" TUBING");
    println!("   Platinum-cured silicone: ID {tube_id} mm, OD {tube_od} mm");
    println!("   Reservoir → pump inlet:  1× per channel, dip-tube manifold");
    println!("   Pump outlet → passthrough: 1× per channel, looped service length");
    println!("   Passthrough → drip head:   1× per channel, routed along rack frame");
    println!("   Quick-disconnects:       AseptiQuik DC at reservoir (×1 trunk) + at drip head input (×5)");
    println!();
    println!(" WALL PASS-THROUGH");
    println!("   Bulkhead gland:        Ø{passthrough_od:.0} mm OD flange, Ø{passthrough_boss_od:.0} mm OD boss × {passthrough_boss_length:.0} mm long");
    println!(
        "   Center bore:           Ø{passthrough_center_bore:.0} mm (5-hole silicone grommet)"
    );
    println!("   Grommet tube holes:    5 × Ø{grommet_hole_dia:.1} mm (press-fit on {tube_od} mm OD tube)");
    println!();
    println!(" DRIP HEADS (×5)");
    println!("   Size:                  {drip_head_x:.0} × {drip_head_y:.0} × {drip_head_z:.0} mm");
    println!("   Nozzles per head:      {drip_nozzles} × Ø{drip_nozzle_od} mm body, Ø{drip_nozzle_bore} mm bore");
    println!("   Nozzle pitch:          {drip_nozzle_pitch:.2} mm (one per chip column)");
    println!("   Mount:                 Hangs from rack frame, clear of ±15° rocker tilt envelope");
    println!();
    println!(" DRIP TRAY");
    println!(
        "   Size:                  {tray_x:.0} × {tray_y:.0} × {tray_outer_h:.0} mm (2.5 mm PETG)"
    );
    println!("   Drain:                 Ø{tray_drain_od:.0} mm, back-left corner → waste bottle");
    println!("   Splash ridge:          3 × 10 mm along front edge");
    println!();
    println!(" WASTE BOTTLE");
    println!(
        "   Size:                  Ø{waste_od:.0} × {waste_h:.0} mm (~4 L), PETG wall {waste_wall} mm"
    );
    println!("   Inlet barb (side):     fits 9 mm ID drain tube");
    println!("   HEPA vent:             4 mm bore + 0.2 μm PTFE vent filter");
    println!();
    println!(" MATERIALS (printable prototype today, stainless for final)");
    println!("   Reservoir / lid:       PETG 3 mm (proto), 316L stainless 1.5 mm (final)");
    println!("   Pump manifold:         PETG 8 mm");
    println!("   Passthrough gland:     PETG 3 mm + silicone grommet (Shore A 40)");
    println!("   Drip heads:            PETG 3 mm");
    println!("   Drip tray:             PETG 2.5 mm");
    println!("   Tubing:                Masterflex platinum-cured silicone (96420-13 equiv)");
    println!("   Quick-disconnects:     CPC AseptiQuik DC (gamma/EtO sterilized)");
    println!("   HEPA vents:            Millipore Millex-FG 0.2 μm PTFE");
    println!();
    println!(" STERILE BARRIER STRATEGY");
    println!(
        "   1. Pre-sterilize: autoclave reservoir + dip-tube + silicone tubing at 121°C/20 min"
    );
    println!("   2. Seal: lid O-ring provides primary seal, HEPA vent maintains 0.2 μm barrier");
    println!(
        "   3. Connect: AseptiQuik DC connectors create sterile connection in non-sterile air"
    );
    println!("   4. Swap: empty reservoir → close QD at lid → exchange reservoir → reopen QD (chips never exposed)");
    println!();
    println!(" FAILURE MODES");
    println!("   - Tube occlusion:        low-level sensor timeout → alarm, no silent starve");
    println!("   - Pump jam (single ch.): other 4 shelves keep flowing, operator swaps head");
    println!("   - Lid leak:              drip tray + waste bottle catch ≤500 mL spill");
    println!("   - HEPA blockage:         reservoir vacuum → low-level sensor reads dry → alarm");
    println!("   - QD disconnect:         detent + lock ring on AseptiQuik prevents casual pull");
    println!();
    println!(" EXPORTED STLs");
    println!("   output/media_reservoir_body.stl");
    println!("   output/media_reservoir_lid.stl");
    println!("   output/media_reservoir_pump_manifold.stl");
    println!("   output/media_reservoir_passthrough.stl");
    println!("   output/media_reservoir_grommet.stl");
    println!("   output/media_reservoir_drip_head.stl");
    println!("   output/media_reservoir_drip_tray.stl");
    println!("   output/media_reservoir_waste_bottle.stl");
    println!("══════════════════════════════════════════════════════════════");

    // STEP export (STL → STEP via stltostp)
    for stl in [
        "output/media_reservoir_body.stl",
        "output/media_reservoir_lid.stl",
        "output/media_reservoir_pump_manifold.stl",
        "output/media_reservoir_passthrough.stl",
        "output/media_reservoir_grommet.stl",
        "output/media_reservoir_drip_head.stl",
        "output/media_reservoir_drip_tray.stl",
        "output/media_reservoir_waste_bottle.stl",
    ] {
        laminarforge_cad::stl_to_step(stl);
    }
}
