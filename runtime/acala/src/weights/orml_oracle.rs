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

//! Autogenerated weights for orml_oracle
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

/// Weight functions for orml_oracle.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> orml_oracle::WeightInfo for WeightInfo<T> {
	// Storage: AcalaOracle HasDispatched (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: OperatorMembershipAcala Members (r:1 w:0)
	// Storage: AcalaOracle Values (r:1 w:0)
	// Storage: AcalaOracle RawValues (r:0 w:1)
	/// The range of component `c` is `[0, 4]`.
	fn feed_values(c: u32, ) -> Weight {
		// Minimum execution time: 16_786 nanoseconds.
		Weight::from_ref_time(18_699_504)
			// Standard Error: 39_395
			.saturating_add(Weight::from_ref_time(5_669_247).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(c.into())))
	}
	// Storage: AcalaOracle HasDispatched (r:0 w:1)
	fn on_finalize() -> Weight {
		// Minimum execution time: 5_846 nanoseconds.
		Weight::from_ref_time(6_164_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
