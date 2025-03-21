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

//! Autogenerated weights for `pallet_proofofsql_verifier`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2025-02-13, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `miklap`, CPU: `11th Gen Intel(R) Core(TM) i7-11850H @ 2.50GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/zkv-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-proofofsql-verifier
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --heap-pages=4096
// --header
// /home/mdamico/devel/zkVerify/HEADER-APACHE2
// --output
// verifiers/proofofsql/src/weight.rs
// --template
// /home/mdamico/devel/zkVerify/node/zkv-pallets-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_proofofsql_verifier`.
pub trait WeightInfo {
    fn verify_proof() -> Weight;
    fn get_vk() -> Weight;
    fn validate_vk() -> Weight;
    fn compute_statement_hash() -> Weight;
    fn register_vk() -> Weight;
    fn unregister_vk() -> Weight;
}

// For backwards compatibility and tests.
impl WeightInfo for () {
    fn verify_proof() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_102_327_256_000 picoseconds.
        Weight::from_parts(1_129_200_581_000, 0)
    }
    /// Storage: `SettlementProofOfSqlPallet::Vks` (r:1 w:0)
    /// Proof: `SettlementProofOfSqlPallet::Vks` (`max_values`: None, `max_size`: Some(26980), added: 29455, mode: `MaxEncodedLen`)
    fn get_vk() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `27015`
        //  Estimated: `30445`
        // Minimum execution time: 14_593_000 picoseconds.
        Weight::from_parts(14_951_000, 30445)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
    }
    fn validate_vk() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 161_000 picoseconds.
        Weight::from_parts(181_000, 0)
    }
    fn compute_statement_hash() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 88_660_000 picoseconds.
        Weight::from_parts(89_071_000, 0)
    }
    /// Storage: `SettlementProofOfSqlPallet::Disabled` (r:1 w:0)
    /// Proof: `SettlementProofOfSqlPallet::Disabled` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `SettlementProofOfSqlPallet::Tickets` (r:1 w:1)
    /// Proof: `SettlementProofOfSqlPallet::Tickets` (`max_values`: None, `max_size`: Some(97), added: 2572, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
    /// Storage: `SettlementProofOfSqlPallet::Vks` (r:1 w:1)
    /// Proof: `SettlementProofOfSqlPallet::Vks` (`max_values`: None, `max_size`: Some(26980), added: 29455, mode: `MaxEncodedLen`)
    fn register_vk() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `6`
        //  Estimated: `30445`
        // Minimum execution time: 124_627_000 picoseconds.
        Weight::from_parts(127_822_000, 30445)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(3_u64))
    }
    /// Storage: `SettlementProofOfSqlPallet::Tickets` (r:1 w:1)
    /// Proof: `SettlementProofOfSqlPallet::Tickets` (`max_values`: None, `max_size`: Some(97), added: 2572, mode: `MaxEncodedLen`)
    /// Storage: `Balances::Holds` (r:1 w:1)
    /// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(121), added: 2596, mode: `MaxEncodedLen`)
    /// Storage: `SettlementProofOfSqlPallet::Vks` (r:1 w:1)
    /// Proof: `SettlementProofOfSqlPallet::Vks` (`max_values`: None, `max_size`: Some(26980), added: 29455, mode: `MaxEncodedLen`)
    fn unregister_vk() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `27193`
        //  Estimated: `30445`
        // Minimum execution time: 54_067_000 picoseconds.
        Weight::from_parts(55_968_000, 30445)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(3_u64))
    }
}