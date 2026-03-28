/*
 * pid.cpp — PID controller with anti-windup
 * LaminarForge Incubation Enclosure Environmental Controller
 */

#include "pid.h"

PIDController::PIDController(float kp, float ki, float kd,
                             float outputMin, float outputMax,
                             float integralMax)
    : _kp(kp), _ki(ki), _kd(kd),
      _outputMin(outputMin), _outputMax(outputMax),
      _integralMax(integralMax),
      _integral(0.0f), _prevError(0.0f), _firstRun(true),
      _lastP(0.0f), _lastD(0.0f), _lastOutput(0.0f)
{
}

float PIDController::compute(float setpoint, float processValue, float dt) {
    if (dt <= 0.0f) {
        return _lastOutput;
    }

    float error = setpoint - processValue;

    // Proportional term.
    _lastP = _kp * error;

    // Integral term with anti-windup clamping.
    _integral += error * dt;
    if (_integral > _integralMax) {
        _integral = _integralMax;
    } else if (_integral < -_integralMax) {
        _integral = -_integralMax;
    }
    float iTerm = _ki * _integral;

    // Derivative term (on error, skip first iteration to avoid spike).
    float dTerm = 0.0f;
    if (!_firstRun) {
        dTerm = _kd * (error - _prevError) / dt;
    }
    _firstRun = false;
    _lastD = dTerm;

    _prevError = error;

    // Sum and clamp output.
    float output = _lastP + iTerm + dTerm;
    if (output > _outputMax) {
        output = _outputMax;

        // Back-calculate integral to prevent further windup when saturated.
        // Only clamp if integral is contributing to saturation.
        if (iTerm > 0.0f) {
            _integral -= error * dt;
        }
    } else if (output < _outputMin) {
        output = _outputMin;

        if (iTerm < 0.0f) {
            _integral -= error * dt;
        }
    }

    _lastOutput = output;
    return output;
}

void PIDController::reset() {
    _integral   = 0.0f;
    _prevError  = 0.0f;
    _firstRun   = true;
    _lastP      = 0.0f;
    _lastD      = 0.0f;
    _lastOutput = 0.0f;
}

void PIDController::setTuning(float kp, float ki, float kd) {
    _kp = kp;
    _ki = ki;
    _kd = kd;
}
