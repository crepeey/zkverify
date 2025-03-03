// Copyright 2024, Horizen Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_risc0_verifier_verify_proof`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2025-02-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `f4a2cbe83be5`, CPU: `AMD Ryzen 7 7700 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-node
// benchmark
// pallet
// --runtime
// /app/zkv_runtime.compact.compressed.wasm
// --genesis-builder=runtime
// --pallet
// pallet-risc0-verifier-verify-proof
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --heap-pages=4096
// --header
// /data/benchmark/HEADER-APACHE2
// --output
// /data/benchmark/runtime/src/weights/pallet_risc0_verifier_verify_proof.rs
// --template
// /data/benchmark/node/zkv-deploy-weight-template.hbs
// --base-path=/tmp/tmp.d6M4oX3Xjs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_risc0_verifier_verify_proof` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_risc0_verifier::WeightInfoVerifyProof for ZKVWeight<T> {
    fn verify_proof_succinct() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 31_929_585_000 picoseconds.
        Weight::from_parts(31_998_655_000, 0)
    }
    fn verify_proof_segment_poseidon2_16() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 30_784_605_000 picoseconds.
        Weight::from_parts(30_827_807_000, 0)
    }
    fn verify_proof_segment_sha_256_16() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 7_118_807_000 picoseconds.
        Weight::from_parts(7_131_591_000, 0)
    }
    fn verify_proof_segment_poseidon2_17() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 33_321_128_000 picoseconds.
        Weight::from_parts(33_386_300_000, 0)
    }
    fn verify_proof_segment_sha_256_17() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 7_005_855_000 picoseconds.
        Weight::from_parts(7_025_973_000, 0)
    }
    fn verify_proof_segment_poseidon2_18() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 35_282_031_000 picoseconds.
        Weight::from_parts(35_331_172_000, 0)
    }
    fn verify_proof_segment_sha_256_18() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 7_385_408_000 picoseconds.
        Weight::from_parts(7_410_936_000, 0)
    }
    fn verify_proof_segment_poseidon2_19() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 37_423_962_000 picoseconds.
        Weight::from_parts(37_485_258_000, 0)
    }
    fn verify_proof_segment_sha_256_19() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 7_881_901_000 picoseconds.
        Weight::from_parts(7_902_679_000, 0)
    }
    fn verify_proof_segment_poseidon2_20() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 39_937_973_000 picoseconds.
        Weight::from_parts(39_968_701_000, 0)
    }
    fn verify_proof_segment_sha_256_20() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 8_588_527_000 picoseconds.
        Weight::from_parts(8_613_374_000, 0)
    }
    fn verify_proof_segment_poseidon2_21() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 42_611_754_000 picoseconds.
        Weight::from_parts(42_733_082_000, 0)
    }
    fn verify_proof_segment_sha_256_21() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 8_522_493_000 picoseconds.
        Weight::from_parts(8_544_494_000, 0)
    }
}
