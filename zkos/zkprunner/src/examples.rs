use anyhow::{anyhow, Result};
use env_logger::Env;
use log::info;
use methods::{EXP_ID, EXP_PATH, RECURSIVE_ID, RECURSIVE_PATH, SHA3_ID, SHA3_PATH};
use miden::{Program, ProofOptions};
use miden_air::{Felt, FieldElement, ProcessorAir, PublicInputs};
use miden_core::utils::Serializable;
use risc0_zkvm::host::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};
use sha3::{Digest, Sha3_256};
use utils::inputs::{MidenAirInput, MidenRiscInput};
use winter_air::proof::{Commitments, Context, OodFrame, Queries, StarkProof};
use winter_air::Air;
use winter_crypto::hashers::DefaultSha2;
use winter_crypto::hashers::Sha2_256;
use winter_math::fields::f64::{BaseElement, INV_NONDET};
use winter_verifier::VerifierChannel;

use utils::fibonacci_miden;

#[allow(dead_code)]
fn recursive_miden() -> Result<()> {
    println!("============================================================");

    let proof_options = get_proof_options_miden();

    // instantiate and prepare the example
    let example = fibonacci_miden::get_example(1024);

    let fibonacci_miden::Example {
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

    let mut proof_context = Vec::new();
    proof.context.write_into(&mut proof_context);

    let (verifier_channel, air_input) =
        get_verifier_channel(&proof, &outputs, &pub_inputs, program.clone())?;

    // run verify in order to generate nondet inv inputs
    miden::verify(program.hash().clone(), &pub_inputs[..], &outputs[..], proof).unwrap();

    let risc_inputs = MidenRiscInput {
        context: proof_context,
        verifier_channel,
        inv_nondet: INV_NONDET.lock().clone().into_iter().collect(),
    };

    let mut prover = Prover::new(&std::fs::read(RECURSIVE_PATH).unwrap(), RECURSIVE_ID).unwrap();
    let miden_risc_inputs = rkyv::to_bytes::<_, 256>(&risc_inputs).unwrap();
    prover.add_input_u8_slice_aux(&miden_risc_inputs);
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
    MidenAirInput,
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
    let air_input = MidenAirInput {
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn exp() {
    let mut prover = Prover::new(&std::fs::read(EXP_PATH).unwrap(), EXP_ID).unwrap();
    let a = 0xFFFFFFFF00000000u64;
    let b = 2u64;
    prover.add_input(to_vec(&a).unwrap().as_slice()).unwrap();
    prover.add_input(to_vec(&b).unwrap().as_slice()).unwrap();
    let receipt = prover.run().unwrap();
    let result: u64 = from_slice(&receipt.get_journal_vec().unwrap()).unwrap();
    println!("{}*{} = {}", &a, &b, &result);
    receipt.verify(EXP_ID).unwrap();
    let res_felt = BaseElement::new(result);
    assert_eq!(res_felt, BaseElement::new(a) * BaseElement::new(b));
}

pub fn get_proof_options_miden() -> ProofOptions {
    ProofOptions::with_sha2()
}
