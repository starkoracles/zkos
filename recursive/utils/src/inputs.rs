use std::collections::BTreeMap;

use miden_air::{Felt, FieldElement, PublicInputs};
use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as sDeserialize, Serialize as sSerialize};
use winter_air::{EvaluationFrame, ProofOptions, TraceInfo};
use winter_prover::crypto::ElementHasher;
use winter_verifier::VerifierChannel;

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[archive(compare(PartialEq))]
#[derive(Clone, Eq)]
pub struct RiscInput<E: FieldElement> {
    pub trace_commitments: Vec<[u8; 32]>,
    pub constraint_commitment: [u8; 32],
    pub ood_main_trace_frame: EvaluationFrame<E>,
    pub ood_aux_trace_frame: Option<EvaluationFrame<E>>,
}

#[derive(sSerialize, sDeserialize, Debug)]
pub struct AirInput {
    pub trace_info: TraceInfo,
    pub public_inputs: PublicInputs,
    pub proof_options: ProofOptions,
}

#[derive(sSerialize, sDeserialize, Debug)]
pub struct Output<E: FieldElement> {
    pub ood_constraint_evaluation_1: E,
    pub ood_constraint_evaluation_2: E,
}

#[derive(sSerialize, sDeserialize, Debug)]
pub struct FibAirInput {
    pub trace_info: TraceInfo,
    pub proof_options: ProofOptions,
}

#[derive(Archive, Deserialize, Serialize)]
pub struct FibRiscInput<E: FieldElement, H: ElementHasher<BaseField = E::BaseField>> {
    pub result: E,
    pub context: Vec<u8>,
    pub verifier_channel: VerifierChannel<E, H>,
    pub inv_nondet: Vec<(E, E)>,
}
