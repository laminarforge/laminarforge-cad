// ─── LaminarForge COGS Model — 100 / 500 / 2000 Chip Scale Analysis ────────
//
// Ticket: T-8D3EE16B (laminarforge / microfluidic-chip / chip-stack-thermal-container)
//
// Projects per-chip and per-campaign cost-of-goods at three scale tiers:
//   Tier 1: 100 chips/month  — current pilot, 1× 100-chip container (A-B722A1A2)
//   Tier 2: 500 chips/month  — 5× Tier-1 systems
//   Tier 3: 2000 chips/month — walk-in chamber (A-A9E1FE1F, $108k capex)
//
// Run via:
//     laminarforge_build action='run' bin='cogs_model'
//
// Subcommands:
//     project    [--tier N]           Print breakdown for one tier (default all)
//     compare                          Side-by-side comparison of all 3 tiers
//     sensitivity [opts]               Override inputs and recompute
//     csv                              Dump the full matrix as CSV (stdout)
//
// Assumption sources:
//     • Tier-1 capex: A-B722A1A2  ($18k core / $21k all-in / 3-yr amort)
//     • Tier-3 capex: A-A9E1FE1F  ($108k walk-in / 5-yr amort)
//     • Rev C chip cost $50 from microfluidic_chip_cnc_revc; IM target $6.85/chip @ 10k/yr
//     • Labor @ $75/hr fully loaded (salary + bennies + overhead)
//     • Houston biotech-zoned real estate $35/sq-ft/yr (rent + utilities)
//     • Overhead uplift 30 % on direct costs
//     • Campaign = 21 days, one chip = one experimental unit

use std::env;
use std::process;

// ─── Inputs ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
struct Assumptions {
    // Chip unit cost (Rev C CNC baseline $50; IM target $6.85)
    chip_unit_cost: f64,
    // Fully-loaded cell-culture tech rate (USD/hr)
    labor_rate: f64,
    // Media in USD/mL (baseline $0.50)
    media_cost_per_ml: f64,
    // Overhead uplift on direct costs (0.30 = 30 %)
    overhead_pct: f64,
    // Gross-margin target for RUO pricing (0.30 = 30 %)
    target_gross_margin: f64,
    // Real-estate blended rate USD/sq-ft/yr (rent + utilities)
    facilities_rate: f64,
    // Campaign length in days (used only for prose; cost is per-campaign)
    campaign_days: u32,
    // Media volume per change per chip (mL)
    media_ml_per_change: f64,
    // Number of media changes per 21-day campaign
    media_changes_per_campaign: u32,
    // ECM coating cost per chip (Matrigel 200µg × $5/µg)
    ecm_cost_per_chip: f64,
    // Pipette tip events per media change × tip cost
    tip_events_per_change: u32,
    tip_cost: f64,
    // Sterile consumables (waste bottles, filters, etc.) per chip-campaign
    sterile_consumables_per_chip: f64,
    // Lot-level campaign reagents (amortized across chips in a campaign)
    antibody_panel_per_campaign: f64,
    rna_kit_per_chip: f64,
    qpcr_per_chip: f64,           // 20-gene panel, endpoint only
    media_bulk_per_campaign: f64, // media purchased in bulk per campaign
}

impl Assumptions {
    fn baseline() -> Self {
        Self {
            chip_unit_cost: 50.0,
            labor_rate: 75.0,
            media_cost_per_ml: 0.50,
            overhead_pct: 0.30,
            target_gross_margin: 0.30,
            facilities_rate: 35.0,
            campaign_days: 21,
            media_ml_per_change: 5.0,
            media_changes_per_campaign: 10,
            ecm_cost_per_chip: 1.0,
            tip_events_per_change: 9,
            tip_cost: 0.10,
            sterile_consumables_per_chip: 5.0,
            antibody_panel_per_campaign: 500.0,
            rna_kit_per_chip: 14.0,
            qpcr_per_chip: 200.0,
            media_bulk_per_campaign: 7.50,
        }
    }
}

#[derive(Clone, Debug)]
struct Tier {
    id: u8,
    name: &'static str,
    chips_per_month: u32,
    capex_usd: f64,
    amort_years: f64,
    labor_hours_per_campaign: f64,
    footprint_sqft: f64,
    // Assays run per chip across campaign (20-gene panel once, IHC once, imaging daily);
    // used only for per-assay break-down. Default 20 = 20 different endpoint assays.
    assays_per_chip: u32,
}

fn tiers() -> [Tier; 3] {
    [
        Tier {
            id: 1,
            name: "Tier 1 — pilot (100/mo)",
            chips_per_month: 100,
            capex_usd: 21_000.0, // all-in turnkey A-B722A1A2
            amort_years: 3.0,    // aggressive 3-yr for custom systems
            labor_hours_per_campaign: 40.0,
            footprint_sqft: 200.0,
            assays_per_chip: 20,
        },
        Tier {
            id: 2,
            name: "Tier 2 — Wave 2 (500/mo)",
            chips_per_month: 500,
            capex_usd: 5.0 * 18_000.0, // 5× Tier-1 core systems = $90k
            amort_years: 3.0,
            labor_hours_per_campaign: 80.0,
            footprint_sqft: 500.0,
            assays_per_chip: 20,
        },
        Tier {
            id: 3,
            name: "Tier 3 — walk-in (2000/mo)",
            chips_per_month: 2_000,
            capex_usd: 108_000.0, // walk-in A-A9E1FE1F
            amort_years: 5.0,     // 5-yr for walk-in chamber
            labor_hours_per_campaign: 200.0,
            footprint_sqft: 1_200.0,
            assays_per_chip: 20,
        },
    ]
}

// ─── Cost model ───────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct Breakdown {
    tier_name: String,
    tier_id: u8,
    chips_per_month: u32,
    assays_per_chip: u32,

    // Per-chip-campaign direct costs
    chip_unit: f64,
    media_per_chip: f64,
    ecm_per_chip: f64,
    tips_per_chip: f64,
    sterile_per_chip: f64,
    rna_per_chip: f64,
    qpcr_per_chip: f64,

    // Per-campaign (batch) costs allocated per-chip
    antibody_allocated_per_chip: f64,
    media_bulk_allocated_per_chip: f64,
    labor_allocated_per_chip: f64,

    // Monthly fixed costs allocated per-chip
    capex_monthly: f64,
    capex_per_chip: f64,
    facilities_monthly: f64,
    facilities_per_chip: f64,

    // Subtotals
    direct_per_chip: f64,
    overhead_per_chip: f64,
    cogs_per_chip: f64,
    cogs_per_campaign_total: f64,

    // Derived
    cogs_per_assay: f64,
    break_even_price: f64,
}

fn compute(tier: &Tier, a: &Assumptions) -> Breakdown {
    let chips = tier.chips_per_month as f64;

    // ── Per-chip direct consumables ───────────────────────────────────────
    let media_per_chip =
        a.media_ml_per_change * a.media_changes_per_campaign as f64 * a.media_cost_per_ml; // 5 mL × 10 × $0.50 = $25
    let tips_per_chip =
        a.tip_events_per_change as f64 * a.media_changes_per_campaign as f64 * a.tip_cost; // 9 × 10 × $0.10 = $9

    // ── Per-campaign reagents allocated across chips ──────────────────────
    // Assume 1 campaign per month at each tier (the 100/500/2000 chips are
    // all run as a single parallel cohort per month). Antibody panel and
    // media-bulk are batch costs → amortize across the monthly cohort.
    let antibody_allocated_per_chip = a.antibody_panel_per_campaign / chips;
    let media_bulk_allocated_per_chip = a.media_bulk_per_campaign / chips;

    // ── Labor allocated per chip ──────────────────────────────────────────
    let labor_total = tier.labor_hours_per_campaign * a.labor_rate;
    let labor_allocated_per_chip = labor_total / chips;

    // ── Capex amortized per month, per chip ───────────────────────────────
    let capex_monthly = tier.capex_usd / (tier.amort_years * 12.0);
    let capex_per_chip = capex_monthly / chips;

    // ── Facilities per month, per chip ────────────────────────────────────
    let facilities_monthly = tier.footprint_sqft * a.facilities_rate / 12.0;
    let facilities_per_chip = facilities_monthly / chips;

    // ── Direct per-chip total (before overhead) ───────────────────────────
    let direct_per_chip = a.chip_unit_cost
        + media_per_chip
        + a.ecm_cost_per_chip
        + tips_per_chip
        + a.sterile_consumables_per_chip
        + a.rna_kit_per_chip
        + a.qpcr_per_chip
        + antibody_allocated_per_chip
        + media_bulk_allocated_per_chip
        + labor_allocated_per_chip
        + capex_per_chip
        + facilities_per_chip;

    let overhead_per_chip = direct_per_chip * a.overhead_pct;
    let cogs_per_chip = direct_per_chip + overhead_per_chip;
    let cogs_per_campaign_total = cogs_per_chip * chips;

    let cogs_per_assay = cogs_per_chip / tier.assays_per_chip as f64;
    // Break-even ASP for target gross margin: price = cogs / (1 - margin)
    let break_even_price = cogs_per_chip / (1.0 - a.target_gross_margin);

    Breakdown {
        tier_name: tier.name.to_string(),
        tier_id: tier.id,
        chips_per_month: tier.chips_per_month,
        assays_per_chip: tier.assays_per_chip,

        chip_unit: a.chip_unit_cost,
        media_per_chip,
        ecm_per_chip: a.ecm_cost_per_chip,
        tips_per_chip,
        sterile_per_chip: a.sterile_consumables_per_chip,
        rna_per_chip: a.rna_kit_per_chip,
        qpcr_per_chip: a.qpcr_per_chip,

        antibody_allocated_per_chip,
        media_bulk_allocated_per_chip,
        labor_allocated_per_chip,

        capex_monthly,
        capex_per_chip,
        facilities_monthly,
        facilities_per_chip,

        direct_per_chip,
        overhead_per_chip,
        cogs_per_chip,
        cogs_per_campaign_total,

        cogs_per_assay,
        break_even_price,
    }
}

// ─── Output formatters ────────────────────────────────────────────────────

fn usd(v: f64) -> String {
    if v.abs() >= 1_000.0 {
        format!("${:>10.2}", v)
    } else {
        format!("${:>10.2}", v)
    }
}

fn print_breakdown(b: &Breakdown) {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║ {:<65} ║", b.tier_name);
    println!("╠═══════════════════════════════════════════════════════════════════╣");
    println!(
        "║ Throughput: {:>5} chips/month | {:>2} assays/chip                       ║",
        b.chips_per_month, b.assays_per_chip
    );
    println!("╠═══════════════════════════════════════════════════════════════════╣");
    println!("║ COST CATEGORY                                    $/CHIP-CAMPAIGN   ║");
    println!("╠═══════════════════════════════════════════════════════════════════╣");
    println!(
        "║ Chip unit (Rev C CNC)                           {}       ║",
        usd(b.chip_unit)
    );
    println!(
        "║ Media consumables                               {}       ║",
        usd(b.media_per_chip)
    );
    println!(
        "║ ECM coating (Matrigel)                          {}       ║",
        usd(b.ecm_per_chip)
    );
    println!(
        "║ Pipette tips                                    {}       ║",
        usd(b.tips_per_chip)
    );
    println!(
        "║ Sterile waste/consumables                       {}       ║",
        usd(b.sterile_per_chip)
    );
    println!(
        "║ RNA extraction kit                              {}       ║",
        usd(b.rna_per_chip)
    );
    println!(
        "║ qPCR 20-gene panel                              {}       ║",
        usd(b.qpcr_per_chip)
    );
    println!(
        "║ Antibody panel IHC (lot/campaign)               {}       ║",
        usd(b.antibody_allocated_per_chip)
    );
    println!(
        "║ Media bulk (campaign allocation)                {}       ║",
        usd(b.media_bulk_allocated_per_chip)
    );
    println!(
        "║ Labor (fully loaded @ $75/hr)                   {}       ║",
        usd(b.labor_allocated_per_chip)
    );
    println!(
        "║ Capex amortization                              {}       ║",
        usd(b.capex_per_chip)
    );
    println!(
        "║ Facilities (rent + utilities)                   {}       ║",
        usd(b.facilities_per_chip)
    );
    println!("╠═══════════════════════════════════════════════════════════════════╣");
    println!(
        "║ Direct subtotal                                 {}       ║",
        usd(b.direct_per_chip)
    );
    println!(
        "║ Overhead @ 30 %                                 {}       ║",
        usd(b.overhead_per_chip)
    );
    println!(
        "║ COGS / chip-campaign                            {}       ║",
        usd(b.cogs_per_chip)
    );
    println!("╠═══════════════════════════════════════════════════════════════════╣");
    println!(
        "║ COGS / assay ({:>2} assays/chip)                  {}       ║",
        b.assays_per_chip,
        usd(b.cogs_per_assay)
    );
    println!(
        "║ Break-even ASP @ 30 % GM target                 {}       ║",
        usd(b.break_even_price)
    );
    println!(
        "║ Monthly COGS (full cohort)                      {}       ║",
        usd(b.cogs_per_campaign_total)
    );
    println!("╚═══════════════════════════════════════════════════════════════════╝");
    println!();
}

fn print_compare(bs: &[Breakdown]) {
    println!(
        "╔══════════════════════════════════════════════════════════════════════════════════╗"
    );
    println!(
        "║ LaminarForge COGS — Tier Comparison                                              ║"
    );
    println!(
        "╠══════════════════════════════════════════════════════════════════════════════════╣"
    );
    println!(
        "║ {:<38}  {:>12}  {:>12}  {:>12} ║",
        "Cost element ($/chip-campaign)", "Tier 1", "Tier 2", "Tier 3"
    );
    println!(
        "╠══════════════════════════════════════════════════════════════════════════════════╣"
    );
    let rows: [(&str, fn(&Breakdown) -> f64); 15] = [
        ("Chip unit", |b| b.chip_unit),
        ("Media", |b| b.media_per_chip),
        ("ECM coating", |b| b.ecm_per_chip),
        ("Tips", |b| b.tips_per_chip),
        ("Sterile consumables", |b| b.sterile_per_chip),
        ("RNA kit", |b| b.rna_per_chip),
        ("qPCR", |b| b.qpcr_per_chip),
        ("Antibody (alloc)", |b| b.antibody_allocated_per_chip),
        ("Media bulk (alloc)", |b| b.media_bulk_allocated_per_chip),
        ("Labor", |b| b.labor_allocated_per_chip),
        ("Capex amortization", |b| b.capex_per_chip),
        ("Facilities", |b| b.facilities_per_chip),
        ("Direct subtotal", |b| b.direct_per_chip),
        ("Overhead (30 %)", |b| b.overhead_per_chip),
        ("COGS / chip", |b| b.cogs_per_chip),
    ];
    for (label, f) in rows.iter() {
        println!(
            "║ {:<38} {:>13} {:>13} {:>13} ║",
            label,
            usd(f(&bs[0])),
            usd(f(&bs[1])),
            usd(f(&bs[2]))
        );
    }
    println!(
        "╠══════════════════════════════════════════════════════════════════════════════════╣"
    );
    println!(
        "║ {:<38} {:>13} {:>13} {:>13} ║",
        "COGS / assay",
        usd(bs[0].cogs_per_assay),
        usd(bs[1].cogs_per_assay),
        usd(bs[2].cogs_per_assay)
    );
    println!(
        "║ {:<38} {:>13} {:>13} {:>13} ║",
        "Break-even ASP @ 30 % GM",
        usd(bs[0].break_even_price),
        usd(bs[1].break_even_price),
        usd(bs[2].break_even_price)
    );
    println!(
        "║ {:<38} {:>13} {:>13} {:>13} ║",
        "Monthly COGS (cohort)",
        usd(bs[0].cogs_per_campaign_total),
        usd(bs[1].cogs_per_campaign_total),
        usd(bs[2].cogs_per_campaign_total)
    );
    println!(
        "╚══════════════════════════════════════════════════════════════════════════════════╝"
    );
    println!();

    // ASCII scale curve (log-log): $/chip vs tier chips/month
    let xs: Vec<f64> = bs
        .iter()
        .map(|b| (b.chips_per_month as f64).log10())
        .collect();
    let ys: Vec<f64> = bs.iter().map(|b| b.cogs_per_chip.log10()).collect();
    println!("── Scale curve ($/chip vs chips/month, log-log) ──");
    let ymin = ys.iter().cloned().fold(f64::INFINITY, f64::min);
    let ymax = ys.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let rows = 10usize;
    let cols = 40usize;
    let xmin = xs.iter().cloned().fold(f64::INFINITY, f64::min);
    let xmax = xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let mut grid = vec![vec![' '; cols]; rows];
    for (xi, yi) in xs.iter().zip(ys.iter()) {
        let cx = (((xi - xmin) / (xmax - xmin + 1e-9)) * (cols as f64 - 1.0)).round() as usize;
        let cy = (((ymax - yi) / (ymax - ymin + 1e-9)) * (rows as f64 - 1.0)).round() as usize;
        let cx = cx.min(cols - 1);
        let cy = cy.min(rows - 1);
        grid[cy][cx] = '*';
    }
    // connect points with dots
    for w in 0..bs.len() - 1 {
        let (x0, y0) = (xs[w], ys[w]);
        let (x1, y1) = (xs[w + 1], ys[w + 1]);
        let steps = 30;
        for s in 0..=steps {
            let t = s as f64 / steps as f64;
            let xi = x0 + (x1 - x0) * t;
            let yi = y0 + (y1 - y0) * t;
            let cx = (((xi - xmin) / (xmax - xmin + 1e-9)) * (cols as f64 - 1.0)).round() as usize;
            let cy = (((ymax - yi) / (ymax - ymin + 1e-9)) * (rows as f64 - 1.0)).round() as usize;
            let cx = cx.min(cols - 1);
            let cy = cy.min(rows - 1);
            if grid[cy][cx] == ' ' {
                grid[cy][cx] = '.';
            }
        }
    }
    println!("  $/chip");
    for (r, row) in grid.iter().enumerate() {
        let v = ymax - (ymax - ymin) * (r as f64 / (rows as f64 - 1.0));
        let usd_axis = 10f64.powf(v);
        let line: String = row.iter().collect();
        println!("  ${:>7.0} │ {}", usd_axis, line);
    }
    let axis: String = "─".repeat(cols);
    println!("          └{}", axis);
    println!(
        "           {:<10}{:>18}{:>12}",
        "100/mo", "500/mo", "2000/mo"
    );
    println!();
}

fn print_csv(bs: &[Breakdown]) {
    println!("tier_id,tier_name,chips_per_month,assays_per_chip,chip_unit,media,ecm,tips,sterile,rna,qpcr,antibody_alloc,media_bulk_alloc,labor_alloc,capex_per_chip,facilities_per_chip,direct_per_chip,overhead_per_chip,cogs_per_chip,cogs_per_assay,break_even_asp_30pct_gm,monthly_cogs_cohort");
    for b in bs {
        println!(
            "{},{:?},{},{},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4},{:.4}",
            b.tier_id,
            b.tier_name,
            b.chips_per_month,
            b.assays_per_chip,
            b.chip_unit,
            b.media_per_chip,
            b.ecm_per_chip,
            b.tips_per_chip,
            b.sterile_per_chip,
            b.rna_per_chip,
            b.qpcr_per_chip,
            b.antibody_allocated_per_chip,
            b.media_bulk_allocated_per_chip,
            b.labor_allocated_per_chip,
            b.capex_per_chip,
            b.facilities_per_chip,
            b.direct_per_chip,
            b.overhead_per_chip,
            b.cogs_per_chip,
            b.cogs_per_assay,
            b.break_even_price,
            b.cogs_per_campaign_total,
        );
    }
}

// ─── CLI plumbing (hand-rolled, no clap dep) ──────────────────────────────

fn parse_f64_flag(args: &[String], key: &str) -> Option<f64> {
    let mut iter = args.iter();
    while let Some(a) = iter.next() {
        if a == key {
            if let Some(v) = iter.next() {
                if let Ok(f) = v.parse::<f64>() {
                    return Some(f);
                }
            }
        } else if let Some(rest) = a.strip_prefix(&format!("{}=", key)) {
            if let Ok(f) = rest.parse::<f64>() {
                return Some(f);
            }
        }
    }
    None
}

fn parse_u8_flag(args: &[String], key: &str) -> Option<u8> {
    parse_f64_flag(args, key).map(|v| v as u8)
}

fn has_flag(args: &[String], key: &str) -> bool {
    args.iter().any(|a| a == key)
}

fn help() {
    eprintln!("LaminarForge COGS model — T-8D3EE16B\n");
    eprintln!("USAGE:");
    eprintln!("    cogs_model project     [--tier N]");
    eprintln!("    cogs_model compare");
    eprintln!(
        "    cogs_model sensitivity [--chip-cost X] [--labor-rate X] [--media X] [--overhead X]"
    );
    eprintln!("    cogs_model csv");
    eprintln!();
    eprintln!("EXAMPLES:");
    eprintln!("    cogs_model compare");
    eprintln!("    cogs_model project --tier 3");
    eprintln!("    cogs_model sensitivity --chip-cost 5 --labor-rate 50");
    eprintln!("    cogs_model csv > tiers.csv");
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // When invoked by `cargo run --bin cogs_model` with no args,
    // default to `compare` so running the binary does something useful.
    let cmd = args
        .first()
        .cloned()
        .unwrap_or_else(|| "compare".to_string());

    let baseline = Assumptions::baseline();
    let ts = tiers();

    match cmd.as_str() {
        "project" => {
            let tier_flag = parse_u8_flag(&args, "--tier");
            match tier_flag {
                Some(id) => {
                    if let Some(t) = ts.iter().find(|t| t.id == id) {
                        let b = compute(t, &baseline);
                        print_breakdown(&b);
                    } else {
                        eprintln!("Unknown tier: {}", id);
                        process::exit(1);
                    }
                }
                None => {
                    for t in ts.iter() {
                        let b = compute(t, &baseline);
                        print_breakdown(&b);
                    }
                }
            }
        }
        "compare" => {
            let bs: Vec<Breakdown> = ts.iter().map(|t| compute(t, &baseline)).collect();
            print_compare(&bs);
        }
        "sensitivity" => {
            let mut a = baseline.clone();
            if let Some(v) = parse_f64_flag(&args, "--chip-cost") {
                a.chip_unit_cost = v;
            }
            if let Some(v) = parse_f64_flag(&args, "--labor-rate") {
                a.labor_rate = v;
            }
            if let Some(v) = parse_f64_flag(&args, "--media") {
                a.media_cost_per_ml = v;
            }
            if let Some(v) = parse_f64_flag(&args, "--overhead") {
                a.overhead_pct = v;
            }
            println!(
                "Sensitivity: chip=${:.2}  labor=${:.0}/hr  media=${:.2}/mL  overhead={:.0}%\n",
                a.chip_unit_cost,
                a.labor_rate,
                a.media_cost_per_ml,
                a.overhead_pct * 100.0
            );
            let bs: Vec<Breakdown> = ts.iter().map(|t| compute(t, &a)).collect();
            print_compare(&bs);
        }
        "csv" => {
            let bs: Vec<Breakdown> = ts.iter().map(|t| compute(t, &baseline)).collect();
            print_csv(&bs);
        }
        "help" | "--help" | "-h" => help(),
        other => {
            // Unknown subcommand but no args — treat as default compare.
            if other.is_empty() || has_flag(&args, "--help") {
                help();
            } else {
                eprintln!("Unknown subcommand: {}", other);
                help();
                process::exit(1);
            }
        }
    }
}
