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

//! Autogenerated weights for `crate::parachains::configuration`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2025-02-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `0227cf961efd`, CPU: `AMD Ryzen 7 7700 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// /usr/local/bin/zkv-relay
// benchmark
// pallet
// --chain
// dev
// --pallet
// crate::parachains::slashing
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
// /data/benchmark/runtime/src/weights/parachains/configuration.rs
// --template
// /data/benchmark/relay-node/benchmarks/zkv-deploy-weight-template.hbs
// --base-path=/tmp/tmp.1rbLszjw13

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weights for `crate::parachains::slashing` using the zkVerify node and recommended hardware.
pub struct ZKVWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> crate::parachains::slashing::WeightInfo for ZKVWeight<T> {
    /// Storage: `Session::CurrentIndex` (r:1 w:0)
    /// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Historical::HistoricalSessions` (r:1 w:0)
    /// Proof: `Historical::HistoricalSessions` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
    /// Storage: `ParasSlashing::UnappliedSlashes` (r:1 w:1)
    /// Proof: `ParasSlashing::UnappliedSlashes` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Offences::ConcurrentReportsIndex` (r:1 w:1)
    /// Proof: `Offences::ConcurrentReportsIndex` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Offences::Reports` (r:1 w:1)
    /// Proof: `Offences::Reports` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Staking::SlashRewardFraction` (r:1 w:0)
    /// Proof: `Staking::SlashRewardFraction` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
    /// Storage: `Staking::ActiveEra` (r:1 w:0)
    /// Proof: `Staking::ActiveEra` (`max_values`: Some(1), `max_size`: Some(13), added: 508, mode: `MaxEncodedLen`)
    /// Storage: `Staking::ErasStartSessionIndex` (r:1 w:0)
    /// Proof: `Staking::ErasStartSessionIndex` (`max_values`: None, `max_size`: Some(16), added: 2491, mode: `MaxEncodedLen`)
    /// Storage: `Staking::Invulnerables` (r:1 w:0)
    /// Proof: `Staking::Invulnerables` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Staking::ValidatorSlashInEra` (r:1 w:1)
    /// Proof: `Staking::ValidatorSlashInEra` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
    /// Storage: `Staking::SlashingSpans` (r:1 w:1)
    /// Proof: `Staking::SlashingSpans` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// Storage: `Staking::SpanSlash` (r:1 w:1)
    /// Proof: `Staking::SpanSlash` (`max_values`: None, `max_size`: Some(76), added: 2551, mode: `MaxEncodedLen`)
    /// Storage: `Staking::DisabledValidators` (r:1 w:1)
    /// Proof: `Staking::DisabledValidators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Session::Validators` (r:1 w:0)
    /// Proof: `Session::Validators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Session::DisabledValidators` (r:1 w:1)
    /// Proof: `Session::DisabledValidators` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `System::Digest` (r:1 w:1)
    /// Proof: `System::Digest` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
    /// Storage: `Staking::UnappliedSlashes` (r:1 w:1)
    /// Proof: `Staking::UnappliedSlashes` (`max_values`: None, `max_size`: None, mode: `Measured`)
    /// The range of component `n` is `[4, 20]`.
    fn report_dispute_lost(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1973 + n * (152 ±0)`
        //  Estimated: `5438 + n * (152 ±0)`
        // Minimum execution time: 106_841_000 picoseconds.
        Weight::from_parts(107_868_481, 5438)
            // Standard Error: 6_590
            .saturating_add(Weight::from_parts(327_172, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(17_u64))
            .saturating_add(T::DbWeight::get().writes(10_u64))
            .saturating_add(Weight::from_parts(0, 152).saturating_mul(n.into()))
    }
}
