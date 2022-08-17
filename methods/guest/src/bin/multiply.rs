#![no_main]
#![no_std]
use winter_air::proof::StarkProof;
// use miden_verifier::verify;

use risc0_zkvm_guest::env;

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let proof: StarkProof = env::read();
    // // Load the first number from the host
    // let a: u64 = env::read();
    // // Load the second number from the host
    // let b: u64 = env::read();
    // // Verify that neither of them are 1 (i.e. nontrivial factors)
    // if a == 1 || b == 1 {
    //     panic!("Trivial factors")
    // }
    // // Compute the product while being careful with integer overflow
    // let product = a.checked_mul(b).expect("Integer overflow");
    // env::commit(&product);
}
