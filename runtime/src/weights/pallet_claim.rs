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
//! DATE: 2025-02-13, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `4d288594ad96`, CPU: `AMD Ryzen 7 7700 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-node
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
// /data/benchmark/HEADER-APACHE2
// --output
// /data/benchmark/runtime/src/weights/pallet_claim.rs
// --template
// /data/benchmark/node/zkv-deploy-weight-template.hbs
// --base-path=/tmp/tmp.90s8jUmEwW

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `pallet_claim` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> pallet_claim::WeightInfo for ZKVWeight<T> {
    /// Storage: `Claim::AirdropActive` (r:1 w:1)
    /// Proof: `Claim::AirdropActive` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Claim::PalletAccountId` (r:1 w:0)
    /// Proof: `Claim::PalletAccountId` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:0)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Claim::TotalClaimable` (r:1 w:1)
    /// Proof: `Claim::TotalClaimable` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    /// Storage: `Claim::Beneficiaries` (r:1000 w:1000)
    /// Proof: `Claim::Beneficiaries` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
    /// Storage: `Claim::AirdropId` (r:1 w:1)
    /// Proof: `Claim::AirdropId` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[0, 1000]`.
    fn begin_airdrop(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `198`
        //  Estimated: `3593 + n * (2531 ±0)`
        // Minimum execution time: 13_365_000 picoseconds.
        Weight::from_parts(11_196_770, 3593)
            // Standard Error: 3_638
            .saturating_add(Weight::from_parts(2_746_612, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
            .saturating_add(Weight::from_parts(0, 2531).saturating_mul(n.into()))
    }
    /// Storage: `Claim::AirdropActive` (r:1 w:0)
    /// Proof: `Claim::AirdropActive` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Claim::Beneficiaries` (r:1 w:1)
    /// Proof: `Claim::Beneficiaries` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
    /// Storage: `Claim::PalletAccountId` (r:1 w:0)
    /// Proof: `Claim::PalletAccountId` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Claim::TotalClaimable` (r:1 w:1)
    /// Proof: `Claim::TotalClaimable` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    fn claim() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1030`
        //  Estimated: `3593`
        // Minimum execution time: 54_062_000 picoseconds.
        Weight::from_parts(54_632_000, 3593)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: `Claim::AirdropActive` (r:1 w:0)
    /// Proof: `Claim::AirdropActive` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Claim::Beneficiaries` (r:1 w:1)
    /// Proof: `Claim::Beneficiaries` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
    /// Storage: `Claim::PalletAccountId` (r:1 w:0)
    /// Proof: `Claim::PalletAccountId` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:2 w:2)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Claim::TotalClaimable` (r:1 w:1)
    /// Proof: `Claim::TotalClaimable` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    fn claim_for() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1030`
        //  Estimated: `6196`
        // Minimum execution time: 53_941_000 picoseconds.
        Weight::from_parts(54_552_000, 6196)
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `Claim::AirdropActive` (r:1 w:0)
    /// Proof: `Claim::AirdropActive` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Claim::PalletAccountId` (r:1 w:0)
    /// Proof: `Claim::PalletAccountId` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:1 w:0)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Claim::TotalClaimable` (r:1 w:1)
    /// Proof: `Claim::TotalClaimable` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    /// Storage: `Claim::Beneficiaries` (r:999 w:999)
    /// Proof: `Claim::Beneficiaries` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[1, 1000]`.
    fn add_beneficiaries(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `249`
        //  Estimated: `3593 + n * (2531 ±0)`
        // Minimum execution time: 15_209_000 picoseconds.
        Weight::from_parts(15_489_000, 3593)
            // Standard Error: 3_894
            .saturating_add(Weight::from_parts(2_770_031, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
            .saturating_add(T::DbWeight::get().writes(1_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
            .saturating_add(Weight::from_parts(0, 2531).saturating_mul(n.into()))
    }
    /// Storage: `Claim::AirdropActive` (r:1 w:1)
    /// Proof: `Claim::AirdropActive` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
    /// Storage: `Claim::Beneficiaries` (r:999 w:999)
    /// Proof: `Claim::Beneficiaries` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
    /// Storage: `Claim::PalletAccountId` (r:1 w:0)
    /// Proof: `Claim::PalletAccountId` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
    /// Storage: `System::Account` (r:2 w:2)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Claim::AirdropId` (r:1 w:0)
    /// Proof: `Claim::AirdropId` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `Claim::TotalClaimable` (r:0 w:1)
    /// Proof: `Claim::TotalClaimable` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
    /// The range of component `n` is `[1, 1000]`.
    fn end_airdrop(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `386 + n * (61 ±0)`
        //  Estimated: `6196 + n * (2531 ±0)`
        // Minimum execution time: 47_870_000 picoseconds.
        Weight::from_parts(30_633_274, 6196)
            // Standard Error: 1_490
            .saturating_add(Weight::from_parts(1_156_094, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
            .saturating_add(Weight::from_parts(0, 2531).saturating_mul(n.into()))
    }
}
