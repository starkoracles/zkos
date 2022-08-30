use miden_air::PublicInputs;
use rkyv::{Archive, Deserialize, Serialize};
use serde::{Deserialize as sDeserialize, Serialize as sSerialize};
use winter_air::{ProofOptions, TraceInfo};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug))]
#[derive(Clone, Eq)]
pub struct RiscInput {
    pub trace_commitments: Vec<[u8; 32]>,
}

#[derive(sSerialize, sDeserialize, Debug)]
pub struct AirInput {
    pub trace_info: TraceInfo,
    pub public_inputs: PublicInputs,
    pub proof_options: ProofOptions,
}
