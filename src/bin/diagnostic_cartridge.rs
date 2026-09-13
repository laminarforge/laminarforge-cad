use clap::Parser;
use laminarforge_cad::{
    diagnostic_cartridge_model::{Parameters, OUTPUTS},
    runtime_cad::{run, Args},
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    run::<Parameters>(&args, "diagnostic_cartridge", &OUTPUTS, |p, i| p.build(i))
}
