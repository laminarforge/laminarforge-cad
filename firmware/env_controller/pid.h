/*
 * pid.h — PID controller with anti-windup
 * LaminarForge Incubation Enclosure Environmental Controller
 */

#ifndef PID_H
#define PID_H

class PIDController {
public:
    PIDController(float kp, float ki, float kd,
                  float outputMin, float outputMax,
                  float integralMax);

    // Compute PID output given current process value and setpoint.
    // dt is the time step in seconds.
    float compute(float setpoint, float processValue, float dt);

    // Reset integral and derivative state (call on setpoint change or safety event).
    void reset();

    // Update tuning parameters at runtime.
    void setTuning(float kp, float ki, float kd);

    // Getters for diagnostics.
    float getProportional() const { return _lastP; }
    float getIntegral()     const { return _integral; }
    float getDerivative()   const { return _lastD; }
    float getOutput()       const { return _lastOutput; }

private:
    float _kp, _ki, _kd;
    float _outputMin, _outputMax;
    float _integralMax;

    float _integral;
    float _prevError;
    bool  _firstRun;

    // Diagnostics.
    float _lastP;
    float _lastD;
    float _lastOutput;
};

#endif // PID_H
