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

//! Autogenerated weights for `xcm::pallet_xcm_benchmarks_fungible`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2025-03-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `a5161f9965bc`, CPU: `AMD Ryzen 7 7700 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-relay
// benchmark
// pallet
// --chain
// dev
// --pallet
// xcm::pallet-xcm-benchmarks-fungible
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
// /data/benchmark/runtime/src/weights/xcm/pallet_xcm_benchmarks_fungible.rs
// --template
// /data/benchmark/relay-node/benchmarks/zkv-deploy-weight-template-xcm.hbs
// --base-path=/tmp/tmp.1rbLszjw13

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `xcm::pallet_xcm_benchmarks_fungible` using the zkVerify node and recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo<T> {
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    pub(crate) fn withdraw_asset() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `101`
        //  Estimated: `3593`
        // Minimum execution time: 28_985_000 picoseconds.
        Weight::from_parts(29_425_000, 3593)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `System::Account` (r:2 w:2)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    pub(crate) fn transfer_asset() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `101`
        //  Estimated: `6196`
        // Minimum execution time: 41_037_000 picoseconds.
        Weight::from_parts(41_708_000, 6196)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `System::Account` (r:3 w:3)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:0)
    /// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `XcmPallet::SupportedVersion` (r:1 w:0)
    /// Proof: `XcmPallet::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
    /// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueueHeads` (r:1 w:1)
    /// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
    pub(crate) fn transfer_reserve_asset() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `246`
        //  Estimated: `8799`
        // Minimum execution time: 95_850_000 picoseconds.
        Weight::from_parts(97_663_000, 8799)
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(5_u64))
    }
    /// Storage: `Benchmark::Override` (r:0 w:0)
    /// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
    pub(crate) fn reserve_asset_deposited() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
        Weight::from_parts(18_446_744_073_709_551_000, 0)
    }
    /// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:0)
    /// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `XcmPallet::SupportedVersion` (r:1 w:0)
    /// Proof: `XcmPallet::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
    /// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `System::Account` (r:2 w:2)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Dmp::DownwardMessageQueueHeads` (r:1 w:1)
    /// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
    pub(crate) fn initiate_reserve_withdraw() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `246`
        //  Estimated: `6196`
        // Minimum execution time: 62_337_000 picoseconds.
        Weight::from_parts(63_970_000, 6196)
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    pub(crate) fn receive_teleported_asset() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `140`
        //  Estimated: `3593`
        // Minimum execution time: 30_016_000 picoseconds.
        Weight::from_parts(30_708_000, 3593)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    pub(crate) fn deposit_asset() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `3593`
        // Minimum execution time: 20_679_000 picoseconds.
        Weight::from_parts(21_240_000, 3593)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:0)
    /// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `XcmPallet::SupportedVersion` (r:1 w:0)
    /// Proof: `XcmPallet::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
    /// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Dmp::DownwardMessageQueueHeads` (r:1 w:1)
    /// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
    pub(crate) fn deposit_reserve_asset() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `42`
        //  Estimated: `3593`
        // Minimum execution time: 54_603_000 picoseconds.
        Weight::from_parts(55_704_000, 3593)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: `System::Account` (r:1 w:1)
    /// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
    /// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:0)
    /// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `XcmPallet::SupportedVersion` (r:1 w:0)
    /// Proof: `XcmPallet::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
    /// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueueHeads` (r:1 w:1)
    /// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
    pub(crate) fn initiate_teleport() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `94`
        //  Estimated: `3593`
        // Minimum execution time: 42_600_000 picoseconds.
        Weight::from_parts(43_341_000, 3593)
            .saturating_add(T::DbWeight::get().reads(5_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
}
