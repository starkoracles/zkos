// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use winter_air::ProofOptions;
use winter_math::{
    fields::f64_risc0::{AccelBaseElementRisc0, NativeMontMul},
    FieldElement,
};
use winter_prover::Prover;
use winter_prover::Trace;
use winter_prover::TraceTable;

use super::fib_air::FibAir;

// FIBONACCI PROVER
// ================================================================================================

pub struct FibProver<A> {
    options: ProofOptions,
    _marker: std::marker::PhantomData<A>,
}

const TRACE_WIDTH: usize = 2;

impl<A: NativeMontMul> FibProver<A> {
    pub fn new(options: ProofOptions) -> Self {
        Self {
            options,
            _marker: std::marker::PhantomData,
        }
    }

    /// Builds an execution trace for computing a Fibonacci sequence of the specified length such
    /// that each row advances the sequence by 2 terms.
    pub fn build_trace(&self, sequence_length: usize) -> TraceTable<AccelBaseElementRisc0<A>> {
        assert!(
            sequence_length.is_power_of_two(),
            "sequence length must be a power of 2"
        );

        let mut trace = TraceTable::new(TRACE_WIDTH, sequence_length / 2);
        trace.fill(
            |state| {
                state[0] = AccelBaseElementRisc0::ONE;
                state[1] = AccelBaseElementRisc0::ONE;
            },
            |_, state| {
                state[0] += state[1];
                state[1] += state[0];
            },
        );

        trace
    }
}

impl<A: NativeMontMul> Prover for FibProver<A> {
    type BaseField = AccelBaseElementRisc0<A>;
    type Air = FibAir<A>;
    type Trace = TraceTable<AccelBaseElementRisc0<A>>;

    fn get_pub_inputs(&self, trace: &Self::Trace) -> AccelBaseElementRisc0<A> {
        let last_step = trace.length() - 1;
        trace.get(1, last_step)
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}
