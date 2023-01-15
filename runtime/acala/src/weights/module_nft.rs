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

//! Autogenerated weights for module_nft
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

/// Weight functions for module_nft.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_nft::WeightInfo for WeightInfo<T> {
	// Storage: OrmlNFT NextClassId (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: Proxy Proxies (r:1 w:1)
	// Storage: OrmlNFT Classes (r:0 w:1)
	fn create_class() -> Weight {
		// Minimum execution time: 73_980 nanoseconds.
		Weight::from_ref_time(75_904_000)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: OrmlNFT Classes (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: OrmlNFT NextTokenId (r:1 w:1)
	// Storage: OrmlNFT Tokens (r:0 w:1)
	// Storage: OrmlNFT TokensByOwner (r:0 w:1)
	/// The range of component `i` is `[1, 1000]`.
	fn mint(i: u32, ) -> Weight {
		// Minimum execution time: 86_554 nanoseconds.
		Weight::from_ref_time(23_070_337)
			// Standard Error: 8_333
			.saturating_add(Weight::from_ref_time(18_392_416).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(i.into())))
	}
	// Storage: OrmlNFT Classes (r:1 w:0)
	// Storage: OrmlNFT Tokens (r:1 w:1)
	// Storage: Balances Reserves (r:2 w:2)
	// Storage: System Account (r:2 w:2)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: OrmlNFT TokensByOwner (r:0 w:2)
	fn transfer() -> Weight {
		// Minimum execution time: 92_620 nanoseconds.
		Weight::from_ref_time(95_136_000)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: OrmlNFT Classes (r:1 w:1)
	// Storage: OrmlNFT Tokens (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: OrmlNFT TokensByOwner (r:0 w:1)
	fn burn() -> Weight {
		// Minimum execution time: 69_109 nanoseconds.
		Weight::from_ref_time(70_993_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: OrmlNFT Classes (r:1 w:1)
	// Storage: OrmlNFT Tokens (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: OrmlNFT TokensByOwner (r:0 w:1)
	/// The range of component `b` is `[0, 3670016]`.
	fn burn_with_remark(b: u32, ) -> Weight {
		// Minimum execution time: 70_429 nanoseconds.
		Weight::from_ref_time(70_977_000)
			// Standard Error: 3
			.saturating_add(Weight::from_ref_time(2_045).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: OrmlNFT Classes (r:1 w:1)
	// Storage: Balances Reserves (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Proxy Proxies (r:1 w:1)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	// Storage: OrmlNFT NextTokenId (r:0 w:1)
	fn destroy_class() -> Weight {
		// Minimum execution time: 83_979 nanoseconds.
		Weight::from_ref_time(85_740_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	// Storage: OrmlNFT Classes (r:1 w:1)
	fn update_class_properties() -> Weight {
		// Minimum execution time: 18_305 nanoseconds.
		Weight::from_ref_time(18_816_000)
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
