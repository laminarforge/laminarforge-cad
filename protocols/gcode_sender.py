"""
LaminarForge Liquid Handler — G-code Sender
=============================================

Serial communication wrapper for FluidNC (ESP32-based motion controller).

FluidNC speaks standard GRBL-compatible G-code over serial (USB CDC).
Each command is sent as a line, and the controller responds with "ok"
or "error:N" after execution.  We use a synchronous send-and-wait
approach since liquid handling is inherently sequential — you cannot
overlap aspirate and dispense steps.

Protocol:
  1. Send line (e.g., "G1 X100 Y50 F6000\\n")
  2. Wait for "ok\\n" or "error:N\\n"
  3. Proceed to next command

The I-axis (4th linear axis) drives the syringe plunger via a lead screw.
Positive I = plunger down = dispense.  Negative I = plunger up = aspirate.
I=0 is fully retracted (empty syringe).  I=58.3 is fully extended (100 uL).
"""

import time
import sys
from typing import Optional, List

try:
    import serial
except ImportError:
    print("ERROR: pyserial not installed.  Run: pip install pyserial")
    sys.exit(1)

from config import (
    Z_SAFE, Z_WELL_TOP, Z_ASPIRATE, Z_DISPENSE, Z_WASH, Z_BLOT,
    FEED_XY_RAPID, FEED_Z_RAPID, FEED_Z_SLOW,
    FLOW_WASH, FLOW_ASPIRATE, FLOW_GENTLE, FLOW_AAV, FLOW_HARVEST,
    volume_to_mm,
    WASH_BLEACH, WASH_DI, WASH_BLOT, WASTE,
    WASH_VOL, WASH_BLEACH_REPS, WASH_DI_REPS,
    WASH_BLEACH_SOAK_S, WASH_BLOT_TOUCH_S,
    DeckPosition,
)


class GCodeSender:
    """
    Synchronous G-code sender for FluidNC over serial.

    Usage:
        sender = GCodeSender("/dev/tty.usbmodem1101")
        sender.connect()
        sender.home()
        sender.move_xy(100, 50)
        sender.lower_to(Z_ASPIRATE)
        sender.aspirate(80.0, FLOW_ASPIRATE)
        sender.raise_to_safe()
        sender.disconnect()
    """

    def __init__(self, port: str, baudrate: int = 115200, timeout: float = 30.0,
                 dry_run: bool = False):
        """
        Args:
            port: Serial port path (e.g., "/dev/tty.usbmodem1101")
            baudrate: Serial baud rate (FluidNC default: 115200)
            timeout: Read timeout in seconds per command
            dry_run: If True, print G-code to stdout instead of sending
        """
        self.port = port
        self.baudrate = baudrate
        self.timeout = timeout
        self.dry_run = dry_run
        self.ser: Optional[serial.Serial] = None

        # Syringe position tracking (mm of I-axis travel).
        # 0 = fully retracted (empty), positive = plunger extended (has liquid)
        self._syringe_pos_mm = 0.0

        # Command log for debugging / protocol replay
        self._log: List[str] = []

    # ── Connection ──────────────────────────────────────────────

    def connect(self) -> None:
        """Open serial connection and wait for FluidNC startup banner."""
        if self.dry_run:
            print("[DRY RUN] Simulating connection to FluidNC")
            return

        print(f"Connecting to FluidNC on {self.port} at {self.baudrate} baud...")
        self.ser = serial.Serial(self.port, self.baudrate, timeout=self.timeout)
        time.sleep(2.0)  # Wait for ESP32 boot + FluidNC init

        # Drain any startup messages
        while self.ser.in_waiting:
            line = self.ser.readline().decode("utf-8", errors="replace").strip()
            if line:
                print(f"  [STARTUP] {line}")

        print("Connected to FluidNC.")

    def disconnect(self) -> None:
        """Close serial connection."""
        if self.ser and self.ser.is_open:
            self.ser.close()
            print("Disconnected from FluidNC.")

    # ── Low-level G-code ────────────────────────────────────────

    def send(self, gcode: str) -> str:
        """
        Send a single G-code line and wait for response.

        Returns the response line ("ok" on success).
        Raises RuntimeError on error response or timeout.
        """
        gcode = gcode.strip()
        self._log.append(gcode)

        if self.dry_run:
            print(f"  [GCODE] {gcode}")
            return "ok"

        self.ser.write((gcode + "\n").encode("utf-8"))
        self.ser.flush()

        # Wait for "ok" or "error:N"
        while True:
            line = self.ser.readline().decode("utf-8", errors="replace").strip()
            if not line:
                raise RuntimeError(f"Timeout waiting for response to: {gcode}")
            if line == "ok":
                return "ok"
            if line.startswith("error"):
                raise RuntimeError(f"FluidNC error for '{gcode}': {line}")
            # Ignore informational messages (e.g., "[MSG:...]")

    def send_realtime(self, char: str) -> None:
        """
        Send a real-time command character (not line-buffered).
        Used for feed hold (!), cycle start (~), reset (0x18), status (?).
        """
        if self.dry_run:
            print(f"  [REALTIME] {repr(char)}")
            return
        self.ser.write(char.encode("utf-8"))

    def wait_idle(self, poll_interval: float = 0.25) -> None:
        """
        Poll status until machine reports Idle state.
        FluidNC responds to '?' with a status line like '<Idle|MPos:...>'.
        """
        if self.dry_run:
            return

        while True:
            self.ser.write(b"?")
            self.ser.flush()
            line = self.ser.readline().decode("utf-8", errors="replace").strip()
            if "<Idle" in line:
                return
            time.sleep(poll_interval)

    # ── Machine setup ───────────────────────────────────────────

    def home(self) -> None:
        """
        Home all axes.  FluidNC uses $H command (GRBL homing cycle).
        Z homes first (up), then X and Y simultaneously.
        I-axis (syringe) homes to fully retracted position.
        """
        print("Homing all axes...")
        self.send("$H")
        self.wait_idle()
        self._syringe_pos_mm = 0.0
        print("Homing complete. All axes at machine zero.")

    def set_absolute(self) -> None:
        """Switch to absolute positioning mode (G90)."""
        self.send("G90")

    def set_relative(self) -> None:
        """Switch to incremental positioning mode (G91)."""
        self.send("G91")

    def set_mm(self) -> None:
        """Set units to millimeters (G21)."""
        self.send("G21")

    def initialize(self) -> None:
        """Standard initialization sequence after homing."""
        self.set_absolute()
        self.set_mm()
        # Ensure syringe is fully retracted
        self.send("G1 I0 F10")
        self.wait_idle()
        self._syringe_pos_mm = 0.0

    # ── Z-axis moves ────────────────────────────────────────────

    def raise_to_safe(self) -> None:
        """Raise Z to safe clearance height for XY travel."""
        self.send(f"G0 Z{Z_SAFE}")
        self.wait_idle()

    def lower_to(self, z: float, slow: bool = True) -> None:
        """
        Lower Z to a target position.

        Args:
            z: Target Z position (negative = down)
            slow: Use slow feed rate for final approach into wells
        """
        f = FEED_Z_SLOW if slow else FEED_Z_RAPID
        # Two-stage: rapid to just above, then slow into well
        if slow and z < Z_WELL_TOP:
            self.send(f"G0 Z{Z_WELL_TOP}")
            self.wait_idle()
        self.send(f"G1 Z{z} F{f}")
        self.wait_idle()

    # ── XY moves ────────────────────────────────────────────────

    def move_xy(self, x: float, y: float) -> None:
        """
        Move to XY position at rapid speed.
        IMPORTANT: Z must be at safe height before calling this.
        """
        self.send(f"G0 X{x:.3f} Y{y:.3f}")
        self.wait_idle()

    def move_to_position(self, pos: DeckPosition) -> None:
        """
        Safe move sequence: raise Z, move XY, lower to working Z.
        This is the standard "go to a deck station" move.
        """
        self.raise_to_safe()
        self.move_xy(pos.x, pos.y)
        self.lower_to(pos.z_working)

    def move_to_well(self, x: float, y: float, z: float) -> None:
        """
        Safe move to a chip well: raise Z, move XY, lower into well.
        """
        self.raise_to_safe()
        self.move_xy(x, y)
        self.lower_to(z, slow=True)

    # ── Syringe (I-axis) moves ──────────────────────────────────

    def aspirate(self, volume_ul: float, flow_rate) -> None:
        """
        Aspirate liquid into the syringe (pull plunger up = negative I move).

        The syringe draws liquid by retracting the plunger.  In our coordinate
        system, the I-axis position represents plunger extension:
          I=0 → fully retracted (empty)
          I=58.3 → fully extended (100 uL)

        Aspirating INCREASES I-position (plunger moves to create vacuum,
        which pulls liquid in through the needle).

        Wait: we need to be careful here.  The Hamilton 1710RN is a positive
        displacement syringe.  The plunger pushes liquid out when extended.
        For aspiration, the plunger retracts (I decreases), creating suction.

        Convention:
          - Aspirate: I moves negative (plunger retracts, creating vacuum)
          - Dispense: I moves positive (plunger extends, pushing liquid out)

        Actually, let's align with the physical setup:
          - I=0: plunger fully retracted (syringe empty, max vacuum potential)
          - I=max: plunger fully extended (syringe has expelled all liquid)

        For our lead-screw setup where positive I = plunger pushes down:
          - Aspirate: negative I delta (retract plunger, draw liquid in)
          - Dispense: positive I delta (extend plunger, push liquid out)

        We track absolute syringe position. Starting at 0 (empty), after
        aspirating 80 uL the position goes to -volume_to_mm(80).
        But FluidNC absolute coordinates... let's use relative moves for
        the I-axis to keep it simple and safe.
        """
        travel_mm = volume_to_mm(volume_ul)
        f = flow_rate.f_value if hasattr(flow_rate, 'f_value') else flow_rate

        # Relative mode for syringe move only
        self.send("G91")  # Incremental
        self.send(f"G1 I-{travel_mm:.4f} F{f:.4f}")
        self.wait_idle()
        self.send("G90")  # Back to absolute

        self._syringe_pos_mm -= travel_mm

    def dispense(self, volume_ul: float, flow_rate) -> None:
        """
        Dispense liquid from the syringe (push plunger down = positive I move).
        See aspirate() docstring for coordinate convention.
        """
        travel_mm = volume_to_mm(volume_ul)
        f = flow_rate.f_value if hasattr(flow_rate, 'f_value') else flow_rate

        self.send("G91")  # Incremental
        self.send(f"G1 I{travel_mm:.4f} F{f:.4f}")
        self.wait_idle()
        self.send("G90")  # Back to absolute

        self._syringe_pos_mm += travel_mm

    def empty_syringe(self, flow_rate=None) -> None:
        """
        Fully expel any remaining liquid in the syringe.
        Moves plunger to I=0 (fully extended / fully expelled).
        """
        if flow_rate is None:
            flow_rate = FLOW_WASH
        f = flow_rate.f_value if hasattr(flow_rate, 'f_value') else flow_rate

        self.send(f"G1 I0 F{f:.4f}")
        self.wait_idle()
        self._syringe_pos_mm = 0.0

    # ── Compound operations ─────────────────────────────────────

    def aspirate_from(self, x: float, y: float, z: float,
                      volume_ul: float, flow_rate) -> None:
        """Move to position, lower into well, aspirate, raise."""
        self.move_to_well(x, y, z)
        self.aspirate(volume_ul, flow_rate)
        self.raise_to_safe()

    def dispense_to(self, x: float, y: float, z: float,
                    volume_ul: float, flow_rate) -> None:
        """Move to position, lower into well, dispense, raise."""
        self.move_to_well(x, y, z)
        self.dispense(volume_ul, flow_rate)
        self.raise_to_safe()

    def expel_to_waste(self) -> None:
        """Move to waste position and expel all syringe contents."""
        self.move_to_position(WASTE)
        self.empty_syringe(FLOW_WASH)
        self.raise_to_safe()

    # ── Wash cycle ──────────────────────────────────────────────

    def wash_cycle(self) -> None:
        """
        Full needle wash cycle between samples.

        This prevents cross-contamination between chambers that contain
        different cell types / AAV serotypes.

        Sequence:
          1. Bleach wash x2: aspirate 50 uL 10% bleach, hold 5s, expel to waste
             - Bleach denatures proteins and inactivates viruses.
             - 5-second hold ensures contact time for AAV capsid denaturation.
          2. DI rinse x3:  aspirate 50 uL DI water, expel to waste
             - Removes residual bleach (which would kill cells).
             - 3 rinses ensures <0.01% bleach carryover.
          3. Blot pad touch: lower to blot pad for 1s
             - Wicks the hanging droplet off the needle tip.
             - Prevents carryover of the ~0.5 uL drop that clings to the needle.
        """
        print("  Wash cycle: bleach...")
        for i in range(WASH_BLEACH_REPS):
            # Aspirate bleach
            self.move_to_position(WASH_BLEACH)
            self.aspirate(WASH_VOL, FLOW_WASH)
            # Hold bleach in syringe for denaturation contact time
            print(f"    Bleach soak {i+1}/{WASH_BLEACH_REPS} ({WASH_BLEACH_SOAK_S}s)...")
            self.dwell(WASH_BLEACH_SOAK_S)
            # Expel to waste
            self.expel_to_waste()

        print("  Wash cycle: DI rinse...")
        for i in range(WASH_DI_REPS):
            self.move_to_position(WASH_DI)
            self.aspirate(WASH_VOL, FLOW_WASH)
            self.expel_to_waste()

        print("  Wash cycle: blot...")
        self.raise_to_safe()
        self.move_xy(WASH_BLOT.x, WASH_BLOT.y)
        self.lower_to(WASH_BLOT.z_working, slow=True)
        self.dwell(WASH_BLOT_TOUCH_S)
        self.raise_to_safe()

        print("  Wash cycle complete.")

    # ── Timing ──────────────────────────────────────────────────

    def dwell(self, seconds: float) -> None:
        """
        G-code dwell (pause in place).
        G4 P parameter is in seconds for FluidNC (GRBL uses milliseconds,
        but FluidNC follows LinuxCNC convention with seconds).
        """
        self.send(f"G4 P{seconds:.1f}")
        self.wait_idle()

    def pause_for_incubation(self, minutes: int, description: str = "") -> None:
        """
        Long pause for biological incubation steps.

        Raises Z to safe height and uses Python sleep (not G4 dwell)
        so the user can see a countdown and the serial connection
        stays alive via periodic status polls.
        """
        self.raise_to_safe()
        total_seconds = minutes * 60
        print(f"\n{'='*60}")
        print(f"INCUBATION: {description}")
        print(f"Duration: {minutes} minutes ({total_seconds} seconds)")
        print(f"{'='*60}")

        for remaining in range(total_seconds, 0, -30):
            mins = remaining // 60
            secs = remaining % 60
            print(f"  Time remaining: {mins:02d}:{secs:02d}")
            wait_time = min(30, remaining)
            time.sleep(wait_time)

            # Keep serial alive with status query
            if not self.dry_run and self.ser:
                self.send_realtime("?")

        print("  Incubation complete.\n")

    # ── User prompts ────────────────────────────────────────────

    def confirm_proceed(self, message: str) -> bool:
        """Ask user to confirm before proceeding. Returns True if confirmed."""
        print(f"\n>>> {message}")
        response = input("    Proceed? [y/N]: ").strip().lower()
        return response in ("y", "yes")

    def abort(self, message: str = "Protocol aborted by user.") -> None:
        """Emergency stop: raise Z, retract syringe, disconnect."""
        print(f"\n!!! ABORT: {message}")
        try:
            self.raise_to_safe()
            self.empty_syringe()
        except Exception:
            pass
        self.disconnect()
        sys.exit(1)

    # ── Logging ─────────────────────────────────────────────────

    def get_log(self) -> List[str]:
        """Return all G-code commands sent this session."""
        return list(self._log)

    def save_log(self, filepath: str) -> None:
        """Save G-code log to file for protocol documentation."""
        with open(filepath, "w") as f:
            for line in self._log:
                f.write(line + "\n")
        print(f"G-code log saved to {filepath}")
