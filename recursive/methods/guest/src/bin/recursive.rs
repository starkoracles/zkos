#![no_main]
#![no_std]
extern crate alloc;

use alloc::vec::Vec;
use risc0_zkvm_guest::env;
use utils::inputs::RiscInput;
use winter_air::proof::{Commitments, Context, OodFrame, Queries, StarkProof};
use winter_crypto::{hashers::Blake3_192, ByteDigest, RandomCoin};
use winter_math::fields::{f64::BaseElement, QuadExtension};

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let arg_bytes: &[u8] = env::read_raw();
    let risc_input = unsafe { rkyv::archived_root::<RiscInput>(&arg_bytes[..]) };
    let public_coin_seed = Vec::new();
    let mut public_coin: RandomCoin<BaseElement, Blake3_192<BaseElement>> =
        RandomCoin::new(&public_coin_seed);
    let first_digest = ByteDigest::new(risc_input.trace_commitments[0]);
    public_coin.reseed(first_digest);

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
