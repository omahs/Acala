// This file is part of Acala.

// Copyright (C) 2020-2023 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for orml_authority
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `187e78510d7a`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("karura-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=karura-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/karura/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for orml_authority.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> orml_authority::WeightInfo for WeightInfo<T> {
	fn dispatch_as() -> Weight {
		// Minimum execution time: 13_464 nanoseconds.
		Weight::from_ref_time(14_021_000)
	}
	// Storage: Authority NextTaskIndex (r:1 w:1)
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn schedule_dispatch_without_delay() -> Weight {
		// Minimum execution time: 26_220 nanoseconds.
		Weight::from_ref_time(27_452_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Authority NextTaskIndex (r:1 w:1)
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn schedule_dispatch_with_delay() -> Weight {
		// Minimum execution time: 26_780 nanoseconds.
		Weight::from_ref_time(27_457_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn fast_track_scheduled_dispatch() -> Weight {
		// Minimum execution time: 33_069 nanoseconds.
		Weight::from_ref_time(33_598_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn delay_scheduled_dispatch() -> Weight {
		// Minimum execution time: 32_552 nanoseconds.
		Weight::from_ref_time(33_751_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Scheduler Lookup (r:1 w:1)
	// Storage: Scheduler Agenda (r:1 w:1)
	fn cancel_scheduled_dispatch() -> Weight {
		// Minimum execution time: 24_934 nanoseconds.
		Weight::from_ref_time(26_194_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Authority SavedCalls (r:0 w:1)
	fn authorize_call() -> Weight {
		// Minimum execution time: 14_007 nanoseconds.
		Weight::from_ref_time(14_518_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Authority SavedCalls (r:1 w:1)
	fn remove_authorized_call() -> Weight {
		// Minimum execution time: 18_115 nanoseconds.
		Weight::from_ref_time(18_639_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Authority SavedCalls (r:1 w:1)
	fn trigger_call() -> Weight {
		// Minimum execution time: 23_046 nanoseconds.
		Weight::from_ref_time(23_935_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
