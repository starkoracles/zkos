#![no_main]
#![no_std]
extern crate alloc;

use alloc::format;
use alloc::vec::Vec;
use anyhow::{anyhow, Result};
use miden_air::FieldElement;
use risc0_zkvm_guest::{env, sha};
use rkyv::{option::ArchivedOption, Archive, Deserialize};
use utils::fib::fib_air::FibAir;
use utils::inputs::{FibAirInput, FibRiscInput, Output};
use winter_air::{Air, AuxTraceRandElements, ConstraintCompositionCoefficients, EvaluationFrame};
use winter_crypto::ElementHasher;
use winter_crypto::{
    hashers::{Sha2_256, ShaHasherT},
    ByteDigest, RandomCoin,
};
use winter_math::{fields::f64::BaseElement, fields::StarkField};
use winter_utils::Serializable;
use winter_verifier::{evaluate_constraints, FriVerifier, FriVerifierChannel, VerifierChannel};

risc0_zkvm_guest::entry!(main);

pub struct GuestSha2;

impl ShaHasherT for GuestSha2 {
    fn digest(data: &[u8]) -> [u8; 32] {
        sha::digest_u8_slice(data).get_u8()
    }
}

type E = BaseElement;
type H = Sha2_256<E, GuestSha2>;
type C = VerifierChannel<E, H>;

pub fn aux_trace_segments(
    verifier_channel: &C,
    public_coin: &mut RandomCoin<E, Sha2_256<E, GuestSha2>>,
    air: &FibAir,
) -> Result<AuxTraceRandElements<E>> {
    let mut aux_trace_rand_elements = AuxTraceRandElements::<E>::new();
    for (i, commitment) in verifier_channel
        .read_trace_commitments()
        .iter()
        .skip(1)
        .enumerate()
    {
        let rand_elements = air
            .get_aux_trace_segment_random_elements(i, public_coin)
            .map_err(|_| anyhow!("Random coin error"))?;
        aux_trace_rand_elements.add_segment_elements(rand_elements);
        public_coin.reseed(*commitment);
    }
    Ok(aux_trace_rand_elements)
}

pub fn get_constraint_coffs(
    public_coin: &mut RandomCoin<E, Sha2_256<E, GuestSha2>>,
    air: &FibAir,
) -> Result<ConstraintCompositionCoefficients<E>> {
    let constraint_coeffs = air
        .get_constraint_composition_coefficients(public_coin)
        .map_err(|_| anyhow!("Random coin error"))?;
    Ok(constraint_coeffs)
}

pub fn init_public_coin_seed<S: Serializable>(
    public_coin_seed: &mut Vec<u8>,
    result: S,
    context: &[u8],
) {
    result.write_into(public_coin_seed);
    public_coin_seed.extend(context);
}

pub fn main() {
    // Deserialize public inputs
    let aux_input: &[u8] = env::read_aux_input();
    let pub_inputs = unsafe { rkyv::archived_root::<FibRiscInput<E, H>>(&aux_input[..]) };

    let mut verifier_channel: C = pub_inputs
        .verifier_channel
        .deserialize(&mut rkyv::Infallible)
        .unwrap();
    // Extract result (pub input to Fib proof)
    let result = pub_inputs
        .result
        .deserialize(&mut rkyv::Infallible)
        .unwrap();

    // Extract context
    let context = pub_inputs.context.as_slice();

    // Extract Fibonacci AIR
    let air_input: FibAirInput = env::read();
    let air = FibAir::new(air_input.trace_info, result, air_input.proof_options);

    // build a seed for the public coin; the initial seed is the hash of public inputs and proof
    // context, but as the protocol progresses, the coin will be reseeded with the info received
    // from the prover
    let mut public_coin_seed = Vec::new();
    init_public_coin_seed(&mut public_coin_seed, result, context);

    let mut public_coin: RandomCoin<E, Sha2_256<E, GuestSha2>> = RandomCoin::new(&public_coin_seed);

    // reseed the coin with the commitment to the main trace segment
    public_coin.reseed(verifier_channel.read_trace_commitments()[0]);

    // process auxiliary trace segments (if any), to build a set of random elements for each segment
    let aux_trace_rand_elements = aux_trace_segments(&verifier_channel, &mut public_coin, &air)
        .expect("aux trace segments failed");

    // build random coefficients for the composition polynomial
    let constraint_coeffs =
        get_constraint_coffs(&mut public_coin, &air).expect("constraint_coeffs_error");
    env::log(&format!("constraint coeffs: {:?}", &constraint_coeffs));

    // 2 ----- constraint commitment --------------------------------------------------------------
    // let constraint_commitment = ByteDigest::new(pub_inputs.constraint_commitment);
    env::log(&format!("constraint commitment"));
    public_coin.reseed(verifier_channel.read_constraint_commitment());
    let z = public_coin
        .draw::<E>()
        .map_err(|_| anyhow!("Random coin error"))
        .expect("constraint_commitment");

    // 3 ----- OOD consistency check --------------------------------------------------------------
    // make sure that evaluations obtained by evaluating constraints over the out-of-domain frame
    // are consistent with the evaluations of composition polynomial columns sent by the prover

    // read the out-of-domain trace frames (the main trace frame and auxiliary trace frame, if
    // provided) sent by the prover and evaluate constraints over them; also, reseed the public
    // coin with the OOD frames received from the prover.
    // let ood_main_trace_frame: EvaluationFrame<E> = EvaluationFrame::from_rows(
    //     pub_inputs
    //         .ood_main_trace_frame
    //         .current
    //         .deserialize(&mut rkyv::Infallible)
    //         .unwrap(),
    //     pub_inputs
    //         .ood_main_trace_frame
    //         .next
    //         .deserialize(&mut rkyv::Infallible)
    //         .unwrap(),
    // );

    // // let ood_aux_trace_frame: Option<EvaluationFrame<E>> = match &pub_inputs.ood_aux_trace_frame {
    // //     ArchivedOption::None => None,
    // //     ArchivedOption::Some(row) => Some(EvaluationFrame::from_rows(
    // //         row.current.deserialize(&mut rkyv::Infallible).unwrap(),
    // //         row.next.deserialize(&mut rkyv::Infallible).unwrap(),
    // //     )),
    // // };

    env::log(&format!("ood_frame"));
    // let (ood_main_trace_frame, ood_aux_trace_frame) = verifier_channel.read_ood_trace_frame();

    // let ood_constraint_evaluation_1 = evaluate_constraints(
    //     &air,
    //     constraint_coeffs,
    //     &ood_trace_frame.main_frame,
    //     &ood_trace_frame.aux_frame,
    //     aux_trace_rand_elements,
    //     z,
    // );

    // env::log(&format!("reseed ood_frame"));
    // if let Some(ref aux_trace_frame) = ood_aux_trace_frame {
    //     // when the trace contains auxiliary segments, append auxiliary trace elements at the
    //     // end of main trace elements for both current and next rows in the frame. this is
    //     // needed to be consistent with how the prover writes OOD frame into the channel.

    //     let mut current = ood_main_trace_frame.current().to_vec();
    //     current.extend_from_slice(aux_trace_frame.current());
    //     public_coin.reseed(H::hash_elements(&current));

    //     let mut next = ood_main_trace_frame.next().to_vec();
    //     next.extend_from_slice(aux_trace_frame.next());
    //     public_coin.reseed(H::hash_elements(&next));
    // } else {
    //     public_coin.reseed(H::hash_elements(ood_main_trace_frame.current()));
    //     public_coin.reseed(H::hash_elements(ood_main_trace_frame.next()));
    // }

    // // // read evaluations of composition polynomial columns sent by the prover, and reduce them into
    // // // a single value by computing sum(z^i * value_i), where value_i is the evaluation of the ith
    // // // column polynomial at z^m, where m is the total number of column polynomials; also, reseed
    // // // the public coin with the OOD constraint evaluations received from the prover.
    // // let ood_constraint_evaluations: Vec<E> = pub_inputs
    // //     .ood_constraint_evaluations
    // //     .deserialize(&mut rkyv::Infallible)
    // //     .unwrap();
    // env::log(&format!("ood_constraint_evaluation_2"));
    // let ood_constraint_evaluation_2 = verifier_channel
    //     .read_ood_constraint_evaluations()
    //     .iter()
    //     .enumerate()
    //     .fold(E::ZERO, |result, (i, &value)| {
    //         result + z.exp((i as u32).into()) * value
    //     });
    // public_coin.reseed(H::hash_elements(
    //     &verifier_channel.read_ood_constraint_evaluations(),
    // ));

    // // finally, make sure the values are the same
    // if ood_constraint_evaluation_1 != ood_constraint_evaluation_2 {
    //     panic!("Inconsistent OOD constraint evaluations");
    // }

    // // 4 ----- FRI commitments --------------------------------------------------------------------
    // // draw coefficients for computing DEEP composition polynomial from the public coin; in the
    // // interactive version of the protocol, the verifier sends these coefficients to the prover
    // // and the prover uses them to compute the DEEP composition polynomial. the prover, then
    // // applies FRI protocol to the evaluations of the DEEP composition polynomial.
    // let deep_coefficients = air
    //     .get_deep_composition_coefficients::<E, H>(&mut public_coin)
    //     .map_err(|msg| anyhow!(msg))
    //     .unwrap();

    // // instantiates a FRI verifier with the FRI layer commitments read from the channel. From the
    // // verifier's perspective, this is equivalent to executing the commit phase of the FRI protocol.
    // // The verifier uses these commitments to update the public coin and draw random points alpha
    // // from them; in the interactive version of the protocol, the verifier sends these alphas to
    // // the prover, and the prover uses them to compute and commit to the subsequent FRI layers.
    // let fri_verifier: FriVerifier<E, E, C, H> = FriVerifier::new(
    //     &mut verifier_channel,
    //     &mut public_coin,
    //     air.options().to_fri_options(),
    //     air.trace_poly_degree(),
    // )
    // .expect("fri verifier init failed");

    // // 5 ----- trace and constraint queries -------------------------------------------------------
    // // read proof-of-work nonce sent by the prover and update the public coin with it
    // let pow_nonce = pub_inputs.pow_nonce;
    // public_coin.reseed_with_int(pow_nonce);

    // // make sure the proof-of-work specified by the grinding factor is satisfied
    // if public_coin.leading_zeros() < air.options().grinding_factor() {
    //     panic!("QuerySeedProofOfWorkVerificationFailed");
    // }

    // // draw pseudo-random query positions for the LDE domain from the public coin; in the
    // // interactive version of the protocol, the verifier sends these query positions to the prover,
    // // and the prover responds with decommitments against these positions for trace and constraint
    // // composition polynomial evaluations.
    // let query_positions = public_coin
    //     .draw_integers(air.options().num_queries(), air.lde_domain_size())
    //     .map_err(|_| anyhow!("random coin error"))
    //     .unwrap();
}
