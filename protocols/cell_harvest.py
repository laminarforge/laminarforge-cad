#!/usr/bin/env python3
"""
LaminarForge Liquid Handler — Cell Harvest Protocol
=====================================================

Enzymatic cell detachment and collection from all 16 chambers
of the Rev C chip.

Biology:
  After AAV transduction and expression (typically 48-72 hours post-
  transduction), cells are harvested from the microfluidic chambers
  for downstream analysis:
    - Flow cytometry (GFP/RFP reporter quantification)
    - qPCR (transgene copy number)
    - Single-cell RNA-seq
    - ELISA (protein expression levels)

  Cells are detached from the PMMA/glass surface using Accutase, a
  gentler alternative to trypsin that preserves surface markers:
    - Works at 37°C (incubator provides this)
    - 5-7 minutes incubation
    - Neutralized by media containing BSA + EDTA (no serum needed)

  Critical consideration: cells must pass through the 500μm × 200μm
  microchannels during harvest.  After enzymatic detachment, most
  mammalian cells are 10-30μm diameter (round, not spread).  The
  200μm channel depth provides >6× clearance.  However, cell clumps
  can form if over-dissociated or under-dissociated.  The gentle flush
  rate (0.1 uL/s) provides enough shear to transport detached cells
  through channels without damaging them.

Protocol overview:
  1. Collect conditioned media from outlets (for secretome analysis)
  2. PBS wash to remove residual media/serum (which inhibits Accutase)
  3. Deliver Accutase through inlets
  4. Incubate 5-7 minutes at 37°C
  5. Flush with media+BSA+EDTA at higher flow rate to collect cells
  6. Deposit harvested cells into collection plate (96-well PCR plate)

Timing:
  - Pre-collection + PBS wash: ~20 min
  - Accutase delivery: ~10 min
  - Incubation: 6 min
  - Cell flush + collection: ~25 min
  - Total: ~60 min

Ticket: T-1D7F7496
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
    FLOW_ASPIRATE, FLOW_GENTLE, FLOW_HARVEST,
    ACCUTASE_VOL, ACCUTASE_INCUBATION_MIN,
    HARVEST_FLUSH_VOL, HARVEST_FLUSH_REPS,
    CONDITIONED_MEDIA_VOL,
    PBS_WASH_VOL, PBS_WASH_REPS,
    RESERVOIR_PBS, RESERVOIR_ACCUTASE, RESERVOIR_MEDIA_BSA, WASTE,
    collection_well_xy,
)

DEFAULT_PORT = "/dev/tty.usbmodem1101"


def collect_conditioned_media(sender: GCodeSender, cal: ChipCalibration,
                              chamber: int, collection_idx: int,
                              log_file=None) -> None:
    """
    Collect conditioned media from outlet for secretome/metabolite analysis.
    This captures the media that has been in contact with cells for 24-48h
    and contains secreted proteins, metabolites, and shed AAV particles.
    """
    outlet = cal.get_outlet(chamber)
    label = CHAMBER_LABELS[chamber - 1]
    outlet_name = WELL_LABELS[chamber - 1][1]
    cx, cy = collection_well_xy(collection_idx)

    print(f"  Ch{chamber:>2} collect conditioned media from {outlet_name}...", end="", flush=True)

    # Aspirate conditioned media from outlet
    sender.move_to_well(outlet.x, outlet.y, Z_ASPIRATE)
    sender.aspirate(CONDITIONED_MEDIA_VOL, FLOW_ASPIRATE)
    sender.raise_to_safe()

    # Dispense into collection plate (first 16 wells for conditioned media)
    sender.move_to_well(cx, cy, Z_DISPENSE)
    sender.dispense(CONDITIONED_MEDIA_VOL, FLOW_ASPIRATE)
    sender.raise_to_safe()

    sender.wash_cycle()
    print(" done")

    if log_file:
        log_file.write(f"{datetime.now().isoformat()} | Ch{chamber} | conditioned_media | "
                       f"{CONDITIONED_MEDIA_VOL}uL → well {collection_idx}\n")


def pbs_wash_chamber(sender: GCodeSender, cal: ChipCalibration,
                     chamber: int) -> None:
    """
    PBS wash to remove residual media/serum from chamber.
    Serum proteins inhibit Accutase activity — must be thoroughly removed.
    """
    inlet = cal.get_inlet(chamber)
    outlet = cal.get_outlet(chamber)

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


def deliver_accutase(sender: GCodeSender, cal: ChipCalibration,
                     chamber: int, log_file=None) -> None:
    """
    Deliver Accutase enzymatic dissociation reagent to a single chamber.
    Accutase is delivered through the inlet at gentle rate so it flows
    across the entire chamber floor where cells are attached.
    """
    inlet = cal.get_inlet(chamber)
    outlet = cal.get_outlet(chamber)
    label = CHAMBER_LABELS[chamber - 1]
    inlet_name = WELL_LABELS[chamber - 1][0]

    print(f"  Ch{chamber:>2} ({label[:25]:<25}) Accutase → {inlet_name}...", end="", flush=True)

    # First remove PBS from outlet to make room
    sender.move_to_well(outlet.x, outlet.y, Z_ASPIRATE)
    sender.aspirate(PBS_WASH_VOL, FLOW_ASPIRATE)
    sender.expel_to_waste()

    # Aspirate Accutase from reservoir
    sender.move_to_position(RESERVOIR_ACCUTASE)
    sender.aspirate(ACCUTASE_VOL, FLOW_ASPIRATE)
    sender.raise_to_safe()

    # Dispense into inlet (gentle — let it flow across chamber)
    sender.move_to_well(inlet.x, inlet.y, Z_DISPENSE)
    sender.dispense(ACCUTASE_VOL, FLOW_GENTLE)
    sender.raise_to_safe()

    # Wash needle
    sender.wash_cycle()
    print(" done")

    if log_file:
        log_file.write(f"{datetime.now().isoformat()} | Ch{chamber} | accutase_deliver | "
                       f"{ACCUTASE_VOL}uL\n")


def harvest_cells(sender: GCodeSender, cal: ChipCalibration,
                  chamber: int, collection_idx: int,
                  log_file=None) -> None:
    """
    Flush detached cells from chamber and collect into plate.

    Uses media+BSA+EDTA solution which:
      - Neutralizes Accutase (BSA binds enzyme)
      - Prevents re-adhesion (EDTA chelates Ca2+)
      - Provides gentle cell-compatible medium

    The harvest flow rate (0.1 uL/s) is higher than normal media delivery
    (0.01 uL/s) to help physically detach and transport cells.  Multiple
    flushes ensure high recovery.  Shear at 0.1 uL/s through 500×200μm
    channels is ~3 Pa — enough to transport detached cells without lysing.
    """
    inlet = cal.get_inlet(chamber)
    outlet = cal.get_outlet(chamber)
    label = CHAMBER_LABELS[chamber - 1]
    outlet_name = WELL_LABELS[chamber - 1][1]
    cx, cy = collection_well_xy(collection_idx)

    print(f"  Ch{chamber:>2} flush + collect from {outlet_name}...", end="", flush=True)

    for rep in range(HARVEST_FLUSH_REPS):
        # Aspirate neutralizing media from reservoir
        sender.move_to_position(RESERVOIR_MEDIA_BSA)
        sender.aspirate(HARVEST_FLUSH_VOL, FLOW_ASPIRATE)
        sender.raise_to_safe()

        # Dispense into inlet at harvest rate (faster = more cell dislodgement)
        sender.move_to_well(inlet.x, inlet.y, Z_DISPENSE)
        sender.dispense(HARVEST_FLUSH_VOL, FLOW_HARVEST)
        sender.raise_to_safe()

        # Brief dwell to let cells accumulate in outlet well
        sender.dwell(10.0)

        # Aspirate cell suspension from outlet
        sender.move_to_well(outlet.x, outlet.y, Z_ASPIRATE)
        sender.aspirate(HARVEST_FLUSH_VOL, FLOW_ASPIRATE)
        sender.raise_to_safe()

        # Dispense into collection plate
        # All flushes go to the same collection well (pool the harvest)
        sender.move_to_well(cx, cy, Z_DISPENSE)
        sender.dispense(HARVEST_FLUSH_VOL, FLOW_ASPIRATE)
        sender.raise_to_safe()

    # Final wash
    sender.wash_cycle()
    print(f" done ({HARVEST_FLUSH_REPS} flushes)")

    if log_file:
        total_vol = HARVEST_FLUSH_VOL * HARVEST_FLUSH_REPS
        log_file.write(f"{datetime.now().isoformat()} | Ch{chamber} | harvest | "
                       f"{total_vol}uL total ({HARVEST_FLUSH_REPS} x {HARVEST_FLUSH_VOL}uL) "
                       f"→ well {collection_idx}\n")


def run_cell_harvest(sender: GCodeSender, cal: ChipCalibration,
                     chambers: list = None, skip_confirm: bool = False) -> None:
    """
    Full cell harvest protocol.

    Phase 1: Collect conditioned media (for secretome analysis)
    Phase 2: PBS wash (remove serum that inhibits Accutase)
    Phase 3: Deliver Accutase to all chambers
    Phase 4: Incubate (5-7 min at 37°C)
    Phase 5: Flush and collect cells from each chamber
    """
    if chambers is None:
        chambers = list(range(1, 17))

    total = len(chambers)

    print()
    print("=" * 60)
    print(f"CELL HARVEST PROTOCOL — {total} chambers")
    print(f"  Accutase:       {ACCUTASE_VOL} uL per chamber")
    print(f"  Incubation:     {ACCUTASE_INCUBATION_MIN} min at 37°C")
    print(f"  Harvest flush:  {HARVEST_FLUSH_VOL} uL × {HARVEST_FLUSH_REPS} reps")
    print(f"  Flush rate:     {FLOW_HARVEST.ul_per_s} uL/s")
    print(f"  Collection:     wells 0-{total-1} (conditioned media)")
    print(f"                  wells {total}-{2*total-1} (harvested cells)")
    print("=" * 60)

    if not skip_confirm:
        if not sender.confirm_proceed(
            "This will IRREVERSIBLY detach all cells. Ensure experiment is complete. Start?"):
            print("Aborted.")
            return

    # Open log
    log_path = f"logs/cell_harvest_{datetime.now().strftime('%Y%m%d_%H%M%S')}.log"
    try:
        os.makedirs("logs", exist_ok=True)
        log_file = open(log_path, "w")
        log_file.write(f"# Cell harvest started at {datetime.now().isoformat()}\n")
        log_file.write(f"# Chambers: {chambers}\n\n")
    except Exception:
        log_file = None

    start_time = time.time()

    # ── Phase 1: Collect conditioned media ──
    print(f"\n--- Phase 1: Collect Conditioned Media ({total} chambers) ---")
    print("Collection plate wells 0-15 = conditioned media samples")
    for i, ch in enumerate(chambers):
        collect_conditioned_media(sender, cal, ch, i, log_file)

    # ── Phase 2: PBS wash ──
    print(f"\n--- Phase 2: PBS Wash ({PBS_WASH_REPS}x per chamber) ---")
    print("Removing serum proteins that inhibit Accutase.")
    for i, ch in enumerate(chambers):
        print(f"  [{i+1}/{total}] Ch{ch} PBS wash...", end="", flush=True)
        pbs_wash_chamber(sender, cal, ch)
        print(" done")

    # ── Phase 3: Deliver Accutase ──
    print(f"\n--- Phase 3: Accutase Delivery ({total} chambers) ---")
    for i, ch in enumerate(chambers):
        print(f"  [{i+1}/{total}]", end="")
        deliver_accutase(sender, cal, ch, log_file)

    # ── Phase 4: Incubate ──
    print(f"\n--- Phase 4: Accutase Incubation ({ACCUTASE_INCUBATION_MIN} min) ---")
    print("Cells are enzymatically detaching from substrate.")
    print("DO NOT rock during Accutase incubation — keep platform level.")
    # Pause rocker by sending signal to env controller
    sender.pause_for_incubation(ACCUTASE_INCUBATION_MIN,
                                 f"Accutase digestion ({ACCUTASE_INCUBATION_MIN} min at 37°C)")

    # ── Phase 5: Harvest cells ──
    print(f"\n--- Phase 5: Cell Harvest ({total} chambers) ---")
    print(f"Collection plate wells {total}-{2*total-1} = harvested cell suspensions")
    for i, ch in enumerate(chambers):
        print(f"  [{i+1}/{total}]", end="")
        # Collection wells for cells start after conditioned media wells
        harvest_cells(sender, cal, ch, total + i, log_file)

    elapsed = time.time() - start_time
    mins = int(elapsed) // 60

    print()
    print("=" * 60)
    print(f"CELL HARVEST COMPLETE — {total} chambers in {mins} min")
    print(f"  Collection plate layout:")
    print(f"    Wells 0-{total-1}:  Conditioned media (secretome analysis)")
    print(f"    Wells {total}-{2*total-1}: Cell suspensions (flow cytometry / qPCR)")
    print(f"  Keep collection plate on ice until analysis.")
    print("=" * 60)

    if log_file:
        log_file.write(f"\n# Completed at {datetime.now().isoformat()}\n")
        log_file.write(f"# Total elapsed: {mins} min\n")
        log_file.close()
        print(f"Log saved: {log_path}")


def main():
    parser = argparse.ArgumentParser(description="LaminarForge cell harvest protocol")
    parser.add_argument("--port", default=DEFAULT_PORT, help="Serial port")
    parser.add_argument("--dry-run", action="store_true", help="Print G-code without sending")
    parser.add_argument("--chambers", type=str, default=None,
                        help="Comma-separated chamber numbers (default: all 16)")
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

    run_cell_harvest(sender, cal, chambers, args.skip_confirm)

    sender.raise_to_safe()
    sender.disconnect()


if __name__ == "__main__":
    main()
