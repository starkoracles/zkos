use miden_air::PublicInputs;
use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as sDeserialize, Serialize as sSerialize};
use winter_air::{EvaluationFrame, ProofOptions, TraceInfo};
use winter_math::fields::f64::BaseElement;

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[archive(compare(PartialEq))]
#[derive(Clone, Eq)]
pub struct RiscInput {
    pub trace_commitments: Vec<[u8; 32]>,
    pub constraint_commitment: [u8; 32],
    pub ood_main_trace_frame: EvaluationFrame<BaseElement>,
    pub ood_aux_trace_frame: Option<EvaluationFrame<BaseElement>>,
}

#[derive(sSerialize, sDeserialize, Debug)]
pub struct AirInput {
    pub trace_info: TraceInfo,
    pub public_inputs: PublicInputs,
    pub proof_options: ProofOptions,
}
