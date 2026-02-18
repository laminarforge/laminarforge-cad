// ─── Shared Dimensions for LAMP Device v1 ───
//
// All CAD models import from here to stay in sync.
// Change a dimension once, all models update.

/// Number of tube slots
pub const NUM_SLOTS: usize = 8;

/// Center-to-center spacing between tube slots (mm)
pub const SLOT_SPACING: f64 = 10.0;

/// Aluminum heating block dimensions (mm)
pub const BLOCK_LENGTH: f64 = (NUM_SLOTS as f64 - 1.0) * SLOT_SPACING + 14.0; // 84mm
pub const BLOCK_WIDTH: f64 = 16.0;
pub const BLOCK_HEIGHT: f64 = 15.0;

/// Tube specs
pub const TUBE_OD: f64 = 6.35;
pub const TUBE_HOLE_DIAMETER: f64 = 6.5; // block hole, slight clearance
pub const TUBE_HOLE_DEPTH: f64 = 13.0; // blind hole depth in block
pub const TUBE_PASSTHROUGH_DIAMETER: f64 = 7.5; // mount hole, easy insertion

/// Heating block bores
pub const HEATER_BORE_DIAMETER: f64 = 6.2;
pub const HEATER_BORE_DEPTH: f64 = 70.0;
pub const THERMISTOR_BORE_DIAMETER: f64 = 3.5;
pub const THERMISTOR_BORE_DEPTH: f64 = 50.0;
pub const BORE_Z_OFFSET_FROM_BOTTOM: f64 = 3.5; // center of bores above block bottom
pub const HEATER_Y_OFFSET: f64 = -3.0;
pub const THERMISTOR_Y_OFFSET: f64 = 3.0;

/// Optical mount dimensions (mm)
pub const MOUNT_HEIGHT: f64 = 12.0;
pub const MOUNT_WIDTH: f64 = 22.0;
pub const MOUNT_LENGTH: f64 = BLOCK_LENGTH; // same as block

/// Optical axis
pub const OPTICAL_CENTER_Z: f64 = 5.0; // from mount bottom
pub const COMPONENT_HOLE_DIAMETER: f64 = 5.2; // LED/sensor press-fit
pub const COMPONENT_HOLE_DEPTH: f64 = 6.0;
pub const OPTICAL_CHANNEL_WIDTH: f64 = 3.0;
pub const OPTICAL_CHANNEL_HEIGHT: f64 = 3.0;

/// Enclosure dimensions (mm)
pub const ENCLOSURE_WALL: f64 = 3.0;
pub const ENCLOSURE_FLOOR: f64 = 3.0;
pub const POCKET_CLEARANCE: f64 = 0.5;
pub const SHELF_DEPTH: f64 = MOUNT_WIDTH + 6.0; // 28mm
pub const ELECTRONICS_DEPTH: f64 = 42.0;

/// Derived enclosure dimensions
pub const INNER_X: f64 = BLOCK_LENGTH + 10.0; // 94mm
pub const INNER_Y: f64 = SHELF_DEPTH + ELECTRONICS_DEPTH; // 70mm
pub const WALL_HEIGHT: f64 = BLOCK_HEIGHT + MOUNT_HEIGHT + 5.0; // 32mm
pub const OUTER_X: f64 = INNER_X + ENCLOSURE_WALL * 2.0; // 100mm
pub const OUTER_Y: f64 = INNER_Y + ENCLOSURE_WALL * 2.0; // 76mm
pub const OUTER_Z: f64 = WALL_HEIGHT + ENCLOSURE_FLOOR; // 35mm

/// Lid dimensions
pub const LID_THICKNESS: f64 = 3.0;
pub const LID_LIP_DEPTH: f64 = 5.0;
pub const LID_LIP_CLEARANCE: f64 = 0.3;
pub const LID_TUBE_HOLE_DIAMETER: f64 = 8.0;

/// Compute the X position of the first tube hole (leftmost slot)
pub fn first_slot_x() -> f64 {
    -((NUM_SLOTS as f64 - 1.0) * SLOT_SPACING) / 2.0
}

/// Compute the shelf center Y in enclosure coordinates
pub fn shelf_center_y() -> f64 {
    -(INNER_Y / 2.0) + SHELF_DEPTH / 2.0
}

/// Compute the floor Z in enclosure coordinates
pub fn floor_z() -> f64 {
    -(OUTER_Z / 2.0) + ENCLOSURE_FLOOR
}

// ─── Fluid Handling System Constants ───

/// Silicone tubing (outlet pinch valve section)
pub const SILICONE_ID: f64 = 1.6;
pub const SILICONE_OD: f64 = 3.2;
pub const SILICONE_WALL: f64 = (SILICONE_OD - SILICONE_ID) / 2.0; // 0.8mm

/// PTFE tubing (main fluid path)
pub const PTFE_ID: f64 = 0.5;
pub const PTFE_OD: f64 = 1.5;

/// 608ZZ ball bearings (used in laser cutter Z-axis)
pub const BEARING_608_BORE: f64 = 8.0;
pub const BEARING_608_OD: f64 = 22.0;
pub const BEARING_608_WIDTH: f64 = 7.0;

/// NEMA17 stepper motor dimensions
pub const NEMA17_BODY: f64 = 42.3;
pub const NEMA17_SHAFT_DIAMETER: f64 = 5.0;
pub const NEMA17_SHAFT_LENGTH: f64 = 24.0;
pub const NEMA17_BOSS_DIAMETER: f64 = 22.0;
pub const NEMA17_BOSS_HEIGHT: f64 = 2.0;
pub const NEMA17_HOLE_SPACING: f64 = 31.0; // M3 bolt hole center-to-center
pub const NEMA17_HOLE_DIAMETER: f64 = 3.2; // M3 clearance

/// Syringe dimensions (1mL disposable, Luer slip)
pub const SYRINGE_BARREL_OD: f64 = 6.5;
pub const SYRINGE_BARREL_LENGTH: f64 = 65.0;
pub const SYRINGE_FLANGE_WIDTH: f64 = 14.0;
pub const SYRINGE_FLANGE_THICKNESS: f64 = 1.5;

/// T8 lead screw
pub const LEADSCREW_DIAMETER: f64 = 8.0;
pub const LEADSCREW_NUT_OD: f64 = 22.0;
pub const LEADSCREW_NUT_HEIGHT: f64 = 10.0;

/// Syringe pump guide rod (anti-rotation, parallel to lead screw)
pub const SYRINGE_GUIDE_ROD_DIAMETER: f64 = 4.0;
pub const SYRINGE_GUIDE_ROD_OFFSET: f64 = 12.0; // offset from lead screw center

/// Linear rail (X-axis)
pub const LINEAR_ROD_DIAMETER: f64 = 8.0;
pub const LM8UU_OD: f64 = 15.0;
pub const LM8UU_LENGTH: f64 = 24.0;

/// Number of fluid channels
pub const NUM_FLUID_CHANNELS: usize = 2; // master mix + primers (lysis optional)

// ─── Still Air Box Constants ───

/// Interior dimensions
pub const SAB_INNER_WIDTH: f64 = 400.0;
pub const SAB_INNER_DEPTH: f64 = 300.0;
pub const SAB_INNER_HEIGHT: f64 = 300.0;

/// Acrylic panel thickness
pub const SAB_PANEL_THICKNESS: f64 = 3.0;

/// Frame rail cross-section
pub const SAB_RAIL_WIDTH: f64 = 20.0; // rail outer width
pub const SAB_RAIL_HEIGHT: f64 = 20.0;
pub const SAB_CHANNEL_DEPTH: f64 = 5.0; // panel slides in this deep
pub const SAB_CHANNEL_WIDTH: f64 = SAB_PANEL_THICKNESS + 0.5; // 3.5mm slot for 3mm acrylic

/// Swab collector dimensions (from retractable_swab_collector.rs)
pub const COLLECTOR_OD: f64 = 12.0;

/// Docking port
pub const SAB_DOCK_PORT_ID: f64 = COLLECTOR_OD + 0.3; // 12.3mm snug fit
pub const SAB_DOCK_PORT_DEPTH: f64 = 25.0;
pub const SAB_NUM_PORTS: usize = 4;

/// Arm holes
pub const SAB_ARM_HOLE_DIAMETER: f64 = 90.0; // fits a gloved hand
pub const SAB_ARM_HOLE_SPACING: f64 = 150.0; // center-to-center

// ─── Workstation Enclosure Constants ───
// 3-zone partitioned enclosure: Sterile Work, Equipment, Printers
// Same construction method as SAB: printed frame + laser-cut acrylic panels

/// Overall outer dimensions (mm)
pub const WS_TOTAL_WIDTH: f64 = 1800.0; // ~1.8m
pub const WS_DEPTH: f64 = 700.0;
pub const WS_HEIGHT: f64 = 600.0;

/// Zone widths (interior, between rails)
pub const WS_ZONE1_WIDTH: f64 = 500.0; // Sterile work zone
pub const WS_ZONE2_WIDTH: f64 = 700.0; // Equipment zone
pub const WS_ZONE3_WIDTH: f64 = 560.0; // Printer zone (remainder)

/// Structural frame
pub const WS_RAIL_SIZE: f64 = 20.0; // same as SAB rails
pub const WS_PANEL_THICKNESS: f64 = 3.0; // acrylic panels
pub const WS_CHANNEL_DEPTH: f64 = 5.0; // panel slot depth
pub const WS_CHANNEL_WIDTH: f64 = WS_PANEL_THICKNESS + 0.5; // 3.5mm

/// Partition wall thickness (acrylic between zones)
pub const WS_PARTITION_THICKNESS: f64 = 3.0;

/// Maximum printable rail length (Bambu A1: 256mm)
pub const WS_MAX_PRINT_LENGTH: f64 = 240.0; // safe margin from 256mm bed

/// Exhaust port (Zone 3 back panel)
pub const WS_EXHAUST_DIAMETER: f64 = 100.0; // standard 4" duct

/// Door/panel opening heights
pub const WS_DOOR_HEIGHT: f64 = 500.0; // most of the front face
pub const WS_DOOR_CLEARANCE: f64 = 2.0;

// ─── Microfluidic Chip Constants ───
// Simple v1 chip: inlet → channel → chamber → channel → outlet
// Designed as a MOLD for PDMS casting (resin-printed negative)

/// Chip overall dimensions (mm)
pub const CHIP_LENGTH: f64 = 50.0; // glass slide size
pub const CHIP_WIDTH: f64 = 25.0;
pub const CHIP_MOLD_THICKNESS: f64 = 5.0; // mold base

/// Channel dimensions
pub const CHIP_CHANNEL_WIDTH: f64 = 0.4; // 400μm
pub const CHIP_CHANNEL_DEPTH: f64 = 0.15; // 150μm (raised on mold)
pub const CHIP_CHANNEL_LENGTH: f64 = 8.0; // inlet/outlet channel length

/// Culture chamber
pub const CHIP_CHAMBER_LENGTH: f64 = 10.0;
pub const CHIP_CHAMBER_WIDTH: f64 = 5.0;
pub const CHIP_CHAMBER_DEPTH: f64 = 0.15; // same as channels

/// Inlet/outlet port holes (for tubing connection)
pub const CHIP_PORT_DIAMETER: f64 = 1.5; // matches PTFE tubing OD
pub const CHIP_PORT_DEPTH: f64 = 3.0;

/// Alignment marks
pub const CHIP_ALIGNMENT_MARK_SIZE: f64 = 1.0;

// ─── CO2 Laser Cutter Constants ───
// Reference: OpenBuilds FreeBURN, Lasersaur
// 40-50W glass CO2 tube, 600×400mm work area

/// Frame (2020 V-slot extrusion layout)
pub const LASER_WORK_X: f64 = 600.0; // work area width
pub const LASER_WORK_Y: f64 = 400.0; // work area depth
pub const LASER_FRAME_X: f64 = 800.0; // outer frame width
pub const LASER_FRAME_Y: f64 = 600.0; // outer frame depth
pub const LASER_FRAME_Z: f64 = 300.0; // frame height

/// Extrusion profile
pub const LASER_EXTRUSION_SIZE: f64 = 20.0; // 2020 V-slot

/// CO2 laser tube
pub const LASER_TUBE_DIAMETER: f64 = 50.0;
pub const LASER_TUBE_LENGTH: f64 = 850.0; // 40W tube ~850mm

/// Mirror specs
pub const LASER_MIRROR_DIAMETER: f64 = 20.0; // Si/Mo mirror
pub const LASER_MIRROR_THICKNESS: f64 = 3.0;
pub const LASER_MIRROR_ADJUSTMENT_SCREW: f64 = 3.0; // M3

/// MGN12 linear rail
pub const MGN12_RAIL_WIDTH: f64 = 12.0;
pub const MGN12_RAIL_HEIGHT: f64 = 8.0;
pub const MGN12_CARRIAGE_LENGTH: f64 = 27.0;
pub const MGN12_CARRIAGE_WIDTH: f64 = 27.0;
pub const MGN12_CARRIAGE_HEIGHT: f64 = 10.0;
pub const MGN12_HOLE_SPACING: f64 = 20.0; // bolt pattern
