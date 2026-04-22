#!/bin/bash
#
# Export JLCPCB-ready fabrication package for the ESP32-S3 controller PCB.
# Uses kicad-cli for Gerbers, drill files, and position file.
#
# Layers (JLCPCB 2-layer standard):
#   F.Cu, B.Cu, F.SilkS, B.SilkS, F.Mask, B.Mask, Edge.Cuts
#
# Drill files: Excellon format, PTH and NPTH separate.
# Pick-and-place: CSV in mm with Mid X / Mid Y / Layer / Rotation.

set -euo pipefail

REPO_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
OUT_DIR="${REPO_DIR}/pcb/output"
FAB_DIR="${OUT_DIR}/controller_fab"
PCB_FILE="${OUT_DIR}/controller.kicad_pcb"
KICAD_CLI="/opt/homebrew/bin/kicad-cli"

if [ ! -f "$PCB_FILE" ]; then
  echo "PCB file not found: $PCB_FILE" >&2
  exit 1
fi

echo "Fab output dir: $FAB_DIR"
mkdir -p "$FAB_DIR/gerbers" "$FAB_DIR/drills"

# ── 1) Gerbers ─────────────────────────────────────────────────────────────
echo ">> Exporting Gerbers..."
"$KICAD_CLI" pcb export gerbers \
  --output "$FAB_DIR/gerbers/" \
  --layers "F.Cu,B.Cu,F.Silkscreen,B.Silkscreen,F.Mask,B.Mask,Edge.Cuts" \
  --no-x2 \
  --subtract-soldermask \
  --use-drill-file-origin \
  "$PCB_FILE"

# ── 2) Drill files (Excellon, PTH + NPTH separate) ────────────────────────
echo ">> Exporting drill files..."
"$KICAD_CLI" pcb export drill \
  --output "$FAB_DIR/drills/" \
  --format excellon \
  --drill-origin plot \
  --excellon-units mm \
  --excellon-separate-th \
  --generate-map \
  --map-format pdf \
  "$PCB_FILE"

# ── 3) Pick-and-place CSV ─────────────────────────────────────────────────
echo ">> Exporting pick-and-place..."
"$KICAD_CLI" pcb export pos \
  --output "$FAB_DIR/controller_cpl_jlcpcb.csv" \
  --side both \
  --format csv \
  --units mm \
  --use-drill-file-origin \
  "$PCB_FILE"

# ── 4) Copy BOM (already JLCPCB-ready from controller_pcb.rs) ─────────────
echo ">> Copying BOM..."
cp "$OUT_DIR/controller_bom.csv" "$FAB_DIR/controller_bom_jlcpcb.csv"

# ── 5) Also copy DSN + SES for reference ──────────────────────────────────
cp "$OUT_DIR/controller.dsn" "$FAB_DIR/controller.dsn"
if [ -f "$OUT_DIR/controller.ses" ]; then
  cp "$OUT_DIR/controller.ses" "$FAB_DIR/controller.ses"
fi

# ── 6) Zip it up for upload ───────────────────────────────────────────────
echo ">> Creating controller_jlcpcb_v1.zip..."
cd "$FAB_DIR"
rm -f controller_jlcpcb_v1.zip
zip -r controller_jlcpcb_v1.zip gerbers/ drills/ controller_cpl_jlcpcb.csv controller_bom_jlcpcb.csv

echo ""
echo "=== Fab package created ==="
ls -la "$FAB_DIR"
echo ""
echo "Zip contents:"
unzip -l "$FAB_DIR/controller_jlcpcb_v1.zip"
