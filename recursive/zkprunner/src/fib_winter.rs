use anyhow::{anyhow, Context, Result};
use log::{debug, info};
use methods::{FIB_VERIFY_ID, FIB_VERIFY_PATH};
use miden::StarkProof;
use risc0_zkvm::{host::Prover, serde::to_vec};
use utils::fib::example::{Example, FibExample};
use utils::fib::fib_air::FibAir;
use utils::inputs::{FibAirInput, FibRiscInput};
use winter_air::{Air, FieldExtension, HashFunction, ProofOptions};
use winter_crypto::hashers::{DefaultSha2, Sha2_256};
use winter_math::fields::f64::{BaseElement, INV_NONDET, INV_NONDET_QUAD};
use winter_math::fields::QuadExtension;
use winter_verifier::{Serializable, VerifierChannel};

type B = BaseElement;
type E = QuadExtension<B>;
type H = Sha2_256<B, DefaultSha2>;

pub fn fib_winter(proof_options: ProofOptions) -> Result<()> {
    info!("Generating winter fib proofs...");

    // Initialize Risc0 prover
    let mut prover = Prover::new(&std::fs::read(FIB_VERIFY_PATH).unwrap(), FIB_VERIFY_ID).unwrap();

    let (pub_inputs_1024, fib_air_input_1024) =
        generate_winter_fib_proof(proof_options.clone(), 1024)?;
    let (pub_inputs_2048, fib_air_input_2048) =
        generate_winter_fib_proof(proof_options.clone(), 2048)?;

    let pub_inputs_aux = rkyv::to_bytes::<_, 256>(&[pub_inputs_1024, pub_inputs_2048]).unwrap();
    prover.add_input_u8_slice_aux(&pub_inputs_aux);

    prover
        .add_input(
            to_vec(&fib_air_input_1024)
                .context("failed to_vec")?
                .as_slice(),
        )
        .context("failed to add fib_air_input_1024 to prover")?;

    prover
        .add_input(
            to_vec(&fib_air_input_2048)
                .context("failed to_vec")?
                .as_slice(),
        )
        .context("failed to add pub_inputs_2048 to prover")?;

    // Generate a proof of Winterfell verification using Risc0 prover
    info!("Running risc0 prover...");
    let receipt = prover.run().unwrap();
    info!("Verifying receipt of the two fib proofs in risc0");
    receipt.verify(FIB_VERIFY_ID).unwrap();

    Ok(())
}

fn generate_winter_fib_proof(
    proof_options: ProofOptions,
    n: usize,
) -> Result<(FibRiscInput<E, H>, FibAirInput)> {
    // Generate a Fibonacci proof using Winterfell prover
    let e = FibExample::new(n, proof_options);
    let proof = e.prove();
    debug!("--------------------------------");
    debug!("Trace length: {}", proof.context.trace_length());
    debug!("Trace queries length: {}", proof.trace_queries.len());
    verify_with_winter(proof.clone(), e.result.clone())?;

    // Expose verification data as public inputs to Risc0 prover
    let air = FibAir::new(proof.get_trace_info(), e.result, proof.options().clone());
    let verifier_channel: VerifierChannel<E, H> =
        VerifierChannel::new::<FibAir>(&air, proof.clone()).map_err(|msg| anyhow!(msg))?;

    let mut proof_context = Vec::new();
    proof.context.write_into(&mut proof_context);
    let pub_inputs = FibRiscInput {
        result: e.result,
        context: proof_context,
        verifier_channel,
        inv_nondet: INV_NONDET.lock().clone().into_iter().collect(),
        inv_nondet_quad: INV_NONDET_QUAD.lock().clone().into_iter().collect(),
    };
    // Expose FibAirInput as public input to Risc0 prover
    let fib_air_input = FibAirInput {
        trace_info: proof.get_trace_info(),
        proof_options: proof.options().clone(),
    };

    Ok((pub_inputs, fib_air_input))
}

fn verify_with_winter(proof: StarkProof, result: B) -> Result<()> {
    winter_verifier::verify::<FibAir>(proof, result).map_err(|msg| anyhow!(msg))
}
