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

//! Autogenerated weights for orml_vesting
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `295f33c1d5e7`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("acala-dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=acala-dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/acala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for orml_vesting.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> orml_vesting::WeightInfo for WeightInfo<T> {
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Storage: Vesting VestingSchedules (r:1 w:1)
	fn vested_transfer() -> Weight {
		// Minimum execution time: 20_436 nanoseconds.
		Weight::from_ref_time(21_041_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: ParachainSystem ValidationData (r:1 w:0)
	// Storage: Vesting VestingSchedules (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	/// The range of component `i` is `[1, 100]`.
	fn claim(i: u32, ) -> Weight {
		// Minimum execution time: 31_236 nanoseconds.
		Weight::from_ref_time(32_625_189)
			// Standard Error: 619
			.saturating_add(Weight::from_ref_time(736).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: System Account (r:1 w:0)
	// Storage: Vesting VestingSchedules (r:0 w:1)
	/// The range of component `i` is `[1, 100]`.
	fn update_vesting_schedules(i: u32, ) -> Weight {
		// Minimum execution time: 15_597 nanoseconds.
		Weight::from_ref_time(16_318_661)
			// Standard Error: 394
			.saturating_add(Weight::from_ref_time(51_603).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
