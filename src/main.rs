use std::marker::PhantomData;

use anyhow::{Context, Ok, Result};
use env_logger::Env;
use korrekt::{
    circuit_analyzer::{self, analyzer::Analyzer, halo2_proofs_libs::*},
    io,
    sample_circuits::summa as sample_circuits,
};
use log::{info, LevelFilter};
use num::{BigInt, Num};

fn main() -> Result<(), anyhow::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let circuit =
        sample_circuits::lookup_circuits::multiple_lookups_summa::MyCircuit::<Fr>(PhantomData);
    let k = 6;

    let mut analyzer = Analyzer::new(&circuit, k).unwrap();

    let modulus = bn256::fr::MODULUS_STR;
    let without_prefix = modulus.trim_start_matches("0x");
    let prime = BigInt::from_str_radix(without_prefix, 16)
        .unwrap()
        .to_string();

    let analyzer_type = io::analyzer_io::retrieve_user_input_for_analyzer_type()
        .context("Failed to retrieve the user inputs!")?;

    let t = analyzer
        .dispatch_analysis(analyzer_type, &prime)
        .context("Failed to perform analysis!")?;
    Ok(())
}
