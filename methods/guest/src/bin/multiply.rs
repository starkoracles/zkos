#![no_main]
#![no_std]
use winter_air::proof::{Commitments, Context, OodFrame, Queries};
// use miden_verifier::verify;

use risc0_zkvm_guest::env;

risc0_zkvm_guest::entry!(main);
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
// This will generate a PartialEq impl between our unarchived and archived types
#[archive(compare(PartialEq))]
// We can pass attributes through to generated types with archive_attr
#[archive_attr(derive(Debug))]
struct ProgArgs {
    pub a: u64,
    pub b: u64,
}
pub fn main() {
    let arg_bytes: &[u8] = env::read_raw();
    let archived = unsafe { rkyv::archived_root::<ProgArgs>(&arg_bytes[..]) };
    assert_eq!(context.trace_length(), 4096);
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
