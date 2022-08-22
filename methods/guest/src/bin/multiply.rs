#![no_main]
#![no_std]
use winter_air::proof::{Commitments, Context, OodFrame, Queries, StarkProof};
use winter_utils::collections::Vec;
// use miden_verifier::verify;

use risc0_zkvm_guest::env;

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let arg_bytes: &[u8] = env::read_raw();
    let constraint_queries = unsafe { rkyv::archived_root::<Queries>(&arg_bytes[..]) };
    // let parsed_constraints = constraint_queries.pasre().expect("parse to succeed");
    let trace_queries = unsafe { rkyv::archived_root::<Vec<Queries>>(&arg_bytes[..]) };

    // assert_eq!(archived.context.trace_length(), 4096);
    // let commitments: Commitments = env::read();
    // let ood_frame: OodFrame = env::read();
    // Load the first number from the host
    // let args: ProgArgs = env::read();
    // let a: u64 = args.a;
    // // Load the second number from the host
    // let b: u64 = args.b;
    // // Verify that neither of them are 1 (i.e. nontrivial factors)
    // if a == 1 || b == 1 {
    //     panic!("Trivial factors")
    // }
    // // Compute the product while being careful with integer overflow
    // let product = a.checked_mul(b).expect("Integer overflow");
    // env::commit(&product);
}
