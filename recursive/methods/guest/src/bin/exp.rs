#![no_main]
#![no_std]
extern crate alloc;

use miden_air::FieldElement;
use risc0_zkvm_guest::env;
use winter_math::fields::f64::BaseElement;

risc0_zkvm_guest::entry!(main);

pub fn local_exp(input: BaseElement, exp: u64) -> BaseElement {
    // Special case for handling 0^0 = 1
    if exp == 0 {
        return BaseElement::ONE;
    }

    let mut acc = BaseElement::ONE;
    let bit_length = 64 - exp.leading_zeros();
    for i in 0..bit_length {
        acc = acc * acc;
        if exp & (1 << (bit_length - 1 - i)) != 0 {
            acc *= input;
        }
    }

    acc
}

pub fn main() {
    let base: u64 = env::read();
    let exp: u64 = env::read();
    for i in 0..10 {
        let b = BaseElement::new(base);
        let res = local_exp(b, exp);
    }
    let b = BaseElement::new(base);
    let res = local_exp(b, exp);
    env::commit(&res);
}
