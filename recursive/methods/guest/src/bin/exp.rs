#![no_main]
#![no_std]
extern crate alloc;

use risc0_zkvm_guest::{env, mul};

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let a: u64 = env::read();
    let b: u64 = env::read();
    let res = mul::mul_goldilocks(&a, &b).get_u64();
    env::commit(&res);
}
