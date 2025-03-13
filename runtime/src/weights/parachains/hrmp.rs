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

//! Autogenerated weights for `crate::parachains::hrmp`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2025-02-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `2e8537906dc6`, CPU: `AMD Ryzen 7 7700 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-relay
// benchmark
// pallet
// --chain
// dev
// --pallet
// crate::parachains::hrmp
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
// /data/benchmark/runtime/src/weights/parachains/hrmp.rs
// --template
// /data/benchmark/relay-node/benchmarks/zkv-deploy-weight-template.hbs
// --base-path=/tmp/tmp.1rbLszjw13

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `crate::parachains::hrmp` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> crate::parachains::hrmp::WeightInfo for ZKVWeight<T> {
    /// Storage: `Paras::ParaLifecycles` (r:1 w:0)
    /// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequests` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpChannels` (r:1 w:0)
    /// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpEgressChannelsIndex` (r:1 w:0)
    /// Proof: `Hrmp::HrmpEgressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequestCount` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequestsList` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `XcmPallet::SupportedVersion` (r:1 w:0)
    /// Proof: `XcmPallet::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
    /// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueueHeads` (r:1 w:1)
    /// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn hrmp_init_open_channel() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `350`
        //  Estimated: `3815`
        // Minimum execution time: 38_152_000 picoseconds.
        Weight::from_parts(39_444_000, 3815)
            .saturating_add(T::DbWeight::get().reads(9_u64))
            .saturating_add(T::DbWeight::get().writes(5_u64))
    }
    /// Storage: `Hrmp::HrmpOpenChannelRequests` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpIngressChannelsIndex` (r:1 w:0)
    /// Proof: `Hrmp::HrmpIngressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpAcceptedChannelRequestCount` (r:1 w:1)
    /// Proof: `Hrmp::HrmpAcceptedChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `XcmPallet::SupportedVersion` (r:1 w:0)
    /// Proof: `XcmPallet::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
    /// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueueHeads` (r:1 w:1)
    /// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn hrmp_accept_open_channel() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `340`
        //  Estimated: `3805`
        // Minimum execution time: 38_312_000 picoseconds.
        Weight::from_parts(39_183_000, 3805)
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `Hrmp::HrmpChannels` (r:1 w:0)
    /// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpCloseChannelRequests` (r:1 w:1)
    /// Proof: `Hrmp::HrmpCloseChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpCloseChannelRequestsList` (r:1 w:1)
    /// Proof: `Hrmp::HrmpCloseChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `XcmPallet::SupportedVersion` (r:1 w:0)
    /// Proof: `XcmPallet::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
    /// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueueHeads` (r:1 w:1)
    /// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn hrmp_close_channel() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `453`
        //  Estimated: `3918`
        // Minimum execution time: 39_514_000 picoseconds.
        Weight::from_parts(40_777_000, 3918)
            .saturating_add(T::DbWeight::get().reads(6_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: `Hrmp::HrmpIngressChannelsIndex` (r:128 w:128)
    /// Proof: `Hrmp::HrmpIngressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpEgressChannelsIndex` (r:128 w:128)
    /// Proof: `Hrmp::HrmpEgressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpChannels` (r:254 w:254)
    /// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpAcceptedChannelRequestCount` (r:0 w:1)
    /// Proof: `Hrmp::HrmpAcceptedChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpChannelContents` (r:0 w:254)
    /// Proof: `Hrmp::HrmpChannelContents` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequestCount` (r:0 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// The range of component `i` is `[0, 127]`.
    /// The range of component `e` is `[0, 127]`.
    fn force_clean_hrmp(i: u32, e: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `197 + e * (100 ±0) + i * (100 ±0)`
        //  Estimated: `3659 + e * (2575 ±0) + i * (2575 ±0)`
        // Minimum execution time: 1_388_795_000 picoseconds.
        Weight::from_parts(1_393_955_000, 3659)
            // Standard Error: 124_314
            .saturating_add(Weight::from_parts(3_965_248, 0).saturating_mul(i.into()))
            // Standard Error: 124_314
            .saturating_add(Weight::from_parts(4_053_190, 0).saturating_mul(e.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(i.into())))
            .saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(e.into())))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(i.into())))
            .saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(e.into())))
            .saturating_add(Weight::from_parts(0, 2575).saturating_mul(e.into()))
            .saturating_add(Weight::from_parts(0, 2575).saturating_mul(i.into()))
    }
    /// Storage: `Hrmp::HrmpOpenChannelRequestsList` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequests` (r:128 w:128)
    /// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Paras::ParaLifecycles` (r:256 w:0)
    /// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpIngressChannelsIndex` (r:128 w:128)
    /// Proof: `Hrmp::HrmpIngressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpEgressChannelsIndex` (r:128 w:128)
    /// Proof: `Hrmp::HrmpEgressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequestCount` (r:128 w:128)
    /// Proof: `Hrmp::HrmpOpenChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpAcceptedChannelRequestCount` (r:128 w:128)
    /// Proof: `Hrmp::HrmpAcceptedChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpChannels` (r:0 w:128)
    /// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// The range of component `c` is `[0, 128]`.
    fn force_process_hrmp_open(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `425 + c * (136 ±0)`
        //  Estimated: `1880 + c * (5086 ±0)`
        // Minimum execution time: 7_564_000 picoseconds.
        Weight::from_parts(2_357_190, 1880)
            // Standard Error: 14_628
            .saturating_add(Weight::from_parts(24_363_467, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(c.into())))
            .saturating_add(T::DbWeight::get().writes(1_u64))
            .saturating_add(T::DbWeight::get().writes((6_u64).saturating_mul(c.into())))
            .saturating_add(Weight::from_parts(0, 5086).saturating_mul(c.into()))
    }
    /// Storage: `Hrmp::HrmpCloseChannelRequestsList` (r:1 w:1)
    /// Proof: `Hrmp::HrmpCloseChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpChannels` (r:128 w:128)
    /// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpEgressChannelsIndex` (r:128 w:128)
    /// Proof: `Hrmp::HrmpEgressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpIngressChannelsIndex` (r:128 w:128)
    /// Proof: `Hrmp::HrmpIngressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpCloseChannelRequests` (r:0 w:128)
    /// Proof: `Hrmp::HrmpCloseChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpChannelContents` (r:0 w:128)
    /// Proof: `Hrmp::HrmpChannelContents` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// The range of component `c` is `[0, 128]`.
    fn force_process_hrmp_close(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `268 + c * (124 ±0)`
        //  Estimated: `1728 + c * (2600 ±0)`
        // Minimum execution time: 6_762_000 picoseconds.
        Weight::from_parts(3_601_911, 1728)
            // Standard Error: 8_676
            .saturating_add(Weight::from_parts(14_774_858, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(c.into())))
            .saturating_add(T::DbWeight::get().writes(1_u64))
            .saturating_add(T::DbWeight::get().writes((5_u64).saturating_mul(c.into())))
            .saturating_add(Weight::from_parts(0, 2600).saturating_mul(c.into()))
    }
    /// Storage: `Hrmp::HrmpOpenChannelRequestsList` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequests` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequestCount` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// The range of component `c` is `[0, 128]`.
    fn hrmp_cancel_open_request(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `959 + c * (13 ±0)`
        //  Estimated: `4228 + c * (15 ±0)`
        // Minimum execution time: 18_274_000 picoseconds.
        Weight::from_parts(28_995_284, 4228)
            // Standard Error: 2_250
            .saturating_add(Weight::from_parts(90_603, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 15).saturating_mul(c.into()))
    }
    /// Storage: `Hrmp::HrmpOpenChannelRequestsList` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequests` (r:128 w:128)
    /// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// The range of component `c` is `[0, 128]`.
    fn clean_open_channel_requests(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `176 + c * (63 ±0)`
        //  Estimated: `1655 + c * (2538 ±0)`
        // Minimum execution time: 5_410_000 picoseconds.
        Weight::from_parts(6_412_826, 1655)
            // Standard Error: 8_229
            .saturating_add(Weight::from_parts(4_318_993, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
            .saturating_add(T::DbWeight::get().writes(1_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
            .saturating_add(Weight::from_parts(0, 2538).saturating_mul(c.into()))
    }
    /// Storage: `Hrmp::HrmpOpenChannelRequests` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequestsList` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequestCount` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Paras::ParaLifecycles` (r:1 w:0)
    /// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpChannels` (r:1 w:0)
    /// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpEgressChannelsIndex` (r:1 w:0)
    /// Proof: `Hrmp::HrmpEgressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `XcmPallet::SupportedVersion` (r:2 w:0)
    /// Proof: `XcmPallet::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueues` (r:2 w:2)
    /// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueueHeads` (r:2 w:2)
    /// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpIngressChannelsIndex` (r:1 w:0)
    /// Proof: `Hrmp::HrmpIngressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpAcceptedChannelRequestCount` (r:1 w:1)
    /// Proof: `Hrmp::HrmpAcceptedChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// The range of component `c` is `[0, 1]`.
    fn force_open_hrmp_channel(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `350 + c * (235 ±0)`
        //  Estimated: `6290 + c * (235 ±0)`
        // Minimum execution time: 58_130_000 picoseconds.
        Weight::from_parts(60_873_722, 6290)
            // Standard Error: 148_319
            .saturating_add(Weight::from_parts(14_732_177, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(14_u64))
            .saturating_add(T::DbWeight::get().writes(8_u64))
            .saturating_add(Weight::from_parts(0, 235).saturating_mul(c.into()))
    }
    /// Storage: `Paras::ParaLifecycles` (r:1 w:0)
    /// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequests` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpChannels` (r:1 w:0)
    /// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpEgressChannelsIndex` (r:1 w:0)
    /// Proof: `Hrmp::HrmpEgressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequestCount` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequestsList` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `XcmPallet::SupportedVersion` (r:2 w:0)
    /// Proof: `XcmPallet::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueues` (r:2 w:2)
    /// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueueHeads` (r:2 w:2)
    /// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpIngressChannelsIndex` (r:1 w:0)
    /// Proof: `Hrmp::HrmpIngressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpAcceptedChannelRequestCount` (r:1 w:1)
    /// Proof: `Hrmp::HrmpAcceptedChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn establish_system_channel() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `350`
        //  Estimated: `6290`
        // Minimum execution time: 58_169_000 picoseconds.
        Weight::from_parts(59_582_000, 6290)
            .saturating_add(T::DbWeight::get().reads(14_u64))
            .saturating_add(T::DbWeight::get().writes(8_u64))
    }
    /// Storage: `Hrmp::HrmpChannels` (r:1 w:1)
    /// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn poke_channel_deposits() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `196`
        //  Estimated: `3661`
        // Minimum execution time: 12_213_000 picoseconds.
        Weight::from_parts(12_704_000, 3661)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: `Paras::ParaLifecycles` (r:2 w:0)
    /// Proof: `Paras::ParaLifecycles` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequests` (r:2 w:2)
    /// Proof: `Hrmp::HrmpOpenChannelRequests` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpChannels` (r:2 w:0)
    /// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpEgressChannelsIndex` (r:2 w:0)
    /// Proof: `Hrmp::HrmpEgressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequestCount` (r:2 w:2)
    /// Proof: `Hrmp::HrmpOpenChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpOpenChannelRequestsList` (r:1 w:1)
    /// Proof: `Hrmp::HrmpOpenChannelRequestsList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `XcmPallet::SupportedVersion` (r:2 w:0)
    /// Proof: `XcmPallet::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueues` (r:2 w:2)
    /// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Dmp::DownwardMessageQueueHeads` (r:2 w:2)
    /// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpIngressChannelsIndex` (r:2 w:0)
    /// Proof: `Hrmp::HrmpIngressChannelsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Hrmp::HrmpAcceptedChannelRequestCount` (r:2 w:2)
    /// Proof: `Hrmp::HrmpAcceptedChannelRequestCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
    fn establish_channel_with_system() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `350`
        //  Estimated: `6290`
        // Minimum execution time: 102_202_000 picoseconds.
        Weight::from_parts(105_188_000, 6290)
            .saturating_add(T::DbWeight::get().reads(21_u64))
            .saturating_add(T::DbWeight::get().writes(11_u64))
    }
}
