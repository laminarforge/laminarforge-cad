pub mod pcb;

// ─── Shared Dimensions for LAMP Device v1 ───
//
// All CAD models import from here to stay in sync.
// Change a dimension once, all models update.

/// Number of tube slots
pub const NUM_SLOTS: usize = 8;

/// Center-to-center spacing between tube slots (mm)
pub const SLOT_SPACING: f64 = 10.0;

/// Tube specs (Novas Bio 0.2mL PCR tube, datasheet: 0.2400" = 6.096mm OD)
pub const TUBE_OD: f64 = 6.096;
pub const TUBE_PASSTHROUGH_DIAMETER: f64 = 7.0; // mount hole, easy insertion

/// PCB heater dimensions (mm)
pub const PCB_THICKNESS: f64 = 1.6; // standard 2-layer FR-4
pub const PCB_LENGTH: f64 = 100.0; // full board length, spans enclosure
pub const PCB_WIDTH: f64 = 80.0; // full board width

/// Heater zone (serpentine copper trace on PCB bottom layer)
pub const HEATER_ZONE_LENGTH: f64 = (NUM_SLOTS as f64 - 1.0) * SLOT_SPACING + 14.0; // 84mm, matches tube array span
pub const HEATER_ZONE_WIDTH: f64 = 22.0; // slightly wider than tubes for heat margin

/// Copper spreader plate (LEGACY — replaced by aluminum heating block in v1)
pub const SPREADER_THICKNESS: f64 = 1.0;
pub const SPREADER_LENGTH: f64 = HEATER_ZONE_LENGTH;
pub const SPREADER_WIDTH: f64 = HEATER_ZONE_WIDTH;

/// Tube holder (3D-printed PETG grip block — LEGACY, replaced by aluminum heating block)
pub const HOLDER_HEIGHT: f64 = 18.0;
pub const HOLDER_WIDTH: f64 = 22.0;
pub const HOLDER_LENGTH: f64 = HEATER_ZONE_LENGTH;
pub const HOLDER_TUBE_DIAMETER: f64 = 6.45; // comfortable fit after FDM shrinkage
pub const HOLDER_SCREW_SPACING_X: f64 = 70.0;
pub const HOLDER_SCREW_DIAMETER: f64 = 3.2;

/// Aluminum heating block (replaces copper spreader + PETG tube holder)
/// CNC-machined 6061-T6 aluminum. Holds tubes directly with integrated cartridge heater.
/// Sits on PCB, connects via J4 (heater) and J3 (thermistor).
pub const BLOCK_LENGTH: f64 = HEATER_ZONE_LENGTH; // 84mm
pub const BLOCK_WIDTH: f64 = HEATER_ZONE_WIDTH;   // 22mm
pub const BLOCK_HEIGHT: f64 = 25.0; // 15mm wells + 10mm floor (heater bore + thermal mass)
pub const BLOCK_TUBE_DIAMETER: f64 = 6.2; // tighter fit for CNC aluminum (0.1mm clearance vs 0.35mm for FDM)
pub const BLOCK_WELL_DEPTH: f64 = 15.0; // immerses full conical bottom of PCR tube
pub const HEATER_BORE_DIAMETER: f64 = 6.1; // 0.1mm clearance for 6mm cartridge heater
pub const HEATER_BORE_DEPTH: f64 = 78.0; // from left end, nearly full block length
pub const HEATER_BORE_Z_OFFSET: f64 = 5.0; // center height from block bottom
pub const THERMISTOR_BORE_DIAMETER: f64 = 3.0; // 10K NTC glass bead thermistor
pub const THERMISTOR_BORE_DEPTH: f64 = 7.0; // from front face, reads temp at heater level
pub const BLOCK_MOUNT_HOLE_DIAMETER: f64 = 3.2; // M3 clearance
pub const BLOCK_MOUNT_HOLE_X: f64 = 39.0; // from center, near block ends
pub const BLOCK_MOUNT_HOLE_Y: f64 = 8.0; // from center, outside wells and heater bore

/// Enclosure dimensions (mm)
pub const ENCLOSURE_WALL: f64 = 3.0;
pub const ENCLOSURE_FLOOR: f64 = 3.0;
pub const POCKET_CLEARANCE: f64 = 0.5;
pub const SHELF_DEPTH: f64 = BLOCK_WIDTH + 6.0; // 28mm
pub const ELECTRONICS_DEPTH: f64 = 42.0;

/// Derived enclosure dimensions
pub const INNER_X: f64 = HEATER_ZONE_LENGTH + 10.0; // 94mm
pub const INNER_Y: f64 = SHELF_DEPTH + ELECTRONICS_DEPTH; // 70mm
pub const WALL_HEIGHT: f64 = PCB_THICKNESS + BLOCK_HEIGHT + 5.0; // 31.6mm
pub const OUTER_X: f64 = INNER_X + ENCLOSURE_WALL * 2.0; // 100mm
pub const OUTER_Y: f64 = INNER_Y + ENCLOSURE_WALL * 2.0; // 76mm
pub const OUTER_Z: f64 = WALL_HEIGHT + ENCLOSURE_FLOOR; // ~31.6mm

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

/// NEMA17 stepper motor dimensions
pub const NEMA17_BODY: f64 = 42.3;
pub const NEMA17_SHAFT_DIAMETER: f64 = 5.0;
pub const NEMA17_SHAFT_LENGTH: f64 = 24.0;
pub const NEMA17_BOSS_DIAMETER: f64 = 22.0;
pub const NEMA17_BOSS_HEIGHT: f64 = 2.0;
pub const NEMA17_HOLE_SPACING: f64 = 31.0; // M3 bolt hole center-to-center
pub const NEMA17_HOLE_DIAMETER: f64 = 3.2; // M3 clearance

/// Syringe dimensions (Hamilton 1725 TLL, 250μL gastight, PN 81120)
/// Barrel ID: 2.30mm, PTFE Luer Lock termination
/// NOTE: barrel_length, flange_width, flange_thickness are estimates —
/// verify with actual syringe in hand and update before final prints.
pub const SYRINGE_BARREL_OD: f64 = 7.75; // confirmed from Hamilton spec
pub const SYRINGE_BARREL_LENGTH: f64 = 60.0; // estimated — measure actual
pub const SYRINGE_FLANGE_WIDTH: f64 = 16.0; // estimated — measure actual
pub const SYRINGE_FLANGE_THICKNESS: f64 = 2.0; // estimated — measure actual

/// T8 lead screw
pub const LEADSCREW_DIAMETER: f64 = 8.0;
pub const LEADSCREW_NUT_OD: f64 = 22.0;
pub const LEADSCREW_NUT_HEIGHT: f64 = 10.0;

/// Linear rail (X-axis)
pub const LINEAR_ROD_DIAMETER: f64 = 8.0;
pub const LM8UU_OD: f64 = 15.0;
pub const LM8UU_LENGTH: f64 = 24.0;

/// Number of fluid channels (dedicated syringe per channel, zero cross-contamination)
pub const NUM_FLUID_CHANNELS: usize = 3; // lysis buffer + master mix + primer mix

/// Syringe pump guide rod (anti-rotation)
pub const SYRINGE_GUIDE_ROD_DIAMETER: f64 = 4.0;
pub const SYRINGE_GUIDE_ROD_OFFSET: f64 = 12.0; // from lead screw center

// ─── V-Slot Gantry Constants (Liquid Handler XYZ+I) ───
// MULA-style 4-axis gantry: X/Y belt-driven, Z geared-belt 3:1, I lead-screw (plunger)

/// V-slot 2040 aluminum extrusion (OpenBuilds standard)
pub const VSLOT_2040_W: f64 = 20.0; // mm
pub const VSLOT_2040_H: f64 = 40.0; // mm
pub const VSLOT_SLOT_WIDTH: f64 = 6.0; // channel opening for M5 T-nuts

/// OpenBuilds Mini V-Wheel (Delrin)
pub const VWHEEL_OD: f64 = 15.23; // mm outer diameter
pub const VWHEEL_BORE: f64 = 5.0;  // M5 bore
pub const VWHEEL_WIDTH: f64 = 8.80; // mm

/// Eccentric spacer (for V-wheel preload adjustment)
pub const ECCENTRIC_OD: f64 = 7.12; // mm body OD
pub const ECCENTRIC_BORE: f64 = 5.0; // M5

/// GT2 timing belt
pub const GT2_BELT_WIDTH: f64 = 6.0; // mm (standard 6mm)
pub const GT2_BELT_PITCH: f64 = 2.0; // mm tooth pitch

/// GT2 20-tooth pulley (on motor shaft)
pub const GT2_20T_PITCH_DIA: f64 = 12.73; // mm pitch diameter
pub const GT2_20T_OD: f64 = 14.0; // mm approximate outer
pub const GT2_PULLEY_BORE: f64 = 5.0; // mm shaft bore

/// GT2 60-tooth pulley (Z-axis driven, 3:1 reduction)
pub const GT2_60T_PITCH_DIA: f64 = 38.20; // mm pitch diameter

/// F625ZZ flanged bearing (GT2 idler)
pub const F625ZZ_BORE: f64 = 5.0;
pub const F625ZZ_OD: f64 = 16.0;
pub const F625ZZ_WIDTH: f64 = 5.0;
pub const F625ZZ_FLANGE_OD: f64 = 17.0;

/// Hamilton 1710RN syringe (100μL gastight, PN 81030)
/// Selected per gap analysis: 1nL/microstep resolution, covers 10-100μL range
/// Dimensions verified from Hamilton datasheet (PN 81065 / Knowledge Center)
pub const H1710_BARREL_OD: f64 = 7.75;    // confirmed: 0.305" (NOT 6.6mm — that's the 1701/10μL)
pub const H1710_BARREL_ID: f64 = 1.46;    // confirmed from Hamilton spec
pub const H1710_BARREL_LENGTH: f64 = 65.0; // ~65mm (verified, was 60mm estimate)
pub const H1710_FLANGE_WIDTH: f64 = 12.5;  // ~12.5mm (verified, was 14mm estimate)
pub const H1710_FLANGE_THICKNESS: f64 = 2.0; // estimated — verify in hand
pub const H1710_NEEDLE_OD: f64 = 0.72;    // 22s gauge blunt

/// V-slot M5 mounting (T-nut interface)
pub const VSLOT_M5_HOLE: f64 = 5.3; // M5 clearance hole
pub const VSLOT_M5_CBORE: f64 = 9.0; // counterbore for M5 button head

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

// ─── Microfluidic Chip Constants (PDMS mold, resin-printed) ───
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

// ─── CNC-Milled Microfluidic Chip Constants (v1 — 1-chamber, microscope slide) ───
// v1 cell culture chip: CNC-milled from cast PMMA (acrylic)
// Standard microscope slide format: 75mm × 25mm (ISO 8037-1)
// Single-depth milling, channels + chamber all 200μm deep
// Two-part design: channel plate (milled) + cover plate (flat)

/// Channel plate dimensions (mm) — standard microscope slide format
pub const CNC_CHIP_LENGTH: f64 = 75.0;
pub const CNC_CHIP_WIDTH: f64 = 25.0;
pub const CNC_CHIP_THICKNESS: f64 = 3.0;

/// Cover plate (flat, bonded on top with Weld-On 3)
pub const CNC_COVER_THICKNESS: f64 = 1.0;

/// Channel dimensions — 500μm wide × 200μm deep
pub const CNC_CHANNEL_WIDTH: f64 = 0.5; // 500μm
pub const CNC_CHANNEL_DEPTH: f64 = 0.2; // 200μm

/// Culture chamber — centered on chip
pub const CNC_CHAMBER_LENGTH: f64 = 10.0;
pub const CNC_CHAMBER_WIDTH: f64 = 3.0;
pub const CNC_CHAMBER_DEPTH: f64 = 0.2; // same depth as channels
pub const CNC_CHAMBER_CORNER_RADIUS: f64 = 0.5; // min R from end mill

/// Port through-holes — fits 1/16" OD barb fittings
pub const CNC_PORT_DIAMETER: f64 = 1.5;
pub const CNC_PORT_INSET: f64 = 5.0; // mm from short edge

/// Alignment pin holes — diagonal corners, non-through
pub const CNC_ALIGN_DIAMETER: f64 = 1.0;
pub const CNC_ALIGN_DEPTH: f64 = 0.5;
pub const CNC_ALIGN_INSET: f64 = 3.0; // mm from each edge

// ─── CNC-Milled Microfluidic Chip Constants (v2 — 16-chamber, well-plate format) ───
// 16-chamber AAV selectivity screening chip
// ANSI/SLAS microplate footprint: 127.76mm × 85.48mm
// Compatible with standard plate readers, incubator racks, automated handlers
// Two-part design: channel plate (milled) + cover plate (flat)
// 4×4 chamber grid, each with dedicated inlet/outlet channels and ports

/// v2 chip overall dimensions (mm) — ANSI/SLAS microplate footprint
pub const CNC16_CHIP_LENGTH: f64 = 127.76; // X axis (long edge)
pub const CNC16_CHIP_WIDTH: f64 = 85.48; // Y axis (short edge)
pub const CNC16_CHIP_THICKNESS: f64 = 3.0; // channel plate
pub const CNC16_COVER_THICKNESS: f64 = 1.0; // cover plate

/// v2 number of chambers
pub const CNC16_NUM_CHAMBERS: usize = 16;
pub const CNC16_GRID_COLS: usize = 4; // columns along X
pub const CNC16_GRID_ROWS: usize = 4; // rows along Y

/// v2 chamber dimensions — 3mm wide × 10mm long × 200μm deep
pub const CNC16_CHAMBER_WIDTH: f64 = 3.0; // X direction
pub const CNC16_CHAMBER_LENGTH: f64 = 10.0; // Y direction
pub const CNC16_CHAMBER_DEPTH: f64 = 0.2; // 200μm

/// v2 channel dimensions — 500μm wide × 200μm deep
pub const CNC16_CHANNEL_WIDTH: f64 = 0.5; // 500μm
pub const CNC16_CHANNEL_DEPTH: f64 = 0.2; // 200μm

/// v2 port through-holes — 1.6mm dia, interference fit for 1/16" OD PTFE tubing
/// (1/16" = 1.5875mm; 1.6mm hole gives ~12μm interference press-fit in PMMA)
pub const CNC16_PORT_DIAMETER: f64 = 1.6;

/// v2 grid spacing (center-to-center between chambers)
/// With 4 columns across 127.76mm, use ~25mm column spacing (centered)
/// With 4 rows across 85.48mm, use ~18mm row spacing (centered)
pub const CNC16_COL_SPACING: f64 = 25.0; // X spacing between column centers
pub const CNC16_ROW_SPACING: f64 = 18.0; // Y spacing between row centers

/// v2 channel length from port to chamber edge
/// NOTE: Reduced from 5.0mm to 2.0mm to prevent channel overlap between adjacent rows.
/// At 5.0mm, inlet channels from row N overlapped with outlet channels from row N+1
/// (same X position, shared Y range), creating cross-contamination paths.
/// At 2.0mm, inter-row port wall is 2.4mm (>1× port diameter) — structurally safe.
pub const CNC16_CHANNEL_LENGTH: f64 = 2.0; // mm, port-to-chamber distance

/// v2 alignment pin holes — 4 corners
pub const CNC16_ALIGN_DIAMETER: f64 = 1.0;
pub const CNC16_ALIGN_DEPTH: f64 = 0.5;
pub const CNC16_ALIGN_INSET: f64 = 4.0; // mm from each edge

/// v2 mounting holes — M3 clearance, 4 near corners
pub const CNC16_MOUNT_DIAMETER: f64 = 3.2; // M3 clearance
pub const CNC16_MOUNT_INSET: f64 = 6.0; // mm from each edge

/// v2 label area (engraved rectangle for chip ID)
pub const CNC16_LABEL_LENGTH: f64 = 20.0; // X
pub const CNC16_LABEL_WIDTH: f64 = 5.0; // Y
pub const CNC16_LABEL_DEPTH: f64 = 0.1; // shallow engraving

/// v2 corner radius — ANSI/SLAS 1-2004 §4.1.2.1 requires 3.18 ± 1.6mm
pub const CNC16_CORNER_RADIUS: f64 = 3.18;

// ─── CNC-Milled Microfluidic Chip Constants (Rev C — 16-chamber, well-plate format) ───
// Open-well design for 100-chip scale. Gravity-driven rocker flow, robotic liquid handler.
// Two-sided CNC: wells + vias on TOP face, channels + chambers on BOTTOM face.
// Sealed by 150μm borosilicate glass bottom (PSA tape bonding).
// Well positions aligned to 96-well grid (9mm pitch) for Opentrons/Hamilton compatibility.

/// Rev C plate dimensions (mm) — ANSI/SLAS microplate footprint
pub const REVC_CHIP_LENGTH: f64 = 127.76; // X axis (long edge)
pub const REVC_CHIP_WIDTH: f64 = 85.48;   // Y axis (short edge)
pub const REVC_CORNER_RADIUS: f64 = 3.18; // ANSI/SLAS 1-2004 §4.1.2.1

/// Rev C plate thickness — 14.20mm PMMA + 0.15mm glass = 14.35mm total
/// ANSI/SLAS 2-2004: plate height = 14.35mm ± 0.25mm (at wells), ±0.76mm overall
/// Minimum external clearance to plate bottom: 1.0mm (in well area)
pub const REVC_PMMA_THICKNESS: f64 = 14.20;
pub const REVC_GLASS_THICKNESS: f64 = 0.15; // 150μm borosilicate (#1.5 coverslip)
pub const REVC_TOTAL_HEIGHT: f64 = 14.35;   // ANSI/SLAS 2-2004 standard plate height

/// Rev C well dimensions — open-top wells on TOP face
/// 4mm diameter fits 9mm pitch grid, compatible with 384-well robotic tips
pub const REVC_WELL_DIAMETER: f64 = 4.0;
pub const REVC_WELL_DEPTH: f64 = 10.0;    // ~125μL volume per well (π×2²×10)
pub const REVC_WELL_VOLUME_UL: f64 = 125.7; // reference only, π×r²×h

/// Rev C via dimensions — vertical through-hole connecting well bottom to channel
/// Drilled from well bottom through remaining floor to bottom face
/// Aspect ratio 4:1 — standard CNC drill operation (validated by Folch lab 96-well device)
pub const REVC_VIA_DIAMETER: f64 = 1.0;
pub const REVC_VIA_LENGTH: f64 = 4.0; // 14.20 - 10.0 - 0.2 = 4.0mm floor

/// Rev C chamber dimensions — milled into BOTTOM face
pub const REVC_CHAMBER_WIDTH: f64 = 3.0;   // X direction (unchanged from Rev B)
pub const REVC_CHAMBER_LENGTH: f64 = 7.0;  // Y direction (was 10mm in Rev B, fits 9mm well spacing)
pub const REVC_CHAMBER_DEPTH: f64 = 0.2;   // 200μm (unchanged from Rev B)

/// Rev C channel dimensions — milled into BOTTOM face, connect vias to chambers
pub const REVC_CHANNEL_WIDTH: f64 = 0.5;   // 500μm (unchanged from Rev B)
pub const REVC_CHANNEL_DEPTH: f64 = 0.2;   // 200μm (unchanged from Rev B)
pub const REVC_CHANNEL_LENGTH: f64 = 1.0;  // via center to chamber edge

/// Rev C grid layout — 4×4 chambers, each with inlet + outlet well = 32 wells
pub const REVC_NUM_CHAMBERS: usize = 16;
pub const REVC_GRID_COLS: usize = 4;
pub const REVC_GRID_ROWS: usize = 4;

/// 96-well plate footprint (ANSI/SLAS 4-2004)
pub const PLATE96_LENGTH: f64 = 127.76; // mm (X direction, 12 columns)
pub const PLATE96_WIDTH: f64 = 85.48; // mm (Y direction, 8 rows)

/// 96-well grid reference (ANSI/SLAS 4-2004)
/// A1 center from plate corner (0,0): X=14.38mm ± 0.50mm, Y=11.24mm ± 0.50mm
/// Pitch: 9.00mm ± 0.05mm in both X and Y
pub const WELL96_A1_X: f64 = 14.38;
pub const WELL96_A1_Y: f64 = 11.24;
pub const WELL96_PITCH: f64 = 9.0;

/// Rev C column positions — 96-well grid columns 2, 5, 8, 11 (1-indexed)
/// Spacing: 27mm (3 × 9mm pitch)
/// In chip-centered coordinates (origin at plate center):
pub const REVC_COL_GRID_INDICES: [usize; 4] = [2, 5, 8, 11]; // 1-indexed grid columns
pub const REVC_COL_XS_CENTERED: [f64; 4] = [-40.50, -13.50, 13.50, 40.50];
pub const REVC_COL_SPACING: f64 = 27.0;

/// Rev C row pairs — inlet/outlet wells in adjacent 96-well rows
/// Pairs: (A,B), (C,D), (E,F), (G,H) — 9mm between inlet and outlet
/// Chamber center Y is midpoint of each pair
/// 18mm between chamber row centers (matches Rev B)
pub const REVC_ROW_PAIR_GRID_INDICES: [(usize, usize); 4] = [(1, 2), (3, 4), (5, 6), (7, 8)]; // (inlet_row, outlet_row), 1-indexed
pub const REVC_CHAMBER_CENTER_YS: [f64; 4] = [-27.00, -9.00, 9.00, 27.00]; // chip-centered
pub const REVC_ROW_SPACING: f64 = 18.0;

/// Rev C inlet well Y positions (chip-centered)
pub const REVC_INLET_YS: [f64; 4] = [-31.50, -13.50, 4.50, 22.50];
/// Rev C outlet well Y positions (chip-centered)
pub const REVC_OUTLET_YS: [f64; 4] = [-22.50, -4.50, 13.50, 31.50];

/// Rev C alignment pin holes — 4 corners, same as Rev B
pub const REVC_ALIGN_DIAMETER: f64 = 1.0;
pub const REVC_ALIGN_DEPTH: f64 = 0.5;
pub const REVC_ALIGN_INSET: f64 = 4.0;

/// Rev C mounting holes — M3 clearance, 4 near corners
pub const REVC_MOUNT_DIAMETER: f64 = 3.2;
pub const REVC_MOUNT_INSET: f64 = 6.0;

/// Rev C label area (engraved rectangle for chip ID)
pub const REVC_LABEL_LENGTH: f64 = 20.0;
pub const REVC_LABEL_WIDTH: f64 = 5.0;
pub const REVC_LABEL_DEPTH: f64 = 0.1;

/// Rev C chamber labels — revised tissue panel (Schwann cells replace DRG replicate)
pub const REVC_CHAMBER_LABELS: [&str; 16] = [
    "DRG sensory neurons (TARGET)",
    "Hepatocytes (liver)",
    "Motor neurons",
    "Cortical neurons",
    "Astrocytes",
    "Cardiomyocytes (heart)",
    "Skeletal muscle",
    "Endothelial cells",
    "Kidney (renal epithelial)",
    "Lung epithelial",
    "PBMCs / T cells",
    "Pancreatic beta cells",
    "Enteric neurons",
    "Schwann cells",
    "Positive control (HEK293)",
    "Negative control (empty)",
];

/// Rev C well labels — 96-well grid positions for each chamber (inlet, outlet)
/// Format: (column_letter, inlet_row, outlet_row) per the 96-well grid
/// Chamber order: row-major through the 4×4 grid
pub const REVC_WELL_POSITIONS: [(&str, &str); 16] = [
    ("A2",  "B2"),   // Ch1:  DRG sensory neurons
    ("A5",  "B5"),   // Ch2:  Hepatocytes
    ("A8",  "B8"),   // Ch3:  Motor neurons
    ("A11", "B11"),  // Ch4:  Cortical neurons
    ("C2",  "D2"),   // Ch5:  Astrocytes
    ("C5",  "D5"),   // Ch6:  Cardiomyocytes
    ("C8",  "D8"),   // Ch7:  Skeletal muscle
    ("C11", "D11"),  // Ch8:  Endothelial cells
    ("E2",  "F2"),   // Ch9:  Kidney epithelial
    ("E5",  "F5"),   // Ch10: Lung epithelial
    ("E8",  "F8"),   // Ch11: PBMCs
    ("E11", "F11"),  // Ch12: Pancreatic beta
    ("G2",  "H2"),   // Ch13: Enteric neurons
    ("G5",  "H5"),   // Ch14: Schwann cells
    ("G8",  "H8"),   // Ch15: HEK293 (pos ctrl)
    ("G11", "H11"),  // Ch16: Empty (neg ctrl)
];

// ─── PCB Heater Trace Constants ───
// Serpentine copper trace on B.Cu for resistive heating

/// Heater trace width (mm) — 220μm = 0.22mm (tuned for ~10Ω target)
pub const HEATER_TRACE_WIDTH: f64 = 0.22;
/// Gap between traces (mm)
pub const HEATER_TRACE_GAP: f64 = 0.18;
/// Pitch = trace + gap
pub const HEATER_TRACE_PITCH: f64 = HEATER_TRACE_WIDTH + HEATER_TRACE_GAP;
/// Copper resistivity (Ω·m)
pub const COPPER_RESISTIVITY: f64 = 1.68e-8;
/// 1oz copper thickness (m)
pub const COPPER_THICKNESS_M: f64 = 35.0e-6;
/// Target heater resistance (Ω)
pub const HEATER_TARGET_RESISTANCE: f64 = 10.0;

// ─── PCB Thermal Via Constants ───

/// Via drill diameter (mm)
pub const VIA_DRILL: f64 = 0.35;
/// Via pad diameter (mm)
pub const VIA_PAD: f64 = 0.7;
/// Via grid spacing (mm)
pub const VIA_GRID: f64 = 1.5;

// ─── PCB Layout Zones ───
// Board is 100mm x 80mm. Y=0 at bottom edge.
// Heater zone: centered horizontally, near bottom

/// Heater zone Y start from board bottom edge (mm)
pub const HEATER_ZONE_Y_START: f64 = 8.0;
/// Electronics zone Y start (mm) — above heater zone
pub const ELECTRONICS_ZONE_Y_START: f64 = HEATER_ZONE_Y_START + HEATER_ZONE_WIDTH;
/// Heater zone X start — centered on 100mm board
pub const HEATER_ZONE_X_START: f64 = (PCB_LENGTH - HEATER_ZONE_LENGTH) / 2.0;

// ─── PCB Trace Widths ───

/// Power trace width (mm)
pub const POWER_TRACE_WIDTH: f64 = 0.5;
/// Signal trace width (mm)
pub const SIGNAL_TRACE_WIDTH: f64 = 0.25;
/// Minimum trace clearance (mm)
pub const MIN_CLEARANCE: f64 = 0.2;

// ─── Fluidic Circuit Board (FCB) Constants ───
// Two-plate PMMA construction: bottom plate has channels, top plate has through-holes
// FCB sits below the multi-chamber CNC chip and routes fluids to/from it
// Architecture: shared bus + per-chamber distribution via equal-length serpentine channels

/// FCB overall dimensions (mm)
pub const FCB_LENGTH: f64 = 120.0; // X axis (left-to-right)
pub const FCB_WIDTH: f64 = 80.0; // Y axis (front-to-back)
pub const FCB_TOTAL_THICKNESS: f64 = 6.0; // two bonded 3mm plates
pub const FCB_PLATE_THICKNESS: f64 = 3.0; // each plate

/// Number of cell culture chambers on the chip above
pub const FCB_NUM_CHAMBERS: usize = 10;

/// Chamber spacing on the chip (center-to-center, along X)
pub const FCB_CHAMBER_SPACING: f64 = 6.0;

/// Chip dimensions (the chip that sits on top)
pub const FCB_CHIP_LENGTH: f64 = 75.0;
pub const FCB_CHIP_WIDTH: f64 = 25.0;

/// Port positions on the chip — inset from short edges (Y axis)
pub const FCB_CHIP_PORT_INSET: f64 = 5.0; // mm from chip short edge

/// Port diameter (through-holes, matches chip)
pub const FCB_PORT_DIAMETER: f64 = 1.5;

/// O-ring groove around each chip-interface port (top face)
pub const FCB_ORING_GROOVE_OD: f64 = 3.5; // outer diameter of groove
pub const FCB_ORING_GROOVE_WIDTH: f64 = 0.75; // groove annular width
pub const FCB_ORING_GROOVE_DEPTH: f64 = 0.5; // groove depth into top face

/// Main bus channel (shared among all 4 fluid inputs)
pub const FCB_BUS_WIDTH: f64 = 0.5; // 500μm
pub const FCB_BUS_DEPTH: f64 = 0.3; // 300μm

/// Distribution channels (bus to each chamber inlet)
pub const FCB_DIST_WIDTH: f64 = 0.5; // 500μm
pub const FCB_DIST_DEPTH: f64 = 0.2; // 200μm

/// Outlet collection channels
pub const FCB_OUTLET_WIDTH: f64 = 0.5; // 500μm
pub const FCB_OUTLET_DEPTH: f64 = 0.2; // 200μm

/// Fluid input ports (left edge of FCB)
pub const FCB_NUM_INPUTS: usize = 4; // Media, AAV, PBS, Trigger
pub const FCB_INPUT_SPACING: f64 = 10.0; // mm between input ports
pub const FCB_INPUT_PORT_DIAMETER: f64 = 1.5;
pub const FCB_INPUT_EDGE_INSET: f64 = 5.0; // mm from left edge to port center

/// Outlet collection ports (right edge of FCB)
pub const FCB_NUM_OUTPUTS: usize = 2; // Waste + Sample collection
pub const FCB_OUTPUT_SPACING: f64 = 10.0;
pub const FCB_OUTPUT_PORT_DIAMETER: f64 = 1.5;
pub const FCB_OUTPUT_EDGE_INSET: f64 = 5.0; // mm from right edge

/// Valve interface ports (bottom edge of FCB)
pub const FCB_VALVE_PORT_DIAMETER: f64 = 1.5;
pub const FCB_VALVE_EDGE_INSET: f64 = 5.0; // mm from bottom edge

/// Mounting holes — M3 clearance
pub const FCB_MOUNT_DIAMETER: f64 = 3.2; // M3 clearance hole
pub const FCB_CORNER_MOUNT_INSET: f64 = 5.0; // from edges, for chip clamp
pub const FCB_BASE_MOUNT_INSET: f64 = 5.0; // from edges, for baseplate

/// Compute the X position of the first (leftmost) chamber center
/// 10 chambers at 6mm spacing = 54mm span, centered on chip
pub fn fcb_first_chamber_x() -> f64 {
    -((FCB_NUM_CHAMBERS as f64 - 1.0) * FCB_CHAMBER_SPACING) / 2.0 // -27.0mm
}

/// Compute the Y positions of inlet/outlet ports on the chip
/// Chip is 25mm wide (Y), ports are 5mm from each short edge
pub fn fcb_chip_inlet_y() -> f64 {
    -(FCB_CHIP_WIDTH / 2.0 - FCB_CHIP_PORT_INSET) // -7.5mm from chip center
}

pub fn fcb_chip_outlet_y() -> f64 {
    FCB_CHIP_WIDTH / 2.0 - FCB_CHIP_PORT_INSET // +7.5mm from chip center
}

// ─── Fluidic Circuit Board Constants (v2 — 16-chamber, well-plate chip) ───
// Two-plate PMMA construction: bottom plate has channels, top plate has through-holes
// FCB sits below the 16-chamber ANSI/SLAS chip (127.76 × 85.48 mm)
// Architecture: shared bus + column trunks + per-chamber valve-inline distribution
// 4×4 grid: 4 columns (X) × 4 rows (Y), matching chip layout

/// FCB16 overall dimensions (mm) — sized for ANSI/SLAS chip + edge ports
pub const FCB16_LENGTH: f64 = 180.0; // X axis
pub const FCB16_WIDTH: f64 = 140.0; // Y axis
pub const FCB16_PLATE_THICKNESS: f64 = 3.0; // each plate
pub const FCB16_TOTAL_THICKNESS: f64 = 6.0; // two bonded plates

/// Chip that sits on top (16-chamber ANSI/SLAS)
pub const FCB16_CHIP_LENGTH: f64 = CNC16_CHIP_LENGTH; // 127.76mm
pub const FCB16_CHIP_WIDTH: f64 = CNC16_CHIP_WIDTH; // 85.48mm

/// Grid layout (matches chip)
pub const FCB16_NUM_CHAMBERS: usize = 16;
pub const FCB16_GRID_COLS: usize = 4;
pub const FCB16_GRID_ROWS: usize = 4;
pub const FCB16_COL_SPACING: f64 = CNC16_COL_SPACING; // 25.0mm
pub const FCB16_ROW_SPACING: f64 = CNC16_ROW_SPACING; // 18.0mm

/// Channel from port to chamber edge on chip
pub const FCB16_CHANNEL_LENGTH: f64 = CNC16_CHANNEL_LENGTH; // 5.0mm

/// Port diameter (through-holes, matches chip)
pub const FCB16_PORT_DIAMETER: f64 = 1.5;

/// O-ring groove around each chip-interface port (top face of FCB)
pub const FCB16_ORING_GROOVE_OD: f64 = 3.5;
pub const FCB16_ORING_GROOVE_WIDTH: f64 = 0.75;
pub const FCB16_ORING_GROOVE_DEPTH: f64 = 0.5;

/// Main bus channel (shared among all 4 fluid inputs)
pub const FCB16_BUS_WIDTH: f64 = 0.5; // 500μm
pub const FCB16_BUS_DEPTH: f64 = 0.3; // 300μm

/// Distribution channels (bus → valve → inlet)
pub const FCB16_DIST_WIDTH: f64 = 0.5; // 500μm
pub const FCB16_DIST_DEPTH: f64 = 0.2; // 200μm

/// Outlet collection channels
pub const FCB16_OUTLET_WIDTH: f64 = 0.5; // 500μm
pub const FCB16_OUTLET_DEPTH: f64 = 0.2; // 200μm

/// Fluid input ports (left edge)
pub const FCB16_NUM_INPUTS: usize = 4; // Media, AAV, PBS, Trigger
pub const FCB16_INPUT_SPACING: f64 = 15.0;
pub const FCB16_INPUT_PORT_DIAMETER: f64 = 1.5;
pub const FCB16_INPUT_EDGE_INSET: f64 = 5.0; // from left edge

/// Outlet collection ports (right edge)
pub const FCB16_NUM_OUTPUTS: usize = 2; // Waste + Sample
pub const FCB16_OUTPUT_SPACING: f64 = 15.0;
pub const FCB16_OUTPUT_PORT_DIAMETER: f64 = 1.5;
pub const FCB16_OUTPUT_EDGE_INSET: f64 = 5.0; // from right edge

/// Valve ports (bottom edge, 16 pairs for 16 chambers)
pub const FCB16_VALVE_PORT_DIAMETER: f64 = 1.5;
pub const FCB16_VALVE_EDGE_INSET: f64 = 5.0; // from bottom edge
pub const FCB16_VALVE_PAIR_OFFSET: f64 = 2.0; // between OUT and RETURN in a pair
pub const FCB16_VALVE_ROW_SPACING: f64 = 5.0; // between pairs within a column group

/// Mounting holes — M3 clearance
pub const FCB16_MOUNT_DIAMETER: f64 = 3.2;
pub const FCB16_CHIP_CLAMP_INSET: f64 = 8.0; // from chip edge, for clamp bolts
pub const FCB16_BASE_MOUNT_INSET: f64 = 6.0; // from FCB edges

/// Column X centers (from chip design, relative to FCB/chip center)
pub fn fcb16_col_xs() -> [f64; 4] {
    let half_span = (FCB16_GRID_COLS as f64 - 1.0) * FCB16_COL_SPACING / 2.0; // 37.5
    [
        -half_span,
        -half_span + FCB16_COL_SPACING,
        -half_span + 2.0 * FCB16_COL_SPACING,
        -half_span + 3.0 * FCB16_COL_SPACING,
    ]
}

/// Row Y centers (from chip design, relative to FCB/chip center)
pub fn fcb16_row_ys() -> [f64; 4] {
    let half_span = (FCB16_GRID_ROWS as f64 - 1.0) * FCB16_ROW_SPACING / 2.0; // 27.0
    [
        half_span,                    // row 0: +27.0
        half_span - FCB16_ROW_SPACING,     // row 1: +9.0
        half_span - 2.0 * FCB16_ROW_SPACING, // row 2: -9.0
        half_span - 3.0 * FCB16_ROW_SPACING, // row 3: -27.0
    ]
}

/// Inlet port Y for a given row (center_y - half_chamber - channel_length)
pub fn fcb16_inlet_y(row_center_y: f64) -> f64 {
    row_center_y - CNC16_CHAMBER_LENGTH / 2.0 - FCB16_CHANNEL_LENGTH // center - 5 - 5 = center - 10
}

/// Outlet port Y for a given row
pub fn fcb16_outlet_y(row_center_y: f64) -> f64 {
    row_center_y + CNC16_CHAMBER_LENGTH / 2.0 + FCB16_CHANNEL_LENGTH // center + 5 + 5 = center + 10
}
