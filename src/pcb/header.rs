use super::nets::NET_NAMES;
use std::fmt::Write;

pub fn write_header(pcb: &mut String) {
    pcb.push_str(
        r#"(kicad_pcb
  (version 20240108)
  (generator "laminarforge_pcb_gen")
  (generator_version "1.0")
  (general
    (thickness 1.6)
    (legacy_teardrops no)
  )
  (paper "A4")
  (layers
    (0 "F.Cu" signal)
    (31 "B.Cu" signal)
    (32 "B.Adhes" user "B.Adhesive")
    (33 "F.Adhes" user "F.Adhesive")
    (34 "B.Paste" user)
    (35 "F.Paste" user)
    (36 "B.SilkS" user "B.Silkscreen")
    (37 "F.SilkS" user "F.Silkscreen")
    (38 "B.Mask" user "B.Mask")
    (39 "F.Mask" user "F.Mask")
    (40 "Dwgs.User" user "User.Drawings")
    (41 "Cmts.User" user "User.Comments")
    (42 "Eco1.User" user "User.Eco1")
    (43 "Eco2.User" user "User.Eco2")
    (44 "Edge.Cuts" user)
    (45 "Margin" user)
    (46 "B.CrtYd" user "B.Courtyard")
    (47 "F.CrtYd" user "F.Courtyard")
    (48 "B.Fab" user "B.Fabrication")
    (49 "F.Fab" user "F.Fabrication")
  )
"#,
    );
}

pub fn write_nets(pcb: &mut String) {
    for (i, name) in NET_NAMES.iter().enumerate() {
        if i == 0 {
            writeln!(pcb, "  (net 0 \"\")").unwrap();
        } else {
            writeln!(pcb, "  (net {} \"{}\")", i, name).unwrap();
        }
    }
    pcb.push('\n');
}

pub fn write_setup(pcb: &mut String) {
    // Design rules tuned for JLCPCB 2-layer standard capabilities:
    //   Min clearance: 0.127mm (5mil), we use 0.15mm
    //   Min track: 0.127mm, we use 0.25mm
    //   Min via: 0.45mm pad / 0.2mm drill, we use 0.7/0.35mm
    //   Min hole clearance: ~0.15mm, we use 0.2mm
    //   Min solder mask bridge: 0.09mm
    pcb.push_str(
        r#"  (setup
    (pad_to_mask_clearance 0)
    (allow_soldermask_bridges_in_footprints yes)
    (pcbplotparams
      (layerselection 0x00010fc_ffffffff)
      (plot_on_all_layers_selection 0x0000000_00000000)
      (disableapertmacros no)
      (usegerberextensions yes)
      (usegerberattributes yes)
      (usegerberadvancedattributes yes)
      (creategerberjobfile yes)
      (dashed_line_dash_ratio 12.000000)
      (dashed_line_gap_ratio 3.000000)
      (svgprecision 4)
      (plotframeref no)
      (viasonmask no)
      (mode 1)
      (useauxorigin no)
      (hpglpennumber 1)
      (hpglpenspeed 20)
      (hpglpendiameter 15.000000)
      (pdf_front_fp_property_popups yes)
      (pdf_back_fp_property_popups yes)
      (dxf_units mm)
      (dxfpolygonmode yes)
      (dxfimperialunits no)
      (dxfusepcbnewfont yes)
      (psnegative no)
      (psa4output no)
      (plotreference yes)
      (plotvalue yes)
      (plotfptext yes)
      (plotinvisibletext no)
      (sketchpadsonfab no)
      (subtractmaskfromsilk yes)
      (outputformat 1)
      (mirror no)
      (drillshape 0)
      (scaleselection 1)
      (outputdirectory "gerbers/")
    )
  )
  (net_class Default "Default"
    (clearance 0.127)
    (trace_width 0.25)
    (via_dia 0.7)
    (via_drill 0.35)
    (uvia_dia 0.3)
    (uvia_drill 0.1)
  )
"#,
    );
}
