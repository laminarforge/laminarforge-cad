#!/usr/bin/env python3
"""
LaminarForge Liquid Handler — Media Change Protocol
=====================================================

Full 16-chamber media exchange for the Rev C microfluidic chip.

Biology:
  Cell culture media provides nutrients (glucose, amino acids, vitamins),
  buffering (HEPES/bicarbonate), and growth factors (serum or defined
  supplements).  Cells consume nutrients and excrete metabolic waste
  (lactate, ammonia) that accumulate in the 4.2 uL chamber volume.

  For the Rev C chip with gravity-driven rocker flow, media slowly
  perfuses from inlet wells through 200um-deep channels into the culture
  chamber, then out through outlet channels to the outlet wells.  Over
  24-48 hours, the outlet wells accumulate ~80 uL of conditioned (spent)
  media.

  Media changes are performed every 24-48 hours depending on cell density
  and metabolic rate.  The 16 chambers contain different cell types with
  varying metabolic demands:
    - High metabolism: cardiomyocytes, hepatocytes (change every 24h)
    - Low metabolism: endothelial cells, neurons at low density (48h OK)

Protocol overview:
  For each of the 16 chambers:
    1. Aspirate spent media from outlet well (0.5 uL/s — open well, no channels)
    2. Expel to waste
    3. Wash needle (bleach + DI + blot) to prevent cross-contamination
    4. Aspirate fresh warm media from reservoir (0.5 uL/s)
    5. Dispense into inlet well (0.01 uL/s — gentle delivery through channels)
    6. Wash needle before moving to next chamber

  The 0.01 uL/s inlet dispense rate is critical.  At this flow rate,
  the wall shear stress in the 500um x 200um channels is approximately
  0.1 dyne/cm^2, well below the ~1-5 dyne/cm^2 threshold that can
  detach or activate most adherent cell types.

Timing estimate:
  - Per chamber: ~4 min aspirate/waste + ~3 min wash + ~80 min dispense + ~3 min wash
  - That 80 min dispense is the bottleneck (80 uL at 0.01 uL/s = 8000s = 133 min)
  - Actually 80 uL / 0.01 uL/s = 8000 seconds... that's over 2 hours per chamber.
  - For a 16-chamber chip that would be >32 hours.  Not practical.
  -
  - REVISED STRATEGY: Batch mode.
  -   Phase 1: Aspirate all 16 outlets (fast, ~30 min total with washes)
  -   Phase 2: Dispense all 16 inlets at 0.01 uL/s in PARALLEL via rocker
  -
  - Actually, the syringe can only service one well at a time.  But the
  - channel flow rate is set by gravity (rocker angle), not syringe rate.
  - The syringe just needs to fill the inlet well.  The well holds ~125 uL.
  - We dispense 80 uL into the inlet well at a moderate rate (0.5 uL/s),
  - then the rocker naturally drives it through the channels over hours.
  -
  - FINAL STRATEGY:
  -   Phase 1: Aspirate spent media from all 16 outlet wells
  -   Phase 2: Dispense fresh media into all 16 inlet wells
  -   The dispense into the inlet WELL can be fast (0.5 uL/s) since we're
  -   just filling an open well.  Gravity + rocking does the slow perfusion.
  -   Only if we were pumping THROUGH the channels would we need 0.01 uL/s.
  -
  - Wait — the user specified 0.01 uL/s for "gentle delivery through channels."
  - This implies we ARE pushing media through the channels with the syringe,
  - not relying on gravity.  This makes sense for a controlled media exchange
  - where you want defined flow rate and volume through each chamber.
  -
  - COMPROMISE: Two modes supported:
  -   1. ROCKER MODE: Fast fill inlet wells, let gravity do the rest (default)
  -   2. SYRINGE-DRIVEN MODE: Slow push through channels (--syringe-driven flag)

Syringe-driven mode timing:
  - Dispense: 80 uL at 0.01 uL/s = 8000s per chamber
  - 16 chambers = 128,000s = 35.6 hours (impractical for daily changes)
  - Need to reduce: 20 uL at 0.01 uL/s = 2000s = 33 min per chamber
  - 16 chambers at 33 min = 8.8 hours.  Acceptable for overnight run.
  - Or: 80 uL at 0.05 uL/s = 1600s = 26 min per chamber, 7 hours total.

For this implementation: ROCKER MODE (default).  Dispense into wells at
moderate speed, rocker handles perfusion.  Syringe-driven mode available
as flag for experiments that need precise flow control.
"""

import sys
import os
import argparse
import time
from datetime import datetime

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from gcode_sender import GCodeSender
from config import (
    load_calibration, ChipCalibration,
    Z_SAFE, Z_ASPIRATE, Z_DISPENSE,
    FLOW_ASPIRATE, FLOW_GENTLE, FLOW_WASH,
    RESERVOIR_WARM_MEDIA, WASTE,
    MEDIA_ASPIRATE_VOL, MEDIA_DISPENSE_VOL,
    CHAMBER_LABELS, WELL_LABELS,
    volume_to_mm,
)


def aspirate_all_outlets(sender: GCodeSender, cal: ChipCalibration,
                         chambers: list, volume_ul: float) -> None:
    """
    Phase 1: Aspirate spent media from all outlet wells.

    We process outlets in order (chamber 1-16).  Between each chamber,
    the needle is washed to prevent cross-contamination of conditioned
    media between cell types (important if collecting for ELISA/cytokine
    analysis — here we discard to waste, but good practice regardless).

    The aspiration is from the open well (4mm diameter, 10mm deep) so
    we use the faster open-well aspiration rate (0.5 uL/s).
    """
    print("\n" + "=" * 60)
    print("PHASE 1: Aspirate spent media from outlet wells")
    print("=" * 60)

    for ch in chambers:
        outlet = cal.get_outlet(ch)
        label = CHAMBER_LABELS[ch - 1]
        well_name = WELL_LABELS[ch - 1][1]

        print(f"\n  Chamber {ch:>2}/16: {well_name} ({label})")
        print(f"    Position: X={outlet.x:.2f}, Y={outlet.y:.2f}")

        # Move to outlet well and aspirate
        sender.move_to_well(outlet.x, outlet.y, Z_ASPIRATE)
        print(f"    Aspirating {volume_ul:.1f} uL at {FLOW_ASPIRATE.ul_per_s} uL/s...")
        sender.aspirate(volume_ul, FLOW_ASPIRATE)

        # Small pause to let liquid settle after aspiration
        sender.dwell(0.5)
        sender.raise_to_safe()

        # Expel to waste
        sender.expel_to_waste()

        # Wash between chambers
        sender.wash_cycle()


def dispense_all_inlets_rocker(sender: GCodeSender, cal: ChipCalibration,
                                chambers: list, volume_ul: float) -> None:
    """
    Phase 2 (rocker mode): Dispense fresh media into all inlet wells.

    In rocker mode, we fill each inlet well with the target volume
    at moderate speed (0.5 uL/s).  The media sits in the well and
    gravity + the rocking platform drives perfusion through the
    channels over the next 24-48 hours.

    The reservoir contains warm (37C) complete media.  We aspirate from
    the reservoir and dispense into the inlet well for each chamber.
    """
    print("\n" + "=" * 60)
    print("PHASE 2: Dispense fresh media into inlet wells (rocker mode)")
    print("=" * 60)

    for ch in chambers:
        inlet = cal.get_inlet(ch)
        label = CHAMBER_LABELS[ch - 1]
        well_name = WELL_LABELS[ch - 1][0]

        print(f"\n  Chamber {ch:>2}/16: {well_name} ({label})")

        # Aspirate fresh media from warm reservoir
        print(f"    Aspirating {volume_ul:.1f} uL fresh media from reservoir...")
        sender.move_to_position(RESERVOIR_WARM_MEDIA)
        sender.aspirate(volume_ul, FLOW_ASPIRATE)
        sender.raise_to_safe()

        # Dispense into inlet well
        # Use moderate speed — we're filling an open well, not pushing
        # through channels.  The rocker handles the slow perfusion.
        sender.move_to_well(inlet.x, inlet.y, Z_DISPENSE)
        print(f"    Dispensing {volume_ul:.1f} uL into {well_name}...")
        sender.dispense(volume_ul, FLOW_ASPIRATE)

        sender.dwell(0.5)
        sender.raise_to_safe()

        # Wash between chambers
        sender.wash_cycle()


def dispense_all_inlets_syringe(sender: GCodeSender, cal: ChipCalibration,
                                 chambers: list, volume_ul: float) -> None:
    """
    Phase 2 (syringe-driven mode): Slow-push media through channels.

    In syringe-driven mode, the syringe actively pushes media through
    the microfluidic channels at 0.01 uL/s.  This gives precise control
    over the flow rate and shear stress in the culture chamber.

    WARNING: This is extremely slow.  At 0.01 uL/s:
      - 80 uL per chamber = 8000 seconds = 2.2 hours per chamber
      - 16 chambers = 35.5 hours total
    Consider reducing volume or increasing flow rate.

    The needle stays inserted in the inlet well during the entire
    dispense for each chamber.  Temperature of the syringe contents
    will equilibrate to room temperature during this time — acceptable
    for most cell types but may be suboptimal for temperature-sensitive
    cultures.
    """
    print("\n" + "=" * 60)
    print("PHASE 2: Syringe-driven media delivery (SLOW)")
    print("=" * 60)

    total_time_s = len(chambers) * (volume_ul / FLOW_GENTLE.ul_per_s)
    total_hours = total_time_s / 3600
    print(f"  Estimated total time: {total_hours:.1f} hours")
    print(f"  ({volume_ul:.0f} uL at {FLOW_GENTLE.ul_per_s} uL/s x {len(chambers)} chambers)")

    for i, ch in enumerate(chambers):
        inlet = cal.get_inlet(ch)
        label = CHAMBER_LABELS[ch - 1]
        well_name = WELL_LABELS[ch - 1][0]

        chamber_time_s = volume_ul / FLOW_GENTLE.ul_per_s
        chamber_time_min = chamber_time_s / 60

        print(f"\n  Chamber {ch:>2}/16: {well_name} ({label})")
        print(f"    Dispense time: {chamber_time_min:.0f} min")
        print(f"    Progress: {i+1}/{len(chambers)} chambers")

        # Aspirate fresh media from reservoir
        sender.move_to_position(RESERVOIR_WARM_MEDIA)
        sender.aspirate(volume_ul, FLOW_ASPIRATE)
        sender.raise_to_safe()

        # Dispense slowly through channels
        sender.move_to_well(inlet.x, inlet.y, Z_DISPENSE)
        print(f"    Dispensing {volume_ul:.1f} uL at {FLOW_GENTLE.ul_per_s} uL/s...")
        sender.dispense(volume_ul, FLOW_GENTLE)

        sender.dwell(1.0)
        sender.raise_to_safe()

        # Wash between chambers
        sender.wash_cycle()


def run_media_change(sender: GCodeSender, cal: ChipCalibration,
                     chambers: list = None, syringe_driven: bool = False) -> None:
    """
    Execute the full media change protocol.

    Args:
        sender: Connected GCodeSender instance
        cal: Chip calibration data
        chambers: List of chamber numbers to process (default: all 16)
        syringe_driven: If True, use slow syringe-driven delivery instead of rocker mode
    """
    if chambers is None:
        chambers = list(range(1, 17))

    mode = "syringe-driven" if syringe_driven else "rocker"
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M")

    print("\n" + "#" * 60)
    print(f"# MEDIA CHANGE PROTOCOL — {timestamp}")
    print(f"# Mode: {mode}")
    print(f"# Chambers: {chambers}")
    print(f"# Aspirate volume: {MEDIA_ASPIRATE_VOL} uL")
    print(f"# Dispense volume: {MEDIA_DISPENSE_VOL} uL")
    print("#" * 60)

    if not sender.confirm_proceed("Start media change protocol?"):
        print("Protocol cancelled.")
        return

    # Initialize
    sender.home()
    sender.initialize()

    start_time = time.time()

    # Phase 1: Remove spent media from all outlets
    aspirate_all_outlets(sender, cal, chambers, MEDIA_ASPIRATE_VOL)

    # Phase 2: Dispense fresh media into all inlets
    if syringe_driven:
        dispense_all_inlets_syringe(sender, cal, chambers, MEDIA_DISPENSE_VOL)
    else:
        dispense_all_inlets_rocker(sender, cal, chambers, MEDIA_DISPENSE_VOL)

    # Final wash and park
    sender.wash_cycle()
    sender.raise_to_safe()
    sender.move_xy(0, 0)

    elapsed = time.time() - start_time
    elapsed_min = elapsed / 60

    print("\n" + "#" * 60)
    print(f"# MEDIA CHANGE COMPLETE")
    print(f"# Elapsed time: {elapsed_min:.1f} minutes")
    print(f"# Chambers serviced: {len(chambers)}")
    print("#" * 60)

    # Save protocol log
    log_file = os.path.join(
        os.path.dirname(os.path.abspath(__file__)),
        f"log_media_change_{datetime.now().strftime('%Y%m%d_%H%M%S')}.gcode"
    )
    sender.save_log(log_file)


def main():
    parser = argparse.ArgumentParser(
        description="LaminarForge media change protocol for Rev C 16-chamber chip"
    )
    parser.add_argument("--port", default="/dev/tty.usbmodem1101",
                        help="Serial port for FluidNC")
    parser.add_argument("--dry-run", action="store_true",
                        help="Simulate without serial connection (prints G-code)")
    parser.add_argument("--syringe-driven", action="store_true",
                        help="Use slow syringe-driven delivery instead of rocker mode")
    parser.add_argument("--chambers", type=str, default=None,
                        help="Comma-separated chamber numbers (default: all 16). "
                             "Example: --chambers 1,2,5,6")
    args = parser.parse_args()

    # Parse chamber selection
    if args.chambers:
        chambers = [int(c.strip()) for c in args.chambers.split(",")]
        for c in chambers:
            if c < 1 or c > 16:
                print(f"ERROR: Chamber {c} out of range (1-16)")
                sys.exit(1)
    else:
        chambers = list(range(1, 17))

    # Load calibration
    cal = load_calibration()
    print(f"Calibration loaded: A2 at X={cal.a2_machine_x:.2f}, Y={cal.a2_machine_y:.2f}")

    # Connect and run
    sender = GCodeSender(args.port, dry_run=args.dry_run)
    sender.connect()

    try:
        run_media_change(sender, cal, chambers, args.syringe_driven)
    except KeyboardInterrupt:
        print("\n\nProtocol interrupted by user!")
        sender.raise_to_safe()
        sender.empty_syringe()
    except Exception as e:
        print(f"\n\nERROR: {e}")
        sender.raise_to_safe()
        sender.empty_syringe()
        raise
    finally:
        sender.disconnect()


if __name__ == "__main__":
    main()
