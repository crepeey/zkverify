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

//! Autogenerated weights for `frame_election_provider_support`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2025-03-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `5b1a7e65df16`, CPU: `AMD Ryzen 7 7700 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-relay
// benchmark
// pallet
// --chain
// dev
// --pallet
// frame-election-provider-support
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
// /data/benchmark/runtime/src/weights/frame_election_provider_support.rs
// --template
// /data/benchmark/relay-node/benchmarks/zkv-deploy-weight-template.hbs
// --base-path=/tmp/tmp.1rbLszjw13

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `frame_election_provider_support` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> frame_election_provider_support::WeightInfo for ZKVWeight<T> {
    /// The range of component `v` is `[1000, 2000]`.
    /// The range of component `t` is `[500, 1000]`.
    /// The range of component `d` is `[5, 16]`.
    fn phragmen(v: u32, _t: u32, d: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 6_446_521_000 picoseconds.
        Weight::from_parts(6_477_710_000, 0)
            // Standard Error: 125_611
            .saturating_add(Weight::from_parts(5_357_054, 0).saturating_mul(v.into()))
            // Standard Error: 12_842_078
            .saturating_add(Weight::from_parts(1_320_008_145, 0).saturating_mul(d.into()))
    }
    /// The range of component `v` is `[1000, 2000]`.
    /// The range of component `t` is `[500, 1000]`.
    /// The range of component `d` is `[5, 16]`.
    fn phragmms(v: u32, _t: u32, d: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 4_731_713_000 picoseconds.
        Weight::from_parts(4_752_071_000, 0)
            // Standard Error: 116_431
            .saturating_add(Weight::from_parts(4_648_187, 0).saturating_mul(v.into()))
            // Standard Error: 11_903_517
            .saturating_add(Weight::from_parts(1_313_863_272, 0).saturating_mul(d.into()))
    }
}
