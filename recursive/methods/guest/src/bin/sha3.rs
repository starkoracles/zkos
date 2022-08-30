#![no_main]
#![no_std]
use risc0_zkvm_guest::{env, sha};

risc0_zkvm_guest::entry!(main);
use winter_crypto::hashers::{Sha2_256, ShaHasherT};

pub struct GuestSha2;

impl ShaHasherT for GuestSha2 {
    fn digest(data: &[u8]) -> [u8; 32] {
        sha::digest_u8_slice(data).get_u8()
    }
}

pub fn main() {
    let input: &str = env::read();
    let digest = sha::digest_u8_slice(input.as_bytes());
    let d2 = GuestSha2::digest(input.as_bytes());
    env::commit(&hex::encode(&d2));
}
