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

//! Autogenerated weights for module_cdp_treasury
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

/// Weight functions for module_cdp_treasury.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_cdp_treasury::WeightInfo for WeightInfo<T> {
	// Storage: Tokens Accounts (r:1 w:0)
	// Storage: AuctionManager TotalCollateralInAuction (r:1 w:1)
	// Storage: CdpTreasury ExpectedCollateralAuctionSize (r:1 w:0)
	// Storage: AuctionManager TotalTargetInAuction (r:1 w:1)
	// Storage: Auction AuctionsIndex (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: AuctionManager CollateralAuctions (r:0 w:1)
	// Storage: Auction AuctionEndTime (r:0 w:1)
	// Storage: Auction Auctions (r:0 w:1)
	/// The range of component `b` is `[1, 50]`.
	fn auction_collateral(b: u32, ) -> Weight {
		// Minimum execution time: 41_698 nanoseconds.
		Weight::from_ref_time(41_818_451)
			// Standard Error: 18_057
			.saturating_add(Weight::from_ref_time(11_041_130).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(b.into())))
	}
	// Storage: Tokens Accounts (r:4 w:4)
	// Storage: AuctionManager TotalCollateralInAuction (r:1 w:0)
	// Storage: Dex TradingPairStatuses (r:3 w:0)
	// Storage: Dex LiquidityPool (r:3 w:2)
	// Storage: StableAsset Pools (r:2 w:0)
	// Storage: AggregatedDex AggregatedSwapPaths (r:1 w:0)
	// Storage: Homa TotalStakingBonded (r:1 w:0)
	// Storage: Homa ToBondPool (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Homa TotalVoidLiquid (r:1 w:0)
	// Storage: System Account (r:2 w:1)
	// Storage: EvmAccounts EvmAddresses (r:1 w:0)
	fn exchange_collateral_to_stable() -> Weight {
		// Minimum execution time: 186_651 nanoseconds.
		Weight::from_ref_time(193_927_000)
			.saturating_add(T::DbWeight::get().reads(21))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: CdpTreasury ExpectedCollateralAuctionSize (r:0 w:1)
	fn set_expected_collateral_auction_size() -> Weight {
		// Minimum execution time: 14_465 nanoseconds.
		Weight::from_ref_time(15_040_000)
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:2 w:1)
	fn extract_surplus_to_treasury() -> Weight {
		// Minimum execution time: 40_278 nanoseconds.
		Weight::from_ref_time(41_471_000)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
