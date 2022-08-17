use methods::{MULTIPLY_ID, MULTIPLY_PATH};
use miden::ProofOptions;
use risc0_zkvm::host::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};

pub mod fibonacci;

fn main() {
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
    println!("Program output: {:?}", outputs);
    assert_eq!(
        expected_result, outputs,
        "Program result was computed incorrectly"
    );

    let mut prover = Prover::new(&std::fs::read(MULTIPLY_PATH).unwrap(), MULTIPLY_ID).unwrap();
    prover
        .add_input(to_vec(&proof).unwrap().as_slice())
        .unwrap();
    let receipt = prover.run().unwrap();

    // // Pick two numbers
    // let a: u64 = 17;
    // let b: u64 = 23;

    // // Multiply them inside the ZKP
    // // First, we make the prover, loading the 'multiply' method
    // let mut prover = Prover::new(&std::fs::read(MULTIPLY_PATH).unwrap(), MULTIPLY_ID).unwrap();
    // // Next we send a & b to the guest
    // prover.add_input(to_vec(&a).unwrap().as_slice()).unwrap();
    // prover.add_input(to_vec(&b).unwrap().as_slice()).unwrap();
    // // Run prover & generate receipt
    // let receipt = prover.run().unwrap();

    // // Extract journal of receipt (i.e. output c, where c = a * b)
    // let c: u64 = from_slice(&receipt.get_journal_vec().unwrap()).unwrap();

    // // Print an assertion
    // println!("I know the factors of {}, and I can prove it!", c);

    // // Here is where one would send 'receipt' over the network...

    // // Verify receipt, panic if it's wrong
    // receipt.verify(MULTIPLY_ID).unwrap();
}

pub fn get_proof_options() -> ProofOptions {
    ProofOptions::with_96_bit_security()
}
