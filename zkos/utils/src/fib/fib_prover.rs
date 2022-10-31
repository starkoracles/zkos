// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use winter_air::ProofOptions;
use winter_math::{fields::f64::BaseElement, FieldElement};
use winter_prover::Prover;
use winter_prover::Trace;
use winter_prover::TraceTable;

use super::fib_air::FibAir;

// FIBONACCI PROVER
// ================================================================================================

pub struct FibProver {
    options: ProofOptions,
}

const TRACE_WIDTH: usize = 2;

impl FibProver {
    pub fn new(options: ProofOptions) -> Self {
        Self { options }
    }

    /// Builds an execution trace for computing a Fibonacci sequence of the specified length such
    /// that each row advances the sequence by 2 terms.
    pub fn build_trace(&self, sequence_length: usize) -> TraceTable<BaseElement> {
        assert!(
            sequence_length.is_power_of_two(),
            "sequence length must be a power of 2"
        );

        let mut trace = TraceTable::new(TRACE_WIDTH, sequence_length / 2);
        trace.fill(
            |state| {
                state[0] = BaseElement::ONE;
                state[1] = BaseElement::ONE;
            },
            |_, state| {
                state[0] += state[1];
                state[1] += state[0];
            },
        );

        trace
    }
}

impl Prover for FibProver {
    type BaseField = BaseElement;
    type Air = FibAir;
    type Trace = TraceTable<BaseElement>;

    fn get_pub_inputs(&self, trace: &Self::Trace) -> BaseElement {
        let last_step = trace.length() - 1;
        trace.get(1, last_step)
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}
