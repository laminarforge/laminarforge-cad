//! Machine-readable A0 interface contract for the active 16-slot cassette.
//!
//! The STL fit-check generator and the STEP draft generator both consume these
//! values. Keep biological procedures and vendor-specific selections out of
//! this module; it controls only shared mechanical interfaces and reservations.

use crate::{REVC_CHIP_LENGTH, REVC_CHIP_WIDTH, REVC_TOTAL_HEIGHT};

pub const COLS: usize = 4;
pub const ROWS: usize = 4;
pub const SLOT_COUNT: usize = COLS * ROWS;

pub const CHIP_GUTTER_X: f64 = 24.0;
pub const CHIP_GUTTER_Y: f64 = 24.0;
pub const SLOT_PITCH_X: f64 = REVC_CHIP_LENGTH + CHIP_GUTTER_X;
pub const SLOT_PITCH_Y: f64 = REVC_CHIP_WIDTH + CHIP_GUTTER_Y;
pub const SLOT_ARRAY_X: f64 = COLS as f64 * REVC_CHIP_LENGTH + (COLS as f64 - 1.0) * CHIP_GUTTER_X;
pub const SLOT_ARRAY_Y: f64 = ROWS as f64 * REVC_CHIP_WIDTH + (ROWS as f64 - 1.0) * CHIP_GUTTER_Y;

pub const CARRIER_MARGIN_X: f64 = 58.0;
pub const CARRIER_MARGIN_Y: f64 = 64.0;
pub const CARRIER_X: f64 = SLOT_ARRAY_X + CARRIER_MARGIN_X * 2.0;
pub const CARRIER_Y: f64 = SLOT_ARRAY_Y + CARRIER_MARGIN_Y * 2.0;
pub const CARRIER_Z: f64 = 24.0;
pub const CHIP_CLEARANCE: f64 = 1.20;
pub const DRAWING_TARGET_CHIP_CLEARANCE: f64 = 0.80;
pub const CHIP_POCKET_DEPTH: f64 = 7.0;
pub const TOP_FACE_CSG_OVERLAP: f64 = 0.20;
pub const CHIP_PROTRUSION_ABOVE_CARRIER: f64 = REVC_TOTAL_HEIGHT - CHIP_POCKET_DEPTH;
pub const OPTICAL_WINDOW_MARGIN: f64 = 24.0;

pub const GASKET_LAND_OVERHANG: f64 = 18.0;
pub const GASKET_LAND_W: f64 = 8.0;
pub const PER_SLOT_GASKET_OUTER_X: f64 = REVC_CHIP_LENGTH + GASKET_LAND_OVERHANG;
pub const PER_SLOT_GASKET_OUTER_Y: f64 = REVC_CHIP_WIDTH + GASKET_LAND_OVERHANG;
pub const PER_SLOT_GASKET_INNER_X: f64 = PER_SLOT_GASKET_OUTER_X - 2.0 * GASKET_LAND_W;
pub const PER_SLOT_GASKET_INNER_Y: f64 = PER_SLOT_GASKET_OUTER_Y - 2.0 * GASKET_LAND_W;
pub const PER_SLOT_GASKET_ARRAY_X: f64 =
    (COLS as f64 - 1.0) * SLOT_PITCH_X + PER_SLOT_GASKET_OUTER_X;
pub const PER_SLOT_GASKET_ARRAY_Y: f64 =
    (ROWS as f64 - 1.0) * SLOT_PITCH_Y + PER_SLOT_GASKET_OUTER_Y;

pub const PERIMETER_GASKET_W: f64 = 12.0;
pub const PERIMETER_TO_SLOT_LAND_CLEARANCE: f64 = 2.0;
pub const PERIMETER_GASKET_INNER_X: f64 =
    PER_SLOT_GASKET_ARRAY_X + 2.0 * PERIMETER_TO_SLOT_LAND_CLEARANCE;
pub const PERIMETER_GASKET_INNER_Y: f64 =
    PER_SLOT_GASKET_ARRAY_Y + 2.0 * PERIMETER_TO_SLOT_LAND_CLEARANCE;
pub const PERIMETER_GASKET_OUTER_X: f64 = PERIMETER_GASKET_INNER_X + 2.0 * PERIMETER_GASKET_W;
pub const PERIMETER_GASKET_OUTER_Y: f64 = PERIMETER_GASKET_INNER_Y + 2.0 * PERIMETER_GASKET_W;

pub const GASKET_FREE_HEIGHT: f64 = 2.40;
pub const GASKET_TARGET_SQUEEZE: f64 = 0.25;
pub const GASKET_COMPRESSED_HEIGHT: f64 = GASKET_FREE_HEIGHT * (1.0 - GASKET_TARGET_SQUEEZE);
pub const GASKET_GUARD_MIN_SQUEEZE: f64 = 0.20;
pub const GASKET_GUARD_MAX_SQUEEZE: f64 = 0.30;
pub const GASKET_GUARD_MAX_COMPRESSED_HEIGHT: f64 =
    GASKET_FREE_HEIGHT * (1.0 - GASKET_GUARD_MIN_SQUEEZE);
pub const GASKET_GUARD_MIN_COMPRESSED_HEIGHT: f64 =
    GASKET_FREE_HEIGHT * (1.0 - GASKET_GUARD_MAX_SQUEEZE);
pub const GASKET_GROOVE_DEPTH: f64 = GASKET_COMPRESSED_HEIGHT;
pub const GASKET_GROOVE_W: f64 = 3.20;
pub const GASKET_GROOVE_CSG_OVERLAP: f64 = 0.10;
pub const GASKET_GROOVE_CUT_HEIGHT: f64 = GASKET_GROOVE_DEPTH + GASKET_GROOVE_CSG_OVERLAP;
pub const GASKET_ENTRY_BREAK_RADIUS_NOTE: f64 = 0.20;
pub const SEAL_BAND_RA_TARGET_UM: f64 = 0.8;
pub const SEAL_BAND_RA_MAX_UM: f64 = 1.6;
pub const GASKET_LAND_Z: f64 = CHIP_PROTRUSION_ABOVE_CARRIER;
pub const CLOSURE_PLANE_ABOVE_CARRIER: f64 = CHIP_PROTRUSION_ABOVE_CARRIER;

pub const LEAK_GUTTER_SEPARATING_WEB: f64 = 5.0;
pub const LEAK_GUTTER_W: f64 = 6.0;
pub const LEAK_GUTTER_DEPTH: f64 = 3.0;
pub const PERIMETER_STOP_W: f64 = 4.0;
pub const PERIMETER_STOP_CENTER_OFFSET: f64 = LEAK_GUTTER_SEPARATING_WEB / 2.0;
pub const INTERNAL_STOP_DIAMETER: f64 = 4.0;
pub const LEAK_GUTTER_INNER_X: f64 = PERIMETER_GASKET_OUTER_X + 2.0 * LEAK_GUTTER_SEPARATING_WEB;
pub const LEAK_GUTTER_INNER_Y: f64 = PERIMETER_GASKET_OUTER_Y + 2.0 * LEAK_GUTTER_SEPARATING_WEB;
pub const LEAK_GUTTER_OUTER_X: f64 = LEAK_GUTTER_INNER_X + 2.0 * LEAK_GUTTER_W;
pub const LEAK_GUTTER_OUTER_Y: f64 = LEAK_GUTTER_INNER_Y + 2.0 * LEAK_GUTTER_W;
pub const CARRIER_DRAIN_DIAMETER: f64 = 8.0;
pub const CARRIER_DRAIN_LENGTH: f64 = 40.0;
pub const CARRIER_DRAIN_X: f64 = CARRIER_X / 2.0 - 34.0;
pub const CARRIER_DRAIN_Y: f64 = -CARRIER_Y / 2.0 + 16.0;
pub const CARRIER_DRAIN_Z: f64 = CARRIER_Z / 2.0 - 2.0;

pub const LID_X: f64 = CARRIER_X + 18.0;
pub const LID_Y: f64 = CARRIER_Y + 18.0;
pub const LID_Z: f64 = 10.0;
pub const LID_UNDERSIDE_SEAL_SKIN_Z: f64 = 4.0;
pub const LID_UPPER_FRAME_Z: f64 = LID_Z - LID_UNDERSIDE_SEAL_SKIN_Z;
pub const LID_UPPER_RELIEF_X: f64 = SLOT_ARRAY_X + 52.0;
pub const LID_UPPER_RELIEF_Y: f64 = SLOT_ARRAY_Y + 44.0;
pub const LID_SLOT_VIEW_OPENING_X: f64 = REVC_CHIP_LENGTH - 14.0;
pub const LID_SLOT_VIEW_OPENING_Y: f64 = REVC_CHIP_WIDTH - 14.0;
pub const LID_CHIP_TOP_CLEARANCE: f64 = CHIP_CLEARANCE;
pub const LID_CHIP_TOP_RELIEF_X: f64 = REVC_CHIP_LENGTH + 2.0 * LID_CHIP_TOP_CLEARANCE;
pub const LID_CHIP_TOP_RELIEF_Y: f64 = REVC_CHIP_WIDTH + 2.0 * LID_CHIP_TOP_CLEARANCE;
pub const LID_CHIP_TOP_RELIEF_DEPTH: f64 = 0.50;
pub const MIN_LID_GROOVE_FLOOR_Z: f64 = 2.0;
pub const LID_GROOVE_FLOOR_Z: f64 = LID_UNDERSIDE_SEAL_SKIN_Z - GASKET_GROOVE_DEPTH;
pub const WINDOW_X: f64 = SLOT_ARRAY_X + 84.0;
pub const WINDOW_Y: f64 = SLOT_ARRAY_Y + 76.0;
pub const WINDOW_Z: f64 = 3.0;
pub const WINDOW_RAISED_FEATURE_CSG_OVERLAP: f64 = 0.20;
pub const WINDOW_WITNESS_FRAME_EXPOSED_Z: f64 = 1.20;
pub const WINDOW_WITNESS_FRAME_BODY_Z: f64 =
    WINDOW_WITNESS_FRAME_EXPOSED_Z + WINDOW_RAISED_FEATURE_CSG_OVERLAP;
pub const WINDOW_WITNESS_FRAME_CENTER_Z: f64 =
    WINDOW_Z / 2.0 + WINDOW_WITNESS_FRAME_EXPOSED_Z / 2.0 - WINDOW_RAISED_FEATURE_CSG_OVERLAP / 2.0;
pub const WINDOW_RETENTION_TAB_EXPOSED_Z: f64 = 1.40;
pub const WINDOW_RETENTION_TAB_BODY_Z: f64 =
    WINDOW_RETENTION_TAB_EXPOSED_Z + WINDOW_RAISED_FEATURE_CSG_OVERLAP;
pub const WINDOW_RETENTION_TAB_CENTER_Z: f64 =
    WINDOW_Z / 2.0 + WINDOW_RETENTION_TAB_EXPOSED_Z / 2.0 - WINDOW_RAISED_FEATURE_CSG_OVERLAP / 2.0;
pub const WINDOW_FIDUCIAL_EXPOSED_Z: f64 = 1.80;
pub const WINDOW_FIDUCIAL_CSG_OVERLAP: f64 = 0.20;
pub const WINDOW_FIDUCIAL_BODY_Z: f64 = WINDOW_FIDUCIAL_EXPOSED_Z + WINDOW_FIDUCIAL_CSG_OVERLAP;
pub const WINDOW_FIDUCIAL_CENTER_Z: f64 =
    WINDOW_Z / 2.0 + WINDOW_FIDUCIAL_EXPOSED_Z / 2.0 - WINDOW_FIDUCIAL_CSG_OVERLAP / 2.0;
pub const WINDOW_FIDUCIAL_HOLE_CUT_Z: f64 = WINDOW_FIDUCIAL_BODY_Z + 0.20;

pub const PER_SLOT_GASKET_GROOVE_OUTER_X: f64 =
    PER_SLOT_GASKET_OUTER_X - (GASKET_LAND_W - GASKET_GROOVE_W);
pub const PER_SLOT_GASKET_GROOVE_OUTER_Y: f64 =
    PER_SLOT_GASKET_OUTER_Y - (GASKET_LAND_W - GASKET_GROOVE_W);
pub const PER_SLOT_GASKET_GROOVE_INNER_X: f64 =
    PER_SLOT_GASKET_GROOVE_OUTER_X - 2.0 * GASKET_GROOVE_W;
pub const PER_SLOT_GASKET_GROOVE_INNER_Y: f64 =
    PER_SLOT_GASKET_GROOVE_OUTER_Y - 2.0 * GASKET_GROOVE_W;
pub const PER_SLOT_GASKET_GROOVE_ARRAY_X: f64 =
    (COLS as f64 - 1.0) * SLOT_PITCH_X + PER_SLOT_GASKET_GROOVE_OUTER_X;
pub const PER_SLOT_GASKET_GROOVE_ARRAY_Y: f64 =
    (ROWS as f64 - 1.0) * SLOT_PITCH_Y + PER_SLOT_GASKET_GROOVE_OUTER_Y;
pub const PERIMETER_GASKET_GROOVE_OUTER_X: f64 =
    PERIMETER_GASKET_OUTER_X - (PERIMETER_GASKET_W - GASKET_GROOVE_W);
pub const PERIMETER_GASKET_GROOVE_OUTER_Y: f64 =
    PERIMETER_GASKET_OUTER_Y - (PERIMETER_GASKET_W - GASKET_GROOVE_W);
pub const PERIMETER_GASKET_GROOVE_INNER_X: f64 =
    PERIMETER_GASKET_GROOVE_OUTER_X - 2.0 * GASKET_GROOVE_W;
pub const PERIMETER_GASKET_GROOVE_INNER_Y: f64 =
    PERIMETER_GASKET_GROOVE_OUTER_Y - 2.0 * GASKET_GROOVE_W;
pub const LID_CHIP_RELIEF_TO_GROOVE_MARGIN_X: f64 =
    (PER_SLOT_GASKET_GROOVE_INNER_X - LID_CHIP_TOP_RELIEF_X) / 2.0;
pub const LID_CHIP_RELIEF_TO_GROOVE_MARGIN_Y: f64 =
    (PER_SLOT_GASKET_GROOVE_INNER_Y - LID_CHIP_TOP_RELIEF_Y) / 2.0;
pub const ADJACENT_SLOT_GROOVE_GAP_X: f64 = SLOT_PITCH_X - PER_SLOT_GASKET_GROOVE_OUTER_X;
pub const ADJACENT_SLOT_GROOVE_GAP_Y: f64 = SLOT_PITCH_Y - PER_SLOT_GASKET_GROOVE_OUTER_Y;
pub const SLOT_ARRAY_TO_PERIMETER_GROOVE_GAP_X: f64 =
    (PERIMETER_GASKET_GROOVE_INNER_X - PER_SLOT_GASKET_GROOVE_ARRAY_X) / 2.0;
pub const SLOT_ARRAY_TO_PERIMETER_GROOVE_GAP_Y: f64 =
    (PERIMETER_GASKET_GROOVE_INNER_Y - PER_SLOT_GASKET_GROOVE_ARRAY_Y) / 2.0;
pub const PERIMETER_GROOVE_TO_LID_EDGE_X: f64 = (LID_X - PERIMETER_GASKET_GROOVE_OUTER_X) / 2.0;
pub const PERIMETER_GROOVE_TO_LID_EDGE_Y: f64 = (LID_Y - PERIMETER_GASKET_GROOVE_OUTER_Y) / 2.0;

pub const COUPON_X: f64 = 250.0;
pub const COUPON_Y: f64 = 118.0;
pub const COUPON_Z: f64 = 12.0;
pub const COUPON_LEAK_LOOP_X: f64 = 144.0;
pub const COUPON_LEAK_LOOP_Y: f64 = 34.0;
pub const COUPON_LEAK_LOOP_CENTER_X: f64 = -45.0;
pub const COUPON_RECONNECTION_LOOP_X: f64 = 86.0;
pub const COUPON_RECONNECTION_LOOP_Y: f64 = 34.0;
pub const COUPON_RECONNECTION_LOOP_CENTER_X: f64 = 75.0;
pub const COUPON_LOOP_CENTER_Y: f64 = 26.0;
pub const COUPON_SAMPLE_SLOT_X: f64 = 216.0;
pub const COUPON_SAMPLE_SLOT_Y: f64 = 8.0;
pub const COUPON_SAMPLE_SLOT_CENTER_Y: f64 = -10.0;
pub const COUPON_SQUEEZE_STEP_X: f64 = 54.0;
pub const COUPON_SQUEEZE_STEP_Y: f64 = 16.0;
pub const COUPON_SQUEEZE_STEP_CENTER_Y: f64 = -34.0;
pub const COUPON_LABEL_LAND_Y: f64 = 8.0;
pub const COUPON_LABEL_CENTER_Y: f64 = -53.0;
pub const COUPON_STOP_X: f64 = 116.0;

pub const CHIP_FIT_COUPON_X: f64 = 168.0;
pub const CHIP_FIT_COUPON_Y: f64 = 126.0;
pub const CHIP_FIT_COUPON_OVERALL_Z: f64 = CARRIER_Z + CLOSURE_PLANE_ABOVE_CARRIER;

pub const DOCK_X: f64 = CARRIER_X + 170.0;
pub const DOCK_Y: f64 = CARRIER_Y + 150.0;
pub const DOCK_Z: f64 = 22.0;
pub const DOCK_SUPPORT_PLANE_Z: f64 = DOCK_Z / 2.0;
pub const DOCK_THROUGH_CUT_OVERTRAVEL_Z: f64 = 1.0;
pub const DOCK_THROUGH_CUT_HEIGHT: f64 = DOCK_Z + 2.0 * DOCK_THROUGH_CUT_OVERTRAVEL_Z;
pub const DOCK_THROUGH_CUT_CENTER_Z: f64 = 0.0;
pub const DOCK_RAIL_Z: f64 = 18.0;
pub const DOCK_RAIL_W: f64 = 16.0;
pub const DOCK_REAR_RAIL_X: f64 = CARRIER_X + 44.0;
pub const DOCK_LEFT_RAIL_Y: f64 = CARRIER_Y + 46.0;
pub const DOCK_FRONT_LIP_W: f64 = 10.0;
pub const DOCK_FRONT_LIP_X: f64 = CARRIER_X + 44.0;
pub const DOCK_FRONT_LIP_Z: f64 = 10.0;
pub const DOCK_REAR_RAIL_CENTER_Y: f64 = CARRIER_Y / 2.0 + DOCK_RAIL_W / 2.0;
pub const DOCK_LEFT_RAIL_CENTER_X: f64 = -(CARRIER_X / 2.0 + DOCK_RAIL_W / 2.0);
pub const DOCK_FRONT_LIP_CENTER_Y: f64 = -(CARRIER_Y / 2.0 + DOCK_FRONT_LIP_W / 2.0);
pub const SLOT_RECESS_DEPTH: f64 = 5.5;
pub const DOCK_SLOT_RECESS_FLOOR_Z: f64 = DOCK_SUPPORT_PLANE_Z - SLOT_RECESS_DEPTH;
pub const DOCK_SLOT_RECESS_X: f64 = REVC_CHIP_LENGTH + 10.0;
pub const DOCK_SLOT_RECESS_Y: f64 = REVC_CHIP_WIDTH + 10.0;
pub const DOCK_AIR_BYPASS_X: f64 = SLOT_ARRAY_X + 74.0;
pub const DOCK_AIR_BYPASS_Y: f64 = 8.0;
pub const DOCK_FRONT_DRAIN_OPENING_X: f64 = DOCK_X - 70.0;
pub const DOCK_FRONT_DRAIN_OPENING_Y: f64 = 10.0;
pub const DOCK_FRONT_DRAIN_OPENING_CENTER_Y: f64 = -DOCK_Y / 2.0 + 38.0;
pub const DOCK_SIDE_DRAIN_OPENING_X: f64 = 10.0;
pub const DOCK_SIDE_DRAIN_OPENING_Y: f64 = DOCK_Y - 76.0;
pub const DOCK_SIDE_DRAIN_OPENING_CENTER_X: f64 = DOCK_X / 2.0 - 42.0;
pub const DOCK_DRAIN_VISIBILITY_OPENING_X: f64 = 58.0;
pub const DOCK_DRAIN_VISIBILITY_OPENING_Y: f64 = 38.0;
pub const DOCK_DRAIN_VISIBILITY_OPENING_CENTER_X: f64 = DOCK_X / 2.0 - 58.0;
pub const DOCK_DRAIN_VISIBILITY_OPENING_CENTER_Y: f64 = -DOCK_Y / 2.0 + 58.0;
pub const DOCK_POSITION_TOKEN_X: f64 = 24.0;
pub const DOCK_POSITION_TOKEN_Y: f64 = 10.0;
pub const DOCK_POSITION_TOKEN_Z: f64 = 3.0;
pub const DOCK_POSITION_TOKEN_Y_OFFSET: f64 = REVC_CHIP_WIDTH / 2.0 + 16.0;
pub const DOCK_LOGGER_RESERVATION_LAND_X: f64 = 48.0;
pub const DOCK_LOGGER_RESERVATION_LAND_Y: f64 = 32.0;
pub const DOCK_LOGGER_RESERVATION_LAND_Z: f64 = 8.0;
pub const DOCK_LOGGER_RESERVATION_LAND_X_OFFSET: f64 = CARRIER_X / 2.0 + 34.0;
pub const DOCK_LOGGER_RESERVATION_LAND_Y_OFFSET: f64 = CARRIER_Y / 2.0 - 30.0;
pub const DOCK_ROBOT_LIFT_X: f64 = 160.0;
pub const DOCK_ROBOT_LIFT_Y: f64 = 20.0;
pub const DOCK_ROBOT_LIFT_Z: f64 = 7.0;
pub const DOCK_ROBOT_LIFT_EDGE_INSET_Y: f64 = 74.0;
pub const DOCK_LEVELING_PAD_RADIUS: f64 = 16.0;
pub const DOCK_LEVELING_PAD_Z: f64 = 3.0;
pub const DOCK_LEVELING_PAD_EDGE_INSET: f64 = 42.0;
pub const PERIMETER_MOUNT_EDGE_OFFSET: f64 = 22.0;
pub const PERIMETER_MOUNT_HOLE_DIAMETER: f64 = 5.4;

pub const BULKHEAD_X: f64 = CARRIER_X + 90.0;
pub const BULKHEAD_Y: f64 = 34.0;
pub const BULKHEAD_Z: f64 = 76.0;
pub const BULKHEAD_OFFSET_Y: f64 = DOCK_Y / 2.0 + BULKHEAD_Y / 2.0 + 18.0;
pub const GAS_PORT_XS: [f64; 4] = [-240.0, -210.0, -180.0, -150.0];
pub const MEDIA_PORT_XS: [f64; 7] = [-78.0, -52.0, -26.0, 0.0, 26.0, 52.0, 78.0];
pub const WASTE_PORT_XS: [f64; 5] = [150.0, 176.0, 202.0, 228.0, 254.0];
pub const GAS_PORT_DIAMETER: f64 = 8.0;
pub const MEDIA_WASTE_PORT_DIAMETER: f64 = 6.4;
pub const SENSOR_CONNECTOR_CENTER_X: f64 = 320.0;
pub const SENSOR_CONNECTOR_CENTER_Z: f64 = 18.0;
pub const SENSOR_CONNECTOR_X: f64 = 88.0;
pub const SENSOR_CONNECTOR_Z: f64 = 18.0;

pub const LID_FASTENER_OFFSET_FROM_GUTTER: f64 = 7.0;
pub const LID_FASTENER_SIDE_X: f64 = LEAK_GUTTER_OUTER_X / 2.0 + LID_FASTENER_OFFSET_FROM_GUTTER;
pub const LID_FASTENER_FRONT_REAR_Y: f64 =
    LEAK_GUTTER_OUTER_Y / 2.0 + LID_FASTENER_OFFSET_FROM_GUTTER;
pub const LID_FASTENER_CLEARANCE_DIAMETER: f64 = 4.8;
pub const LID_RETAINER_DIAMETER: f64 = 10.8;
pub const CARRIER_LID_RECEIVER_DIAMETER: f64 = 3.3;

pub const DATUM_BOSS_X: f64 = CARRIER_X / 2.0 - 14.0;
pub const DATUM_BOSS_Y: f64 = CARRIER_Y / 2.0 - 66.0;
pub const DATUM_BOSS_DIAMETER: f64 = 18.0;
pub const DATUM_D1_BORE_DIAMETER: f64 = 6.0;
pub const DATUM_D2_SLOT_LENGTH: f64 = 10.0;
pub const DATUM_D2_SLOT_WIDTH: f64 = 6.0;
pub const DATUM_WITNESS_BORE_DIAMETER: f64 = 9.0;
pub const DATUM_BOSS_Z: f64 = 6.0;
pub const SIDE_SERVICE_DATUM_CLEARANCE: f64 = 2.0;
pub const LID_DATUM_PIN_DIAMETER: f64 = 5.80;
pub const LID_DATUM_PIN_ENGAGEMENT: f64 = 2.0;
pub const LID_DATUM_PIN_EXTENSION: f64 =
    CLOSURE_PLANE_ABOVE_CARRIER - DATUM_BOSS_Z + LID_DATUM_PIN_ENGAGEMENT;
pub const LID_DATUM_PIN_EMBEDMENT: f64 = LID_UNDERSIDE_SEAL_SKIN_Z;
pub const LID_DATUM_PIN_TOTAL_Z: f64 = LID_DATUM_PIN_EXTENSION + LID_DATUM_PIN_EMBEDMENT;
pub const LID_DATUM_PIN_SEAT_DIAMETER: f64 = DATUM_D1_BORE_DIAMETER;
pub const LID_DATUM_PIN_SEAT_DEPTH: f64 = LID_DATUM_PIN_EMBEDMENT;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DatumRole {
    RoundLocator,
    RelievedLocator,
    ClearanceWitness,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DatumFeature {
    pub id: &'static str,
    pub x: f64,
    pub y: f64,
    pub role: DatumRole,
}

pub const GLOBAL_BARCODE_LAND_X: f64 = 96.0;
pub const GLOBAL_BARCODE_LAND_Y: f64 = 12.0;
pub const GLOBAL_BARCODE_CENTER_X: f64 = -CARRIER_X / 2.0 + 78.0;
pub const GLOBAL_BARCODE_CENTER_Y: f64 = -CARRIER_Y / 2.0 + 7.0;
pub const GLOBAL_TEXT_LAND_X: f64 = 118.0;
pub const GLOBAL_TEXT_LAND_Y: f64 = 10.0;
pub const GLOBAL_TEXT_CENTER_X: f64 = -CARRIER_X / 2.0 + 190.0;
pub const GLOBAL_TEXT_CENTER_Y: f64 = GLOBAL_BARCODE_CENTER_Y;
pub const SLOT_LABEL_LAND_X: f64 = 26.0;
pub const SLOT_LABEL_LAND_Y: f64 = 10.0;
pub const SLOT_LABEL_PITCH_X: f64 = 36.0;
pub const SLOT_LABEL_CENTER_Y: f64 = CARRIER_Y / 2.0 - 15.0;

pub const OPTICAL_KEEP_OUT_X: f64 = REVC_CHIP_LENGTH - OPTICAL_WINDOW_MARGIN;
pub const OPTICAL_KEEP_OUT_Y: f64 = REVC_CHIP_WIDTH - OPTICAL_WINDOW_MARGIN;
pub const WINDOW_FIDUCIAL_X: f64 = SLOT_ARRAY_X / 2.0 + 24.0;
pub const WINDOW_FIDUCIAL_Y: f64 = SLOT_ARRAY_Y / 2.0 + 22.0;

pub const CARRIER_BASE_BODY_Z: f64 = CARRIER_Z;
pub const SIDE_SERVICE_RELIEF_Z: f64 = 7.0;
pub const CARRIER_OVERALL_Z: f64 = CARRIER_Z + CLOSURE_PLANE_ABOVE_CARRIER;
pub const LID_BASE_BODY_Z: f64 = LID_Z;
pub const LID_OVERALL_Z: f64 = LID_Z + 1.6;
pub const WINDOW_BASE_PANEL_Z: f64 = WINDOW_Z;
pub const WINDOW_OVERALL_Z: f64 = WINDOW_Z + 1.8;
pub const COUPON_OVERALL_Z: f64 = COUPON_Z + 2.0;
pub const DOCK_BASE_BODY_Z: f64 = DOCK_Z;
pub const DOCK_OVERALL_Z: f64 = DOCK_Z + DOCK_RAIL_Z;
pub const BULKHEAD_BASE_BODY_Y: f64 = BULKHEAD_Y;
pub const BULKHEAD_OVERALL_Y: f64 = 60.0;
pub const ASSEMBLY_OVERALL_X: f64 = DOCK_X;
pub const ASSEMBLY_OVERALL_Y: f64 =
    DOCK_Y / 2.0 + COUPON_Y + 38.0 + BULKHEAD_OFFSET_Y + BULKHEAD_Y / 2.0;
pub const ASSEMBLY_OVERALL_Z: f64 = DOCK_Z + BULKHEAD_Z;
pub const CASSETTE_CLOSED_STACK_Z: f64 = CARRIER_Z + CLOSURE_PLANE_ABOVE_CARRIER + LID_Z;

pub fn centered_index(index: usize, count: usize, pitch: f64) -> f64 {
    (index as f64 - (count as f64 - 1.0) / 2.0) * pitch
}

pub fn bottom_face_gasket_groove_cut_z(body_z: f64) -> f64 {
    -body_z / 2.0 + (GASKET_GROOVE_DEPTH - GASKET_GROOVE_CSG_OVERLAP) / 2.0
}

pub fn top_face_gasket_groove_cut_z(body_z: f64) -> f64 {
    body_z / 2.0 - (GASKET_GROOVE_DEPTH - GASKET_GROOVE_CSG_OVERLAP) / 2.0
}

pub fn top_face_cut_height(depth: f64) -> f64 {
    depth + TOP_FACE_CSG_OVERLAP
}

pub fn top_face_cut_z(body_z: f64, depth: f64) -> f64 {
    body_z / 2.0 - depth / 2.0 + TOP_FACE_CSG_OVERLAP / 2.0
}

pub fn bottom_face_cut_height(depth: f64) -> f64 {
    depth + TOP_FACE_CSG_OVERLAP
}

pub fn bottom_face_cut_z(body_z: f64, depth: f64) -> f64 {
    -body_z / 2.0 + depth / 2.0 - TOP_FACE_CSG_OVERLAP / 2.0
}

pub fn slot_center(row: usize, col: usize) -> (f64, f64) {
    (
        centered_index(col, COLS, SLOT_PITCH_X),
        centered_index(row, ROWS, SLOT_PITCH_Y),
    )
}

pub fn slot_number(row: usize, col: usize) -> usize {
    row * COLS + col + 1
}

pub fn dock_slot_recess_cut_height() -> f64 {
    top_face_cut_height(SLOT_RECESS_DEPTH)
}

pub fn dock_slot_recess_cut_z() -> f64 {
    top_face_cut_z(DOCK_Z, SLOT_RECESS_DEPTH)
}

pub fn dock_top_feature_center_z(feature_height: f64) -> f64 {
    DOCK_SUPPORT_PLANE_Z + feature_height / 2.0
}

pub fn dock_position_token_point(row: usize, col: usize) -> (f64, f64) {
    let (x, y) = slot_center(row, col);
    (x, y - DOCK_POSITION_TOKEN_Y_OFFSET)
}

pub fn fastener_points() -> Vec<(f64, f64)> {
    let mut points = Vec::with_capacity(16);
    for row in 0..ROWS {
        let y = centered_index(row, ROWS, SLOT_PITCH_Y);
        points.push((-LID_FASTENER_SIDE_X, y));
        points.push((LID_FASTENER_SIDE_X, y));
    }
    for col in 0..COLS {
        let x = centered_index(col, COLS, SLOT_PITCH_X);
        points.push((x, -LID_FASTENER_FRONT_REAR_Y));
        points.push((x, LID_FASTENER_FRONT_REAR_Y));
    }
    points
}

pub fn inter_slot_stop_points() -> Vec<(f64, f64)> {
    let mut points = Vec::with_capacity((COLS - 1) * (ROWS - 1));
    for row in 1..ROWS {
        let y = centered_index(row, ROWS, SLOT_PITCH_Y) - SLOT_PITCH_Y / 2.0;
        for col in 1..COLS {
            let x = centered_index(col, COLS, SLOT_PITCH_X) - SLOT_PITCH_X / 2.0;
            points.push((x, y));
        }
    }
    points
}

pub fn dock_air_bypass_center_ys() -> Vec<f64> {
    (0..=ROWS)
        .map(|row| centered_index(row, ROWS + 1, SLOT_PITCH_Y))
        .collect()
}

pub fn dock_logger_reservation_land_points() -> [(f64, f64); 4] {
    [
        (
            -DOCK_LOGGER_RESERVATION_LAND_X_OFFSET,
            DOCK_LOGGER_RESERVATION_LAND_Y_OFFSET,
        ),
        (
            DOCK_LOGGER_RESERVATION_LAND_X_OFFSET,
            DOCK_LOGGER_RESERVATION_LAND_Y_OFFSET,
        ),
        (
            -DOCK_LOGGER_RESERVATION_LAND_X_OFFSET,
            -DOCK_LOGGER_RESERVATION_LAND_Y_OFFSET,
        ),
        (
            DOCK_LOGGER_RESERVATION_LAND_X_OFFSET,
            -DOCK_LOGGER_RESERVATION_LAND_Y_OFFSET,
        ),
    ]
}

pub fn dock_robot_lift_points() -> [(f64, f64); 2] {
    [
        (0.0, -DOCK_Y / 2.0 + DOCK_ROBOT_LIFT_EDGE_INSET_Y),
        (0.0, DOCK_Y / 2.0 - DOCK_ROBOT_LIFT_EDGE_INSET_Y),
    ]
}

pub fn dock_leveling_pad_points() -> [(f64, f64); 4] {
    let x = DOCK_X / 2.0 - DOCK_LEVELING_PAD_EDGE_INSET;
    let y = DOCK_Y / 2.0 - DOCK_LEVELING_PAD_EDGE_INSET;
    [(-x, -y), (x, -y), (-x, y), (x, y)]
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DockMaterialProbe {
    pub feature: &'static str,
    pub point: [f64; 3],
    pub expected_inside: bool,
}

/// Representative material probes shared by the STL and STEP verifiers.
///
/// Drain cutters pass through the base deck at their locked XY coordinates.
/// Additive rails and lands are then unioned onto the support plane, so selected
/// overlaps intentionally remain void below the deck top and solid immediately
/// above it. These probes preserve that segmented/bridged A0 fit-check geometry;
/// they do not claim that condensate performance has been validated.
pub fn dock_material_probes() -> Vec<DockMaterialProbe> {
    const PROBE_OFFSET_Z: f64 = 0.05;
    let deck_land_probe_z = DOCK_SUPPORT_PLANE_Z + PROBE_OFFSET_Z;
    let center_bypass_y = dock_air_bypass_center_ys()[ROWS / 2];
    let position_token = dock_position_token_point(0, 0);
    let logger_bridge = (
        DOCK_SIDE_DRAIN_OPENING_CENTER_X,
        -DOCK_LOGGER_RESERVATION_LAND_Y_OFFSET,
    );
    let leveling_bridge = dock_leveling_pad_points()[1];
    let front_lip_bridge = (
        DOCK_DRAIN_VISIBILITY_OPENING_CENTER_X - DOCK_DRAIN_VISIBILITY_OPENING_X / 4.0,
        DOCK_FRONT_LIP_CENTER_Y,
    );
    let front_robot_lift = dock_robot_lift_points()[0];
    let front_center_mount = perimeter_mount_points(DOCK_X, DOCK_Y)[4];

    vec![
        DockMaterialProbe {
            feature: "center air-bypass through opening",
            point: [0.37, center_bypass_y, DOCK_THROUGH_CUT_CENTER_Z],
            expected_inside: false,
        },
        DockMaterialProbe {
            feature: "clear front drain span",
            point: [
                0.0,
                DOCK_FRONT_DRAIN_OPENING_CENTER_Y,
                DOCK_THROUGH_CUT_CENTER_Z,
            ],
            expected_inside: false,
        },
        DockMaterialProbe {
            feature: "clear right drain span",
            point: [
                DOCK_SIDE_DRAIN_OPENING_CENTER_X,
                SLOT_PITCH_Y / 2.0,
                DOCK_THROUGH_CUT_CENTER_Z,
            ],
            expected_inside: false,
        },
        DockMaterialProbe {
            feature: "clear drain-visibility span",
            point: [
                DOCK_DRAIN_VISIBILITY_OPENING_CENTER_X,
                DOCK_DRAIN_VISIBILITY_OPENING_CENTER_Y,
                DOCK_THROUGH_CUT_CENTER_Z,
            ],
            expected_inside: false,
        },
        DockMaterialProbe {
            feature: "front-center perimeter mount through hole",
            point: [
                front_center_mount.0,
                front_center_mount.1,
                DOCK_THROUGH_CUT_CENTER_Z,
            ],
            expected_inside: false,
        },
        DockMaterialProbe {
            feature: "S01 position-token land",
            point: [position_token.0, position_token.1, deck_land_probe_z],
            expected_inside: true,
        },
        DockMaterialProbe {
            feature: "rear datum rail above support plane",
            point: [0.0, DOCK_REAR_RAIL_CENTER_Y, deck_land_probe_z],
            expected_inside: true,
        },
        DockMaterialProbe {
            feature: "front robot-lift land above support plane",
            point: [front_robot_lift.0, front_robot_lift.1, deck_land_probe_z],
            expected_inside: true,
        },
        DockMaterialProbe {
            feature: "right drain void below logger-land bridge",
            point: [logger_bridge.0, logger_bridge.1, DOCK_THROUGH_CUT_CENTER_Z],
            expected_inside: false,
        },
        DockMaterialProbe {
            feature: "solid logger-land bridge above right drain",
            point: [logger_bridge.0, logger_bridge.1, deck_land_probe_z],
            expected_inside: true,
        },
        DockMaterialProbe {
            feature: "front-right drain void below leveling-land bridge",
            point: [
                leveling_bridge.0,
                leveling_bridge.1,
                DOCK_THROUGH_CUT_CENTER_Z,
            ],
            expected_inside: false,
        },
        DockMaterialProbe {
            feature: "solid front-right leveling-land bridge above drain intersection",
            point: [leveling_bridge.0, leveling_bridge.1, deck_land_probe_z],
            expected_inside: true,
        },
        DockMaterialProbe {
            feature: "visibility void below front-lip bridge",
            point: [
                front_lip_bridge.0,
                front_lip_bridge.1,
                DOCK_THROUGH_CUT_CENTER_Z,
            ],
            expected_inside: false,
        },
        DockMaterialProbe {
            feature: "solid front-lip bridge above visibility opening",
            point: [front_lip_bridge.0, front_lip_bridge.1, deck_land_probe_z],
            expected_inside: true,
        },
    ]
}

pub fn perimeter_mount_points(x: f64, y: f64) -> [(f64, f64); 6] {
    let mount_x = x / 2.0 - PERIMETER_MOUNT_EDGE_OFFSET;
    let mount_y = y / 2.0 - PERIMETER_MOUNT_EDGE_OFFSET;
    [
        (-mount_x, -mount_y),
        (mount_x, -mount_y),
        (-mount_x, mount_y),
        (mount_x, mount_y),
        (0.0, -mount_y),
        (0.0, mount_y),
    ]
}

pub fn coupon_stop_points() -> [(f64, f64); 4] {
    [
        (-COUPON_STOP_X, 0.0),
        (COUPON_STOP_X, 0.0),
        (-COUPON_STOP_X, COUPON_SQUEEZE_STEP_CENTER_Y),
        (COUPON_STOP_X, COUPON_SQUEEZE_STEP_CENTER_Y),
    ]
}

pub fn datum_features() -> [DatumFeature; 4] {
    [
        DatumFeature {
            id: "D1",
            x: -DATUM_BOSS_X,
            y: -DATUM_BOSS_Y,
            role: DatumRole::RoundLocator,
        },
        DatumFeature {
            id: "D2",
            x: DATUM_BOSS_X,
            y: -DATUM_BOSS_Y,
            role: DatumRole::RelievedLocator,
        },
        DatumFeature {
            id: "D3",
            x: -DATUM_BOSS_X,
            y: DATUM_BOSS_Y,
            role: DatumRole::ClearanceWitness,
        },
        DatumFeature {
            id: "D4",
            x: DATUM_BOSS_X,
            y: DATUM_BOSS_Y,
            role: DatumRole::ClearanceWitness,
        },
    ]
}

pub fn datum_points() -> [(f64, f64); 4] {
    datum_features().map(|datum| (datum.x, datum.y))
}

pub fn window_fiducial_points() -> [(f64, f64); 3] {
    [
        (-WINDOW_FIDUCIAL_X, WINDOW_FIDUCIAL_Y),
        (WINDOW_FIDUCIAL_X, WINDOW_FIDUCIAL_Y),
        (-WINDOW_FIDUCIAL_X, -WINDOW_FIDUCIAL_Y),
    ]
}

pub fn validate_contract() -> Result<(), String> {
    let checks = [
        (SLOT_COUNT == 16, "slot count must stay 16"),
        (
            PER_SLOT_GASKET_INNER_X > REVC_CHIP_LENGTH && PER_SLOT_GASKET_INNER_Y > REVC_CHIP_WIDTH,
            "gasket-land opening must pass the nominal Rev C chip",
        ),
        (
            PER_SLOT_GASKET_OUTER_X < SLOT_PITCH_X && PER_SLOT_GASKET_OUTER_Y < SLOT_PITCH_Y,
            "independent per-slot gasket lands must not overlap",
        ),
        (
            LID_SLOT_VIEW_OPENING_X < PER_SLOT_GASKET_GROOVE_INNER_X
                && LID_SLOT_VIEW_OPENING_Y < PER_SLOT_GASKET_GROOVE_INNER_Y
                && LID_CHIP_TOP_RELIEF_X < PER_SLOT_GASKET_GROOVE_INNER_X
                && LID_CHIP_TOP_RELIEF_Y < PER_SLOT_GASKET_GROOVE_INNER_Y
                && LID_CHIP_TOP_RELIEF_DEPTH > 0.0
                && LID_CHIP_TOP_RELIEF_DEPTH < GASKET_GROOVE_DEPTH
                && PERIMETER_GASKET_GROOVE_OUTER_X < LID_X
                && PERIMETER_GASKET_GROOVE_OUTER_Y < LID_Y,
            "lid openings, chip-top reliefs, and outer edge must leave continuous material beneath every gasket groove",
        ),
        (
            (LID_UNDERSIDE_SEAL_SKIN_Z + LID_UPPER_FRAME_Z - LID_Z).abs() < 1e-9
                && LID_GROOVE_FLOOR_Z >= MIN_LID_GROOVE_FLOOR_Z
                && LID_UPPER_RELIEF_X < LID_X
                && LID_UPPER_RELIEF_Y < LID_Y
                && LID_CHIP_RELIEF_TO_GROOVE_MARGIN_X > 0.0
                && LID_CHIP_RELIEF_TO_GROOVE_MARGIN_Y > 0.0
                && ADJACENT_SLOT_GROOVE_GAP_X > 0.0
                && ADJACENT_SLOT_GROOVE_GAP_Y > 0.0
                && SLOT_ARRAY_TO_PERIMETER_GROOVE_GAP_X > 0.0
                && SLOT_ARRAY_TO_PERIMETER_GROOVE_GAP_Y > 0.0
                && PERIMETER_GROOVE_TO_LID_EDGE_X > 0.0
                && PERIMETER_GROOVE_TO_LID_EDGE_Y > 0.0,
            "split lid construction must preserve a continuous groove-support skin and positive groove clearances",
        ),
        (
            WINDOW_WITNESS_FRAME_CENTER_Z - WINDOW_WITNESS_FRAME_BODY_Z / 2.0
                < WINDOW_Z / 2.0
                && WINDOW_RETENTION_TAB_CENTER_Z - WINDOW_RETENTION_TAB_BODY_Z / 2.0
                    < WINDOW_Z / 2.0
                && (WINDOW_WITNESS_FRAME_CENTER_Z + WINDOW_WITNESS_FRAME_BODY_Z / 2.0
                    - (WINDOW_Z / 2.0 + WINDOW_WITNESS_FRAME_EXPOSED_Z))
                    .abs()
                    < 1e-9
                && (WINDOW_RETENTION_TAB_CENTER_Z + WINDOW_RETENTION_TAB_BODY_Z / 2.0
                    - (WINDOW_Z / 2.0 + WINDOW_RETENTION_TAB_EXPOSED_Z))
                    .abs()
                    < 1e-9,
            "window witness frames and retention tabs must overlap the panel without changing exposed height",
        ),
        (
            WINDOW_FIDUCIAL_CENTER_Z - WINDOW_FIDUCIAL_BODY_Z / 2.0 < WINDOW_Z / 2.0
                && (WINDOW_FIDUCIAL_CENTER_Z + WINDOW_FIDUCIAL_BODY_Z / 2.0
                    - (WINDOW_Z / 2.0 + WINDOW_FIDUCIAL_EXPOSED_Z))
                    .abs()
                    < 1e-9
                && WINDOW_FIDUCIAL_HOLE_CUT_Z > WINDOW_FIDUCIAL_BODY_Z,
            "window fiducials must overlap the panel while preserving exposed height and a through-cut center",
        ),
        (
            (GASKET_LAND_Z - CLOSURE_PLANE_ABOVE_CARRIER).abs() < 1e-9
                && (GASKET_GROOVE_DEPTH - GASKET_COMPRESSED_HEIGHT).abs() < 1e-9
                && (bottom_face_gasket_groove_cut_z(LID_Z)
                    + GASKET_GROOVE_CUT_HEIGHT / 2.0
                    - (-LID_Z / 2.0 + GASKET_GROOVE_DEPTH))
                    .abs()
                    < 1e-9
                && bottom_face_gasket_groove_cut_z(LID_Z)
                    - GASKET_GROOVE_CUT_HEIGHT / 2.0
                    < -LID_Z / 2.0,
            "seal lands must meet the lid closure face and the groove must set gasket compression",
        ),
        (
            LID_FASTENER_SIDE_X - LID_RETAINER_DIAMETER / 2.0 > LEAK_GUTTER_OUTER_X / 2.0
                && LID_FASTENER_FRONT_REAR_Y - LID_RETAINER_DIAMETER / 2.0
                    > LEAK_GUTTER_OUTER_Y / 2.0,
            "fastener retainers and carrier receivers must clear the leak gutter",
        ),
        (
            LID_FASTENER_SIDE_X + CARRIER_LID_RECEIVER_DIAMETER / 2.0 < CARRIER_X / 2.0
                && LID_FASTENER_FRONT_REAR_Y + CARRIER_LID_RECEIVER_DIAMETER / 2.0
                    < CARRIER_Y / 2.0,
            "carrier lid receivers must remain inside the carrier body",
        ),
        (
            LEAK_GUTTER_INNER_X > PERIMETER_GASKET_OUTER_X
                && LEAK_GUTTER_INNER_Y > PERIMETER_GASKET_OUTER_Y,
            "leak gutter must be separated from the perimeter gasket land",
        ),
        (
            PERIMETER_STOP_CENTER_OFFSET - PERIMETER_STOP_W / 2.0 > 0.0
                && PERIMETER_STOP_CENTER_OFFSET + PERIMETER_STOP_W / 2.0
                    < LEAK_GUTTER_SEPARATING_WEB,
            "perimeter compression stops must fit inside the seal-to-gutter web",
        ),
        (
            INTERNAL_STOP_DIAMETER < SLOT_PITCH_X - PER_SLOT_GASKET_OUTER_X
                && INTERNAL_STOP_DIAMETER < SLOT_PITCH_Y - PER_SLOT_GASKET_OUTER_Y,
            "internal compression stops must fit inside inter-slot gaps without touching seals",
        ),
        (
            DATUM_BOSS_X - DATUM_BOSS_DIAMETER / 2.0 > LEAK_GUTTER_OUTER_X / 2.0,
            "datum bosses must clear the gasket and leak gutter",
        ),
        (
            DATUM_D2_SLOT_LENGTH > DATUM_D2_SLOT_WIDTH
                && DATUM_D2_SLOT_WIDTH >= DATUM_D1_BORE_DIAMETER
                && LID_DATUM_PIN_DIAMETER < DATUM_D1_BORE_DIAMETER
                && LID_DATUM_PIN_DIAMETER < DATUM_D2_SLOT_WIDTH
                && DATUM_WITNESS_BORE_DIAMETER > DATUM_D1_BORE_DIAMETER
                && DATUM_D2_SLOT_LENGTH < DATUM_BOSS_DIAMETER
                && DATUM_WITNESS_BORE_DIAMETER < DATUM_BOSS_DIAMETER
                && LID_DATUM_PIN_EXTENSION > CLOSURE_PLANE_ABOVE_CARRIER - DATUM_BOSS_Z
                && (LID_DATUM_PIN_EXTENSION
                    - (CLOSURE_PLANE_ABOVE_CARRIER - DATUM_BOSS_Z)
                    - LID_DATUM_PIN_ENGAGEMENT)
                    .abs()
                    < 1e-9
                && LID_DATUM_PIN_EMBEDMENT <= LID_UNDERSIDE_SEAL_SKIN_Z
                && LID_DATUM_PIN_SEAT_DEPTH == LID_DATUM_PIN_EMBEDMENT
                && LID_DATUM_PIN_TOTAL_Z
                    == LID_DATUM_PIN_EXTENSION + LID_DATUM_PIN_EMBEDMENT
                && LID_DATUM_PIN_DIAMETER < LID_DATUM_PIN_SEAT_DIAMETER
                && DATUM_BOSS_X + DATUM_BOSS_DIAMETER / 2.0 + SIDE_SERVICE_DATUM_CLEARANCE
                    < CARRIER_X / 2.0,
            "datum features must preserve replaceable D1/D2 pins, one relieved locator, and oversized D3/D4 witness bores",
        ),
        (
            GLOBAL_BARCODE_CENTER_Y + GLOBAL_BARCODE_LAND_Y / 2.0 < -LEAK_GUTTER_OUTER_Y / 2.0
                && SLOT_LABEL_CENTER_Y - SLOT_LABEL_LAND_Y / 2.0 > LEAK_GUTTER_OUTER_Y / 2.0,
            "global and per-slot label lands must stay outside the leak gutter",
        ),
        (
            GLOBAL_BARCODE_CENTER_Y - GLOBAL_BARCODE_LAND_Y / 2.0 > -CARRIER_Y / 2.0
                && SIDE_SERVICE_RELIEF_Z < CLOSURE_PLANE_ABOVE_CARRIER,
            "front labels and service reliefs must stay inside the carrier and below the lid closure face",
        ),
        (
            SENSOR_CONNECTOR_CENTER_X - SENSOR_CONNECTOR_X / 2.0
                > WASTE_PORT_XS[WASTE_PORT_XS.len() - 1] + MEDIA_WASTE_PORT_DIAMETER / 2.0,
            "sensor connector cut must clear the W4 port",
        ),
        (
            CARRIER_DRAIN_Y + CARRIER_DRAIN_LENGTH / 2.0 + 1e-9
                >= -LEAK_GUTTER_INNER_Y / 2.0,
            "carrier drain must reach the leak gutter",
        ),
        (
            (DOCK_REAR_RAIL_CENTER_Y - DOCK_RAIL_W / 2.0 - CARRIER_Y / 2.0).abs()
                < 1e-9
                && (DOCK_LEFT_RAIL_CENTER_X + DOCK_RAIL_W / 2.0 + CARRIER_X / 2.0)
                    .abs()
                    < 1e-9
                && (DOCK_FRONT_LIP_CENTER_Y + DOCK_FRONT_LIP_W / 2.0 + CARRIER_Y / 2.0)
                    .abs()
                    < 1e-9,
            "dock datum rails and front lip must contact the carrier envelope",
        ),
        (
            dock_air_bypass_center_ys().len() == ROWS + 1
                && (dock_air_bypass_center_ys()[ROWS / 2]).abs() < 1e-9
                && SLOT_PITCH_Y - DOCK_SLOT_RECESS_Y > DOCK_AIR_BYPASS_Y,
            "dock air-bypass openings must remain centered in the five inter-row/outer gaps",
        ),
        (
            DOCK_FRONT_DRAIN_OPENING_X < DOCK_X
                && DOCK_SIDE_DRAIN_OPENING_Y < DOCK_Y
                && DOCK_DRAIN_VISIBILITY_OPENING_CENTER_X
                    + DOCK_DRAIN_VISIBILITY_OPENING_X / 2.0
                    < DOCK_X / 2.0
                && DOCK_DRAIN_VISIBILITY_OPENING_CENTER_Y.abs()
                    + DOCK_DRAIN_VISIBILITY_OPENING_Y / 2.0
                    < DOCK_Y / 2.0,
            "dock through-drain and visibility openings must stay inside the deck envelope",
        ),
        (
            DOCK_THROUGH_CUT_HEIGHT > DOCK_Z
                && (DOCK_THROUGH_CUT_CENTER_Z).abs() < 1e-9
                && (DOCK_SLOT_RECESS_FLOOR_Z - 5.5).abs() < 1e-9
                && (dock_slot_recess_cut_height()
                    - top_face_cut_height(SLOT_RECESS_DEPTH))
                    .abs()
                    < 1e-9
                && (dock_slot_recess_cut_z() - top_face_cut_z(DOCK_Z, SLOT_RECESS_DEPTH)).abs()
                    < 1e-9
                && (dock_top_feature_center_z(DOCK_POSITION_TOKEN_Z) - 12.5).abs() < 1e-9,
            "dock recess, through-cut, support-plane, and top-land Z helpers must stay locked",
        ),
        (
            dock_material_probes().len() == 14
                && dock_material_probes()
                    .iter()
                    .any(|probe| !probe.expected_inside)
                && dock_material_probes()
                    .iter()
                    .any(|probe| probe.expected_inside),
            "dock verifier contract must retain representative void and solid bridge probes",
        ),
        (
            dock_logger_reservation_land_points().len() == 4
                && DOCK_LOGGER_RESERVATION_LAND_X_OFFSET
                    - DOCK_LOGGER_RESERVATION_LAND_X / 2.0
                    > CARRIER_X / 2.0
                && DOCK_LOGGER_RESERVATION_LAND_X_OFFSET
                    + DOCK_LOGGER_RESERVATION_LAND_X / 2.0
                    < DOCK_X / 2.0
                && DOCK_LOGGER_RESERVATION_LAND_Y_OFFSET
                    + DOCK_LOGGER_RESERVATION_LAND_Y / 2.0
                    < DOCK_Y / 2.0,
            "solid logger reservation lands must stay outside the carrier and inside the dock",
        ),
        (
            perimeter_mount_points(DOCK_X, DOCK_Y).len() == 6
                && DOCK_LEVELING_PAD_EDGE_INSET > DOCK_LEVELING_PAD_RADIUS
                && DOCK_ROBOT_LIFT_EDGE_INSET_Y > DOCK_ROBOT_LIFT_Y / 2.0,
            "dock mount, leveling-pad, and lift-land interfaces must remain inside the deck",
        ),
        (
            COUPON_LEAK_LOOP_CENTER_X + COUPON_LEAK_LOOP_X / 2.0
                < COUPON_RECONNECTION_LOOP_CENTER_X - COUPON_RECONNECTION_LOOP_X / 2.0
                && COUPON_SAMPLE_SLOT_CENTER_Y + COUPON_SAMPLE_SLOT_Y / 2.0
                    < COUPON_LOOP_CENTER_Y - COUPON_LEAK_LOOP_Y / 2.0
                && COUPON_SQUEEZE_STEP_CENTER_Y + COUPON_SQUEEZE_STEP_Y / 2.0
                    < COUPON_SAMPLE_SLOT_CENTER_Y - COUPON_SAMPLE_SLOT_Y / 2.0
                && COUPON_LABEL_CENTER_Y + COUPON_LABEL_LAND_Y / 2.0
                    < COUPON_SQUEEZE_STEP_CENTER_Y - COUPON_SQUEEZE_STEP_Y / 2.0,
            "witness-coupon loops, sample slot, squeeze steps, and labels must not overlap",
        ),
        (
            COUPON_LEAK_LOOP_CENTER_X.abs() + COUPON_LEAK_LOOP_X / 2.0 < COUPON_X / 2.0
                && COUPON_RECONNECTION_LOOP_CENTER_X.abs()
                    + COUPON_RECONNECTION_LOOP_X / 2.0
                    < COUPON_X / 2.0
                && COUPON_LOOP_CENTER_Y + COUPON_LEAK_LOOP_Y / 2.0 < COUPON_Y / 2.0
                && COUPON_STOP_X + INTERNAL_STOP_DIAMETER / 2.0 < COUPON_X / 2.0
                && INTERNAL_STOP_DIAMETER / 2.0
                    < COUPON_LOOP_CENTER_Y - COUPON_LEAK_LOOP_Y / 2.0
                && COUPON_STOP_X - INTERNAL_STOP_DIAMETER / 2.0
                    > centered_index(2, 3, 72.0) + COUPON_SQUEEZE_STEP_X / 2.0,
            "witness-coupon loops and stops must remain inside the coupon without intersecting",
        ),
    ];

    if let Some((_, message)) = checks.into_iter().find(|(passed, _)| !passed) {
        return Err(message.to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a0_contract_is_geometrically_non_intersecting() {
        validate_contract().expect("active 16-slot A0 interface contract must be valid");
    }

    #[test]
    fn a0_locked_dimensions_are_stable() {
        let tolerance = 1e-9;
        for (actual, expected) in [
            (SLOT_PITCH_X, 151.76),
            (SLOT_PITCH_Y, 109.48),
            (SLOT_ARRAY_X, 583.04),
            (SLOT_ARRAY_Y, 413.92),
            (CARRIER_X, 699.04),
            (CARRIER_Y, 541.92),
            (LID_X, 717.04),
            (LID_Y, 559.92),
            (LID_UNDERSIDE_SEAL_SKIN_Z, 4.0),
            (LID_UPPER_FRAME_Z, 6.0),
            (LID_UPPER_RELIEF_X, 635.04),
            (LID_UPPER_RELIEF_Y, 457.92),
            (LID_SLOT_VIEW_OPENING_X, 113.76),
            (LID_SLOT_VIEW_OPENING_Y, 71.48),
            (LID_CHIP_TOP_RELIEF_X, 130.16),
            (LID_CHIP_TOP_RELIEF_Y, 87.88),
            (LID_CHIP_TOP_RELIEF_DEPTH, 0.50),
            (LID_GROOVE_FLOOR_Z, 2.20),
            (LID_CHIP_RELIEF_TO_GROOVE_MARGIN_X, 2.20),
            (LID_CHIP_RELIEF_TO_GROOVE_MARGIN_Y, 2.20),
            (ADJACENT_SLOT_GROOVE_GAP_X, 10.80),
            (ADJACENT_SLOT_GROOVE_GAP_Y, 10.80),
            (SLOT_ARRAY_TO_PERIMETER_GROOVE_GAP_X, 8.80),
            (SLOT_ARRAY_TO_PERIMETER_GROOVE_GAP_Y, 8.80),
            (PERIMETER_GROOVE_TO_LID_EDGE_X, 48.40),
            (PERIMETER_GROOVE_TO_LID_EDGE_Y, 54.40),
            (WINDOW_X, 667.04),
            (WINDOW_Y, 489.92),
            (WINDOW_RAISED_FEATURE_CSG_OVERLAP, 0.20),
            (WINDOW_WITNESS_FRAME_CENTER_Z, 2.00),
            (WINDOW_RETENTION_TAB_CENTER_Z, 2.10),
            (WINDOW_FIDUCIAL_EXPOSED_Z, 1.80),
            (WINDOW_FIDUCIAL_CSG_OVERLAP, 0.20),
            (WINDOW_FIDUCIAL_CENTER_Z, 2.30),
            (DOCK_X, 869.04),
            (DOCK_Y, 691.92),
            (DOCK_SUPPORT_PLANE_Z, 11.0),
            (DOCK_THROUGH_CUT_HEIGHT, 24.0),
            (DOCK_SLOT_RECESS_FLOOR_Z, 5.5),
            (DOCK_RAIL_W, 16.0),
            (DOCK_RAIL_Z, 18.0),
            (DOCK_FRONT_LIP_W, 10.0),
            (DOCK_FRONT_LIP_Z, 10.0),
            (SLOT_RECESS_DEPTH, 5.5),
            (DOCK_AIR_BYPASS_Y, 8.0),
            (DOCK_LOGGER_RESERVATION_LAND_X, 48.0),
            (DOCK_LOGGER_RESERVATION_LAND_Y, 32.0),
            (DOCK_LOGGER_RESERVATION_LAND_Z, 8.0),
            (dock_top_feature_center_z(DOCK_POSITION_TOKEN_Z), 12.5),
            (PERIMETER_MOUNT_HOLE_DIAMETER, 5.4),
            (BULKHEAD_X, 789.04),
            (GASKET_LAND_Z, 7.35),
            (GASKET_GROOVE_DEPTH, 1.80),
            (GASKET_GROOVE_CUT_HEIGHT, 1.90),
            (CLOSURE_PLANE_ABOVE_CARRIER, 7.35),
            (CARRIER_OVERALL_Z, 31.35),
            (CASSETTE_CLOSED_STACK_Z, 41.35),
            (INTERNAL_STOP_DIAMETER, 4.0),
            (CARRIER_LID_RECEIVER_DIAMETER, 3.30),
            (SIDE_SERVICE_RELIEF_Z, 7.0),
            (LID_DATUM_PIN_DIAMETER, 5.80),
            (LID_DATUM_PIN_ENGAGEMENT, 2.0),
            (LID_DATUM_PIN_EXTENSION, 3.35),
            (LID_DATUM_PIN_EMBEDMENT, 4.0),
            (LID_DATUM_PIN_TOTAL_Z, 7.35),
            (LID_DATUM_PIN_SEAT_DIAMETER, 6.0),
            (LID_FASTENER_SIDE_X, 332.52),
            (LID_FASTENER_FRONT_REAR_Y, 247.96),
        ] {
            assert!(
                (actual - expected).abs() < tolerance,
                "{actual} != {expected}"
            );
        }
    }

    #[test]
    fn a0_slot_centers_and_port_maps_are_stable() {
        let tolerance = 1e-9;
        let first = slot_center(0, 0);
        let last = slot_center(3, 3);
        assert!((first.0 + 227.64).abs() < tolerance);
        assert!((first.1 + 164.22).abs() < tolerance);
        assert!((last.0 - 227.64).abs() < tolerance);
        assert!((last.1 - 164.22).abs() < tolerance);
        assert_eq!(fastener_points().len(), 16);
        assert_eq!(inter_slot_stop_points().len(), 9);
        assert_eq!(datum_points().len(), 4);
        assert_eq!(datum_features()[0].role, DatumRole::RoundLocator);
        assert_eq!(datum_features()[1].role, DatumRole::RelievedLocator);
        assert_eq!(datum_features()[2].role, DatumRole::ClearanceWitness);
        assert_eq!(datum_features()[3].role, DatumRole::ClearanceWitness);
        assert_eq!(GAS_PORT_XS, [-240.0, -210.0, -180.0, -150.0]);
        assert_eq!(MEDIA_PORT_XS, [-78.0, -52.0, -26.0, 0.0, 26.0, 52.0, 78.0]);
        assert_eq!(WASTE_PORT_XS, [150.0, 176.0, 202.0, 228.0, 254.0]);
    }

    #[test]
    fn a0_dock_position_tokens_and_material_probes_are_stable() {
        let tolerance = 1e-9;
        let first = dock_position_token_point(0, 0);
        let last = dock_position_token_point(ROWS - 1, COLS - 1);
        assert!((first.0 + 227.64).abs() < tolerance);
        assert!((first.1 + 222.96).abs() < tolerance);
        assert!((last.0 - 227.64).abs() < tolerance);
        assert!((last.1 - 105.48).abs() < tolerance);

        let probes = dock_material_probes();
        assert_eq!(probes.len(), 14);
        let mut labels = std::collections::HashSet::new();
        assert!(probes.iter().all(|probe| labels.insert(probe.feature)));

        for (void_feature, solid_feature) in [
            (
                "right drain void below logger-land bridge",
                "solid logger-land bridge above right drain",
            ),
            (
                "front-right drain void below leveling-land bridge",
                "solid front-right leveling-land bridge above drain intersection",
            ),
            (
                "visibility void below front-lip bridge",
                "solid front-lip bridge above visibility opening",
            ),
        ] {
            let void = probes
                .iter()
                .find(|probe| probe.feature == void_feature)
                .expect("missing dock void probe");
            let solid = probes
                .iter()
                .find(|probe| probe.feature == solid_feature)
                .expect("missing dock bridge probe");
            assert_eq!(&void.point[..2], &solid.point[..2]);
            assert!(!void.expected_inside);
            assert!(solid.expected_inside);
            assert!(void.point[2] < DOCK_SUPPORT_PLANE_Z);
            assert!(solid.point[2] > DOCK_SUPPORT_PLANE_Z);
        }
    }

    #[test]
    fn gasket_groove_cutters_preserve_the_locked_internal_depth() {
        let tolerance = 1e-9;
        let lid_face = -LID_Z / 2.0;
        let lid_cut_center = bottom_face_gasket_groove_cut_z(LID_Z);
        let lid_cut_inner_edge = lid_cut_center + GASKET_GROOVE_CUT_HEIGHT / 2.0;
        let lid_cut_overlap_edge = lid_cut_center - GASKET_GROOVE_CUT_HEIGHT / 2.0;
        assert!((lid_cut_inner_edge - (lid_face + GASKET_GROOVE_DEPTH)).abs() < tolerance);
        assert!((lid_face - lid_cut_overlap_edge - GASKET_GROOVE_CSG_OVERLAP).abs() < tolerance);

        let coupon_face = COUPON_Z / 2.0;
        let coupon_cut_center = top_face_gasket_groove_cut_z(COUPON_Z);
        let coupon_cut_inner_edge = coupon_cut_center - GASKET_GROOVE_CUT_HEIGHT / 2.0;
        let coupon_cut_overlap_edge = coupon_cut_center + GASKET_GROOVE_CUT_HEIGHT / 2.0;
        assert!((coupon_face - coupon_cut_inner_edge - GASKET_GROOVE_DEPTH).abs() < tolerance);
        assert!(
            (coupon_cut_overlap_edge - coupon_face - GASKET_GROOVE_CSG_OVERLAP).abs() < tolerance
        );
    }
}
