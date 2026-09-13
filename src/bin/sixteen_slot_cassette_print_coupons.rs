use clap::Parser;
use laminarforge_cad::{
    cassette_print_coupons::{Parameters, OUTPUTS},
    runtime_cad::{run, Args},
};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    run::<Parameters>(
        &args,
        "sixteen_slot_cassette_print_coupons",
        &OUTPUTS,
        |p, i| p.build(i),
    )
}
