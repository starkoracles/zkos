#![no_main]
#![no_std]
extern crate alloc;

use alloc::vec::Vec;
use anyhow::{anyhow, Result};
use miden_air::ProcessorAir;
use risc0_zkvm_guest::{env, sha};
use rkyv::{option::ArchivedOption, Archive, Deserialize};
use utils::inputs::{AirInput, RiscInput};
use winter_air::{
    proof::{Commitments, Context, OodFrame, Queries, StarkProof},
    Air, AuxTraceRandElements, ConstraintCompositionCoefficients, EvaluationFrame,
};
use winter_crypto::{
    hashers::{Sha2_256, ShaHasherT},
    ByteDigest, RandomCoin,
};
use winter_math::fields::f64::BaseElement;
use winter_verifier::evaluate_constraints;

risc0_zkvm_guest::entry!(main);

pub struct GuestSha2;

impl ShaHasherT for GuestSha2 {
    fn digest(data: &[u8]) -> [u8; 32] {
        sha::digest_u8_slice(data).get_u8()
    }
}

pub fn aux_trace_segments(
    risc_input: &<RiscInput as Archive>::Archived,
    public_coin: &mut RandomCoin<BaseElement, Sha2_256<BaseElement, GuestSha2>>,
    air: &ProcessorAir,
) -> Result<AuxTraceRandElements<BaseElement>> {
    let first_digest = ByteDigest::new(risc_input.trace_commitments[0]);
    public_coin.reseed(first_digest);
    let mut aux_trace_rand_elements = AuxTraceRandElements::<BaseElement>::new();
    for (i, commitment) in risc_input.trace_commitments.iter().skip(1).enumerate() {
        let rand_elements = air
            .get_aux_trace_segment_random_elements(i, public_coin)
            .map_err(|_| anyhow!("Random coin error"))?;
        aux_trace_rand_elements.add_segment_elements(rand_elements);
        let c = ByteDigest::new(*commitment);
        public_coin.reseed(c);
    }
    Ok(aux_trace_rand_elements)
}

pub fn get_constraint_coffs(
    public_coin: &mut RandomCoin<BaseElement, Sha2_256<BaseElement, GuestSha2>>,
    air: &ProcessorAir,
) -> Result<ConstraintCompositionCoefficients<BaseElement>> {
    let constraint_coeffs = air
        .get_constraint_composition_coefficients(public_coin)
        .map_err(|_| anyhow!("Random coin error"))?;
    Ok(constraint_coeffs)
}

pub fn main() {
    let aux_input: &[u8] = env::read_aux_input();
    let air_input: AirInput = env::read();
    let air = ProcessorAir::new(
        air_input.trace_info,
        air_input.public_inputs,
        air_input.proof_options,
    );

    let risc_input = unsafe { rkyv::archived_root::<RiscInput>(&aux_input[..]) };
    let public_coin_seed = Vec::new();
    let mut public_coin: RandomCoin<BaseElement, Sha2_256<BaseElement, GuestSha2>> =
        RandomCoin::new(&public_coin_seed);
    // process auxiliary trace segments (if any), to build a set of random elements for each segment
    let aux_trace_rand_elements =
        aux_trace_segments(&risc_input, &mut public_coin, &air).expect("aux trace segments failed");
    // build random coefficients for the composition polynomial
    let constraint_coeffs =
        get_constraint_coffs(&mut public_coin, &air).expect("constraint_coeffs_error");
    let constraint_commitment = ByteDigest::new(risc_input.constraint_commitment);
    public_coin.reseed(constraint_commitment);
    let z = public_coin
        .draw::<BaseElement>()
        .map_err(|_| anyhow!("Random coin error"))
        .expect("constraint_commitment");

    // TODO remove redundant copy
    let ood_main_trace_frame: EvaluationFrame<BaseElement> = EvaluationFrame::from_rows(
        risc_input
            .ood_main_trace_frame
            .current
            .deserialize(&mut rkyv::Infallible)
            .unwrap(),
        risc_input
            .ood_main_trace_frame
            .next
            .deserialize(&mut rkyv::Infallible)
            .unwrap(),
    );

    let ood_aux_trace_frame: Option<EvaluationFrame<BaseElement>> =
        match &risc_input.ood_aux_trace_frame {
            ArchivedOption::None => None,
            ArchivedOption::Some(row) => Some(EvaluationFrame::from_rows(
                row.current.deserialize(&mut rkyv::Infallible).unwrap(),
                row.next.deserialize(&mut rkyv::Infallible).unwrap(),
            )),
        };

    // evaluate_constraints(
    //     &air,
    //     constraint_coeffs,
    //     &ood_main_trace_frame,
    //     &ood_aux_trace_frame,
    //     aux_trace_rand_elements,
    //     z,
    // );
}
