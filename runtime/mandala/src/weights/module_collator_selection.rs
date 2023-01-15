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

//! Autogenerated weights for module_collator_selection
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-06, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `ea4c8813bb44`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/mandala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_collator_selection.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_collator_selection::WeightInfo for WeightInfo<T> {
	// Storage: CollatorSelection Invulnerables (r:0 w:1)
	/// The range of component `b` is `[1, 50]`.
	fn set_invulnerables(b: u32, ) -> Weight {
		// Minimum execution time: 12_518 nanoseconds.
		Weight::from_ref_time(13_596_419)
			// Standard Error: 755
			.saturating_add(Weight::from_ref_time(30_480).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection DesiredCandidates (r:0 w:1)
	fn set_desired_candidates() -> Weight {
		// Minimum execution time: 12_460 nanoseconds.
		Weight::from_ref_time(12_831_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection CandidacyBond (r:0 w:1)
	fn set_candidacy_bond() -> Weight {
		// Minimum execution time: 12_201 nanoseconds.
		Weight::from_ref_time(13_020_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection NonCandidates (r:1 w:1)
	// Storage: CollatorSelection CandidacyBond (r:1 w:0)
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:1)
	/// The range of component `c` is `[5, 200]`.
	fn register_as_candidate(c: u32, ) -> Weight {
		// Minimum execution time: 50_754 nanoseconds.
		Weight::from_ref_time(63_425_743)
			// Standard Error: 2_804
			.saturating_add(Weight::from_ref_time(180_598).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Storage: Session NextKeys (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:0)
	/// The range of component `c` is `[1, 200]`.
	fn register_candidate(c: u32, ) -> Weight {
		// Minimum execution time: 29_792 nanoseconds.
		Weight::from_ref_time(42_186_933)
			// Standard Error: 2_252
			.saturating_add(Weight::from_ref_time(171_039).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: Session CurrentIndex (r:1 w:0)
	// Storage: CollatorSelection NonCandidates (r:0 w:1)
	/// The range of component `c` is `[6, 200]`.
	fn leave_intent(c: u32, ) -> Weight {
		// Minimum execution time: 24_011 nanoseconds.
		Weight::from_ref_time(29_999_317)
			// Standard Error: 1_468
			.saturating_add(Weight::from_ref_time(146_973).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: CollatorSelection NonCandidates (r:1 w:1)
	// Storage: Session CurrentIndex (r:1 w:0)
	// Storage: Balances Reserves (r:1 w:1)
	fn withdraw_bond() -> Weight {
		// Minimum execution time: 58_550 nanoseconds.
		Weight::from_ref_time(59_970_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: System Account (r:1 w:0)
	// Storage: CollatorSelection SessionPoints (r:1 w:0)
	fn note_author() -> Weight {
		// Minimum execution time: 27_468 nanoseconds.
		Weight::from_ref_time(27_997_000)
			.saturating_add(T::DbWeight::get().reads(2))
	}
	// Storage: CollatorSelection Candidates (r:1 w:0)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	fn new_session() -> Weight {
		// Minimum execution time: 37_487 nanoseconds.
		Weight::from_ref_time(38_191_000)
			.saturating_add(T::DbWeight::get().reads(2))
	}
	// Storage: Session Validators (r:1 w:0)
	// Storage: CollatorSelection Candidates (r:1 w:0)
	// Storage: CollatorSelection SessionPoints (r:0 w:200)
	/// The range of component `r` is `[5, 200]`.
	/// The range of component `c` is `[5, 200]`.
	fn start_session(_r: u32, c: u32, ) -> Weight {
		// Minimum execution time: 17_544 nanoseconds.
		Weight::from_ref_time(9_463_222)
			// Standard Error: 3_568
			.saturating_add(Weight::from_ref_time(1_372_777).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
	// Storage: CollatorSelection SessionPoints (r:201 w:200)
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Storage: Session CurrentIndex (r:1 w:0)
	// Storage: CollatorSelection NonCandidates (r:0 w:190)
	/// The range of component `r` is `[5, 200]`.
	/// The range of component `c` is `[5, 200]`.
	fn end_session(_r: u32, c: u32, ) -> Weight {
		// Minimum execution time: 37_906 nanoseconds.
		Weight::from_ref_time(2_537_916_946)
			// Standard Error: 41_569
			.saturating_add(Weight::from_ref_time(6_475_797).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(198))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
}
