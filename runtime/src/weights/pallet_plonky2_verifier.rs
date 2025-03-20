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

//! Autogenerated weights for `pallet_plonky2_verifier`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2025-03-19, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `c111b50472b8`, CPU: `AMD Ryzen 7 7700 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-relay
// benchmark
// pallet
// --runtime
// /app/zkv_runtime.compact.compressed.wasm
// --genesis-builder=runtime
// --pallet
// pallet-plonky2-verifier
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
// /data/benchmark/runtime/src/weights/pallet_plonky2_verifier.rs
// --template
// /data/benchmark/relay-node/benchmarks/zkv-deploy-weight-template.hbs
// --base-path=/tmp/tmp.QWIqJeGMWh

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_plonky2_verifier` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_plonky2_verifier::WeightInfo for ZKVWeight<T> {
    fn verify_proof() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 43_388_166_000 picoseconds.
        Weight::from_parts(43_437_809_000, 0)
    }
    /// Storage: `SettlementPlonky2Pallet::Vks` (r:1 w:0)
    /// Proof: `SettlementPlonky2Pallet::Vks` (`max_values`: None, `max_size`: Some(50045), added: 52520, mode: `MaxEncodedLen`)
    fn get_vk() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `2812`
        //  Estimated: `53510`
        // Minimum execution time: 6_012_000 picoseconds.
        Weight::from_parts(6_201_000, 53510)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    fn validate_vk() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 9_087_000 picoseconds.
        Weight::from_parts(9_347_000, 0)
    }
    fn compute_statement_hash() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 10_700_000 picoseconds.
        Weight::from_parts(10_900_000, 0)
    }
    /// Storage: `SettlementPlonky2Pallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementPlonky2Pallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `SettlementPlonky2Pallet::Tickets` (r:1 w:1)
    /// Proof: `SettlementPlonky2Pallet::Tickets` (`max_values`: None, `max_size`: Some(96), added: 2571, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
    /// Storage: `SettlementPlonky2Pallet::Vks` (r:1 w:1)
    /// Proof: `SettlementPlonky2Pallet::Vks` (`max_values`: None, `max_size`: Some(50045), added: 52520, mode: `MaxEncodedLen`)
    fn register_vk() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `4`
        //  Estimated: `53510`
        // Minimum execution time: 65_173_000 picoseconds.
        Weight::from_parts(65_934_000, 53510)
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: `SettlementPlonky2Pallet::Tickets` (r:1 w:1)
    /// Proof: `SettlementPlonky2Pallet::Tickets` (`max_values`: None, `max_size`: Some(96), added: 2571, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
    /// Storage: `SettlementPlonky2Pallet::Vks` (r:1 w:1)
    /// Proof: `SettlementPlonky2Pallet::Vks` (`max_values`: None, `max_size`: Some(50045), added: 52520, mode: `MaxEncodedLen`)
    fn unregister_vk() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `2977`
        //  Estimated: `53510`
        // Minimum execution time: 42_369_000 picoseconds.
        Weight::from_parts(43_191_000, 53510)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
}
