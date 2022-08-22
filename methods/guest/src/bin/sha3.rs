#![no_main]
#![no_std]
use risc0_zkvm_guest::env;
use sha3::{Digest, Sha3_256};
use winter_utils::string::String;

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let input: &str = env::read();
    let mut start = String::from(input);
    for i in 0..10 {
        // create a SHA3-256 object
        let mut hasher = Sha3_256::new();

        // write input message
        hasher.update(&start);

        // read hash digest
        let r = hasher.finalize();
        start = hex::encode(&r)
    }
    env::commit(&start);
}
