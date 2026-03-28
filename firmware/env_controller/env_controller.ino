/*
 * env_controller.ino — Main firmware
 * LaminarForge Incubation Enclosure Environmental Controller
 * Target: ESP32-S3 DevKitC (#2, separate from FluidNC motion controller)
 *
 * Controls:
 *   - Enclosure temperature (PID, 2x DS18B20 + 2x silicone heater pads)
 *   - CO2 concentration (bang-bang, SprintIR-6S-5 + solenoid valve)
 *   - Humidity monitoring (BME280, read-only)
 *   - Rocker platform (MG996R servo, pause-aware)
 *   - Peltier reservoir temperature (warm 37C / cold 4C, on/off hysteresis)
 *
 * Libraries required:
 *   - OneWire
 *   - DallasTemperature
 *   - Adafruit_BME280
 *   - ESP32Servo
 */

#include <OneWire.h>
#include <DallasTemperature.h>
#include <Wire.h>
#include <Adafruit_BME280.h>
#include <ESP32Servo.h>

#include "config.h"
#include "pid.h"

// ===========================================================================
// Global Objects
// ===========================================================================

// DS18B20 temperature sensors (shared 1-Wire bus).
OneWire           oneWire(PIN_ONEWIRE);
DallasTemperature tempSensors(&oneWire);
DeviceAddress     sensorAddr[SENSOR_COUNT];

// BME280 humidity sensor.
Adafruit_BME280   bme;
bool              bmeAvailable = false;

// SprintIR CO2 sensor on HardwareSerial 1.
HardwareSerial    co2Serial(1);

// Rocker servo.
Servo             rockerServo;

// PID controller for enclosure temperature.
PIDController     tempPID(TEMP_PID_KP, TEMP_PID_KI, TEMP_PID_KD,
                          TEMP_PID_OUTPUT_MIN, TEMP_PID_OUTPUT_MAX,
                          TEMP_PID_INTEGRAL_MAX);

// ===========================================================================
// State Variables
// ===========================================================================

// Temperature.
float   tempSetpoint       = TEMP_SETPOINT_DEFAULT;
float   tempEnclosureA     = 0.0f;
float   tempEnclosureB     = 0.0f;
float   tempEnclosureAvg   = 0.0f;
float   heaterPwmValue     = 0.0f;
bool    thermalRunaway      = false;

// CO2.
float   co2Setpoint        = CO2_SETPOINT_DEFAULT;
int32_t co2Ppm             = 0;           // current reading in ppm
bool    co2SolenoidOpen    = false;
unsigned long co2LastReadMs = 0;

// CO2 hysteresis thresholds (derived from setpoint).
int32_t co2LowPpm         = CO2_LOW_THRESHOLD;
int32_t co2HighPpm        = CO2_HIGH_THRESHOLD;

// Humidity.
float   humidity           = 0.0f;
float   bmeTemp            = 0.0f;

// Peltier reservoirs.
float   tempWarm           = 0.0f;
float   tempCold           = 0.0f;
bool    peltierWarmOn      = false;
bool    peltierColdOn      = false;

// Rocker.
int     rockerAngle        = ROCKER_CENTER;
int     rockerTargetAngle  = ROCKER_ANGLE_MAX;
bool    rockerPaused       = false;
bool    rockerPhasePositive = true;       // true = at +10, false = at -10
unsigned long rockerPhaseStartMs = 0;
unsigned long rockerTransitionStartMs = 0;
int     rockerTransitionFrom = ROCKER_CENTER;
bool    rockerInTransition  = false;

// Timing.
unsigned long lastTempReadMs    = 0;
unsigned long lastCo2ReadMs     = 0;
unsigned long lastBmeReadMs     = 0;
unsigned long lastReportMs      = 0;
unsigned long lastPidMs         = 0;

// CO2 UART parsing buffer.
char    co2Buffer[32];
uint8_t co2BufIdx          = 0;

// ===========================================================================
// Forward Declarations
// ===========================================================================
void readTemperatures();
void runTemperaturePID();
void readCO2();
void runCO2Control();
void readHumidity();
void updateRocker();
void runPeltierControl();
void printStatusReport();
void processSerialCommand();
void setCO2Setpoint(float pct);
void setServoAngle(int angleDeg);
void emergencyHeaterShutdown();

// ===========================================================================
// Setup
// ===========================================================================
void setup() {
    Serial.begin(SERIAL_BAUD);
    delay(500);
    Serial.println("ENV_CTRL: LaminarForge Incubation Enclosure v1.0");
    Serial.println("ENV_CTRL: Initializing...");

    // -----------------------------------------------------------------------
    // Heater PWM setup (ESP32 LEDC).
    // -----------------------------------------------------------------------
    ledcSetup(HEATER_PWM_CH_A, HEATER_PWM_FREQ, HEATER_PWM_RES);
    ledcSetup(HEATER_PWM_CH_B, HEATER_PWM_FREQ, HEATER_PWM_RES);
    ledcAttachPin(PIN_HEATER_A, HEATER_PWM_CH_A);
    ledcAttachPin(PIN_HEATER_B, HEATER_PWM_CH_B);
    ledcWrite(HEATER_PWM_CH_A, 0);
    ledcWrite(HEATER_PWM_CH_B, 0);
    Serial.println("ENV_CTRL: Heater PWM initialized (25 kHz)");

    // -----------------------------------------------------------------------
    // DS18B20 temperature sensors.
    // -----------------------------------------------------------------------
    tempSensors.begin();
    int found = tempSensors.getDeviceCount();
    Serial.print("ENV_CTRL: Found ");
    Serial.print(found);
    Serial.println(" DS18B20 sensor(s) on 1-Wire bus");

    for (int i = 0; i < SENSOR_COUNT && i < found; i++) {
        if (tempSensors.getAddress(sensorAddr[i], i)) {
            tempSensors.setResolution(sensorAddr[i], 12);  // 12-bit = 0.0625C
            Serial.print("ENV_CTRL:   Sensor ");
            Serial.print(i);
            Serial.print(" addr: ");
            for (int j = 0; j < 8; j++) {
                if (sensorAddr[i][j] < 0x10) Serial.print("0");
                Serial.print(sensorAddr[i][j], HEX);
            }
            Serial.println();
        }
    }

    if (found < SENSOR_COUNT) {
        Serial.print("ENV_CTRL: WARNING - Expected ");
        Serial.print(SENSOR_COUNT);
        Serial.print(" sensors, found ");
        Serial.println(found);
    }

    // Request first reading (non-blocking mode).
    tempSensors.setWaitForConversion(false);
    tempSensors.requestTemperatures();

    // -----------------------------------------------------------------------
    // CO2 sensor UART.
    // -----------------------------------------------------------------------
    co2Serial.begin(CO2_UART_BAUD, SERIAL_8N1, PIN_CO2_RX, PIN_CO2_TX);
    co2LastReadMs = millis();
    Serial.println("ENV_CTRL: SprintIR CO2 UART initialized (9600 baud)");

    // -----------------------------------------------------------------------
    // CO2 solenoid valve (off by default).
    // -----------------------------------------------------------------------
    pinMode(PIN_CO2_SOLENOID, OUTPUT);
    digitalWrite(PIN_CO2_SOLENOID, LOW);
    Serial.println("ENV_CTRL: CO2 solenoid OFF");

    // -----------------------------------------------------------------------
    // BME280 humidity sensor.
    // -----------------------------------------------------------------------
    Wire.begin(PIN_I2C_SDA, PIN_I2C_SCL);
    if (bme.begin(BME280_I2C_ADDR, &Wire)) {
        bmeAvailable = true;
        // Configure for low-power weather monitoring mode.
        bme.setSampling(Adafruit_BME280::MODE_FORCED,
                        Adafruit_BME280::SAMPLING_X1,   // temperature
                        Adafruit_BME280::SAMPLING_NONE,  // pressure
                        Adafruit_BME280::SAMPLING_X1,   // humidity
                        Adafruit_BME280::FILTER_OFF);
        Serial.println("ENV_CTRL: BME280 initialized");
    } else {
        Serial.println("ENV_CTRL: WARNING - BME280 not found at 0x76");
    }

    // -----------------------------------------------------------------------
    // Rocker servo.
    // -----------------------------------------------------------------------
    ESP32PWM::allocateTimer(2);  // Use timer 2 (timers 0,1 used by heaters)
    rockerServo.setPeriodHertz(50);
    rockerServo.attach(PIN_SERVO, 500, 2500);  // MG996R typical range
    setServoAngle(ROCKER_CENTER);
    rockerPhaseStartMs = millis();
    Serial.println("ENV_CTRL: Rocker servo centered");

    // -----------------------------------------------------------------------
    // Pause input from motion controller.
    // -----------------------------------------------------------------------
    pinMode(PIN_PAUSE, INPUT_PULLDOWN);

    // -----------------------------------------------------------------------
    // Peltier H-bridge pins.
    // -----------------------------------------------------------------------
    // Warm reservoir: heating mode (IN1=HIGH, IN2=LOW).
    pinMode(PIN_PELTIER_WARM_EN, OUTPUT);
    pinMode(PIN_PELTIER_WARM_IN1, OUTPUT);
    pinMode(PIN_PELTIER_WARM_IN2, OUTPUT);
    digitalWrite(PIN_PELTIER_WARM_EN, LOW);
    digitalWrite(PIN_PELTIER_WARM_IN1, HIGH);
    digitalWrite(PIN_PELTIER_WARM_IN2, LOW);

    // Cold reservoir: cooling mode (IN1=LOW, IN2=HIGH).
    pinMode(PIN_PELTIER_COLD_EN, OUTPUT);
    pinMode(PIN_PELTIER_COLD_IN1, OUTPUT);
    pinMode(PIN_PELTIER_COLD_IN2, OUTPUT);
    digitalWrite(PIN_PELTIER_COLD_EN, LOW);
    digitalWrite(PIN_PELTIER_COLD_IN1, LOW);
    digitalWrite(PIN_PELTIER_COLD_IN2, HIGH);
    Serial.println("ENV_CTRL: Peltier H-bridges initialized");

    // -----------------------------------------------------------------------
    // Done.
    // -----------------------------------------------------------------------
    lastPidMs = millis();
    Serial.println("ENV_CTRL: Initialization complete");
    Serial.println();
}

// ===========================================================================
// Main Loop
// ===========================================================================
void loop() {
    unsigned long now = millis();

    // -- Temperature sensors (every 1s) ------------------------------------
    if (now - lastTempReadMs >= TEMP_READ_INTERVAL_MS) {
        readTemperatures();
        runTemperaturePID();
        runPeltierControl();
        lastTempReadMs = now;
    }

    // -- CO2 sensor (every 2s) ---------------------------------------------
    if (now - lastCo2ReadMs >= CO2_READ_INTERVAL_MS) {
        readCO2();
        runCO2Control();
        lastCo2ReadMs = now;
    }

    // -- Continuously parse incoming CO2 UART data -------------------------
    while (co2Serial.available()) {
        char c = co2Serial.read();
        if (c == '\n') {
            co2Buffer[co2BufIdx] = '\0';
            parseCO2Response();
            co2BufIdx = 0;
        } else if (c != '\r' && co2BufIdx < sizeof(co2Buffer) - 1) {
            co2Buffer[co2BufIdx++] = c;
        }
    }

    // -- BME280 humidity (every 5s) ----------------------------------------
    if (now - lastBmeReadMs >= BME_READ_INTERVAL_MS) {
        readHumidity();
        lastBmeReadMs = now;
    }

    // -- Rocker platform ---------------------------------------------------
    updateRocker();

    // -- Serial status report (every 5s) -----------------------------------
    if (now - lastReportMs >= REPORT_INTERVAL_MS) {
        printStatusReport();
        lastReportMs = now;
    }

    // -- Process incoming serial commands ----------------------------------
    if (Serial.available()) {
        processSerialCommand();
    }
}

// ===========================================================================
// Temperature
// ===========================================================================
void readTemperatures() {
    // Collect results from previous async request.
    tempEnclosureA = tempSensors.getTempC(sensorAddr[SENSOR_ENCLOSURE_A]);
    tempEnclosureB = tempSensors.getTempC(sensorAddr[SENSOR_ENCLOSURE_B]);
    tempWarm       = tempSensors.getTempC(sensorAddr[SENSOR_WARM_RESERVOIR]);
    tempCold       = tempSensors.getTempC(sensorAddr[SENSOR_COLD_RESERVOIR]);

    // Handle disconnected sensors (DallasTemperature returns -127).
    if (tempEnclosureA < -100.0f) tempEnclosureA = tempEnclosureB;
    if (tempEnclosureB < -100.0f) tempEnclosureB = tempEnclosureA;

    tempEnclosureAvg = (tempEnclosureA + tempEnclosureB) / 2.0f;

    // Start next async conversion.
    tempSensors.requestTemperatures();

    // Thermal runaway protection.
    if (tempEnclosureA > TEMP_SAFETY_MAX || tempEnclosureB > TEMP_SAFETY_MAX) {
        if (!thermalRunaway) {
            Serial.println("ENV_CTRL: !!! THERMAL RUNAWAY DETECTED !!! Heaters killed.");
            thermalRunaway = true;
        }
        emergencyHeaterShutdown();
    }
}

void runTemperaturePID() {
    if (thermalRunaway) {
        emergencyHeaterShutdown();
        return;
    }

    unsigned long now = millis();
    float dt = (now - lastPidMs) / 1000.0f;
    lastPidMs = now;

    if (dt <= 0.0f || dt > 10.0f) {
        // Skip unreasonable dt (startup or overflow).
        return;
    }

    heaterPwmValue = tempPID.compute(tempSetpoint, tempEnclosureAvg, dt);

    uint8_t pwm = (uint8_t)heaterPwmValue;
    ledcWrite(HEATER_PWM_CH_A, pwm);
    ledcWrite(HEATER_PWM_CH_B, pwm);
}

void emergencyHeaterShutdown() {
    ledcWrite(HEATER_PWM_CH_A, 0);
    ledcWrite(HEATER_PWM_CH_B, 0);
    heaterPwmValue = 0.0f;
    tempPID.reset();
}

// ===========================================================================
// CO2
// ===========================================================================
void readCO2() {
    // Send polling command to SprintIR.
    // The SprintIR-6S in polling mode responds to "Z\r\n" with "Z XXXXX\r\n".
    co2Serial.print("Z\r\n");
}

void parseCO2Response() {
    // Expected format: "Z XXXXX" where XXXXX is CO2 in ppm * 10.
    // Example: "Z 05000" = 50000 ppm (filtered value).
    if (co2Buffer[0] == 'Z' || co2Buffer[0] == 'z') {
        // Find the numeric part after space.
        char *numStart = co2Buffer + 1;
        while (*numStart == ' ') numStart++;

        long raw = atol(numStart);
        if (raw > 0) {
            co2Ppm = (int32_t)(raw * 10);  // SprintIR reports ppm/10
            co2LastReadMs = millis();
        }
    }
}

void runCO2Control() {
    unsigned long now = millis();

    // Safety: close solenoid if no valid reading for 30 seconds.
    if (now - co2LastReadMs > CO2_SENSOR_TIMEOUT_MS) {
        if (co2SolenoidOpen) {
            Serial.println("ENV_CTRL: CO2 sensor timeout - closing solenoid (safety)");
            digitalWrite(PIN_CO2_SOLENOID, LOW);
            co2SolenoidOpen = false;
        }
        return;
    }

    // Bang-bang with hysteresis.
    if (co2Ppm < co2LowPpm && !co2SolenoidOpen) {
        digitalWrite(PIN_CO2_SOLENOID, HIGH);
        co2SolenoidOpen = true;
    } else if (co2Ppm > co2HighPpm && co2SolenoidOpen) {
        digitalWrite(PIN_CO2_SOLENOID, LOW);
        co2SolenoidOpen = false;
    }
}

void setCO2Setpoint(float pct) {
    co2Setpoint = pct;
    // Recalculate hysteresis thresholds: +/- 0.2% around setpoint.
    int32_t centerPpm = (int32_t)(pct * 10000.0f);
    co2LowPpm  = centerPpm - 2000;  // setpoint - 0.2%
    co2HighPpm = centerPpm + 2000;  // setpoint + 0.2%
    Serial.print("ENV_CTRL: CO2 setpoint = ");
    Serial.print(pct, 1);
    Serial.print("% (low=");
    Serial.print(co2LowPpm);
    Serial.print(" high=");
    Serial.print(co2HighPpm);
    Serial.println(" ppm)");
}

// ===========================================================================
// Humidity
// ===========================================================================
void readHumidity() {
    if (!bmeAvailable) return;

    bme.takeForcedMeasurement();
    humidity = bme.readHumidity();
    bmeTemp  = bme.readTemperature();

    if (humidity < HUMIDITY_WARNING_MIN) {
        Serial.print("ENV_CTRL: WARNING - Low humidity: ");
        Serial.print(humidity, 1);
        Serial.println("% RH (check water tray)");
    }
}

// ===========================================================================
// Rocker Platform
// ===========================================================================
void setServoAngle(int angleDeg) {
    // Map angle (-10..+10) to servo microseconds.
    // Center = 1500us. 10 degrees ~ 55us (500us / 90deg * 10deg).
    int us = SERVO_CENTER_US + (int)((float)angleDeg * ((float)SERVO_RANGE_US / 90.0f));
    rockerServo.writeMicroseconds(us);
    rockerAngle = angleDeg;
}

void updateRocker() {
    unsigned long now = millis();

    // Check pause signal from motion controller.
    bool pauseSignal = digitalRead(PIN_PAUSE) == HIGH;

    if (pauseSignal && !rockerPaused) {
        // Entering pause: smoothly return to center.
        rockerPaused = true;
        rockerInTransition = true;
        rockerTransitionFrom = rockerAngle;
        rockerTransitionStartMs = now;
    } else if (!pauseSignal && rockerPaused) {
        // Exiting pause: resume from center to current target.
        rockerPaused = false;
        rockerInTransition = true;
        rockerTransitionFrom = ROCKER_CENTER;
        rockerTransitionStartMs = now;
        // Reset phase timer so we get a full hold after transition.
        rockerPhaseStartMs = now + ROCKER_TRANSITION_MS;
    }

    // Handle smooth transitions.
    if (rockerInTransition) {
        unsigned long elapsed = now - rockerTransitionStartMs;
        int target = rockerPaused ? ROCKER_CENTER : rockerTargetAngle;

        if (elapsed >= ROCKER_TRANSITION_MS) {
            // Transition complete.
            setServoAngle(target);
            rockerInTransition = false;
        } else {
            // Linear interpolation.
            float progress = (float)elapsed / (float)ROCKER_TRANSITION_MS;
            // Smoothstep for less jerky motion.
            progress = progress * progress * (3.0f - 2.0f * progress);
            int angle = rockerTransitionFrom +
                        (int)((float)(target - rockerTransitionFrom) * progress);
            setServoAngle(angle);
        }
        return;
    }

    // If paused, stay at center.
    if (rockerPaused) {
        return;
    }

    // Normal rocking pattern: hold at current position for ROCKER_HOLD_MS,
    // then transition to opposite side.
    if (now - rockerPhaseStartMs >= ROCKER_HOLD_MS) {
        // Time to switch sides.
        rockerPhasePositive = !rockerPhasePositive;
        rockerTargetAngle = rockerPhasePositive ? ROCKER_ANGLE_MAX : ROCKER_ANGLE_MIN;

        // Start smooth transition.
        rockerInTransition = true;
        rockerTransitionFrom = rockerAngle;
        rockerTransitionStartMs = now;
        rockerPhaseStartMs = now + ROCKER_TRANSITION_MS;
    }
}

// ===========================================================================
// Peltier Reservoir Control
// ===========================================================================
void runPeltierControl() {
    // Warm reservoir: on/off with 1C hysteresis.
    if (tempWarm > -100.0f) {  // sensor connected
        if (tempWarm < PELTIER_WARM_SETPOINT - PELTIER_HYSTERESIS) {
            if (!peltierWarmOn) {
                digitalWrite(PIN_PELTIER_WARM_EN, HIGH);
                peltierWarmOn = true;
            }
        } else if (tempWarm > PELTIER_WARM_SETPOINT + PELTIER_HYSTERESIS) {
            if (peltierWarmOn) {
                digitalWrite(PIN_PELTIER_WARM_EN, LOW);
                peltierWarmOn = false;
            }
        }
    } else {
        // Sensor disconnected: turn off for safety.
        digitalWrite(PIN_PELTIER_WARM_EN, LOW);
        peltierWarmOn = false;
    }

    // Cold reservoir: on/off with 1C hysteresis.
    if (tempCold > -100.0f) {  // sensor connected
        if (tempCold > PELTIER_COLD_SETPOINT + PELTIER_HYSTERESIS) {
            if (!peltierColdOn) {
                digitalWrite(PIN_PELTIER_COLD_EN, HIGH);
                peltierColdOn = true;
            }
        } else if (tempCold < PELTIER_COLD_SETPOINT - PELTIER_HYSTERESIS) {
            if (peltierColdOn) {
                digitalWrite(PIN_PELTIER_COLD_EN, LOW);
                peltierColdOn = false;
            }
        }
    } else {
        // Sensor disconnected: turn off for safety.
        digitalWrite(PIN_PELTIER_COLD_EN, LOW);
        peltierColdOn = false;
    }
}

// ===========================================================================
// Serial Reporting
// ===========================================================================
void printStatusReport() {
    // Format: ENV: T=37.1C CO2=5.1% RH=87% ROCK=+10 WARM=37.0C COLD=4.2C PAUSE=0
    Serial.print("ENV: T=");
    Serial.print(tempEnclosureAvg, 1);
    Serial.print("C CO2=");
    Serial.print((float)co2Ppm / 10000.0f, 1);
    Serial.print("% RH=");
    Serial.print(humidity, 0);
    Serial.print("% ROCK=");
    if (rockerAngle >= 0) Serial.print("+");
    Serial.print(rockerAngle);
    Serial.print(" WARM=");
    Serial.print(tempWarm, 1);
    Serial.print("C COLD=");
    Serial.print(tempCold, 1);
    Serial.print("C PAUSE=");
    Serial.println(rockerPaused ? "1" : "0");
}

void printFullStatus() {
    Serial.println("--- Full Status ---");
    Serial.print("  Enclosure temp A:  "); Serial.print(tempEnclosureA, 2); Serial.println(" C");
    Serial.print("  Enclosure temp B:  "); Serial.print(tempEnclosureB, 2); Serial.println(" C");
    Serial.print("  Enclosure avg:     "); Serial.print(tempEnclosureAvg, 2); Serial.println(" C");
    Serial.print("  Temp setpoint:     "); Serial.print(tempSetpoint, 1); Serial.println(" C");
    Serial.print("  Heater PWM:        "); Serial.print((int)heaterPwmValue); Serial.println("/255");
    Serial.print("  PID P="); Serial.print(tempPID.getProportional(), 2);
    Serial.print(" I="); Serial.print(tempPID.getIntegral(), 2);
    Serial.print(" D="); Serial.println(tempPID.getDerivative(), 2);
    Serial.print("  Thermal runaway:   "); Serial.println(thermalRunaway ? "YES" : "no");
    Serial.println();
    Serial.print("  CO2:               "); Serial.print(co2Ppm); Serial.println(" ppm");
    Serial.print("  CO2 setpoint:      "); Serial.print(co2Setpoint, 1); Serial.println(" %");
    Serial.print("  Solenoid:          "); Serial.println(co2SolenoidOpen ? "OPEN" : "closed");
    Serial.print("  CO2 last read:     "); Serial.print((millis() - co2LastReadMs) / 1000); Serial.println(" s ago");
    Serial.println();
    Serial.print("  Humidity:          "); Serial.print(humidity, 1); Serial.println(" % RH");
    Serial.print("  BME temp:          "); Serial.print(bmeTemp, 1); Serial.println(" C");
    Serial.println();
    Serial.print("  Rocker angle:      "); Serial.print(rockerAngle); Serial.println(" deg");
    Serial.print("  Rocker paused:     "); Serial.println(rockerPaused ? "YES" : "no");
    Serial.print("  Rocker phase:      "); Serial.println(rockerPhasePositive ? "+10" : "-10");
    Serial.println();
    Serial.print("  Warm reservoir:    "); Serial.print(tempWarm, 1);
    Serial.print(" C (Peltier "); Serial.print(peltierWarmOn ? "ON" : "off"); Serial.println(")");
    Serial.print("  Cold reservoir:    "); Serial.print(tempCold, 1);
    Serial.print(" C (Peltier "); Serial.print(peltierColdOn ? "ON" : "off"); Serial.println(")");
    Serial.println("-------------------");
}

// ===========================================================================
// Serial Command Processing
// ===========================================================================
void processSerialCommand() {
    String line = Serial.readStringUntil('\n');
    line.trim();
    if (line.length() == 0) return;

    if (line.startsWith("SET_TEMP ")) {
        float val = line.substring(9).toFloat();
        if (val >= 20.0f && val <= 45.0f) {
            tempSetpoint = val;
            tempPID.reset();
            thermalRunaway = false;  // allow recovery after setpoint change
            Serial.print("ENV_CTRL: Temp setpoint = ");
            Serial.print(tempSetpoint, 1);
            Serial.println(" C");
        } else {
            Serial.println("ENV_CTRL: ERROR - Temp must be 20-45 C");
        }
    }
    else if (line.startsWith("SET_CO2 ")) {
        float val = line.substring(8).toFloat();
        if (val >= 0.0f && val <= 20.0f) {
            setCO2Setpoint(val);
        } else {
            Serial.println("ENV_CTRL: ERROR - CO2 must be 0-20%");
        }
    }
    else if (line == "PAUSE") {
        if (!rockerPaused) {
            rockerPaused = true;
            rockerInTransition = true;
            rockerTransitionFrom = rockerAngle;
            rockerTransitionStartMs = millis();
            Serial.println("ENV_CTRL: Rocker PAUSED");
        }
    }
    else if (line == "RESUME") {
        if (rockerPaused) {
            rockerPaused = false;
            rockerInTransition = true;
            rockerTransitionFrom = ROCKER_CENTER;
            rockerTransitionStartMs = millis();
            rockerPhaseStartMs = millis() + ROCKER_TRANSITION_MS;
            Serial.println("ENV_CTRL: Rocker RESUMED");
        }
    }
    else if (line == "STATUS") {
        printFullStatus();
    }
    else if (line == "RESET_THERMAL") {
        thermalRunaway = false;
        tempPID.reset();
        Serial.println("ENV_CTRL: Thermal runaway flag cleared");
    }
    else {
        Serial.print("ENV_CTRL: Unknown command: ");
        Serial.println(line);
        Serial.println("  Commands: SET_TEMP <C>, SET_CO2 <%>, PAUSE, RESUME, STATUS, RESET_THERMAL");
    }
}
