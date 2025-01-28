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

//! Autogenerated weights for `pallet_claim`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 42.0.0
//! DATE: 2024-12-23, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `LAPTOP-5V1NHBSA`, CPU: `11th Gen Intel(R) Core(TM) i7-11850H @ 2.50GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /home/danielecker/hl-crypto/zkVerify/target/production/zkv-node
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet-claim
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --heap-pages=4096
// --header
// /home/danielecker/hl-crypto/zkVerify/HEADER-APACHE2
// --output
// pallets/claim/src/weight.rs
// --template
// /home/danielecker/hl-crypto/zkVerify/node/zkv-pallets-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_claim`.
pub trait WeightInfo {
    fn begin_airdrop(n: u32, ) -> Weight;
    fn claim() -> Weight;
    fn claim_for() -> Weight;
    fn add_beneficiaries(n: u32, ) -> Weight;
    fn end_airdrop(n: u32, ) -> Weight;
}

// For backwards compatibility and tests.
impl WeightInfo for () {
    /// Storage: `Claim::AirdropActive` (r:1 w:1)
    /// Proof: `Claim::AirdropActive` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:0)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Claim::TotalClaimable` (r:1 w:1)
    /// Proof: `Claim::TotalClaimable` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    /// Storage: `Claim::Beneficiaries` (r:999 w:999)
    /// Proof: `Claim::Beneficiaries` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
    /// Storage: `Claim::AirdropId` (r:1 w:1)
    /// Proof: `Claim::AirdropId` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[1, 1000]`.
    fn begin_airdrop(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `146`
        //  Estimated: `3593 + n * (2531 ±0)`
        // Minimum execution time: 14_518_000 picoseconds.
        Weight::from_parts(15_151_000, 3593)
            // Standard Error: 29_325
            .saturating_add(Weight::from_parts(2_537_724, 0).saturating_mul(n.into()))
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(n.into())))
            .saturating_add(RocksDbWeight::get().writes(3_u64))
            .saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(n.into())))
            .saturating_add(Weight::from_parts(0, 2531).saturating_mul(n.into()))
    }
    /// Storage: `Claim::AirdropActive` (r:1 w:0)
    /// Proof: `Claim::AirdropActive` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Claim::Beneficiaries` (r:1 w:1)
    /// Proof: `Claim::Beneficiaries` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Claim::TotalClaimable` (r:1 w:1)
    /// Proof: `Claim::TotalClaimable` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    fn claim() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `978`
        //  Estimated: `3593`
        // Minimum execution time: 47_473_000 picoseconds.
        Weight::from_parts(48_934_000, 3593)
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().writes(3_u64))
    }
    /// Storage: `Claim::AirdropActive` (r:1 w:0)
    /// Proof: `Claim::AirdropActive` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Claim::Beneficiaries` (r:1 w:1)
    /// Proof: `Claim::Beneficiaries` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:2 w:2)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Claim::TotalClaimable` (r:1 w:1)
    /// Proof: `Claim::TotalClaimable` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    fn claim_for() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `978`
        //  Estimated: `6196`
        // Minimum execution time: 47_919_000 picoseconds.
        Weight::from_parts(50_430_000, 6196)
            .saturating_add(RocksDbWeight::get().reads(5_u64))
            .saturating_add(RocksDbWeight::get().writes(4_u64))
    }
    /// Storage: `Claim::AirdropActive` (r:1 w:0)
    /// Proof: `Claim::AirdropActive` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:0)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Claim::TotalClaimable` (r:1 w:1)
    /// Proof: `Claim::TotalClaimable` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    /// Storage: `Claim::Beneficiaries` (r:999 w:999)
    /// Proof: `Claim::Beneficiaries` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[1, 1000]`.
    fn add_beneficiaries(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `233 + n * (61 ±0)`
        //  Estimated: `3593 + n * (2531 ±0)`
        // Minimum execution time: 14_461_000 picoseconds.
        Weight::from_parts(14_839_000, 3593)
            // Standard Error: 6_677
            .saturating_add(Weight::from_parts(3_990_897, 0).saturating_mul(n.into()))
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(n.into())))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
            .saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(n.into())))
            .saturating_add(Weight::from_parts(0, 2531).saturating_mul(n.into()))
    }
    /// Storage: `Claim::AirdropActive` (r:1 w:1)
    /// Proof: `Claim::AirdropActive` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Claim::Beneficiaries` (r:999 w:999)
    /// Proof: `Claim::Beneficiaries` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:2 w:2)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Claim::AirdropId` (r:1 w:0)
    /// Proof: `Claim::AirdropId` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Claim::TotalClaimable` (r:0 w:1)
    /// Proof: `Claim::TotalClaimable` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[1, 1000]`.
    fn end_airdrop(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `333 + n * (61 ±0)`
        //  Estimated: `6196 + n * (2531 ±0)`
        // Minimum execution time: 32_447_000 picoseconds.
        Weight::from_parts(30_536_884, 6196)
            // Standard Error: 2_385
            .saturating_add(Weight::from_parts(917_174, 0).saturating_mul(n.into()))
            .saturating_add(RocksDbWeight::get().reads(4_u64))
            .saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(n.into())))
            .saturating_add(RocksDbWeight::get().writes(4_u64))
            .saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(n.into())))
            .saturating_add(Weight::from_parts(0, 2531).saturating_mul(n.into()))
    }
}
