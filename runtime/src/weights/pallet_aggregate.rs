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

//! Autogenerated weights for `pallet_aggregate`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2025-02-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `8f47de55dc97`, CPU: `AMD Ryzen 7 7700 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-relay
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-aggregate
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
// /data/benchmark/runtime/src/weights/pallet_aggregate.rs
// --template
// /data/benchmark/node/zkv-deploy-weight-template.hbs
// --base-path=/tmp/tmp.6Pn5NwdD4V

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_aggregate` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_aggregate::WeightInfo for ZKVWeight<T> {
    /// Storage: `Aggregate::Domains` (r:1 w:1)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(174530), added: 177005, mode: `MaxEncodedLen`)
    /// Storage: `TransactionPayment::NextFeeMultiplier` (r:1 w:0)
    /// Proof: `TransactionPayment::NextFeeMultiplier` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
    fn on_proof_verified() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1436`
        //  Estimated: `177995`
        // Minimum execution time: 56_296_000 picoseconds.
        Weight::from_parts(57_588_000, 177995)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `Aggregate::Domains` (r:1 w:1)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(209429), added: 211904, mode: `MaxEncodedLen`)
    /// Storage: `Aggregate::Published` (r:1 w:1)
    /// Proof: `Aggregate::Published` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[1, 128]`.
    fn aggregate(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `364 + n * (96 ±0)`
        //  Estimated: `212894 + n * (96 ±0)`
        // Minimum execution time: 68_959_000 picoseconds.
        Weight::from_parts(13_992_215, 212894)
            // Standard Error: 50_692
            .saturating_add(Weight::from_parts(54_699_385, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 96).saturating_mul(n.into()))
    }
    /// Storage: `Aggregate::Domains` (r:1 w:0)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(209429), added: 211904, mode: `MaxEncodedLen`)
    fn aggregate_on_invalid_domain() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `109`
        //  Estimated: `212894`
        // Minimum execution time: 5_961_000 picoseconds.
        Weight::from_parts(6_272_000, 212894)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    /// Storage: `Aggregate::Domains` (r:1 w:0)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(209429), added: 211904, mode: `MaxEncodedLen`)
    fn aggregate_on_invalid_id() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `268`
        //  Estimated: `212894`
        // Minimum execution time: 7_434_000 picoseconds.
        Weight::from_parts(7_775_000, 212894)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    /// Storage: `Aggregate::NextDomainId` (r:1 w:1)
    /// Proof: `Aggregate::NextDomainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
    /// Storage: `Aggregate::Domains` (r:0 w:1)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(209429), added: 211904, mode: `MaxEncodedLen`)
    fn register_domain() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `109`
        //  Estimated: `3604`
        // Minimum execution time: 41_618_000 picoseconds.
        Weight::from_parts(42_410_000, 3604)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: `Aggregate::Domains` (r:1 w:1)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(209429), added: 211904, mode: `MaxEncodedLen`)
    fn hold_domain() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `268`
        //  Estimated: `212894`
        // Minimum execution time: 9_758_000 picoseconds.
        Weight::from_parts(10_249_000, 212894)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Aggregate::Domains` (r:1 w:1)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(209429), added: 211904, mode: `MaxEncodedLen`)
    fn unregister_domain() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `268`
        //  Estimated: `212894`
        // Minimum execution time: 10_039_000 picoseconds.
        Weight::from_parts(10_390_000, 212894)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Aggregate::Domains` (r:1 w:1)
    /// Proof: `Aggregate::Domains` (`max_values`: None, `max_size`: Some(209429), added: 211904, mode: `MaxEncodedLen`)
    fn set_delivery_price() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `268`
        //  Estimated: `212894`
        // Minimum execution time: 7_755_000 picoseconds.
        Weight::from_parts(8_065_000, 212894)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
}
