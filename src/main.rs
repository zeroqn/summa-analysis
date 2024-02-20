#![feature(generic_const_exprs)]

use std::marker::PhantomData;

use anyhow::{Context, Ok, Result};
use env_logger::Env;
use korrekt::{
    circuit_analyzer::{self, analyzer::Analyzer, halo2_proofs_libs::*},
    io,
};
use log::{info, LevelFilter};
use num::{BigInt, Num};

fn main() -> Result<(), anyhow::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let circuit =
        summa_solvency::circuits::merkle_sum_tree::MstInclusionCircuit::<4, 2, 4>::init_empty();
    let k = 20;

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

    for log in analyzer.log {
        println!("{}", log);
    }

    Ok(())
}
