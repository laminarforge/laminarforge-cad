#!/usr/bin/env python3
"""
LaminarForge Liquid Handler — Deck Calibration
================================================

Interactive calibration routine for the Rev C 16-chamber chip.

This script guides the operator through jogging the needle to Well A2
(chamber 1 inlet) and saving the machine coordinates.  All other well
positions are then calculated from the known 96-well grid geometry.

Why Well A2?
  Well A2 is the inlet well for chamber 1 (DRG sensory neurons — the
  primary target tissue for AAV selectivity screening).  It is located
  in the top-left region of the chip, making it easy to visually confirm
  alignment.  Column 2 (not column 1) because the Rev C chip uses columns
  2, 5, 8, 11 to center the 4-column chamber grid within the microplate
  footprint.

Calibration procedure:
  1. Home all axes
  2. Jog X, Y, Z using keyboard (arrow keys via terminal input)
  3. Visually align needle tip to center of Well A2
  4. Save position → calibration.json
  5. Verify by moving to a few other wells for visual check

Coordinate system:
  - Machine coordinates: X=0,Y=0 is home position (back-left corner)
  - Positive X = right, Positive Y = forward, Positive Z = up (Z=0 is top)
  - Chip sits on deck adapter plate with ANSI/SLAS footprint alignment pins
"""

import sys
import os
import argparse

# Add parent directory to path for imports
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from gcode_sender import GCodeSender
from config import (
    ChipCalibration, save_calibration, load_calibration,
    Z_SAFE, Z_WELL_TOP, Z_ASPIRATE,
    WELL_LABELS, CHAMBER_LABELS,
    FEED_XY_SLOW, FEED_Z_SLOW,
)


# ── Jog step sizes ─────────────────────────────────────────────

JOG_STEPS = {
    "coarse": 5.0,    # 5mm — rough positioning
    "medium": 1.0,    # 1mm — approach
    "fine":   0.1,    # 100um — final alignment
    "ultra":  0.01,   # 10um — precision tweaking
}


def interactive_jog(sender: GCodeSender) -> tuple:
    """
    Interactive jog mode.  The operator uses keyboard commands to move
    the needle in X, Y, Z until it is centered over Well A2.

    Commands:
      x+/x-  : Move X axis positive/negative
      y+/y-  : Move Y axis positive/negative
      z+/z-  : Move Z axis positive/negative (careful! Z- is DOWN)
      c/m/f/u: Switch step size (coarse/medium/fine/ultra)
      p      : Print current position
      s      : Save current position and exit jog mode
      q      : Quit without saving

    Returns:
      (x, y) tuple of machine coordinates for Well A2
    """
    step_name = "medium"
    step_size = JOG_STEPS[step_name]

    print("\n" + "=" * 60)
    print("INTERACTIVE JOG MODE — Align needle to Well A2")
    print("=" * 60)
    print()
    print("Well A2 is chamber 1 inlet (DRG sensory neurons).")
    print("It is in the TOP-LEFT region of the chip.")
    print("Column 2 (of 12), Row A (of 8) in 96-well grid.")
    print()
    print("Commands:")
    print("  x+ / x-  : Move X (right / left)")
    print("  y+ / y-  : Move Y (forward / back)")
    print("  z+ / z-  : Move Z (up / DOWN — careful!)")
    print("  c        : Coarse step (5.0 mm)")
    print("  m        : Medium step (1.0 mm)")
    print("  f        : Fine step   (0.1 mm)")
    print("  u        : Ultra step  (0.01 mm)")
    print("  p        : Print current position")
    print("  s        : SAVE position and exit")
    print("  q        : Quit without saving")
    print()
    print(f"Current step size: {step_name} ({step_size} mm)")
    print()

    while True:
        try:
            cmd = input(f"[{step_name} {step_size}mm] > ").strip().lower()
        except (EOFError, KeyboardInterrupt):
            print("\nAborted.")
            return None

        if not cmd:
            continue

        # Step size changes
        if cmd == "c":
            step_name, step_size = "coarse", JOG_STEPS["coarse"]
            print(f"  Step: {step_name} ({step_size} mm)")
            continue
        elif cmd == "m":
            step_name, step_size = "medium", JOG_STEPS["medium"]
            print(f"  Step: {step_name} ({step_size} mm)")
            continue
        elif cmd == "f":
            step_name, step_size = "fine", JOG_STEPS["fine"]
            print(f"  Step: {step_name} ({step_size} mm)")
            continue
        elif cmd == "u":
            step_name, step_size = "ultra", JOG_STEPS["ultra"]
            print(f"  Step: {step_name} ({step_size} mm)")
            continue

        # Axis jog commands
        elif cmd == "x+":
            sender.send("G91")
            sender.send(f"G1 X{step_size} F{FEED_XY_SLOW}")
            sender.wait_idle()
            sender.send("G90")
        elif cmd == "x-":
            sender.send("G91")
            sender.send(f"G1 X-{step_size} F{FEED_XY_SLOW}")
            sender.wait_idle()
            sender.send("G90")
        elif cmd == "y+":
            sender.send("G91")
            sender.send(f"G1 Y{step_size} F{FEED_XY_SLOW}")
            sender.wait_idle()
            sender.send("G90")
        elif cmd == "y-":
            sender.send("G91")
            sender.send(f"G1 Y-{step_size} F{FEED_XY_SLOW}")
            sender.wait_idle()
            sender.send("G90")
        elif cmd == "z+":
            sender.send("G91")
            sender.send(f"G1 Z{step_size} F{FEED_Z_SLOW}")
            sender.wait_idle()
            sender.send("G90")
        elif cmd == "z-":
            sender.send("G91")
            sender.send(f"G1 Z-{step_size} F{FEED_Z_SLOW}")
            sender.wait_idle()
            sender.send("G90")

        # Status
        elif cmd == "p":
            # Query machine position
            print("  (Position query — check FluidNC status report)")
            sender.send_realtime("?")
            if sender.ser:
                line = sender.ser.readline().decode("utf-8", errors="replace").strip()
                print(f"  {line}")

        # Save and exit
        elif cmd == "s":
            # Query final position
            print("\nSaving current position as Well A2...")
            # In a real implementation, we would parse the MPos from the
            # status response.  For now, prompt user to enter the values
            # they see on the FluidNC WebUI.
            try:
                x = float(input("  Enter current X position (from FluidNC): "))
                y = float(input("  Enter current Y position (from FluidNC): "))
            except (ValueError, EOFError):
                print("  Invalid input. Try again.")
                continue
            return (x, y)

        elif cmd == "q":
            print("Quit without saving.")
            return None

        else:
            print(f"  Unknown command: {cmd}")


def verify_calibration(sender: GCodeSender, cal: ChipCalibration) -> None:
    """
    Move to several wells for visual verification of calibration accuracy.

    We visit wells at the four corners of the chip grid to check for
    systematic errors (rotation, scale).  Any error >0.5mm means the
    deck adapter plate may be misaligned or the chip is not seated properly.

    Verification wells:
      - A2  (Ch1 inlet)  — top-left (calibration point itself)
      - A11 (Ch4 inlet)  — top-right
      - G2  (Ch13 inlet) — bottom-left
      - G11 (Ch16 inlet) — bottom-right
    """
    verify_chambers = [1, 4, 13, 16]

    print("\n" + "=" * 60)
    print("VERIFICATION — Moving to corner wells")
    print("=" * 60)
    print()
    print("The needle will move to the four corner wells of the chip.")
    print("Visually confirm the needle is centered over each well.")
    print()

    for ch in verify_chambers:
        inlet = cal.get_inlet(ch)
        label = CHAMBER_LABELS[ch - 1]
        inlet_name = WELL_LABELS[ch - 1][0]

        print(f"\nChamber {ch}: {inlet_name} ({label})")
        print(f"  Machine XY: ({inlet.x:.2f}, {inlet.y:.2f})")

        if not sender.confirm_proceed(f"Move to {inlet_name}?"):
            continue

        sender.raise_to_safe()
        sender.move_xy(inlet.x, inlet.y)
        # Lower to just above well — do NOT enter the well during verification
        sender.lower_to(Z_WELL_TOP, slow=True)

        input("  Press Enter when done inspecting...")

    sender.raise_to_safe()
    print("\nVerification complete.")


def print_well_map(cal: ChipCalibration) -> None:
    """Print all calculated well positions in a readable table."""
    wells = cal.get_all_wells()

    print("\n" + "=" * 60)
    print("CALCULATED WELL POSITIONS (absolute machine coordinates)")
    print("=" * 60)
    print()
    print(f"{'Ch':>3} {'Well':>4} {'Type':<7} {'X':>8} {'Y':>8}   {'Tissue'}")
    print("-" * 70)

    current_chamber = 0
    for w in wells:
        if w.chamber != current_chamber:
            if current_chamber > 0:
                print()  # Blank line between chambers
            current_chamber = w.chamber
        print(f"{w.chamber:>3}  {w.name:>4} {w.well_type:<7} {w.x:>8.2f} {w.y:>8.2f}   {w.label}")


def main():
    parser = argparse.ArgumentParser(
        description="LaminarForge deck calibration — teach Well A2 position"
    )
    parser.add_argument("--port", default="/dev/tty.usbmodem1101",
                        help="Serial port for FluidNC")
    parser.add_argument("--dry-run", action="store_true",
                        help="Simulate without serial connection")
    parser.add_argument("--verify-only", action="store_true",
                        help="Skip calibration, just verify existing calibration")
    parser.add_argument("--print-map", action="store_true",
                        help="Print well map from existing calibration and exit")
    args = parser.parse_args()

    # Print-only mode: no serial connection needed
    if args.print_map:
        cal = load_calibration()
        print_well_map(cal)
        return

    # Connect to FluidNC
    sender = GCodeSender(args.port, dry_run=args.dry_run)
    sender.connect()

    try:
        if not args.verify_only:
            # Home and initialize
            sender.home()
            sender.initialize()

            # Move to approximate chip area (center of deck)
            print("\nMoving to approximate chip center area...")
            sender.move_xy(100.0, 100.0)
            sender.lower_to(Z_WELL_TOP)

            # Interactive jog to Well A2
            result = interactive_jog(sender)
            if result is None:
                print("Calibration cancelled.")
                return

            a2_x, a2_y = result

            # Create calibration object and save
            cal = ChipCalibration(a2_machine_x=a2_x, a2_machine_y=a2_y)
            save_calibration(cal)

            print(f"\nWell A2 position saved: X={a2_x:.2f}, Y={a2_y:.2f}")
        else:
            # Load existing calibration
            cal = load_calibration()
            print(f"Loaded calibration: A2 at X={cal.a2_machine_x:.2f}, Y={cal.a2_machine_y:.2f}")

            # Home and initialize for verification moves
            sender.home()
            sender.initialize()

        # Print calculated well map
        print_well_map(cal)

        # Verification
        if sender.confirm_proceed("Run verification? Move to corner wells for visual check."):
            verify_calibration(sender, cal)

        # Park at safe position
        sender.raise_to_safe()
        sender.move_xy(0, 0)
        print("\nCalibration complete. Needle parked at home.")

    except KeyboardInterrupt:
        print("\n\nInterrupted by user.")
        sender.raise_to_safe()
    finally:
        sender.disconnect()


if __name__ == "__main__":
    main()
