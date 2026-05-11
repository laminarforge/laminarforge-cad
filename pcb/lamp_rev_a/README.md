# LaminarForge LAMP Rev A PCBA

This directory is the source package for the real one-board LAMP PCBA.

The goal is one 4-layer ESP32-S3 board that can be checked, laid out in KiCad,
exported for JLCPCB, and iterated without going back to the old generated-layout
workflow.

## Source Of Truth

- `contract.toml`: board requirements, rails, nets, GPIO map, test points, and verification gates.
- `parts.toml`: selected schematic parts plus explicit fab-blocking part-selection gaps.
- `power_architecture.toml`: locked Rev A power topology and the explicit no-buck decision.
- `optical_architecture.toml`: locked Rev A optical topology, wavelength, detector, and front-end decision.
- `optical_mode.md`: human-readable optical-mode decision and bench validation gate.
- `placement.toml`: locked starting refs, coordinates, test points, and eight-slot optical geometry.
- `pin_nets.toml`: conservative footprint pad-to-net assignments for known package pinouts.
- `lamp_rev_a.kicad_sch`: KiCad schematic shell for the one-board Rev A electrical architecture.
- `lamp_rev_a.kicad_pcb`: materialized KiCad board with the Rev A outline, 4-layer stack, placed footprints, test points, and optical-slot guides.
- `lamp_rev_a.kicad_pro`: KiCad project shell.
- `sym-lib-table` / `fp-lib-table`: project-local library tables pointing at the locked `pcb/lib/lcsc` symbols and footprints.
- `lamp_rev_a.kicad_dru`: custom KiCad design rules for power, heater, analog, and USB constraints.
- `kibot.yaml`: intended fab/report export pipeline once the schematic and placed board exist.

Rust should validate contracts and automate exports. KiCad should own the
schematic and PCB layout.

## Iteration Flow

```text
contract.toml + parts.toml
  -> cargo run --release --bin lamp_pcba_check
  -> placement.toml refs and zone checks
  -> pin_nets.toml trusted pad assignments
  -> cargo run --release --bin lamp_rev_a_materialize_board
  -> KiCad schematic capture in lamp_rev_a.kicad_sch
  -> KiCad ERC
  -> footprint/BOM/JLC field lock
  -> KiCad placement cleanup from the materialized board
  -> KiCad interactive routing
  -> KiCad DRC + schematic/PCB parity
  -> KiBot/KiKit fab outputs
  -> physical bring-up measurements
  -> update contract.toml for the next revision
```

## Board Decisions

- One physical board for Rev A.
- 4-layer stack: `F.Cu`, `In1.Cu` ground plane, `In2.Cu` power plane, `B.Cu`.
- ESP32-S3 module, not a bare ESP32 chip.
- USB VBUS is the Rev A electronics 5 V source through a Schottky path; +3V3 is derived locally.
- The 12 V input is reserved for heater power. Do not add a 12 V to 5 V buck unless standalone non-USB operation becomes a new explicit Rev change.
- External heater element driven from the board through a protected high-current output.
- Heater path protection is locked as board-side resettable fuse + TVS + 5.08 mm heater terminal, with an external inline KSD9700 thermal cutoff mounted on the heater assembly.
- Optical mode is locked to 650 nm red-light turbidimetry with eight LED/photodiode channels, muxed into one TIA and ADC path.
- Starting placement is locked in `placement.toml`: 91 top-side footprints, 16 test points, and 8 optical slots at 12 mm pitch.
- Test points are mandatory for rails, heater control/output, I2C, UART, boot/reset, ADC, and mux output.

## Local Checks

Run:

```bash
cargo run --release --bin lamp_pcba_check
cargo run --release --bin lamp_rev_a_materialize_board
cargo run --release --bin lamp_rev_a_fab_preview
```

The current schematic is an architecture shell, not a fabrication-ready circuit.
The checker should not report fab-blocking part-selection gaps. The board now
materializes all selected parts into KiCad with zero physical DRC violations
before routing. The active work is schematic completion, routing the 93 current
ratlines, schematic/PCB parity, and bench validation.

Current KiCad checks:

```bash
kicad-cli sch erc pcb/lamp_rev_a/lamp_rev_a.kicad_sch \
  -o pcb/lamp_rev_a/reports/erc.json --format json

kicad-cli pcb drc --format json \
  --output pcb/lamp_rev_a/reports/drc.json \
  pcb/lamp_rev_a/lamp_rev_a.kicad_pcb
```

Expected DRC state at this stage: `0 violations` and unrouted connections still
listed under `unconnected_items`.

Do not accept autorouter output by itself. Use it to expose bad placement or
constraint problems, then finish and review the layout in KiCad.
