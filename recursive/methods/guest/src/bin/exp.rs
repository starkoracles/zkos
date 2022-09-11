#![no_main]
#![no_std]
extern crate alloc;

use miden_air::FieldElement;
use risc0_zkvm_guest::env;
use winter_math::fields::f64::BaseElement;

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let base: u64 = env::read();
    let exp: u64 = env::read();
    let b = BaseElement::new(base);
    let res = b.exp(exp);
    env::commit(&res);
}
