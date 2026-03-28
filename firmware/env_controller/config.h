/*
 * config.h — Pin definitions, setpoints, and tuning constants
 * LaminarForge Incubation Enclosure Environmental Controller
 * Target: ESP32-S3 DevKitC (separate from FluidNC motion controller)
 */

#ifndef CONFIG_H
#define CONFIG_H

// ---------------------------------------------------------------------------
// GPIO Pin Assignments
// ---------------------------------------------------------------------------

// DS18B20 1-Wire bus (enclosure sensors + reservoir sensors share this bus)
#define PIN_ONEWIRE           4

// Enclosure heater MOSFET PWM outputs (25 kHz)
#define PIN_HEATER_A          5
#define PIN_HEATER_B          6

// SprintIR-6S-5 CO2 sensor UART
#define PIN_CO2_RX           15   // ESP32 RX <- SprintIR TX
#define PIN_CO2_TX           16   // ESP32 TX -> SprintIR RX

// CO2 solenoid valve MOSFET (digital on/off)
#define PIN_CO2_SOLENOID      7

// BME280 I2C (humidity monitoring)
#define PIN_I2C_SDA           8
#define PIN_I2C_SCL           9
#define BME280_I2C_ADDR    0x76

// MG996R rocker servo PWM
#define PIN_SERVO            10

// Pause signal from motion controller (active HIGH = pause)
#define PIN_PAUSE            11

// Peltier H-bridge control (L298N)
// Warm reservoir
#define PIN_PELTIER_WARM_EN  12   // PWM enable (on/off)
#define PIN_PELTIER_WARM_IN1 13   // Direction pin 1
#define PIN_PELTIER_WARM_IN2 14   // Direction pin 2

// Cold reservoir
#define PIN_PELTIER_COLD_EN  17   // PWM enable (on/off)
#define PIN_PELTIER_COLD_IN1 18   // Direction pin 1
#define PIN_PELTIER_COLD_IN2 21   // Direction pin 2

// ---------------------------------------------------------------------------
// PWM Configuration
// ---------------------------------------------------------------------------
#define HEATER_PWM_FREQ     25000  // 25 kHz for MOSFET switching
#define HEATER_PWM_RES          8  // 8-bit resolution (0-255)
#define HEATER_PWM_CH_A         0  // LEDC channel for heater A
#define HEATER_PWM_CH_B         1  // LEDC channel for heater B

// ---------------------------------------------------------------------------
// Temperature Control (PID)
// ---------------------------------------------------------------------------
#define TEMP_SETPOINT_DEFAULT   37.0f  // degrees C
#define TEMP_TOLERANCE           0.5f  // +/- acceptable range
#define TEMP_SAFETY_MAX         42.0f  // thermal runaway cutoff
#define TEMP_PID_KP             50.0f
#define TEMP_PID_KI              0.5f
#define TEMP_PID_KD             10.0f
#define TEMP_PID_OUTPUT_MIN      0.0f
#define TEMP_PID_OUTPUT_MAX    255.0f
#define TEMP_PID_INTEGRAL_MAX  200.0f  // anti-windup clamp

// ---------------------------------------------------------------------------
// CO2 Control (bang-bang with hysteresis)
// ---------------------------------------------------------------------------
#define CO2_SETPOINT_DEFAULT     5.0f  // percent
#define CO2_LOW_THRESHOLD    48000     // ppm: open solenoid below this
#define CO2_HIGH_THRESHOLD   52000     // ppm: close solenoid above this
#define CO2_SENSOR_TIMEOUT_MS 30000    // safety: close valve if no reading
#define CO2_UART_BAUD          9600

// ---------------------------------------------------------------------------
// Humidity Monitoring (BME280)
// ---------------------------------------------------------------------------
#define HUMIDITY_WARNING_MIN    80.0f  // warn if RH drops below this

// ---------------------------------------------------------------------------
// Rocker Platform (MG996R servo)
// ---------------------------------------------------------------------------
#define ROCKER_ANGLE_MAX        10     // degrees tilt
#define ROCKER_ANGLE_MIN       -10     // degrees tilt
#define ROCKER_CENTER            0     // level position
#define ROCKER_HOLD_MS      120000     // 2 minutes hold at each position
#define ROCKER_TRANSITION_MS  2000     // smooth transition duration (ms)
#define SERVO_CENTER_US       1500     // microseconds for 0 degrees
#define SERVO_RANGE_US         500     // microseconds per 90 degrees

// ---------------------------------------------------------------------------
// Peltier Reservoir Control (on/off with hysteresis)
// ---------------------------------------------------------------------------
#define PELTIER_WARM_SETPOINT   37.0f  // degrees C
#define PELTIER_COLD_SETPOINT    4.0f  // degrees C
#define PELTIER_HYSTERESIS       1.0f  // +/- degrees

// ---------------------------------------------------------------------------
// Serial Reporting
// ---------------------------------------------------------------------------
#define REPORT_INTERVAL_MS     5000    // status line every 5 seconds
#define SERIAL_BAUD           115200

// ---------------------------------------------------------------------------
// DS18B20 Sensor Indices (set after address discovery)
// These are the 1-Wire ROM address indices on the shared bus.
// Assign after scanning with DallasTemperature::getAddress().
// ---------------------------------------------------------------------------
#define SENSOR_COUNT              4
#define SENSOR_ENCLOSURE_A        0
#define SENSOR_ENCLOSURE_B        1
#define SENSOR_WARM_RESERVOIR     2
#define SENSOR_COLD_RESERVOIR     3

// ---------------------------------------------------------------------------
// Loop Timing
// ---------------------------------------------------------------------------
#define TEMP_READ_INTERVAL_MS  1000    // read DS18B20 every 1 second
#define CO2_READ_INTERVAL_MS   2000    // poll SprintIR every 2 seconds
#define BME_READ_INTERVAL_MS   5000    // read BME280 every 5 seconds

#endif // CONFIG_H
