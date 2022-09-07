use winter_air::proof::StarkProof;
use winter_air::ProofOptions;
use winter_math::{fields::f64::BaseElement, FieldElement};
use winter_prover::{Prover, Trace};
use winter_verifier::VerifierError;

use super::fib_air::FibAir;
use super::fib_prover::FibProver;

pub trait Example {
    fn prove(&self) -> StarkProof;
    fn verify(&self, proof: StarkProof) -> Result<(), VerifierError>;
    fn verify_with_wrong_inputs(&self, proof: StarkProof) -> Result<(), VerifierError>;
}

impl Example for FibExample {
    fn prove(&self) -> StarkProof {
        // create a prover
        let prover = FibProver::new(self.options.clone());

        // generate execution trace
        let trace = prover.build_trace(self.sequence_length);

        // generate the proof
        prover.prove(trace).unwrap()
    }

    fn verify(&self, proof: StarkProof) -> Result<(), VerifierError> {
        winter_verifier::verify::<FibAir>(proof, self.result)
    }

    fn verify_with_wrong_inputs(&self, proof: StarkProof) -> Result<(), VerifierError> {
        todo!()
    }
}

pub fn compute_fib_term(n: usize) -> BaseElement {
    let mut t0 = BaseElement::ONE;
    let mut t1 = BaseElement::ONE;

    for _ in 0..(n - 1) {
        t1 = t0 + t1;
        core::mem::swap(&mut t0, &mut t1);
    }

    t1
}

pub struct FibExample {
    options: ProofOptions,
    sequence_length: usize,
    pub result: BaseElement,
}

impl FibExample {
    pub fn new(sequence_length: usize, options: ProofOptions) -> FibExample {
        assert!(
            sequence_length.is_power_of_two(),
            "sequence length must be a power of 2"
        );

        // compute Fibonacci sequence
        let result = compute_fib_term(sequence_length);
        FibExample {
            options,
            sequence_length,
            result,
        }
    }
}
