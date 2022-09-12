use anyhow::{anyhow, Context, Result};
use k256::elliptic_curve::sec1::ToEncodedPoint;
use k256::{
    ecdsa::{signature::Signer, Signature, SigningKey},
    elliptic_curve::rand_core::OsRng,
    SecretKey,
};
// Verification
use k256::ecdsa::{signature::Verifier, VerifyingKey};
use methods::{VERIFY_SIG_ID, VERIFY_SIG_PATH};
use risc0_zkvm::{
    host::Prover,
    serde::{from_slice, to_vec},
};

pub fn sign_secret_message() -> Result<()> {
    // Signing
    let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
    let message = b"I own BAYC and a few CryptoPunks!!!";

    // Note: the signature type must be annotated or otherwise inferrable as
    // `Signer` has many impls of the `Signer` trait (for both regular and
    // recoverable signature types).
    let signature: Signature = signing_key.sign(message);

    let verifying_key = VerifyingKey::from(&signing_key);
    let verify_hex = hex::encode(verifying_key.to_bytes());

    let sig_hex = hex::encode(signature.to_der().as_bytes());
    let message_hex = hex::encode(message);

    let mut prover = Prover::new(&std::fs::read(VERIFY_SIG_PATH).unwrap(), VERIFY_SIG_ID).unwrap();
    prover
        .add_input(to_vec(&verify_hex).unwrap().as_slice())
        .unwrap();

    prover
        .add_input(to_vec(&sig_hex).unwrap().as_slice())
        .unwrap();

    prover
        .add_input(to_vec(&message_hex).unwrap().as_slice())
        .unwrap();

    println!(
        "verifying_key: {:?}, sig: {:#?}, message: {:?}",
        &verify_hex, &sig_hex, &message_hex
    );
    assert!(verifying_key.verify(message, &signature).is_ok());
    let receipt = prover.run().unwrap();
    receipt.verify(VERIFY_SIG_ID).unwrap();
    Ok(())
}
