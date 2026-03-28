"""
LaminarForge Liquid Handler — Deck & Protocol Configuration
============================================================

All coordinates, volumes, flow rates, and timing parameters for the
Rev C 16-chamber microfluidic chip liquid handling protocols.

Chip: CNC-milled PMMA, ANSI/SLAS microplate footprint (127.76 x 85.48 mm)
      16 chambers in 4x4 grid, open wells on top face aligned to 96-well
      grid positions (9mm pitch) for robotic compatibility.

Syringe: Hamilton 1710RN 100 uL glass syringe
         I-axis: 1600 steps/mm, 2mm lead screw
         Full stroke: 100 uL over ~58.3mm of barrel travel
         Conversion: 1mm I-axis travel = 1.714 uL

Motion controller: FluidNC (ESP32-based), 4-axis (X, Y, Z, I)
"""

import os
import json
from dataclasses import dataclass, field
from typing import List, Tuple, Optional


# ──────────────────────────────────────────────────────────────
# Syringe parameters (Hamilton 1710RN 100 uL)
# ──────────────────────────────────────────────────────────────

SYRINGE_VOLUME_UL = 100.0          # Full syringe volume
SYRINGE_STROKE_MM = 58.3           # Full mechanical stroke
UL_PER_MM = SYRINGE_VOLUME_UL / SYRINGE_STROKE_MM  # ~1.714 uL/mm
MM_PER_UL = 1.0 / UL_PER_MM       # ~0.583 mm/uL

def volume_to_mm(volume_ul: float) -> float:
    """Convert a volume in uL to I-axis travel in mm."""
    return volume_ul * MM_PER_UL

def mm_to_volume(mm: float) -> float:
    """Convert I-axis travel in mm to volume in uL."""
    return mm * UL_PER_MM


# ──────────────────────────────────────────────────────────────
# I-axis feed rates (mm/min for G-code F parameter)
# FluidNC interprets F values as mm/min for linear axes.
#
# Conversion: flow_rate_ul_per_s / UL_PER_MM * 60 = mm/min
# ──────────────────────────────────────────────────────────────

@dataclass
class FlowRate:
    """Named flow rate with uL/s and pre-computed G-code F value."""
    name: str
    ul_per_s: float

    @property
    def mm_per_s(self) -> float:
        return self.ul_per_s / UL_PER_MM

    @property
    def f_value(self) -> float:
        """G-code F parameter (mm/min)."""
        return self.mm_per_s * 60.0


# Gentle delivery through microfluidic channels (0.01 uL/s).
# This is the flow rate seen by cells inside the 200um-deep chambers.
# At 0.01 uL/s through a 500um x 200um channel cross-section,
# shear stress is well below the ~1 dyne/cm^2 threshold for most cell types.
FLOW_GENTLE = FlowRate("gentle_channel", 0.01)

# Open-well aspiration (0.5 uL/s).
# When aspirating from the open 4mm-diameter, 10mm-deep wells,
# liquid is not flowing through channels so we can go faster.
# Still gentle enough to avoid splashing or pulling air.
FLOW_ASPIRATE = FlowRate("open_well_aspirate", 0.5)

# Moderate flow for harvest flushing (0.1 uL/s).
# Higher shear helps detach trypsinized cells from the chamber floor.
# Still within physiological range to avoid lysing cells.
FLOW_HARVEST = FlowRate("harvest_flush", 0.1)

# Wash cycle — fast aspiration/expulsion of cleaning solutions (5 uL/s).
# Used for bleach/DI rinse cycles between samples.  No cells present.
FLOW_WASH = FlowRate("wash", 5.0)

# Very slow AAV delivery (0.005 uL/s).
# AAV is expensive (~$500/uL at research grade).  Ultra-slow delivery
# maximizes contact time in the chamber and minimizes waste in dead volume.
FLOW_AAV = FlowRate("aav_delivery", 0.005)


# ──────────────────────────────────────────────────────────────
# Z-axis positions (mm, negative = down from home)
# ──────────────────────────────────────────────────────────────

Z_SAFE = 0.0          # Fully retracted — safe for XY travel
Z_WELL_TOP = -60.0    # Just above well rim (4mm diameter wells, 10mm deep)
Z_ASPIRATE = -68.0    # 1-2mm above well bottom for aspiration
Z_DISPENSE = -62.0    # Into well, above liquid surface for dispensing
Z_WASH = -55.0        # Into wash well (larger, shallower vessels)
Z_BLOT = -58.0        # Touch blot pad to wick residual droplet


# ──────────────────────────────────────────────────────────────
# XY feed rates (mm/min) for gantry moves
# ──────────────────────────────────────────────────────────────

FEED_XY_RAPID = 6000   # Rapid XY travel (100 mm/s) — G0 moves
FEED_XY_SLOW = 600     # Slow approach (10 mm/s) — near wells
FEED_Z_RAPID = 3000    # Z rapid (50 mm/s)
FEED_Z_SLOW = 300      # Z slow approach (5 mm/s) — entering wells


# ──────────────────────────────────────────────────────────────
# Deck positions — fixed stations on the deck adapter plate
# These are absolute machine coordinates (mm) set during deck setup.
# ──────────────────────────────────────────────────────────────

@dataclass
class DeckPosition:
    """A named XY position on the deck."""
    name: str
    x: float
    y: float
    z_working: float = Z_WASH  # Default Z for interaction

# Wash station — three adjacent wells on the deck
WASH_BLEACH = DeckPosition("wash_bleach", x=200.0, y=50.0, z_working=Z_WASH)
WASH_DI     = DeckPosition("wash_di",     x=200.0, y=85.0, z_working=Z_WASH)
WASH_BLOT   = DeckPosition("wash_blot",   x=200.0, y=120.0, z_working=Z_BLOT)

# Reagent reservoirs — temperature-controlled wells on deck
RESERVOIR_WARM_MEDIA = DeckPosition("warm_media", x=50.0, y=200.0, z_working=Z_ASPIRATE)
RESERVOIR_COLD_AAV   = DeckPosition("cold_aav",   x=100.0, y=200.0, z_working=Z_ASPIRATE)
RESERVOIR_PBS        = DeckPosition("pbs",         x=150.0, y=200.0, z_working=Z_ASPIRATE)
RESERVOIR_ACCUTASE   = DeckPosition("accutase",    x=150.0, y=150.0, z_working=Z_ASPIRATE)
RESERVOIR_MEDIA_BSA  = DeckPosition("media_bsa_edta", x=50.0, y=150.0, z_working=Z_ASPIRATE)

# Waste and collection
WASTE = DeckPosition("waste", x=250.0, y=50.0, z_working=Z_WASH)

# Sample collection plate — 96-well PCR plate for harvested cells/AAV samples
# Individual well positions calculated from A1 corner + 9mm pitch
COLLECTION_PLATE_A1 = DeckPosition("collection_A1", x=50.0, y=-50.0, z_working=Z_DISPENSE)
COLLECTION_PITCH = 9.0  # mm, standard 96-well pitch


# ──────────────────────────────────────────────────────────────
# Rev C chip well positions
#
# The chip sits on a deck adapter plate.  After calibration,
# we know the machine coordinates of Well A2 (chamber 1 inlet).
# All other wells are calculated from the 96-well grid (9mm pitch).
#
# Grid layout (4 columns x 8 rows):
#   Columns:  2, 5, 8, 11  (96-well grid, 1-indexed)
#   Row pairs: (A,B), (C,D), (E,F), (G,H)
#     A/C/E/G = inlet wells,  B/D/F/H = outlet wells
#
# 16 chambers numbered 1-16, row-major through 4x4 grid.
# ──────────────────────────────────────────────────────────────

# 96-well grid constants (ANSI/SLAS 4-2004)
WELL96_A1_X = 14.38     # mm, A1 center from plate corner
WELL96_A1_Y = 11.24     # mm
WELL96_PITCH = 9.0       # mm

# Rev C column indices in 96-well grid (0-indexed for math)
REVC_COL_INDICES_0 = [1, 4, 7, 10]  # columns 2, 5, 8, 11 (0-indexed)

# Rev C row pair indices (0-indexed): (inlet_row, outlet_row)
REVC_ROW_PAIRS_0 = [(0, 1), (2, 3), (4, 5), (6, 7)]  # (A,B), (C,D), (E,F), (G,H)

# Chamber labels for logging and user display
CHAMBER_LABELS = [
    "DRG sensory neurons (TARGET)",
    "Hepatocytes (liver)",
    "Motor neurons",
    "Cortical neurons",
    "Astrocytes",
    "Cardiomyocytes (heart)",
    "Skeletal muscle",
    "Endothelial cells",
    "Kidney (renal epithelial)",
    "Lung epithelial",
    "PBMCs / T cells",
    "Pancreatic beta cells",
    "Enteric neurons",
    "Schwann cells",
    "Positive control (HEK293)",
    "Negative control (empty)",
]

# Well labels: (inlet_well_name, outlet_well_name) for each chamber
WELL_LABELS = [
    ("A2",  "B2"),   # Ch1
    ("A5",  "B5"),   # Ch2
    ("A8",  "B8"),   # Ch3
    ("A11", "B11"),  # Ch4
    ("C2",  "D2"),   # Ch5
    ("C5",  "D5"),   # Ch6
    ("C8",  "D8"),   # Ch7
    ("C11", "D11"),  # Ch8
    ("E2",  "F2"),   # Ch9
    ("E5",  "F5"),   # Ch10
    ("E8",  "F8"),   # Ch11
    ("E11", "F11"),  # Ch12
    ("G2",  "H2"),   # Ch13
    ("G5",  "H5"),   # Ch14
    ("G8",  "H8"),   # Ch15
    ("G11", "H11"),  # Ch16
]


@dataclass
class ChipWell:
    """A single well on the chip with absolute machine coordinates."""
    name: str           # e.g., "A2"
    chamber: int        # 1-16
    well_type: str      # "inlet" or "outlet"
    label: str          # tissue type
    x: float            # absolute machine X
    y: float            # absolute machine Y


@dataclass
class ChipCalibration:
    """
    Calibration data: maps chip well grid to absolute machine coordinates.

    After jogging the needle to Well A2 (chamber 1 inlet) and recording
    the machine position, all other well positions are calculated using
    the known 9mm pitch of the 96-well grid.

    Well A2 is at grid position (row=0, col=1) in 0-indexed coordinates,
    which means:
      A2.x = A1.x + 1 * 9mm  (one column right of A1)
      A2.y = A1.y + 0 * 9mm  (same row as A1)
    """
    # Machine coordinates of Well A2 (taught position)
    a2_machine_x: float
    a2_machine_y: float

    # Grid position of A2: column index 1 (0-indexed), row index 0
    # We use this to back-calculate the machine position of any grid cell.
    a2_grid_col: int = 1   # A2 is in column 2 (0-indexed: 1)
    a2_grid_row: int = 0   # A2 is in row A (0-indexed: 0)

    def grid_to_machine(self, grid_col_0: int, grid_row_0: int) -> Tuple[float, float]:
        """
        Convert a 96-well grid position (0-indexed) to absolute machine XY.

        The chip X axis aligns with the 96-well column axis.
        The chip Y axis aligns with the 96-well row axis.
        """
        dx = (grid_col_0 - self.a2_grid_col) * WELL96_PITCH
        dy = (grid_row_0 - self.a2_grid_row) * WELL96_PITCH
        return (self.a2_machine_x + dx, self.a2_machine_y + dy)

    def get_all_wells(self) -> List[ChipWell]:
        """
        Calculate absolute machine coordinates for all 32 wells (16 inlets + 16 outlets).
        Returns a list of ChipWell objects ordered by chamber number.
        """
        wells = []
        chamber = 1
        for row_idx, (inlet_row, outlet_row) in enumerate(REVC_ROW_PAIRS_0):
            for col_idx, grid_col in enumerate(REVC_COL_INDICES_0):
                inlet_name, outlet_name = WELL_LABELS[chamber - 1]
                label = CHAMBER_LABELS[chamber - 1]

                # Inlet well
                ix, iy = self.grid_to_machine(grid_col, inlet_row)
                wells.append(ChipWell(
                    name=inlet_name, chamber=chamber,
                    well_type="inlet", label=label, x=ix, y=iy
                ))

                # Outlet well
                ox, oy = self.grid_to_machine(grid_col, outlet_row)
                wells.append(ChipWell(
                    name=outlet_name, chamber=chamber,
                    well_type="outlet", label=label, x=ox, y=oy
                ))

                chamber += 1
        return wells

    def get_inlet(self, chamber: int) -> ChipWell:
        """Get the inlet well for a specific chamber (1-indexed)."""
        wells = self.get_all_wells()
        for w in wells:
            if w.chamber == chamber and w.well_type == "inlet":
                return w
        raise ValueError(f"No inlet found for chamber {chamber}")

    def get_outlet(self, chamber: int) -> ChipWell:
        """Get the outlet well for a specific chamber (1-indexed)."""
        wells = self.get_all_wells()
        for w in wells:
            if w.chamber == chamber and w.well_type == "outlet":
                return w
        raise ValueError(f"No outlet found for chamber {chamber}")


def collection_well_xy(well_index: int) -> Tuple[float, float]:
    """
    Get absolute XY for a collection plate well (0-indexed, row-major).
    Well 0 = A1, Well 1 = A2, ..., Well 11 = A12, Well 12 = B1, etc.
    """
    row = well_index // 12
    col = well_index % 12
    x = COLLECTION_PLATE_A1.x + col * COLLECTION_PITCH
    y = COLLECTION_PLATE_A1.y + row * COLLECTION_PITCH
    return (x, y)


# ──────────────────────────────────────────────────────────────
# Protocol volumes (uL)
# ──────────────────────────────────────────────────────────────

# Media change
MEDIA_ASPIRATE_VOL = 80.0      # Aspirate spent media from outlet
MEDIA_DISPENSE_VOL = 80.0      # Dispense fresh media into inlet

# Wash cycle
WASH_VOL = 50.0                # Per wash aspiration
WASH_BLEACH_REPS = 2           # Bleach wash repetitions
WASH_DI_REPS = 3               # DI rinse repetitions
WASH_BLEACH_SOAK_S = 5.0       # Hold bleach in syringe (seconds)
WASH_BLOT_TOUCH_S = 1.0        # Touch blot pad duration

# AAV transduction
AAV_DELIVERY_VOL = 50.0        # AAV solution per chamber (precious!)
AAV_INCUBATION_MIN = 120       # 2-hour incubation (can extend to 4hr)
AAV_COLLECTION_VOL = 60.0      # Aspirate post-AAV media for titer analysis
PBS_WASH_VOL = 80.0            # PBS rinse volume
PBS_WASH_REPS = 3              # PBS wash repetitions after AAV removal

# Cell harvest
ACCUTASE_VOL = 50.0            # Enzymatic dissociation reagent
ACCUTASE_INCUBATION_MIN = 6    # 5-7 min at 37C (timer midpoint)
HARVEST_FLUSH_VOL = 80.0       # Media+BSA+EDTA flush to collect cells
HARVEST_FLUSH_REPS = 3         # Number of flush cycles
CONDITIONED_MEDIA_VOL = 60.0   # Pre-harvest conditioned media collection


# ──────────────────────────────────────────────────────────────
# Calibration persistence
# ──────────────────────────────────────────────────────────────

CALIBRATION_FILE = os.path.join(
    os.path.dirname(os.path.abspath(__file__)),
    "calibration.json"
)

def save_calibration(cal: ChipCalibration) -> None:
    """Save calibration data to JSON file."""
    data = {
        "a2_machine_x": cal.a2_machine_x,
        "a2_machine_y": cal.a2_machine_y,
        "a2_grid_col": cal.a2_grid_col,
        "a2_grid_row": cal.a2_grid_row,
    }
    with open(CALIBRATION_FILE, "w") as f:
        json.dump(data, f, indent=2)
    print(f"Calibration saved to {CALIBRATION_FILE}")

def load_calibration() -> ChipCalibration:
    """Load calibration data from JSON file."""
    if not os.path.exists(CALIBRATION_FILE):
        raise FileNotFoundError(
            f"No calibration file found at {CALIBRATION_FILE}.\n"
            "Run deck_calibration.py first to teach the Well A2 position."
        )
    with open(CALIBRATION_FILE, "r") as f:
        data = json.load(f)
    return ChipCalibration(
        a2_machine_x=data["a2_machine_x"],
        a2_machine_y=data["a2_machine_y"],
        a2_grid_col=data.get("a2_grid_col", 1),
        a2_grid_row=data.get("a2_grid_row", 0),
    )
