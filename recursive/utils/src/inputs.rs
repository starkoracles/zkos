use rkyv::{Archive, Deserialize, Serialize};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
#[archive(compare(PartialEq))]
#[archive_attr(derive(Debug))]
#[derive(Clone, Eq)]
pub struct RiscInput {
    pub trace_commitments: Vec<[u8; 24]>,
}
