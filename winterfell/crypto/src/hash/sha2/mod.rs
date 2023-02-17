// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use super::{sha, ByteDigest, ElementHasher, Hasher};
use core::marker::PhantomData;
use math::{FieldElement, StarkField};
use sha2_external::Digest;
use utils::collections::Vec;
use utils::ByteWriter;

// SHA3 WITH 256-BIT OUTPUT
// ================================================================================================

/// Implementation of the [Hasher](super::Hasher) trait for SHA3 hash function with 256-bit
/// output.
pub struct Sha2_256<B: StarkField, H: ShaHasherT>(PhantomData<B>, PhantomData<H>);

impl<B: StarkField, H: ShaHasherT> Hasher for Sha2_256<B, H> {
    type Digest = ByteDigest<32>;

    fn hash(bytes: &[u8]) -> Self::Digest {
        ByteDigest(H::digest(bytes).into())
    }

    fn merge(values: &[Self::Digest; 2]) -> Self::Digest {
        ByteDigest(H::digest(ByteDigest::digests_as_bytes(values)).into())
    }

    fn merge_with_int(seed: Self::Digest, value: u64) -> Self::Digest {
        let mut data = [0; 40];
        data[..32].copy_from_slice(&seed.0);
        data[32..].copy_from_slice(&value.to_le_bytes());
        ByteDigest(H::digest(&data).into())
    }
}

impl<B: StarkField, H: ShaHasherT> ElementHasher for Sha2_256<B, H> {
    type BaseField = B;

    fn hash_elements<E: FieldElement<BaseField = Self::BaseField>>(elements: &[E]) -> Self::Digest {
        if B::IS_CANONICAL {
            // when element's internal and canonical representations are the same, we can hash
            // element bytes directly
            let bytes = E::elements_as_bytes(elements);
            ByteDigest(H::digest(bytes).into())
        } else {
            let mut buf = Vec::new();
            // when elements' internal and canonical representations differ, we need to serialize
            // them before hashing
            buf.write(elements);
            ByteDigest(H::digest(&buf).into())
        }
    }
}

// SHA HASHER
// ================================================================================================

/// Wrapper around SHA2 hasher to implement [ByteWriter] trait for it.
pub trait ShaHasherT {
    // fn new() -> Self;
    fn digest(data: &[u8]) -> [u8; 32];
    // fn update(&mut self, data: impl AsRef<[u8]>);
    // fn finalize(self) -> [u8; 32];
}

pub struct DefaultSha2(sha2_external::Sha256);

impl ShaHasherT for DefaultSha2 {
    // fn new() -> Self {
    //     Self(sha2_external::Sha256::new())
    // }

    // fn update(&mut self, data: impl AsRef<[u8]>) {
    //     self.0.update(data);
    // }

    // fn finalize(self) -> [u8; 32] {
    //     self.0.finalize().into()
    // }

    fn digest(data: &[u8]) -> [u8; 32] {
        sha2_external::Sha256::digest(data).into()
    }
}

// struct ShaHasher<H: ShaHasherT>(H);

// impl<H: ShaHasherT> ShaHasher<H> {
//     pub fn new() -> Self {
//         Self(H::new())
//     }

//     pub fn finalize(self) -> [u8; 32] {
//         self.0.finalize()
//     }
// }

// impl<H: ShaHasherT> ByteWriter for ShaHasher<H> {
//     fn write_u8(&mut self, value: u8) {
//         self.0.update(&[value]);
//     }

//     fn write_u8_slice(&mut self, values: &[u8]) {
//         self.0.update(values);
//     }
// }
