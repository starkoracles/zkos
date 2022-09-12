#![no_main]
#![no_std]
extern crate alloc;

use alloc::format;
use k256::{
    ecdsa::{signature::Signer, Signature, SigningKey},
    SecretKey,
};
use risc0_zkvm_guest::{env, sha};
// Verification
use k256::{
    ecdsa::{
        signature::{DigestVerifier, Verifier},
        VerifyingKey,
    },
    EncodedPoint,
};

risc0_zkvm_guest::entry!(main);

pub fn main() {
    let verifying_key_hex: &str = env::read();
    let sig_hex: &str = env::read();
    let message_hex: &str = env::read();

    let signature: Signature = Signature::from_der(&hex::decode(sig_hex).unwrap()).unwrap();
    let message_bytes = hex::decode(message_hex).unwrap();

    let verifying_key_bytes = hex::decode(verifying_key_hex).unwrap();
    let verifying_key = VerifyingKey::from_sec1_bytes(&verifying_key_bytes).unwrap();

    env::log(&format!(
        "verifying_key_hex: {}, sig_hex: {}, message_hex: {}",
        verifying_key_hex, sig_hex, message_hex
    ));

    verifying_key.verify(&message_bytes, &signature).unwrap();
}
