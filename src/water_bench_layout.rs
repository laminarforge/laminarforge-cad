//! Combined dry layout; reference envelopes, no installed-hardware or tube-fit claim.
use super::*;
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Layout {
    source_x_mm: f64,
    source_y_mm: f64,
    dock_x_mm: f64,
    dock_y_mm: f64,
    outlet_x_mm: f64,
    outlet_y_mm: f64,
    minimum_module_gap_mm: f64,
    minimum_outlet_air_gap_mm: f64,
    maximum_outlet_air_gap_mm: f64,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Bench {
    layout: Layout,
    source: source::SourceSaddle,
    dock: super::super::Parameters,
    outlet: outlet::OutletSupport,
    receiver: Receiver,
}
fn shift(b: Bounds, t: [f64; 3]) -> Bounds {
    (
        [b.0[0] + t[0], b.0[1] + t[1], b.0[2] + t[2]],
        [b.1[0] + t[0], b.1[1] + t[1], b.1[2] + t[2]],
    )
}
fn gap_xy(a: Bounds, b: Bounds) -> f64 {
    (b.0[0] - a.1[0])
        .max(a.0[0] - b.1[0])
        .max(b.0[1] - a.1[1])
        .max(a.0[1] - b.1[1])
}
fn translated(p: [f64; 3], t: [f64; 3]) -> [f64; 3] {
    [p[0] + t[0], p[1] + t[1], p[2] + t[2]]
}
pub(super) fn run(args: &Args, raw: &str) -> Result<(), Box<dyn std::error::Error>> {
    let p: Bench = toml::from_str(raw)?;
    finite(&p)?;
    p.source.validate()?;
    p.outlet.validate()?;
    p.receiver.validate()?;
    let d = p.dock.validate()?;
    let l = &p.layout;
    require(
        [
            l.source_x_mm,
            l.source_y_mm,
            l.dock_x_mm,
            l.dock_y_mm,
            l.outlet_x_mm,
            l.outlet_y_mm,
        ]
        .iter()
        .all(|v| v.abs() <= 1000.),
        "layout coordinate outside 1 m candidate range",
    )?;
    require(
        (10. ..=100.).contains(&l.minimum_module_gap_mm)
            && (5. ..=20.).contains(&l.minimum_outlet_air_gap_mm)
            && l.maximum_outlet_air_gap_mm >= l.minimum_outlet_air_gap_mm
            && l.maximum_outlet_air_gap_mm <= 40.,
        "invalid layout gap limits",
    )?;
    let st = [l.source_x_mm, l.source_y_mm, 0.];
    let dt = [l.dock_x_mm, l.dock_y_mm, 0.];
    let ot = [l.outlet_x_mm, l.outlet_y_mm, 0.];
    let tube = translated(p.outlet.tube_interface(), ot);
    let ct = [tube[0], tube[1], 0.];
    let source_bounds = shift(
        p.source
            .exports()
            .pop()
            .ok_or("source assembly missing")?
            .bounds,
        st,
    );
    let outlet_bounds = shift(
        p.outlet
            .exports()
            .pop()
            .ok_or("outlet assembly missing")?
            .bounds,
        ot,
    );
    let hx = d.guide_mount_x_mm + p.dock.guide_length_mm / 2.;
    let hy = p.dock.tray_width_mm / 2.;
    let hz = (p.dock.mount_pad_height_mm + d.guide_height_mm)
        .max(d.cradle_assembly_z_mm + d.cradle_height_mm)
        .max(d.tray_rim_z_mm);
    let dock_bounds = shift(([-hx, -hy, 0.], [hx, hy, hz]), dt);
    let vial_top = p.receiver.vial_z() + p.receiver.vial_height_mm;
    let collector_bounds = shift(
        (
            [
                -p.receiver.scale_length_mm / 2.,
                -p.receiver.scale_width_mm / 2.,
                0.,
            ],
            [
                p.receiver.scale_length_mm / 2.,
                p.receiver.scale_width_mm / 2.,
                vial_top,
            ],
        ),
        ct,
    );
    let base = p.outlet.base_size();
    let base_bounds = shift(
        (
            [-base[0] / 2., -base[1] / 2., 0.],
            [base[0] / 2., base[1] / 2., base[2]],
        ),
        ot,
    );
    let source_dock_gap = gap_xy(source_bounds, dock_bounds);
    let dock_collector_gap = gap_xy(dock_bounds, collector_bounds);
    let base_collector_gap = gap_xy(base_bounds, collector_bounds);
    let air_gap = tube[2] - vial_top;
    require(
        source_dock_gap >= l.minimum_module_gap_mm
            && dock_collector_gap >= l.minimum_module_gap_mm
            && base_collector_gap >= l.minimum_module_gap_mm,
        "component plan envelopes overlap or violate minimum gap",
    )?;
    require(
        gap_xy(source_bounds, outlet_bounds) >= l.minimum_module_gap_mm,
        "source/outlet layout envelopes conflict",
    )?;
    require(
        air_gap >= l.minimum_outlet_air_gap_mm && air_gap <= l.maximum_outlet_air_gap_mm,
        "nominal tube tip/vial air gap outside layout range",
    )?;
    let source_part = || {
        p.source
            .exports()
            .pop()
            .unwrap()
            .part
            .translate(st[0], st[1], 0.)
    };
    let dock_part = || super::super::build(&p.dock, &d, 3).translate(dt[0], dt[1], 0.);
    let outlet_part = || {
        p.outlet
            .exports()
            .pop()
            .unwrap()
            .part
            .translate(ot[0], ot[1], 0.)
    };
    let collector_part = || {
        (p.receiver.scale()
            + p.receiver
                .nest()
                .translate(0., 0., p.receiver.scale_pan_height_mm)
            + p.receiver.vial().translate(0., 0., p.receiver.vial_z()))
        .translate(ct[0], ct[1], 0.)
    };
    let all = [source_bounds, dock_bounds, outlet_bounds, collector_bounds];
    let low = [0, 1, 2].map(|axis| all.iter().map(|b| b.0[axis]).fold(f64::INFINITY, f64::min));
    let high = [0, 1, 2].map(|axis| {
        all.iter()
            .map(|b| b.1[axis])
            .fold(f64::NEG_INFINITY, f64::max)
    });
    let exports = vec![
        Export {
            name: "bench_source_posed_VIEW_ONLY.stl",
            part: source_part(),
            bounds: source_bounds,
            role: "posed_source_and_reference_barrel_not_print_orientation",
        },
        Export {
            name: "bench_dock_posed_VIEW_ONLY.stl",
            part: dock_part(),
            bounds: dock_bounds,
            role: "posed_dock_with_reference_coupon_not_print_orientation",
        },
        Export {
            name: "bench_outlet_posed_VIEW_ONLY.stl",
            part: outlet_part(),
            bounds: outlet_bounds,
            role: "posed_independently_anchored_support_hardware_omitted",
        },
        Export {
            name: "bench_collection_posed_VIEW_ONLY.stl",
            part: collector_part(),
            bounds: collector_bounds,
            role: "generic_scale_vial_and_nest_reference_no_commercial_fit",
        },
        Export {
            name: "bench_assembly_DRY_VIEW_ONLY.stl",
            part: source_part() + dock_part() + outlet_part() + collector_part(),
            bounds: (low, high),
            role: "combined_dry_layout_not_a_manufacturing_or_installed_system",
        },
    ];
    let probes = vec![
        Probe {
            index: 0,
            name: "source_base_at_configured_pose",
            point: translated(p.source.base_probe(), st),
            solid: true,
        },
        Probe {
            index: 1,
            name: "dock_floor_at_configured_pose",
            point: translated([0., 0., p.dock.tray_floor_mm / 2.], dt),
            solid: true,
        },
        Probe {
            index: 2,
            name: "outlet_base_at_configured_pose",
            point: translated([0., base[1] / 3., base[2] / 2.], ot),
            solid: true,
        },
        Probe {
            index: 3,
            name: "reference_scale_at_tube_axis",
            point: translated([0., 0., p.receiver.scale_pan_height_mm / 2.], ct),
            solid: true,
        },
    ];
    let design = json!({"transforms_mm":{"source":st,"dock":dt,"outlet_support":ot,"collection":ct},"nominal_tube_tip_mm":tube,"nominal_vial_top_z_mm":vial_top,"nominal_outlet_air_gap_mm":air_gap,"plan_envelope_axis_gaps_mm":{"source_to_dock":source_dock_gap,"dock_to_collection":dock_collector_gap,"stand_base_to_collection":base_collector_gap},"layout_bounds_mm":[low,high],"liquid_connection_intent":["identified source outlet to coupon inlet via selected tubing/fittings","coupon outlet to free collection outlet"],"connections_modeled":false,"hardware_selected":false,"rules":["VIEW_ONLY parts are posed assemblies and references, not print files.","No tubing centerline, fittings, cables, fasteners or physical fluid transfer are modeled or verified.","Plan-envelope gaps and sampled occupancy do not establish full collision freedom, access, cable routing or fabrication fitness.","Collection center is derived from configured outlet axis. Real pan origin, tube projection, vial and draft shield must be measured.","Stand base is independent of balance, but actual anchoring, rigidity and all load paths remain unverified.","This reference layout does not require buying the unselected generic scale or barrel."]});
    publish(args, "water_bench_layout", raw, &p, exports, probes, design)
}
