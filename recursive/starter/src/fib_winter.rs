use anyhow::{anyhow, Context, Result};
use methods::{FIB_VERIFY_ID, FIB_VERIFY_PATH};
use miden::StarkProof;
use risc0_zkvm::{
    host::Prover,
    serde::{from_slice, to_vec},
};
use utils::fib::example::{Example, FibExample};
use utils::fib::fib_air::FibAir;
use utils::inputs::{FibAirInput, FibRiscInput, Output};
use winter_air::{Air, FieldExtension, HashFunction, ProofOptions};
use winter_crypto::hashers::{DefaultSha2, Sha2_256};
use winter_math::fields::f64::BaseElement;
use winter_verifier::{Serializable, VerifierChannel};

type E = BaseElement;

pub fn fib_winter() -> Result<()> {
    println!("============================================================");

    // Initialize Risc0 prover
    let mut prover = Prover::new(&std::fs::read(FIB_VERIFY_PATH).unwrap(), FIB_VERIFY_ID).unwrap();

    // Generate a Fibonacci proof using Winterfell prover
    let e = FibExample::new(1024, get_proof_options());
    let proof = e.prove();
    println!("--------------------------------");
    println!("Trace length: {}", proof.context.trace_length());
    println!("Trace queries length: {}", proof.trace_queries.len());
    verify_with_winter(proof.clone(), e.result.clone())?;

    // Expose verification data as public inputs to Risc0 prover
    let air = FibAir::new(proof.get_trace_info(), e.result, proof.options().clone());
    let mut verifier_channel: VerifierChannel<E, Sha2_256<E, DefaultSha2>> =
        VerifierChannel::new::<FibAir>(&air, proof.clone()).map_err(|msg| anyhow!(msg))?;
    let trace_commitments: Vec<[u8; 32]> = verifier_channel
        .read_trace_commitments()
        .into_iter()
        .map(|x| x.get_raw())
        .collect();
    let constraint_commitment = verifier_channel.read_constraint_commitment().get_raw();
    let (ood_main_trace_frame, ood_aux_trace_frame) = verifier_channel.read_ood_trace_frame();
    let ood_constraint_evaluations = verifier_channel.read_ood_constraint_evaluations();
    let mut proof_context = Vec::new();
    proof.context.write_into(&mut proof_context);
    let pub_inputs = FibRiscInput {
        trace_commitments,
        constraint_commitment,
        ood_main_trace_frame,
        ood_aux_trace_frame,
        ood_constraint_evaluations,
        result: e.result,
        context: proof_context,
    };
    let trace_commitments_to_send = rkyv::to_bytes::<_, 256>(&pub_inputs).unwrap();
    prover.add_input_u8_slice_aux(&trace_commitments_to_send);

    // Expose FibAirInput as public input to Risc0 prover
    let fib_air_input = FibAirInput {
        trace_info: proof.get_trace_info(),
        proof_options: proof.options().clone(),
    };
    prover
        .add_input(to_vec(&fib_air_input).context("failed to_vec")?.as_slice())
        .context("failed to add input to prover")?;

    // Generate a proof of Winterfell verification using Risc0 prover
    let receipt = prover.run().unwrap();
    receipt.verify(FIB_VERIFY_ID).unwrap();

    Ok(())
}

fn get_proof_options() -> ProofOptions {
    ProofOptions::new(
        27,
        8,
        16,
        HashFunction::Sha2_256,
        FieldExtension::None,
        8,
        256,
    )
}

fn verify_with_winter(proof: StarkProof, result: E) -> Result<()> {
    winter_verifier::verify::<FibAir>(proof, result).map_err(|msg| anyhow!(msg))
}
