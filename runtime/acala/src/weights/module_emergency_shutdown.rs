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

//! Autogenerated weights for module_emergency_shutdown
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

/// Weight functions for module_emergency_shutdown.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_emergency_shutdown::WeightInfo for WeightInfo<T> {
	// Storage: EmergencyShutdown IsShutdown (r:1 w:1)
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	/// The range of component `c` is `[0, 4]`.
	fn emergency_shutdown(c: u32, ) -> Weight {
		// Minimum execution time: 19_699 nanoseconds.
		Weight::from_ref_time(20_671_867)
			// Standard Error: 10_004
			.saturating_add(Weight::from_ref_time(475_904).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EmergencyShutdown IsShutdown (r:1 w:0)
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Storage: EmergencyShutdown CanRefund (r:0 w:1)
	fn open_collateral_refund() -> Weight {
		// Minimum execution time: 19_413 nanoseconds.
		Weight::from_ref_time(20_202_000)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: EmergencyShutdown CanRefund (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: CdpEngine CollateralParams (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	/// The range of component `c` is `[0, 4]`.
	fn refund_collaterals(c: u32, ) -> Weight {
		// Minimum execution time: 48_519 nanoseconds.
		Weight::from_ref_time(50_750_512)
			// Standard Error: 22_097
			.saturating_add(Weight::from_ref_time(1_939_640).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
