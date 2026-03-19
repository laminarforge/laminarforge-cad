// Net ID constants — every net gets a unique integer ID in KiCad.
// Net 0 is always "unconnected".
//
// IMPORTANT: These IDs must match the position in NET_NAMES below.
// Only include nets that are actually used by component pads.

pub const NET_UNCONNECTED: u32 = 0;
pub const NET_GND: u32 = 1;
pub const NET_12V: u32 = 2;
pub const NET_5V: u32 = 3;
pub const NET_3V3: u32 = 4;
pub const NET_HEATER_P: u32 = 5;
pub const NET_SDA: u32 = 6;
pub const NET_SCL: u32 = 7;
pub const NET_MUX_S0: u32 = 8;
pub const NET_MUX_S1: u32 = 9;
pub const NET_MUX_S2: u32 = 10;
pub const NET_MUX_COM: u32 = 11;
pub const NET_ADC_AIN1: u32 = 12;
pub const NET_HEATER_PWM: u32 = 13;
pub const NET_MOSFET_GATE: u32 = 14;
pub const NET_USB_DP: u32 = 15;
pub const NET_USB_DN: u32 = 16;
pub const NET_LED_BASE_0: u32 = 17;
pub const NET_LED_BASE_1: u32 = 18;
pub const NET_LED_BASE_2: u32 = 19;
pub const NET_LED_BASE_3: u32 = 20;
pub const NET_LED_BASE_4: u32 = 21;
pub const NET_LED_BASE_5: u32 = 22;
pub const NET_LED_BASE_6: u32 = 23;
pub const NET_LED_BASE_7: u32 = 24;
pub const NET_MUX_Y0: u32 = 25;
pub const NET_MUX_Y1: u32 = 26;
pub const NET_MUX_Y2: u32 = 27;
pub const NET_MUX_Y3: u32 = 28;
pub const NET_MUX_Y4: u32 = 29;
pub const NET_MUX_Y5: u32 = 30;
pub const NET_MUX_Y6: u32 = 31;
pub const NET_MUX_Y7: u32 = 32;
pub const NET_LED_COL_0: u32 = 33;
pub const NET_LED_COL_1: u32 = 34;
pub const NET_LED_COL_2: u32 = 35;
pub const NET_LED_COL_3: u32 = 36;
pub const NET_LED_COL_4: u32 = 37;
pub const NET_LED_COL_5: u32 = 38;
pub const NET_LED_COL_6: u32 = 39;
pub const NET_LED_COL_7: u32 = 40;
pub const NET_VBUS: u32 = 41;
pub const NET_LED_CATH_0: u32 = 42;
pub const NET_LED_CATH_1: u32 = 43;
pub const NET_LED_CATH_2: u32 = 44;
pub const NET_LED_CATH_3: u32 = 45;
pub const NET_LED_CATH_4: u32 = 46;
pub const NET_LED_CATH_5: u32 = 47;
pub const NET_LED_CATH_6: u32 = 48;
pub const NET_LED_CATH_7: u32 = 49;
pub const NET_ESP_EN: u32 = 50;
pub const NET_ESP_GPIO0: u32 = 51;
pub const NET_USB_CC1: u32 = 52;
pub const NET_USB_CC2: u32 = 53;
pub const NET_UART_TX: u32 = 54;
pub const NET_UART_RX: u32 = 55;
pub const NET_12V_RAW: u32 = 56;
pub const NET_LED_PWR_ANODE: u32 = 57;
pub const NET_LED_ACT_ANODE: u32 = 58;
pub const NET_ESP_GPIO_ACT: u32 = 59;
pub const NET_GATE_DRV: u32 = 60;

pub const TOTAL_NETS: u32 = 61;

pub const NET_NAMES: &[&str] = &[
    "",            // 0
    "GND",         // 1
    "+12V",        // 2
    "+5V",         // 3
    "+3V3",        // 4
    "HEATER_P",    // 5
    "SDA",         // 6
    "SCL",         // 7
    "MUX_S0",      // 8
    "MUX_S1",      // 9
    "MUX_S2",      // 10
    "MUX_COM",     // 11
    "ADC_AIN1",    // 12
    "HEATER_PWM",  // 13
    "MOSFET_GATE", // 14
    "USB_DP",      // 15
    "USB_DN",      // 16
    "LED_BASE_0", "LED_BASE_1", "LED_BASE_2", "LED_BASE_3",
    "LED_BASE_4", "LED_BASE_5", "LED_BASE_6", "LED_BASE_7",
    "MUX_Y0", "MUX_Y1", "MUX_Y2", "MUX_Y3",
    "MUX_Y4", "MUX_Y5", "MUX_Y6", "MUX_Y7",
    "LED_COL_0", "LED_COL_1", "LED_COL_2", "LED_COL_3",
    "LED_COL_4", "LED_COL_5", "LED_COL_6", "LED_COL_7",
    "VBUS",          // 41
    "LED_CATH_0", "LED_CATH_1", "LED_CATH_2", "LED_CATH_3",
    "LED_CATH_4", "LED_CATH_5", "LED_CATH_6", "LED_CATH_7",
    "ESP_EN",        // 50
    "ESP_GPIO0",     // 51
    "USB_CC1",       // 52
    "USB_CC2",       // 53
    "UART_TX",       // 54
    "UART_RX",       // 55
    "+12V_RAW",      // 56
    "LED_PWR_ANODE", // 57
    "LED_ACT_ANODE", // 58
    "ESP_GPIO_ACT",  // 59
    "GATE_DRV",      // 60
];

pub fn net_name_for_id(id: u32) -> &'static str {
    if (id as usize) < NET_NAMES.len() {
        NET_NAMES[id as usize]
    } else {
        "NET"
    }
}
