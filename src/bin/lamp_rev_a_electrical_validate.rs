use laminarforge_cad::lamp_rev_a_electrical::validate_default;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let root = std::env::current_dir()?;
    let summary = validate_default(&root)?;
    println!("LAMP Rev A electrical validation passed.");
    println!("  passing machine gates: {}", summary.pass_count);
    println!(
        "  manual analysis / first-article gates: {}",
        summary.manual_count
    );
    Ok(())
}
