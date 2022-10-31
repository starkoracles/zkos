#![no_main]
#![no_std]
extern crate alloc;

use risc0_zkvm_guest::{env, mul};
use winter_math::fields::f64::BaseElement;

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let a: u64 = env::read();
    let b: u64 = env::read();
    let a_felt = BaseElement::new(a.clone());
    let b_felt = BaseElement::new(b.clone());
    for _i in 0..10 {
        let _res = mul::mul_goldilocks(&a, &b);
        let _res_felt = a_felt * b_felt;
    }
    let res = mul::mul_goldilocks(&a, &b).get_u64();
    let _res_felt = a_felt * b_felt;
    env::commit(&res);
}
