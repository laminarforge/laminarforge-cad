# LaminarForge LAMP Rev A PCBA

This directory is the source package for the real one-board LAMP PCBA.

The goal is one 4-layer ESP32-S3 board that can be checked, laid out in KiCad,
exported for JLCPCB, and iterated without going back to the old generated-layout
workflow.

## Source Of Truth

- `contract.toml`: board requirements, rails, nets, GPIO map, test points, and verification gates.
- `lamp_rev_a.kicad_pcb`: KiCad board seed with the Rev A outline and 4-layer stack.
- `lamp_rev_a.kicad_pro`: KiCad project shell.
- `lamp_rev_a.kicad_dru`: custom KiCad design rules for power, heater, analog, and USB constraints.
- `kibot.yaml`: intended fab/report export pipeline once the schematic and placed board exist.

Rust should validate contracts and automate exports. KiCad should own the
schematic and PCB layout.

## Iteration Flow

```text
contract.toml
  -> cargo run --release --bin lamp_pcba_check
  -> KiCad hierarchical schematic
  -> KiCad ERC
  -> footprint/BOM/JLC field lock
  -> KiCad placement using the board zones in contract.toml
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
- External heater element driven from the board through a protected high-current output.
- Eight optical channels on the same board, but routed as a constrained analog section.
- Test points are mandatory for rails, heater control/output, I2C, UART, boot/reset, ADC, and mux output.

## Local Checks

Run:

```bash
cargo run --release --bin lamp_pcba_check
```

When the KiCad schematic exists, the acceptance path becomes:

```bash
kicad-cli sch erc pcb/lamp_rev_a/lamp_rev_a.kicad_sch \
  -o pcb/lamp_rev_a/reports/erc.json --format json

kicad-cli pcb drc pcb/lamp_rev_a/lamp_rev_a.kicad_pcb \
  -o pcb/lamp_rev_a/reports/drc.json --format json --severity-all
```

Do not accept autorouter output by itself. Use it to expose bad placement or
constraint problems, then finish and review the layout in KiCad.
