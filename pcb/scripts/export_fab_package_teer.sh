#!/bin/bash
#
# Export JLCPCB-ready fabrication package for the TEER Readout Daughter Board.
# Uses kicad-cli for Gerbers, drill files, and position file.
#
# Layers (JLCPCB 2-layer standard):
#   F.Cu, B.Cu, F.SilkS, B.SilkS, F.Mask, B.Mask, Edge.Cuts

set -euo pipefail

REPO_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
OUT_DIR="${REPO_DIR}/pcb/output"
FAB_DIR="${OUT_DIR}/teer_readout_fab"
PCB_FILE="${OUT_DIR}/teer_readout.kicad_pcb"
KICAD_CLI="/Applications/KiCad/KiCad.app/Contents/MacOS/kicad-cli"

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

# ── 2) Drill files ─────────────────────────────────────────────────────────
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
  --output "$FAB_DIR/teer_readout_cpl_jlcpcb.csv" \
  --side both \
  --format csv \
  --units mm \
  --use-drill-file-origin \
  "$PCB_FILE"

# ── 4) Copy BOM (already JLCPCB-ready from teer_readout_pcb.rs) ───────────
echo ">> Copying BOM..."
cp "$OUT_DIR/teer_readout_bom.csv" "$FAB_DIR/teer_readout_bom_jlcpcb.csv"

# ── 5) DSN + SES for reference ────────────────────────────────────────────
cp "$OUT_DIR/teer_readout.dsn" "$FAB_DIR/teer_readout.dsn"
if [ -f "$OUT_DIR/teer_readout.ses" ]; then
  cp "$OUT_DIR/teer_readout.ses" "$FAB_DIR/teer_readout.ses"
fi

# ── 6) Zip up for upload ──────────────────────────────────────────────────
echo ">> Creating teer_readout_jlcpcb_v1.zip..."
cd "$FAB_DIR"
rm -f teer_readout_jlcpcb_v1.zip
zip -r teer_readout_jlcpcb_v1.zip gerbers/ drills/ teer_readout_cpl_jlcpcb.csv teer_readout_bom_jlcpcb.csv

echo ""
echo "=== Fab package created ==="
ls -la "$FAB_DIR"
echo ""
echo "Zip contents:"
unzip -l "$FAB_DIR/teer_readout_jlcpcb_v1.zip"
