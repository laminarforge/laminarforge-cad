// ─── DOE Planner — Design of Experiments CLI for LaminarForge chip campaigns ───
//
// Plans multi-chip campaigns with statistical rigor on the 100-chip rack
// (5 shelves × 4 rows × 5 cols per shelf) described in rack spec A-1A303196.
// Given a set of treatment factors and a design type, emits a chip-to-position
// manifest CSV that the Opentrons Flex seeding scripts (T-32197131) consume.
//
// Supported designs
//   - factorial      2^k full factorial, all combinations × r replicates
//   - fractional     2^(k-p) fractional factorial (Res III, IV, V generators)
//   - split-plot     whole-plot (shelf-level, hard-to-change) × subplot (chip-level)
//   - latin-square   n×n Latin square on a single shelf (row × col blocking)
//   - plackett-burman   12-run PB screening design for up to 11 factors
//   - ccd            Central Composite Design (axial + factorial + center)
//
// Utilities
//   - power          sample-size / power analysis (Cohen's d, t-test, ANOVA, split-plot)
//   - confounds      χ² confound detection between treatment & nuisance factors
//
// References
//   Box, Hunter & Hunter — Statistics for Experimenters (2nd ed., Wiley)
//   Montgomery — Design and Analysis of Experiments (10th ed., Wiley)
//   Plackett & Burman 1946 — Biometrika 33(4)
//   PICS Aide-Memoire PI 011-3 (DOE guidance)

use clap::{Args, Parser, Subcommand};
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_pcg::Pcg64;
use serde::{Deserialize, Serialize};
use statrs::distribution::{ContinuousCDF, FisherSnedecor, Normal, StudentsT};
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

// ══════════════════════════════════════════════════════════════════
// Rack geometry constants (from chip_stack_rack.rs / A-1A303196)
// ══════════════════════════════════════════════════════════════════

const NUM_SHELVES: usize = 5;
const ROWS_PER_SHELF: usize = 4; // per task brief (4 rows × 5 cols = 20/shelf)
const COLS_PER_SHELF: usize = 5;
const CHIPS_PER_SHELF: usize = ROWS_PER_SHELF * COLS_PER_SHELF;
const TOTAL_CHIPS: usize = NUM_SHELVES * CHIPS_PER_SHELF;

// Physical pitches for manifest position_x_mm / position_y_mm.
// Chip (Rev C) footprint is 127.76 × 85.48 mm; we report pocket centers
// using a 128.5 mm × 86.2 mm pitch with a 5 mm gutter, i.e. the values
// implied by chip_stack_rack.rs (pocket_x + gutter_interior, etc).
const POCKET_PITCH_X_MM: f64 = 128.5 + 5.0; // col spacing
const POCKET_PITCH_Y_MM: f64 = 86.2 + 5.0; // row spacing
const POCKET_ORIGIN_X_MM: f64 = 64.0; // col=0 center
const POCKET_ORIGIN_Y_MM: f64 = 42.74; // row=0 center

// ══════════════════════════════════════════════════════════════════
// CLI
// ══════════════════════════════════════════════════════════════════

#[derive(Parser, Debug)]
#[command(
    name = "doe_plan",
    version,
    about = "DOE planner for LaminarForge multi-chip campaigns"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// 2^k full factorial (all combinations × replicates)
    Factorial(FactorialArgs),
    /// 2^(k-p) fractional factorial
    Fractional(FractionalArgs),
    /// Split-plot: whole-plot factor varies shelf-to-shelf, subplot chip-to-chip
    SplitPlot(SplitPlotArgs),
    /// n×n Latin square on one shelf (row × col blocking)
    LatinSquare(LatinArgs),
    /// 12-run Plackett-Burman screening design (up to 11 factors)
    PlackettBurman(PBArgs),
    /// Central composite design for response-surface optimization
    Ccd(CcdArgs),
    /// Sample-size / power calculations (no manifest; prints a report)
    Power(PowerArgs),
    /// χ² confound analysis on an existing manifest CSV
    Confounds(ConfoundsArgs),
}

#[derive(Args, Debug, Clone)]
struct CommonOpts {
    /// RNG seed for reproducible randomization
    #[arg(long, default_value_t = 42)]
    seed: u64,
    /// Output CSV path
    #[arg(long, short = 'o', default_value = "manifest.csv")]
    output: PathBuf,
}

#[derive(Args, Debug)]
struct FactorialArgs {
    /// Factors as name:low:high[,name:low:high...] — exactly 2 levels each
    #[arg(long)]
    factors: String,
    /// Replicates per cell
    #[arg(long, default_value_t = 4)]
    replicates: usize,
    #[command(flatten)]
    common: CommonOpts,
}

#[derive(Args, Debug)]
struct FractionalArgs {
    /// Factors as name:low:high[,...]  — k total
    #[arg(long)]
    factors: String,
    /// Design resolution: 3 (III), 4 (IV), 5 (V)
    #[arg(long, default_value = "IV")]
    resolution: String,
    /// Replicates per run
    #[arg(long, default_value_t = 2)]
    replicates: usize,
    #[command(flatten)]
    common: CommonOpts,
}

#[derive(Args, Debug)]
struct SplitPlotArgs {
    /// Whole-plot factor (shelf-level, hard-to-change): name:level1:level2[:level3...]
    #[arg(long)]
    whole_plot: String,
    /// Subplot factor (chip-level, easy to change): name:level1:level2[:...]
    #[arg(long)]
    subplot: String,
    /// Number of shelves to use (1..=5)
    #[arg(long, default_value_t = 5)]
    n_shelves: usize,
    #[command(flatten)]
    common: CommonOpts,
}

#[derive(Args, Debug)]
struct LatinArgs {
    /// Treatment levels — size n for an n×n square (use 4 or 5 to match a shelf)
    #[arg(long)]
    treatments: String,
    /// Which shelf index (0..5) to place it on
    #[arg(long, default_value_t = 0)]
    shelf: usize,
    #[command(flatten)]
    common: CommonOpts,
}

#[derive(Args, Debug)]
struct PBArgs {
    /// 1..11 factors as name:low:high[,...]; unused columns become "dummy"
    #[arg(long)]
    factors: String,
    /// Replicates per PB run
    #[arg(long, default_value_t = 4)]
    replicates: usize,
    #[command(flatten)]
    common: CommonOpts,
}

#[derive(Args, Debug)]
struct CcdArgs {
    /// k continuous factors: name:low:high[,...]  (usually 2..4)
    #[arg(long)]
    factors: String,
    /// Number of center points
    #[arg(long, default_value_t = 5)]
    center: usize,
    /// Axial (alpha) distance in coded units; default is rotatable alpha = (2^k)^(1/4)
    #[arg(long)]
    alpha: Option<f64>,
    #[command(flatten)]
    common: CommonOpts,
}

#[derive(Args, Debug)]
struct PowerArgs {
    /// Cohen's d effect size (for t-test) OR Cohen's f (for ANOVA)
    #[arg(long)]
    effect_size: f64,
    /// Significance level
    #[arg(long, default_value_t = 0.05)]
    alpha: f64,
    /// Desired power (1 - β)
    #[arg(long, default_value_t = 0.80)]
    power: f64,
    /// Design family: t, factorial, split-plot
    #[arg(long, default_value = "t")]
    design: String,
    /// Number of factors (for factorial / split-plot)
    #[arg(long, default_value_t = 3)]
    k: usize,
    /// Intraclass correlation (split-plot only): correlation within a whole-plot
    #[arg(long, default_value_t = 0.3)]
    icc: f64,
}

#[derive(Args, Debug)]
struct ConfoundsArgs {
    /// Manifest CSV to audit
    #[arg(long)]
    manifest: PathBuf,
    /// χ² p-value below which we flag a confound
    #[arg(long, default_value_t = 0.05)]
    alpha: f64,
}

// ══════════════════════════════════════════════════════════════════
// Manifest row (CSV schema)
// ══════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ManifestRow {
    chip_id: String,
    shelf: usize,
    row: usize,
    col: usize,
    position_x_mm: f64,
    position_y_mm: f64,
    /// Treatment levels, one column per factor (name prefixed with "treatment_")
    #[serde(flatten)]
    treatments: BTreeMap<String, String>,
    block_id: String,
    randomization_seed: u64,
}

// ══════════════════════════════════════════════════════════════════
// Helpers: factor parsing, rack grid, randomized placement
// ══════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
struct Factor {
    name: String,
    levels: Vec<String>,
}

fn parse_factors(s: &str) -> Result<Vec<Factor>, String> {
    let mut out = Vec::new();
    for spec in s.split(',').filter(|s| !s.trim().is_empty()) {
        let parts: Vec<&str> = spec.split(':').collect();
        if parts.len() < 3 {
            return Err(format!(
                "factor spec '{}' must be name:level1:level2[:...]",
                spec
            ));
        }
        out.push(Factor {
            name: parts[0].trim().to_string(),
            levels: parts[1..].iter().map(|s| s.trim().to_string()).collect(),
        });
    }
    Ok(out)
}

fn assert_two_levels(fs: &[Factor]) -> Result<(), String> {
    for f in fs {
        if f.levels.len() != 2 {
            return Err(format!(
                "factor '{}' has {} levels; 2 required (low/high)",
                f.name,
                f.levels.len()
            ));
        }
    }
    Ok(())
}

/// Return the canonical 100 (shelf, row, col) positions of the rack.
fn rack_positions() -> Vec<(usize, usize, usize)> {
    let mut v = Vec::with_capacity(TOTAL_CHIPS);
    for s in 0..NUM_SHELVES {
        for r in 0..ROWS_PER_SHELF {
            for c in 0..COLS_PER_SHELF {
                v.push((s, r, c));
            }
        }
    }
    v
}

fn position_xy(row: usize, col: usize) -> (f64, f64) {
    (
        POCKET_ORIGIN_X_MM + (col as f64) * POCKET_PITCH_X_MM,
        POCKET_ORIGIN_Y_MM + (row as f64) * POCKET_PITCH_Y_MM,
    )
}

fn chip_id(i: usize) -> String {
    format!("C{:03}", i + 1)
}

/// Assign run-vectors to chip positions with random ordering.
/// `runs` is a list of (treatment_map, block_id); each entry is one chip.
/// Returns a vector of ManifestRow in chip-index order.
fn emit_manifest(
    runs: Vec<(BTreeMap<String, String>, String)>,
    seed: u64,
) -> Result<Vec<ManifestRow>, String> {
    if runs.len() > TOTAL_CHIPS {
        return Err(format!(
            "design needs {} chips but rack only has {}",
            runs.len(),
            TOTAL_CHIPS
        ));
    }
    let mut positions = rack_positions();
    positions.truncate(runs.len());
    let mut rng = Pcg64::seed_from_u64(seed);
    let mut perm: Vec<usize> = (0..runs.len()).collect();
    perm.shuffle(&mut rng);

    let mut rows = Vec::with_capacity(runs.len());
    for (i, &p_idx) in perm.iter().enumerate() {
        let (shelf, row, col) = positions[i];
        let (x, y) = position_xy(row, col);
        let (treatments, block_id) = runs[p_idx].clone();
        rows.push(ManifestRow {
            chip_id: chip_id(i),
            shelf,
            row,
            col,
            position_x_mm: x,
            position_y_mm: y,
            treatments,
            block_id,
            randomization_seed: seed,
        });
    }
    Ok(rows)
}

/// Same shape but uses a per-shelf random permutation (split-plot style).
/// `runs_per_shelf` must have length == n_shelves; each element is a
/// Vec<(treatments, block_id)> of length <= CHIPS_PER_SHELF.
fn emit_manifest_split(
    runs_per_shelf: Vec<Vec<(BTreeMap<String, String>, String)>>,
    seed: u64,
) -> Result<Vec<ManifestRow>, String> {
    let mut out = Vec::new();
    let mut chip_idx = 0;
    for (s, runs) in runs_per_shelf.into_iter().enumerate() {
        if runs.len() > CHIPS_PER_SHELF {
            return Err(format!(
                "shelf {} has {} runs > {} positions",
                s,
                runs.len(),
                CHIPS_PER_SHELF
            ));
        }
        let mut positions: Vec<(usize, usize)> = (0..ROWS_PER_SHELF)
            .flat_map(|r| (0..COLS_PER_SHELF).map(move |c| (r, c)))
            .collect();
        positions.truncate(runs.len());
        // seed per-shelf with shelf index for independence
        let mut rng = Pcg64::seed_from_u64(seed.wrapping_add(s as u64 * 1_000_003));
        let mut perm: Vec<usize> = (0..runs.len()).collect();
        perm.shuffle(&mut rng);
        for (i, &p_idx) in perm.iter().enumerate() {
            let (row, col) = positions[i];
            let (x, y) = position_xy(row, col);
            let (treatments, block_id) = runs[p_idx].clone();
            out.push(ManifestRow {
                chip_id: chip_id(chip_idx),
                shelf: s,
                row,
                col,
                position_x_mm: x,
                position_y_mm: y,
                treatments,
                block_id,
                randomization_seed: seed,
            });
            chip_idx += 1;
        }
    }
    Ok(out)
}

// ══════════════════════════════════════════════════════════════════
// CSV writer (flat schema: chip metadata + one column per treatment)
// ══════════════════════════════════════════════════════════════════

fn write_manifest(path: &Path, rows: &[ManifestRow]) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).ok();
        }
    }
    let file = File::create(path)?;
    let mut wtr = csv::Writer::from_writer(file);

    // Collect ordered set of treatment columns across all rows
    let mut cols: Vec<String> = Vec::new();
    for r in rows {
        for k in r.treatments.keys() {
            if !cols.contains(k) {
                cols.push(k.clone());
            }
        }
    }

    // Header
    let mut header = vec![
        "chip_id",
        "shelf",
        "row",
        "col",
        "position_x_mm",
        "position_y_mm",
    ]
    .into_iter()
    .map(String::from)
    .collect::<Vec<_>>();
    for c in &cols {
        header.push(format!("treatment_{}", c));
    }
    header.push("block_id".into());
    header.push("randomization_seed".into());
    wtr.write_record(&header)?;

    for r in rows {
        let mut rec: Vec<String> = vec![
            r.chip_id.clone(),
            r.shelf.to_string(),
            r.row.to_string(),
            r.col.to_string(),
            format!("{:.2}", r.position_x_mm),
            format!("{:.2}", r.position_y_mm),
        ];
        for c in &cols {
            rec.push(r.treatments.get(c).cloned().unwrap_or_default());
        }
        rec.push(r.block_id.clone());
        rec.push(r.randomization_seed.to_string());
        wtr.write_record(&rec)?;
    }
    wtr.flush()?;
    Ok(())
}

// ══════════════════════════════════════════════════════════════════
// Designs
// ══════════════════════════════════════════════════════════════════

/// 2^k full factorial. Returns rows of length 2^k * replicates.
fn design_factorial(
    factors: &[Factor],
    replicates: usize,
) -> Vec<(BTreeMap<String, String>, String)> {
    let k = factors.len();
    let cells = 1usize << k;
    let mut runs = Vec::with_capacity(cells * replicates);
    for rep in 0..replicates {
        for bits in 0..cells {
            let mut m = BTreeMap::new();
            for (i, f) in factors.iter().enumerate() {
                let level_idx = (bits >> i) & 1;
                m.insert(f.name.clone(), f.levels[level_idx].clone());
            }
            runs.push((m, format!("R{}", rep + 1)));
        }
    }
    runs
}

/// 2^(k-p) fractional factorial using textbook generators for resolution III/IV/V.
/// Base design = 2^(k-p) full factorial in the first (k-p) factors; aliased columns
/// are products of base columns per the generator table below.
///
/// Generator tables (Res = minimum length word in defining relation):
///   k=3, res III:  2^(3-1)   I = ABC                      (4 runs)
///   k=4, res IV:   2^(4-1)   I = ABCD                     (8 runs)
///   k=5, res III:  2^(5-2)   I = ABD = ACE = BCDE         (8 runs)
///   k=5, res V:    2^(5-1)   I = ABCDE                    (16 runs)
///   k=6, res IV:   2^(6-2)   I = ABCE = BCDF = ADEF       (16 runs)
///   k=7, res IV:   2^(7-3)   I = ABCE = BCDF = ACDG       (16 runs)
///   k=7, res III:  2^(7-4)   I = ABD = ACE = BCF = ABCG   (8 runs)
fn design_fractional(
    factors: &[Factor],
    resolution: &str,
    replicates: usize,
) -> Result<Vec<(BTreeMap<String, String>, String)>, String> {
    assert_two_levels(factors)?;
    let k = factors.len();
    let res = resolution.to_uppercase();

    // (base_k, generators) — generators specify aliased columns as bit-products of base cols.
    // base_cols = (0..base_k).
    // generator[i] gives the bits of column (base_k + i); bit j set means "multiply in base col j".
    let (base_k, generators): (usize, Vec<u32>) = match (k, res.as_str()) {
        (3, "III") => (2, vec![0b11]),                                // C = AB
        (4, "IV") => (3, vec![0b111]),                                // D = ABC
        (5, "III") => (3, vec![0b011, 0b101]),                        // D=AB, E=AC
        (5, "V") => (4, vec![0b1111]),                                // E = ABCD
        (6, "IV") => (4, vec![0b0111, 0b1110]),                       // E=ABC, F=BCD
        (7, "IV") => (4, vec![0b0111, 0b1110, 0b1011]),               // E=ABC, F=BCD, G=ACD
        (7, "III") => (3, vec![0b011, 0b101, 0b110, 0b111]),          // D=AB,E=AC,F=BC,G=ABC
        _ => {
            return Err(format!(
                "no generator table for k={} resolution={} — supported: (3,III) (4,IV) (5,III) (5,V) (6,IV) (7,III) (7,IV)",
                k, res
            ))
        }
    };
    let n_runs = 1usize << base_k;
    let mut runs = Vec::new();
    for rep in 0..replicates {
        for base_bits in 0..n_runs {
            let mut m = BTreeMap::new();
            // base factors 0..base_k: direct mapping
            for j in 0..base_k {
                let lvl = (base_bits >> j) & 1;
                m.insert(factors[j].name.clone(), factors[j].levels[lvl].clone());
            }
            // derived factors base_k..k: XOR of bits per generator
            for (g_idx, gen) in generators.iter().enumerate() {
                let mut bit = 0u32;
                for j in 0..base_k {
                    if (gen >> j) & 1 == 1 {
                        bit ^= ((base_bits >> j) & 1) as u32;
                    }
                }
                let f_idx = base_k + g_idx;
                if f_idx >= k {
                    break;
                }
                m.insert(
                    factors[f_idx].name.clone(),
                    factors[f_idx].levels[bit as usize].clone(),
                );
            }
            runs.push((m, format!("R{}", rep + 1)));
        }
    }
    Ok(runs)
}

/// Split-plot: whole-plot factor assigned at shelf level (randomized across shelves),
/// subplot factor randomized across the 20 chips on each shelf.
fn design_split_plot(
    whole: &Factor,
    sub: &Factor,
    n_shelves: usize,
    seed: u64,
) -> Result<Vec<Vec<(BTreeMap<String, String>, String)>>, String> {
    if n_shelves == 0 || n_shelves > NUM_SHELVES {
        return Err(format!("n_shelves must be 1..={}", NUM_SHELVES));
    }
    // assign whole-plot levels to shelves as evenly as possible, then shuffle
    let mut wp_assign: Vec<String> = (0..n_shelves)
        .map(|s| whole.levels[s % whole.levels.len()].clone())
        .collect();
    let mut rng = Pcg64::seed_from_u64(seed.wrapping_add(7));
    wp_assign.shuffle(&mut rng);

    let mut out = Vec::with_capacity(n_shelves);
    for (s, wp_lvl) in wp_assign.iter().enumerate() {
        // Subplot: enumerate CHIPS_PER_SHELF runs with sub-plot levels balanced
        let k = sub.levels.len();
        let mut sub_runs: Vec<(BTreeMap<String, String>, String)> =
            Vec::with_capacity(CHIPS_PER_SHELF);
        for i in 0..CHIPS_PER_SHELF {
            let mut m = BTreeMap::new();
            m.insert(whole.name.clone(), wp_lvl.clone());
            m.insert(sub.name.clone(), sub.levels[i % k].clone());
            // block = shelf-level whole-plot id
            sub_runs.push((m, format!("WP{}_{}", s + 1, wp_lvl)));
        }
        out.push(sub_runs);
    }
    Ok(out)
}

/// Standard n×n Latin square (cyclic construction).
fn design_latin_square(treatments: &[String]) -> Vec<Vec<String>> {
    let n = treatments.len();
    (0..n)
        .map(|i| (0..n).map(|j| treatments[(i + j) % n].clone()).collect())
        .collect()
}

/// Plackett-Burman N=12 design (Biometrika 33 1946 p.322). First row is the seed;
/// subsequent rows are cyclic shifts. Last row is all -1. 11 design columns; first k get used.
const PB12_SEED: [i8; 11] = [1, 1, -1, 1, 1, 1, -1, -1, -1, 1, -1];

fn design_plackett_burman(
    factors: &[Factor],
    replicates: usize,
) -> Result<Vec<(BTreeMap<String, String>, String)>, String> {
    assert_two_levels(factors)?;
    if factors.len() > 11 {
        return Err("PB12 supports up to 11 factors".into());
    }
    let n = 12;
    let mut matrix: Vec<[i8; 11]> = Vec::with_capacity(n);
    for i in 0..(n - 1) {
        let mut row = [0i8; 11];
        for j in 0..11 {
            // cyclic shift: row i column j = seed[(j + i) mod 11]
            row[j] = PB12_SEED[(j + i) % 11];
        }
        matrix.push(row);
    }
    matrix.push([-1i8; 11]);

    let mut runs = Vec::new();
    for rep in 0..replicates {
        for row in &matrix {
            let mut m = BTreeMap::new();
            for (i, f) in factors.iter().enumerate() {
                let lvl = if row[i] == 1 { 1 } else { 0 };
                m.insert(f.name.clone(), f.levels[lvl].clone());
            }
            runs.push((m, format!("R{}", rep + 1)));
        }
    }
    Ok(runs)
}

/// Central Composite Design. Assumes the caller wants factorial + 2k axial + n_center
/// center points. Low/high are treated as coded ±1, axial points at ±alpha (outside range).
fn design_ccd(
    factors: &[Factor],
    alpha: Option<f64>,
    center: usize,
) -> Result<Vec<(BTreeMap<String, String>, String)>, String> {
    assert_two_levels(factors)?;
    let k = factors.len();
    if k < 2 {
        return Err("CCD requires k >= 2".into());
    }
    let a = alpha.unwrap_or(((1u32 << k) as f64).powf(0.25)); // rotatable default
    let n_fact = 1usize << k;
    let mut runs: Vec<(BTreeMap<String, String>, String)> = Vec::new();

    // factorial portion: ±1 (mapped to low/high)
    for bits in 0..n_fact {
        let mut m = BTreeMap::new();
        for (i, f) in factors.iter().enumerate() {
            let lvl = (bits >> i) & 1;
            m.insert(f.name.clone(), f.levels[lvl].clone());
        }
        runs.push((m, "fact".into()));
    }
    // axial portion: only one factor at ±alpha, others at 0 (center)
    for (i, f) in factors.iter().enumerate() {
        for sign in [-1.0f64, 1.0f64] {
            let mut m = BTreeMap::new();
            let v = sign * a;
            for (j, g) in factors.iter().enumerate() {
                if i == j {
                    m.insert(g.name.clone(), format!("{:+.3}α", v / a));
                } else {
                    m.insert(g.name.clone(), "0".into());
                }
            }
            let _ = f; // silence
            runs.push((m, "axial".into()));
        }
    }
    // center points: all factors at 0
    for _ in 0..center {
        let mut m = BTreeMap::new();
        for g in factors {
            m.insert(g.name.clone(), "0".into());
        }
        runs.push((m, "center".into()));
    }
    Ok(runs)
}

// ══════════════════════════════════════════════════════════════════
// Power analysis
// ══════════════════════════════════════════════════════════════════

/// Two-sample t-test power via non-central t.
/// effect_d = Cohen's d, n_per_group = n1 = n2.
///
/// Uses the Johnson-Kotz normal approximation to the non-central t CDF
/// (Johnson, Kotz & Balakrishnan, "Continuous Univariate Distributions",
/// Vol. 2, 1995, Eq. 31.26). For df≥30 and |ncp|≤10 this is accurate to
/// within a few parts in 10^4 of the exact non-central t CDF.
fn power_two_sample_t(effect_d: f64, n_per_group: usize, alpha: f64) -> f64 {
    if n_per_group < 2 {
        return 0.0;
    }
    let df = (2 * n_per_group - 2) as f64;
    let ncp = effect_d * ((n_per_group as f64) / 2.0).sqrt();
    let t = StudentsT::new(0.0, 1.0, df).unwrap();
    let t_crit = t.inverse_cdf(1.0 - alpha / 2.0);
    let z = Normal::new(0.0, 1.0).unwrap();

    // Johnson-Kotz approximation for P(T' < x) where T' ~ t(df, ncp):
    //   Let h = x*(1 - 1/(4*df)).
    //   Let s = sqrt(1 + x^2/(2*df)).
    //   Then P(T' < x) ≈ Φ((h - ncp) / s)
    let jk_cdf = |x: f64| -> f64 {
        let h = x * (1.0 - 1.0 / (4.0 * df));
        let s = (1.0 + x.powi(2) / (2.0 * df)).max(1e-12).sqrt();
        z.cdf((h - ncp) / s)
    };
    let power = (1.0 - jk_cdf(t_crit)) + jk_cdf(-t_crit);
    power.clamp(0.0, 1.0)
}

/// Minimum n per group for two-sample t (iterative search).
fn n_for_power_t(effect_d: f64, alpha: f64, target_power: f64) -> usize {
    for n in 2..10_000 {
        if power_two_sample_t(effect_d, n, alpha) >= target_power {
            return n;
        }
    }
    10_000
}

/// Power for a main effect in a 2^k factorial ANOVA (balanced, equal reps).
/// Uses Cohen's f effect size: ncp = N * f^2 where N is total observations,
/// df_effect = 1 (for 2-level factors), df_error = N - 2^k.
fn power_factorial_main(effect_f: f64, k: usize, reps: usize, alpha: f64) -> f64 {
    let cells = 1usize << k;
    let n = cells * reps;
    if n <= cells {
        return 0.0;
    }
    let df_num = 1.0f64;
    let df_den = (n - cells) as f64;
    let ncp = (n as f64) * effect_f.powi(2);
    let f_dist = FisherSnedecor::new(df_num, df_den).unwrap();
    let f_crit = f_dist.inverse_cdf(1.0 - alpha);
    // power = P(F' > f_crit) with non-central F. Use Patnaik's χ² approximation:
    // F' ≈ (chi²_nc(df_num,ncp)/df_num) / (chi²(df_den)/df_den)
    // → P(F_central(df_num*, df_den) > f_crit * df_num / df_num*), where
    //   df_num* = (df_num + ncp)^2 / (df_num + 2*ncp),  scale = (df_num + 2*ncp)/(df_num + ncp)
    let df_num_star = (df_num + ncp).powi(2) / (df_num + 2.0 * ncp).max(1e-9);
    let scale = (df_num + ncp) / df_num;
    let f_dist2 = FisherSnedecor::new(df_num_star, df_den).unwrap();
    1.0 - f_dist2.cdf(f_crit / scale)
}

/// Split-plot variance inflation — effective n is reduced by the design effect
/// DEFF = 1 + (m-1)*ICC where m is cluster size (chips per whole-plot).
fn power_split_plot(effect_d: f64, n_whole_plots: usize, icc: f64, alpha: f64) -> f64 {
    let m = (TOTAL_CHIPS / NUM_SHELVES) as f64; // 20
    let deff = (1.0 + (m - 1.0) * icc).max(1.0);
    let n_eff = ((n_whole_plots as f64) * m / deff).round().max(2.0) as usize;
    // Treat as two-sample t with effective n per group (halved for 2-level whole-plot)
    let n_per_group = (n_eff / 2).max(2);
    power_two_sample_t(effect_d, n_per_group, alpha)
}

// ══════════════════════════════════════════════════════════════════
// Confound detection (χ² of independence)
// ══════════════════════════════════════════════════════════════════

/// Returns (chi2, df, p_value) for a contingency table.
fn chi_square_independence(table: &Vec<Vec<usize>>) -> (f64, usize, f64) {
    let rows = table.len();
    if rows == 0 {
        return (0.0, 0, 1.0);
    }
    let cols = table[0].len();
    let row_tot: Vec<usize> = table.iter().map(|r| r.iter().sum()).collect();
    let mut col_tot = vec![0usize; cols];
    for r in table {
        for (j, v) in r.iter().enumerate() {
            col_tot[j] += v;
        }
    }
    let total: usize = row_tot.iter().sum();
    if total == 0 {
        return (0.0, 0, 1.0);
    }
    let mut chi2 = 0.0f64;
    for i in 0..rows {
        for j in 0..cols {
            let exp = (row_tot[i] as f64) * (col_tot[j] as f64) / (total as f64);
            if exp > 0.0 {
                let d = (table[i][j] as f64) - exp;
                chi2 += d * d / exp;
            }
        }
    }
    let df = rows.saturating_sub(1) * cols.saturating_sub(1);
    if df == 0 {
        return (chi2, 0, 1.0);
    }
    // χ² → p via Gamma(k/2, 2) survival = 1 - γ(k/2, χ²/2)/Γ(k/2)
    use statrs::distribution::ChiSquared;
    let dist = ChiSquared::new(df as f64).unwrap();
    let p = 1.0 - dist.cdf(chi2);
    (chi2, df, p)
}

/// Build a contingency table from two columns of a CSV (values as strings).
fn contingency(a: &[String], b: &[String]) -> (Vec<Vec<usize>>, Vec<String>, Vec<String>) {
    let mut a_idx: HashMap<String, usize> = HashMap::new();
    let mut b_idx: HashMap<String, usize> = HashMap::new();
    let mut a_labels = Vec::new();
    let mut b_labels = Vec::new();
    for v in a {
        if !a_idx.contains_key(v) {
            a_idx.insert(v.clone(), a_labels.len());
            a_labels.push(v.clone());
        }
    }
    for v in b {
        if !b_idx.contains_key(v) {
            b_idx.insert(v.clone(), b_labels.len());
            b_labels.push(v.clone());
        }
    }
    let mut table = vec![vec![0usize; b_labels.len()]; a_labels.len()];
    for (x, y) in a.iter().zip(b.iter()) {
        table[a_idx[x]][b_idx[y]] += 1;
    }
    (table, a_labels, b_labels)
}

// ══════════════════════════════════════════════════════════════════
// Command runners
// ══════════════════════════════════════════════════════════════════

fn run_factorial(a: &FactorialArgs) -> Result<(), Box<dyn Error>> {
    let factors = parse_factors(&a.factors).map_err(as_err)?;
    assert_two_levels(&factors).map_err(as_err)?;
    let runs = design_factorial(&factors, a.replicates);
    let rows = emit_manifest(runs, a.common.seed).map_err(as_err)?;
    write_manifest(&a.common.output, &rows)?;
    println!(
        "wrote {} chips ({} cells × {} reps) to {}",
        rows.len(),
        1 << factors.len(),
        a.replicates,
        a.common.output.display()
    );
    print_balance_report(&rows);
    Ok(())
}

fn run_fractional(a: &FractionalArgs) -> Result<(), Box<dyn Error>> {
    let factors = parse_factors(&a.factors).map_err(as_err)?;
    let runs = design_fractional(&factors, &a.resolution, a.replicates).map_err(as_err)?;
    let rows = emit_manifest(runs, a.common.seed).map_err(as_err)?;
    write_manifest(&a.common.output, &rows)?;
    println!(
        "wrote {} chips (fractional k={} res={} × {} reps) to {}",
        rows.len(),
        factors.len(),
        a.resolution,
        a.replicates,
        a.common.output.display()
    );
    print_balance_report(&rows);
    Ok(())
}

fn run_split_plot(a: &SplitPlotArgs) -> Result<(), Box<dyn Error>> {
    let wholes = parse_factors(&a.whole_plot).map_err(as_err)?;
    let subs = parse_factors(&a.subplot).map_err(as_err)?;
    if wholes.len() != 1 || subs.len() != 1 {
        return Err("split-plot requires exactly one whole-plot and one subplot factor".into());
    }
    let runs =
        design_split_plot(&wholes[0], &subs[0], a.n_shelves, a.common.seed).map_err(as_err)?;
    let rows = emit_manifest_split(runs, a.common.seed).map_err(as_err)?;
    write_manifest(&a.common.output, &rows)?;
    println!(
        "wrote {} chips ({}-shelf split-plot) to {}",
        rows.len(),
        a.n_shelves,
        a.common.output.display()
    );
    print_balance_report(&rows);
    Ok(())
}

fn run_latin(a: &LatinArgs) -> Result<(), Box<dyn Error>> {
    let treatments: Vec<String> = a
        .treatments
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let n = treatments.len();
    if !(n == ROWS_PER_SHELF || n == COLS_PER_SHELF) {
        return Err(format!(
            "Latin square size {} must match shelf row count ({}) or col count ({})",
            n, ROWS_PER_SHELF, COLS_PER_SHELF
        )
        .into());
    }
    if a.shelf >= NUM_SHELVES {
        return Err(format!("shelf index {} out of range 0..{}", a.shelf, NUM_SHELVES).into());
    }
    let sq = design_latin_square(&treatments);
    // produce exactly n*n runs, placing on the chosen shelf
    let mut out = Vec::new();
    for r in 0..n {
        for c in 0..n {
            let mut m = BTreeMap::new();
            m.insert("factor".into(), sq[r][c].clone());
            out.push((m, format!("shelf{}_r{}c{}", a.shelf, r, c)));
        }
    }
    // manifest placement: put n^2 chips at top-left of the chosen shelf, no randomization of
    // treatment/position (Latin square already balanced in both axes). Randomization seed
    // still recorded for downstream reference.
    let mut rows = Vec::new();
    let mut idx = 0;
    for r in 0..n.min(ROWS_PER_SHELF) {
        for c in 0..n.min(COLS_PER_SHELF) {
            let (x, y) = position_xy(r, c);
            let (treatments, block_id) = out[idx].clone();
            rows.push(ManifestRow {
                chip_id: chip_id(idx),
                shelf: a.shelf,
                row: r,
                col: c,
                position_x_mm: x,
                position_y_mm: y,
                treatments,
                block_id,
                randomization_seed: a.common.seed,
            });
            idx += 1;
        }
    }
    write_manifest(&a.common.output, &rows)?;
    println!(
        "wrote {}×{} Latin square ({} chips) to {}",
        n,
        n,
        rows.len(),
        a.common.output.display()
    );
    print_balance_report(&rows);
    Ok(())
}

fn run_pb(a: &PBArgs) -> Result<(), Box<dyn Error>> {
    let factors = parse_factors(&a.factors).map_err(as_err)?;
    let runs = design_plackett_burman(&factors, a.replicates).map_err(as_err)?;
    let rows = emit_manifest(runs, a.common.seed).map_err(as_err)?;
    write_manifest(&a.common.output, &rows)?;
    println!(
        "wrote {} chips (PB12, k={}, reps={}) to {}",
        rows.len(),
        factors.len(),
        a.replicates,
        a.common.output.display()
    );
    print_balance_report(&rows);
    Ok(())
}

fn run_ccd(a: &CcdArgs) -> Result<(), Box<dyn Error>> {
    let factors = parse_factors(&a.factors).map_err(as_err)?;
    let runs = design_ccd(&factors, a.alpha, a.center).map_err(as_err)?;
    let rows = emit_manifest(runs, a.common.seed).map_err(as_err)?;
    write_manifest(&a.common.output, &rows)?;
    println!(
        "wrote {} chips (CCD k={}, center={}, α={:.3}) to {}",
        rows.len(),
        factors.len(),
        a.center,
        a.alpha
            .unwrap_or(((1u32 << factors.len()) as f64).powf(0.25)),
        a.common.output.display()
    );
    Ok(())
}

fn run_power(a: &PowerArgs) -> Result<(), Box<dyn Error>> {
    println!("─── DOE Power Analysis ───");
    println!(
        "design={}  effect_size={}  α={}  target_power={}",
        a.design, a.effect_size, a.alpha, a.power
    );
    match a.design.as_str() {
        "t" => {
            let n = n_for_power_t(a.effect_size, a.alpha, a.power);
            println!(
                "two-sample t-test: need n={} per group (total {})",
                n,
                2 * n
            );
            for r in [2, 4, 6, 8, 10, 12, 16, 20] {
                let p = power_two_sample_t(a.effect_size, r, a.alpha);
                println!("  n_per_group={:3}  power={:.3}", r, p);
            }
        }
        "factorial" => {
            println!(
                "2^{} factorial — main effect (Cohen's f = {}):",
                a.k, a.effect_size
            );
            for reps in [2, 3, 4, 5, 6, 8] {
                let p = power_factorial_main(a.effect_size, a.k, reps, a.alpha);
                let total = (1usize << a.k) * reps;
                println!(
                    "  reps={:2}  total_chips={:3}  power={:.3}{}",
                    reps,
                    total,
                    p,
                    if total <= TOTAL_CHIPS {
                        ""
                    } else {
                        "  [exceeds 100-chip rack]"
                    }
                );
            }
        }
        "split-plot" => {
            println!(
                "split-plot: whole-plot effect, ICC={}  (20 chips/shelf cluster)",
                a.icc
            );
            for wp in [2, 3, 4, 5] {
                let p = power_split_plot(a.effect_size, wp, a.icc, a.alpha);
                println!(
                    "  n_shelves={}  total_chips={}  power={:.3}",
                    wp,
                    wp * (TOTAL_CHIPS / NUM_SHELVES),
                    p
                );
            }
        }
        other => {
            return Err(format!("unknown design '{}' (use t, factorial, split-plot)", other).into())
        }
    }
    Ok(())
}

fn run_confounds(a: &ConfoundsArgs) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(&a.manifest)?;
    let headers = rdr.headers()?.clone();
    let cols: Vec<String> = headers.iter().map(String::from).collect();
    let mut col_map: HashMap<String, Vec<String>> = HashMap::new();
    for c in &cols {
        col_map.insert(c.clone(), Vec::new());
    }
    for rec in rdr.records() {
        let rec = rec?;
        for (i, v) in rec.iter().enumerate() {
            col_map.get_mut(&cols[i]).unwrap().push(v.to_string());
        }
    }

    let treatment_cols: Vec<String> = cols
        .iter()
        .filter(|c| c.starts_with("treatment_"))
        .cloned()
        .collect();
    let nuisance_cols: Vec<String> = cols
        .iter()
        .filter(|c| {
            matches!(c.as_str(), "shelf" | "row" | "col")
                || c.as_str() == "chip_lot"
                || c.as_str() == "seeding_time"
                || c.as_str() == "block_id"
        })
        .cloned()
        .collect();

    println!("─── Confound analysis ({}) ───", a.manifest.display());
    println!(
        "treatments: {}  |  nuisance: {}",
        treatment_cols.join(", "),
        nuisance_cols.join(", ")
    );

    let mut flagged = 0usize;
    for t in &treatment_cols {
        for n_col in &nuisance_cols {
            let a_vec = col_map.get(t).unwrap();
            let b_vec = col_map.get(n_col).unwrap();
            let (table, _, _) = contingency(a_vec, b_vec);
            let (chi2, df, p) = chi_square_independence(&table);
            let mark = if p < a.alpha { "[CONFOUND]" } else { "" };
            if p < a.alpha {
                flagged += 1;
            }
            println!(
                "  {}  ×  {}  χ²={:.2}  df={}  p={:.4}  {}",
                t, n_col, chi2, df, p, mark
            );
        }
    }
    if flagged == 0 {
        println!("\nNo confounds detected at α={}.", a.alpha);
    } else {
        println!(
            "\n{} confound(s) detected at α={} — re-randomize with a different seed.",
            flagged, a.alpha
        );
    }
    Ok(())
}

fn print_balance_report(rows: &[ManifestRow]) {
    let mut shelf_counts = [0usize; NUM_SHELVES];
    for r in rows {
        if r.shelf < NUM_SHELVES {
            shelf_counts[r.shelf] += 1;
        }
    }
    println!("shelf occupancy: {:?}", shelf_counts);
}

fn as_err(s: String) -> Box<dyn Error> {
    s.into()
}

// ══════════════════════════════════════════════════════════════════
// main
// ══════════════════════════════════════════════════════════════════

fn main() {
    let cli = Cli::parse();
    let result: Result<(), Box<dyn Error>> = match &cli.cmd {
        Cmd::Factorial(a) => run_factorial(a),
        Cmd::Fractional(a) => run_fractional(a),
        Cmd::SplitPlot(a) => run_split_plot(a),
        Cmd::LatinSquare(a) => run_latin(a),
        Cmd::PlackettBurman(a) => run_pb(a),
        Cmd::Ccd(a) => run_ccd(a),
        Cmd::Power(a) => run_power(a),
        Cmd::Confounds(a) => run_confounds(a),
    };
    if let Err(e) = result {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

// ══════════════════════════════════════════════════════════════════
// Also emit an optional analyze.py stub for post-campaign validation.
// The user can write the manifest, run the campaign, append a `response`
// column, then run `python analyze.py manifest.csv` to get the ANOVA.
// (Not invoked by default.)
// ══════════════════════════════════════════════════════════════════

#[allow(dead_code)]
const ANALYZE_PY_STUB: &str = r#"#!/usr/bin/env python3
"""analyze.py — post-campaign DOE analysis stub.

Reads a manifest CSV with an appended `response` column and runs:
  - Main-effects + interactions ANOVA (statsmodels)
  - Half-normal plot of effect sizes (matplotlib)
  - Residual diagnostic plots
"""
import sys, pandas as pd, numpy as np
import statsmodels.api as sm
from statsmodels.formula.api import ols

df = pd.read_csv(sys.argv[1])
treatments = [c for c in df.columns if c.startswith("treatment_")]
if "response" not in df.columns:
    sys.exit("add a 'response' column before analysis")
formula = "response ~ " + " + ".join(f"C({t})" for t in treatments)
model = ols(formula, data=df).fit()
print(sm.stats.anova_lm(model, typ=2))
"#;

// ══════════════════════════════════════════════════════════════════
// Tests
// ══════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    fn f2(name: &str, a: &str, b: &str) -> Factor {
        Factor {
            name: name.into(),
            levels: vec![a.into(), b.into()],
        }
    }

    #[test]
    fn parse_factors_ok() {
        let f = parse_factors("dose:low:high,ecm:matrigel:collagen").unwrap();
        assert_eq!(f.len(), 2);
        assert_eq!(f[0].name, "dose");
        assert_eq!(f[1].levels, vec!["matrigel", "collagen"]);
    }

    #[test]
    fn parse_factors_rejects_single_level() {
        let fs = parse_factors("dose:low").err().unwrap();
        assert!(fs.contains("must be"));
    }

    #[test]
    fn factorial_is_balanced() {
        // 2^3 = 8 cells × 4 reps = 32 chips; every level should appear 16 times.
        let factors = vec![
            f2("dose", "low", "high"),
            f2("ecm", "matrigel", "collagen"),
            f2("cell", "caco2", "huvec"),
        ];
        let runs = design_factorial(&factors, 4);
        assert_eq!(runs.len(), 32);
        for f in &factors {
            let mut counts = HashMap::new();
            for (m, _) in &runs {
                *counts.entry(m[&f.name].clone()).or_insert(0usize) += 1;
            }
            for (_, c) in counts {
                assert_eq!(c, 16, "factor {} not balanced", f.name);
            }
        }
    }

    #[test]
    fn fractional_half_of_full() {
        let factors = vec![
            f2("A", "lo", "hi"),
            f2("B", "lo", "hi"),
            f2("C", "lo", "hi"),
            f2("D", "lo", "hi"),
        ];
        let runs = design_fractional(&factors, "IV", 1).unwrap();
        // 2^(4-1) = 8
        assert_eq!(runs.len(), 8);
    }

    #[test]
    fn fractional_res3_k5_generates_8_runs() {
        let factors = (0..5)
            .map(|i| f2(&format!("F{}", i), "lo", "hi"))
            .collect::<Vec<_>>();
        let runs = design_fractional(&factors, "III", 1).unwrap();
        assert_eq!(runs.len(), 8);
        // each factor should be balanced
        for f in &factors {
            let mut hi = 0;
            for (m, _) in &runs {
                if m[&f.name] == "hi" {
                    hi += 1;
                }
            }
            assert_eq!(hi, 4, "factor {} imbalanced in Res III design", f.name);
        }
    }

    #[test]
    fn plackett_burman_12_runs() {
        let factors = (0..7)
            .map(|i| f2(&format!("F{}", i), "lo", "hi"))
            .collect::<Vec<_>>();
        let runs = design_plackett_burman(&factors, 1).unwrap();
        assert_eq!(runs.len(), 12);
        // every factor should have exactly 6 high / 6 low
        for f in &factors {
            let hi = runs.iter().filter(|(m, _)| m[&f.name] == "hi").count();
            assert_eq!(hi, 6, "factor {} imbalanced in PB12", f.name);
        }
    }

    #[test]
    fn seed_reproducibility() {
        let factors = vec![f2("dose", "low", "high"), f2("ecm", "matrigel", "collagen")];
        let runs1 = design_factorial(&factors, 4);
        let runs2 = design_factorial(&factors, 4);
        let r1 = emit_manifest(runs1, 1234).unwrap();
        let r2 = emit_manifest(runs2, 1234).unwrap();
        assert_eq!(r1.len(), r2.len());
        for (a, b) in r1.iter().zip(r2.iter()) {
            assert_eq!(a.chip_id, b.chip_id);
            assert_eq!(a.treatments, b.treatments);
            assert_eq!(a.shelf, b.shelf);
        }
        // Different seed → different order
        let r3 = emit_manifest(design_factorial(&factors, 4), 9999).unwrap();
        let same_pattern = r1
            .iter()
            .zip(r3.iter())
            .all(|(a, b)| a.treatments == b.treatments);
        assert!(
            !same_pattern,
            "different seeds should produce different layouts"
        );
    }

    #[test]
    fn power_monotone_in_sample_size() {
        // With Cohen's d=0.5, n=64 lands at ~0.80 power (classic textbook value).
        // Check monotonicity and that large n reaches >0.95.
        let p1 = power_two_sample_t(0.5, 4, 0.05);
        let p2 = power_two_sample_t(0.5, 16, 0.05);
        let p3 = power_two_sample_t(0.5, 64, 0.05);
        let p4 = power_two_sample_t(0.5, 128, 0.05);
        assert!(
            p1 < p2 && p2 < p3 && p3 < p4,
            "power should be monotone increasing in n"
        );
        assert!(
            p3 > 0.70 && p3 < 0.90,
            "n=64 d=0.5 should give ≈0.80 power, got {}",
            p3
        );
        assert!(p4 > 0.95, "n=128 should give >0.95 power, got {}", p4);
    }

    #[test]
    fn power_factorial_monotone_in_reps() {
        let a = power_factorial_main(0.25, 3, 2, 0.05);
        let b = power_factorial_main(0.25, 3, 8, 0.05);
        assert!(b > a, "more reps should give more power ({} vs {})", a, b);
    }

    #[test]
    fn n_for_power_known_value() {
        // Cohen's rule of thumb: n ≈ 16/d^2. For d=0.5 that's 64 per group.
        // Our non-central t answer should land near that value (within 10%).
        let n = n_for_power_t(0.5, 0.05, 0.80);
        assert!(n >= 55 && n <= 75, "n was {}", n);
    }

    #[test]
    fn chi_square_catches_perfect_confound() {
        // Perfect confound: treatment == shelf
        let treat = vec!["A", "A", "B", "B", "C", "C"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        let shelf = vec!["1", "1", "2", "2", "3", "3"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        let (tbl, _, _) = contingency(&treat, &shelf);
        let (chi2, _df, p) = chi_square_independence(&tbl);
        assert!(chi2 > 0.0);
        assert!(p < 0.05, "perfect confound should give p<0.05, got {}", p);
    }

    #[test]
    fn chi_square_independence_uniform() {
        // Independent uniform table — p should be close to 1
        let tbl = vec![vec![10, 10], vec![10, 10]];
        let (_chi2, _df, p) = chi_square_independence(&tbl);
        assert!(p > 0.5, "uniform table should not flag, got p={}", p);
    }

    #[test]
    fn split_plot_basic() {
        let wp = f2("media", "dmem", "rpmi");
        let sp = Factor {
            name: "dose".into(),
            levels: vec!["0".into(), "10".into(), "100".into()],
        };
        let plots = design_split_plot(&wp, &sp, 4, 42).unwrap();
        assert_eq!(plots.len(), 4);
        for shelf in &plots {
            assert_eq!(shelf.len(), CHIPS_PER_SHELF);
        }
        let rows = emit_manifest_split(plots, 42).unwrap();
        assert_eq!(rows.len(), 4 * CHIPS_PER_SHELF);
    }

    #[test]
    fn latin_square_is_latin() {
        let t: Vec<String> = vec!["A".into(), "B".into(), "C".into(), "D".into()];
        let sq = design_latin_square(&t);
        // Every row and column contains each treatment exactly once.
        for r in 0..4 {
            let mut s: Vec<String> = sq[r].clone();
            s.sort();
            assert_eq!(s, vec!["A", "B", "C", "D"]);
        }
        for c in 0..4 {
            let mut s: Vec<String> = (0..4).map(|r| sq[r][c].clone()).collect();
            s.sort();
            assert_eq!(s, vec!["A", "B", "C", "D"]);
        }
    }

    #[test]
    fn ccd_has_factorial_axial_center_parts() {
        let fs = vec![f2("x", "-1", "1"), f2("y", "-1", "1")];
        let runs = design_ccd(&fs, Some(1.414), 5).unwrap();
        // 2^2 factorial + 2*2 axial + 5 center = 13
        assert_eq!(runs.len(), 13);
        let n_center = runs.iter().filter(|(_, b)| b == "center").count();
        let n_fact = runs.iter().filter(|(_, b)| b == "fact").count();
        let n_axial = runs.iter().filter(|(_, b)| b == "axial").count();
        assert_eq!(n_center, 5);
        assert_eq!(n_fact, 4);
        assert_eq!(n_axial, 4);
    }

    #[test]
    fn rack_has_100_chips() {
        assert_eq!(rack_positions().len(), 100);
        assert_eq!(TOTAL_CHIPS, 100);
    }
}

// Silence unused-warning for the Write/PB imports when some features are off
#[allow(dead_code)]
fn _touch(_: &mut dyn Write) {}
