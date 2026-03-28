#!/usr/bin/env python3
"""
LaminarForge Liquid Handler — AAV Transduction Protocol
=========================================================

Automated AAV delivery and incubation for selective cell type targeting
across all 16 chambers of the Rev C chip.

Biology:
  Adeno-Associated Virus (AAV) is used as a gene delivery vector to
  transduce specific cell types.  Different AAV serotypes (AAV1, AAV5,
  AAV8, AAV9, AAVrh10, AAV-PHP.eB, etc.) have natural tropism for
  different tissues.  By exposing 16 different cell types to the same
  AAV serotype simultaneously, we can quantify transduction selectivity.

  Key parameters:
    - Multiplicity of infection (MOI): typically 10,000-100,000 vg/cell
    - Incubation time: 2-4 hours at 37°C
    - Virus concentration: depends on titer, diluted in serum-free media
    - Volume per chamber: 50 uL (fills chamber + some inlet/outlet volume)

Protocol overview:
  1. Pre-wash: remove old media from all outlets
  2. For each chamber:
     a. Aspirate AAV solution from cold reservoir (4°C Peltier)
     b. Dispense into inlet well at ultra-slow rate (0.005 uL/s)
     c. Wash needle THOROUGHLY (AAV is sticky, carryover = cross-contamination)
  3. Incubate 2-4 hours (rocker maintains gentle flow)
  4. Post-incubation: collect conditioned media from outlets (for titer analysis)
  5. PBS wash x3 to remove unbound AAV
  6. Resume normal media flow

Timing:
  - AAV delivery: ~40 min (16 chambers × ~2.5 min each, includes thorough wash)
  - Incubation: 120 min (2 hours, adjustable)
  - Post-collection + PBS wash: ~30 min
  - Total: ~3.5 hours

IMPORTANT: AAV is expensive (~$200-500 per prep).  Ultra-slow delivery
(0.005 uL/s) maximizes contact time in the chamber and minimizes waste
in dead volume.

Ticket: T-44504A18
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
    CHAMBER_LABELS, WELL_LABELS,
    Z_ASPIRATE, Z_DISPENSE,
    FLOW_ASPIRATE, FLOW_AAV, FLOW_GENTLE,
    AAV_DELIVERY_VOL, AAV_INCUBATION_MIN, AAV_COLLECTION_VOL,
    PBS_WASH_VOL, PBS_WASH_REPS,
    RESERVOIR_COLD_AAV, RESERVOIR_PBS, WASTE,
    collection_well_xy,
)

DEFAULT_PORT = "/dev/tty.usbmodem1101"


def deliver_aav_single(sender: GCodeSender, cal: ChipCalibration,
                       chamber: int, log_file=None) -> None:
    """
    Deliver AAV to a single chamber.

    1. Aspirate AAV from cold reservoir (0.5 uL/s — keep cold time short)
    2. Dispense into inlet (0.005 uL/s — ultra-slow for max chamber contact)
    3. THOROUGH wash (extra bleach cycle to denature AAV capsid)
    """
    inlet = cal.get_inlet(chamber)
    label = CHAMBER_LABELS[chamber - 1]
    inlet_name = WELL_LABELS[chamber - 1][0]

    print(f"  Ch{chamber:>2} ({label[:25]:<25}) → {inlet_name}", end="", flush=True)

    # Aspirate AAV from cold reservoir
    print(" aspirate...", end="", flush=True)
    sender.move_to_position(RESERVOIR_COLD_AAV)
    sender.aspirate(AAV_DELIVERY_VOL, FLOW_ASPIRATE)
    sender.raise_to_safe()

    # Dispense into inlet (ultra-slow: 0.005 uL/s)
    # At 50 uL / 0.005 uL/s = 10,000 seconds? No — that's too slow.
    # Actually 50 uL / 0.005 uL/s = 10,000s which is wrong.
    # The ultra-slow rate is for channel delivery to avoid shear.
    # But for AAV, we're filling the well, not pushing through channels.
    # Use FLOW_GENTLE (0.01 uL/s) for through-channel delivery.
    # The AAV solution will flow through channels via gravity/rocker after.
    print(" dispense...", end="", flush=True)
    sender.move_to_well(inlet.x, inlet.y, Z_DISPENSE)
    sender.dispense(AAV_DELIVERY_VOL, FLOW_GENTLE)
    sender.raise_to_safe()

    # Thorough wash — extra bleach to denature AAV
    print(" wash...", end="", flush=True)
    sender.wash_cycle()
    # Extra bleach wash for AAV decontamination
    from config import WASH_BLEACH, WASH_VOL, FLOW_WASH, WASH_BLEACH_SOAK_S
    sender.move_to_position(WASH_BLEACH)
    sender.aspirate(WASH_VOL, FLOW_WASH)
    sender.dwell(WASH_BLEACH_SOAK_S * 2)  # Double soak for AAV
    sender.expel_to_waste()
    sender.wash_cycle()  # Final rinse

    print(" done")

    if log_file:
        log_file.write(f"{datetime.now().isoformat()} | Ch{chamber} | aav_deliver | "
                       f"{AAV_DELIVERY_VOL}uL @ {FLOW_GENTLE.ul_per_s}uL/s\n")


def collect_post_aav(sender: GCodeSender, cal: ChipCalibration,
                     chamber: int, collection_idx: int, log_file=None) -> None:
    """
    Collect conditioned media from outlet after AAV incubation.
    This sample contains unbound AAV and can be titered to calculate
    transduction efficiency (input MOI - remaining = transduced).
    """
    outlet = cal.get_outlet(chamber)
    label = CHAMBER_LABELS[chamber - 1]
    outlet_name = WELL_LABELS[chamber - 1][1]
    cx, cy = collection_well_xy(collection_idx)

    print(f"  Ch{chamber:>2} collect from {outlet_name}...", end="", flush=True)

    # Aspirate from outlet
    sender.move_to_well(outlet.x, outlet.y, Z_ASPIRATE)
    sender.aspirate(AAV_COLLECTION_VOL, FLOW_ASPIRATE)
    sender.raise_to_safe()

    # Dispense into collection plate
    sender.move_to_well(cx, cy, Z_DISPENSE)
    sender.dispense(AAV_COLLECTION_VOL, FLOW_ASPIRATE)
    sender.raise_to_safe()

    # Wash
    sender.wash_cycle()
    print(" done")

    if log_file:
        log_file.write(f"{datetime.now().isoformat()} | Ch{chamber} | aav_collect | "
                       f"{AAV_COLLECTION_VOL}uL → collection well {collection_idx}\n")


def pbs_wash_single(sender: GCodeSender, cal: ChipCalibration,
                    chamber: int) -> None:
    """PBS wash to remove unbound AAV from a single chamber."""
    inlet = cal.get_inlet(chamber)
    outlet = cal.get_outlet(chamber)
    inlet_name = WELL_LABELS[chamber - 1][0]
    outlet_name = WELL_LABELS[chamber - 1][1]

    for rep in range(PBS_WASH_REPS):
        # Aspirate old from outlet
        sender.move_to_well(outlet.x, outlet.y, Z_ASPIRATE)
        sender.aspirate(PBS_WASH_VOL, FLOW_ASPIRATE)
        sender.expel_to_waste()

        # Dispense fresh PBS into inlet
        sender.move_to_position(RESERVOIR_PBS)
        sender.aspirate(PBS_WASH_VOL, FLOW_ASPIRATE)
        sender.move_to_well(inlet.x, inlet.y, Z_DISPENSE)
        sender.dispense(PBS_WASH_VOL, FLOW_GENTLE)
        sender.raise_to_safe()

    sender.wash_cycle()


def run_aav_transduction(sender: GCodeSender, cal: ChipCalibration,
                         chambers: list = None, incubation_min: int = None,
                         skip_confirm: bool = False) -> None:
    """
    Full AAV transduction protocol.

    Phase 1: Deliver AAV to all chambers
    Phase 2: Incubate (2-4 hours, rocker running)
    Phase 3: Collect post-AAV samples
    Phase 4: PBS wash x3
    Phase 5: Resume normal media
    """
    if chambers is None:
        chambers = list(range(1, 17))
    if incubation_min is None:
        incubation_min = AAV_INCUBATION_MIN

    total = len(chambers)

    print()
    print("=" * 60)
    print(f"AAV TRANSDUCTION PROTOCOL — {total} chambers")
    print(f"  AAV volume:     {AAV_DELIVERY_VOL} uL per chamber")
    print(f"  Delivery rate:  {FLOW_GENTLE.ul_per_s} uL/s (gentle channel)")
    print(f"  Incubation:     {incubation_min} min at 37°C")
    print(f"  Collection:     {AAV_COLLECTION_VOL} uL per chamber")
    print(f"  PBS wash:       {PBS_WASH_VOL} uL × {PBS_WASH_REPS} reps")
    print("=" * 60)

    if not skip_confirm:
        if not sender.confirm_proceed(
            "AAV is expensive. Verify reservoir is loaded and cold (4°C). Start?"):
            print("Aborted.")
            return

    # Open log
    log_path = f"logs/aav_transduction_{datetime.now().strftime('%Y%m%d_%H%M%S')}.log"
    try:
        os.makedirs("logs", exist_ok=True)
        log_file = open(log_path, "w")
        log_file.write(f"# AAV transduction started at {datetime.now().isoformat()}\n")
        log_file.write(f"# Chambers: {chambers}\n")
        log_file.write(f"# Incubation: {incubation_min} min\n\n")
    except Exception:
        log_file = None

    start_time = time.time()

    # ── Phase 1: Pre-wash all outlets ──
    print("\n--- Phase 1: Pre-wash (remove old media from outlets) ---")
    for i, ch in enumerate(chambers):
        outlet = cal.get_outlet(ch)
        print(f"  [{i+1}/{total}] Ch{ch} aspirate outlet...", end="", flush=True)
        sender.move_to_well(outlet.x, outlet.y, Z_ASPIRATE)
        sender.aspirate(80.0, FLOW_ASPIRATE)
        sender.expel_to_waste()
        sender.wash_cycle()
        print(" done")

    # ── Phase 2: Deliver AAV ──
    print(f"\n--- Phase 2: AAV Delivery ({total} chambers) ---")
    for i, ch in enumerate(chambers):
        print(f"  [{i+1}/{total}]", end="")
        deliver_aav_single(sender, cal, ch, log_file)

    # ── Phase 3: Incubate ──
    print(f"\n--- Phase 3: Incubation ({incubation_min} min at 37°C) ---")
    print("Rocker platform continues running for gentle AAV distribution.")
    sender.pause_for_incubation(incubation_min, "AAV incubation at 37°C")

    # ── Phase 4: Collect post-AAV conditioned media ──
    print(f"\n--- Phase 4: Collect Post-AAV Samples ---")
    print("Collecting conditioned media for transduction efficiency analysis.")
    for i, ch in enumerate(chambers):
        print(f"  [{i+1}/{total}]", end="")
        collect_post_aav(sender, cal, ch, i, log_file)

    # ── Phase 5: PBS wash x3 ──
    print(f"\n--- Phase 5: PBS Wash ({PBS_WASH_REPS}x) ---")
    print("Removing unbound AAV from all chambers.")
    for i, ch in enumerate(chambers):
        print(f"  [{i+1}/{total}] Ch{ch} PBS wash...", end="", flush=True)
        pbs_wash_single(sender, cal, ch)
        print(" done")

    elapsed = time.time() - start_time
    mins = int(elapsed) // 60

    print()
    print("=" * 60)
    print(f"AAV TRANSDUCTION COMPLETE — {total} chambers in {mins} min")
    print("  Samples collected in collection plate (wells 0-15)")
    print("  Resume normal media changes per schedule.")
    print("=" * 60)

    if log_file:
        log_file.write(f"\n# Completed at {datetime.now().isoformat()}\n")
        log_file.write(f"# Total elapsed: {mins} min\n")
        log_file.close()
        print(f"Log saved: {log_path}")


def main():
    parser = argparse.ArgumentParser(description="LaminarForge AAV transduction protocol")
    parser.add_argument("--port", default=DEFAULT_PORT, help="Serial port")
    parser.add_argument("--dry-run", action="store_true", help="Print G-code without sending")
    parser.add_argument("--chambers", type=str, default=None,
                        help="Comma-separated chamber numbers (default: all 16)")
    parser.add_argument("--incubation", type=int, default=None,
                        help=f"Incubation time in minutes (default: {AAV_INCUBATION_MIN})")
    parser.add_argument("--skip-confirm", action="store_true")
    args = parser.parse_args()

    chambers = None
    if args.chambers:
        chambers = [int(c.strip()) for c in args.chambers.split(",")]

    cal = load_calibration()
    sender = GCodeSender(args.port, dry_run=args.dry_run)
    sender.connect()
    sender.home()
    sender.initialize()

    run_aav_transduction(sender, cal, chambers, args.incubation, args.skip_confirm)

    sender.raise_to_safe()
    sender.disconnect()


if __name__ == "__main__":
    main()
