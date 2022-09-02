use std::ops::Deref;

use anyhow::{anyhow, Result};
use methods::{RECURSIVE_ID, RECURSIVE_PATH, SHA3_ID, SHA3_PATH};
use miden::{Program, ProofOptions};
use miden_air::{Felt, ProcessorAir, PublicInputs};
use risc0_zkvm::host::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};
use sha3::{Digest, Sha3_256};
use utils::inputs::{AirInput, RiscInput};
use winter_air::proof::{Commitments, Context, OodFrame, Queries, StarkProof};
use winter_air::Air;
use winter_crypto::hashers::DefaultSha2;
use winter_crypto::hashers::Sha2_256;
use winter_math::fields::f64::BaseElement;
use winter_math::fields::QuadExtension;
use winter_verifier::VerifierChannel;

pub mod fibonacci;

fn recursive() -> Result<()> {
    println!("============================================================");

    let proof_options = get_proof_options();

    // instantiate and prepare the example
    let example = fibonacci::get_example(1024);

    let fibonacci::Example {
        program,
        inputs,
        num_outputs,
        pub_inputs,
        expected_result,
    } = example;
    println!("--------------------------------");

    // execute the program and generate the proof of execution
    let (outputs, proof) = miden::prove(&program, &inputs, num_outputs, &proof_options).unwrap();
    println!("--------------------------------");
    println!("Trace length: {}", proof.context.trace_length());
    println!("Trace queries length: {}", proof.trace_queries.len());
    println!("Program output: {:?}", outputs);
    assert_eq!(
        expected_result, outputs,
        "Program result was computed incorrectly"
    );

    let (mut verifier_channel, air_input) =
        get_verifier_channel(&proof, &outputs, &pub_inputs, program)?;
    let trace_commitments: Vec<[u8; 32]> = verifier_channel
        .read_trace_commitments()
        .into_iter()
        .map(|x| x.get_raw())
        .collect();
    let constraint_commitment = verifier_channel.read_constraint_commitment().get_raw();
    let (ood_main_trace_frame, ood_aux_trace_frame) = verifier_channel.read_ood_trace_frame();
    println!("ood size is: {}", ood_main_trace_frame.current.len());

    let risc_inputs = RiscInput {
        trace_commitments,
        constraint_commitment,
        ood_main_trace_frame,
        ood_aux_trace_frame,
    };

    let mut prover = Prover::new(&std::fs::read(RECURSIVE_PATH).unwrap(), RECURSIVE_ID).unwrap();
    let trace_commitments_to_send = rkyv::to_bytes::<_, 256>(&risc_inputs).unwrap();
    prover.add_input_u8_slice_aux(&trace_commitments_to_send);
    prover.add_input(to_vec(&air_input)?.as_slice())?;
    let receipt = prover.run().unwrap();
    receipt.verify(RECURSIVE_ID).unwrap();
    Ok(())
}

fn get_verifier_channel(
    proof: &StarkProof,
    outputs: &Vec<u64>,
    inputs: &Vec<u64>,
    program: Program,
) -> Result<(
    VerifierChannel<BaseElement, Sha2_256<BaseElement, DefaultSha2>>,
    AirInput,
)> {
    let mut stack_input_felts: Vec<Felt> = Vec::with_capacity(inputs.len());
    for &input in inputs.iter().rev() {
        stack_input_felts.push(
            input
                .try_into()
                .map_err(|_| anyhow!("cannot map input into felts"))?,
        );
    }

    let mut stack_output_felts: Vec<Felt> = Vec::with_capacity(outputs.len());
    for &output in outputs.iter() {
        stack_output_felts.push(
            output
                .try_into()
                .map_err(|_| anyhow!("cannot map output into felts"))?,
        );
    }

    let pub_inputs = PublicInputs::new(program.hash(), stack_input_felts, stack_output_felts);
    let air_input = AirInput {
        trace_info: proof.get_trace_info(),
        public_inputs: pub_inputs.clone(),
        proof_options: proof.options().clone(),
    };
    let air = ProcessorAir::new(proof.get_trace_info(), pub_inputs, proof.options().clone());
    Ok((
        (VerifierChannel::new::<ProcessorAir>(&air, proof.clone()).map_err(|msg| anyhow!(msg))?),
        air_input,
    ))
}

fn sha3() {
    let mut prover = Prover::new(&std::fs::read(SHA3_PATH).unwrap(), SHA3_ID).unwrap();
    let input = "my name is cpunkzzz asdfhjklasdf a lot of bytes";
    prover
        .add_input(to_vec(&input).unwrap().as_slice())
        .unwrap();
    let receipt = prover.run().unwrap();
    let hashed: String = from_slice(&receipt.get_journal_vec().unwrap()).unwrap();
    println!("I know the preimage of {} and I can prove it!", hashed);
    receipt.verify(SHA3_ID).unwrap();

    let mut hasher = sha2::Sha256::new();
    hasher.update(&input);
    let result = hasher.finalize();
    let s = hex::encode(&result);
    assert_eq!(&s, &hashed);
}

fn main() -> Result<()> {
    recursive()?;
    // sha3();
    Ok(())
}

pub fn get_proof_options() -> ProofOptions {
    ProofOptions::with_sha2()
}
