use methods::{MULTIPLY_ID, MULTIPLY_PATH};
use miden::ProofOptions;
use risc0_zkvm::host::Prover;
use risc0_zkvm::serde::{from_slice, to_vec};
use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
// This will generate a PartialEq impl between our unarchived and archived types
#[archive(compare(PartialEq))]
// We can pass attributes through to generated types with archive_attr
#[archive_attr(derive(Debug))]
struct ProgArgs {
    pub a: u64,
    pub b: u64,
}

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
    println!("Trace length: {}", proof.context.trace_length());
    println!(
        "context num of words: {}",
        to_vec(&proof.context).unwrap().as_slice().len()
    );
    println!(
        "commitments num of words: {}",
        to_vec(&proof.commitments).unwrap().as_slice().len()
    );
    println!(
        "trace query num of words: {}",
        to_vec(&proof.trace_queries).unwrap().as_slice().len()
    );
    println!(
        "Constraint query num of words: {}",
        to_vec(&proof.constraint_queries).unwrap().as_slice().len()
    );
    println!(
        "OOD frame query num of words: {}",
        to_vec(&proof.ood_frame).unwrap().as_slice().len()
    );
    println!(
        "fri proof num of words: {}",
        to_vec(&proof.fri_proof).unwrap().as_slice().len()
    );
    println!("Program output: {:?}", outputs);
    assert_eq!(
        expected_result, outputs,
        "Program result was computed incorrectly"
    );

    let mut prover = Prover::new(&std::fs::read(MULTIPLY_PATH).unwrap(), MULTIPLY_ID).unwrap();
    let args = ProgArgs { a: 17, b: 23 };
    let bytes = rkyv::to_bytes::<_, 256>(&args).unwrap();
    prover.add_input_u8_slice(&bytes);
    let archived = unsafe { rkyv::archived_root::<ProgArgs>(&bytes[..]) };
    println!("archived: {}, {}", archived.a, archived.b);
    // prover
    //     .add_input(to_vec(&proof.context).unwrap().as_slice())
    //     .unwrap();
    // prover
    //     .add_input(to_vec(&proof.commitments).unwrap().as_slice())
    //     .unwrap();
    // prover
    //     .add_input(to_vec(&proof.ood_frame).unwrap().as_slice())
    //     .unwrap();
    let receipt = prover.run().unwrap();
    receipt.verify(MULTIPLY_ID).unwrap();

    // // Pick two numbers
    // let a: u64 = 17;
    // let b: u64 = 23;
    // let args = ProgArgs { a, b };

    // // Multiply them inside the ZKP
    // // First, we make the prover, loading the 'multiply' method
    // let mut prover = Prover::new(&std::fs::read(MULTIPLY_PATH).unwrap(), MULTIPLY_ID).unwrap();
    // // Next we send a & b to the guest
    // prover.add_input(to_vec(&args).unwrap().as_slice()).unwrap();
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
