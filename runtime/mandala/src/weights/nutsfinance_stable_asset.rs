// This file is part of Acala.

// Copyright (C) 2020-2022 Acala Foundation.
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

//! Autogenerated weights for nutsfinance_stable_asset
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-16, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/production/acala
// benchmark
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

/// Weight functions for nutsfinance_stable_asset.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> nutsfinance_stable_asset::WeightInfo for WeightInfo<T> {
	// Storage: StableAsset PoolCount (r:1 w:1)
	// Storage: StableAsset Pools (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn create_pool() -> Weight {
		(21_014_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: StableAsset Pools (r:1 w:1)
	fn modify_a() -> Weight {
		(16_015_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: StableAsset Pools (r:1 w:1)
	// Storage: Tokens Accounts (r:6 w:6)
	// Storage: System Account (r:2 w:2)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:0)
	fn mint(u: u32, ) -> Weight {
		(75_023_000 as Weight)
			// Standard Error: 287_000
			.saturating_add((19_271_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(u as Weight)))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(u as Weight)))
	}
	// Storage: StableAsset Pools (r:1 w:1)
	// Storage: Tokens Accounts (r:5 w:5)
	// Storage: System Account (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:0)
	fn swap(u: u32, ) -> Weight {
		(74_632_000 as Weight)
			// Standard Error: 232_000
			.saturating_add((5_802_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(u as Weight)))
			.saturating_add(T::DbWeight::get().writes(7 as Weight))
	}
	// Storage: StableAsset Pools (r:1 w:1)
	// Storage: Tokens Accounts (r:6 w:6)
	// Storage: System Account (r:1 w:0)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	fn redeem_proportion(u: u32, ) -> Weight {
		(72_821_000 as Weight)
			// Standard Error: 261_000
			.saturating_add((16_278_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(u as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(u as Weight)))
	}
	// Storage: StableAsset Pools (r:1 w:1)
	// Storage: Tokens Accounts (r:5 w:4)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	fn redeem_single(u: u32, ) -> Weight {
		(87_931_000 as Weight)
			// Standard Error: 712_000
			.saturating_add((3_615_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(u as Weight)))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: StableAsset Pools (r:1 w:1)
	// Storage: Tokens Accounts (r:6 w:6)
	// Storage: AssetRegistry AssetMetadatas (r:1 w:0)
	// Storage: System Account (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	fn redeem_multi(u: u32, ) -> Weight {
		(68_569_000 as Weight)
			// Standard Error: 206_000
			.saturating_add((14_720_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((2 as Weight).saturating_mul(u as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((2 as Weight).saturating_mul(u as Weight)))
	}
	fn mint_xcm(u: u32) -> Weight {
		(85_694_000 as Weight) // Standard Error: 187_000
			.saturating_add((46_172_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(u as Weight)))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(u as Weight)))
	}
	fn mint_xcm_fail() -> Weight {
		(33_115_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn send_mint_xcm() -> Weight {
		(33_115_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
