use anyhow::{anyhow, Result};
use clap::Parser;
use env_logger::Env;
use log::info;
use methods::{EXP_ID, EXP_PATH, RECURSIVE_ID, RECURSIVE_PATH, SHA3_ID, SHA3_PATH};
use risc0_zkvm::host::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};
use sha3::{Digest, Sha3_256};
use utils::inputs::{MidenAirInput, MidenRiscInput};
use winter_air::proof::{Commitments, Context, OodFrame, Queries, StarkProof};
use winter_air::{Air, FieldExtension, HashFunction, ProofOptions};
use winter_crypto::hashers::DefaultSha2;
use winter_crypto::hashers::Sha2_256;
use winter_math::fields::f64::{BaseElement, INV_NONDET};
use winter_math::log2;
use winter_verifier::VerifierChannel;

pub mod examples;
pub mod fib_winter;

/// Choose security definitions for zkp-runner
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct ProofArgs {
    /// Number of FRI queries to run in the winter proof
    #[arg(short, long, default_value_t = 9)]
    fri_queries: u32,

    /// Blowup factor (b) -- R = log2(b).
    #[arg(short, long, default_value_t = 128)]
    blowup_factor: u32,

    /// Grinding factor
    #[arg(short, long, default_value_t = 20)]
    grinding_factor: u32,
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
    let args = ProofArgs::parse();
    fib_winter::fib_winter(get_proof_options(args))?;

    // TODO - add proper cmd options
    // examples::recursive_miden()?;
    // examples::sha3();
    // examples::exp();
    Ok(())
}

fn get_proof_options(args: ProofArgs) -> ProofOptions {
    let R = log2(args.blowup_factor as usize);
    // λ ≥ min{ζ + R · s, log2|K|} − 1 from ethSTARK paper
    // Since we are using extension field of degree 2, K = P^2.
    // P is greater than 2^62 and therefore log2|K| > 124.
    // We are grinding for G bits. R = log2(blowup_factor) and thus λ = G + R*FRIQueries - 1
    info!(
        "Generating winter proofs with {}bits of security",
        args.grinding_factor + (R * args.fri_queries) - 1
    );
    ProofOptions::new(
        args.fri_queries as usize,
        args.blowup_factor as usize,
        args.grinding_factor,
        HashFunction::Sha2_256,
        FieldExtension::Quadratic,
        8,
        256,
    )
}
