// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.


mod moonbeam_xcm_benchmarks_fungible;
mod moonbeam_xcm_benchmarks_generic;

use frame_support::weights::Weight;
use moonbeam_xcm_benchmarks_fungible::WeightInfo as XcmFungibleWeight;
use moonbeam_xcm_benchmarks_generic::SubstrateWeight as XcmGeneric;
use sp_std::prelude::*;
use xcm::{
	latest::{prelude::*, Weight as XCMWeight},
	DoubleEncoded,
};
use crate::weights::moonbeam_xcm_benchmarks_generic::WeightInfo;

trait WeighMultiAssets {
	fn weigh_multi_assets(&self, weight: Weight) -> XCMWeight;
}

const MAX_ASSETS: u32 = 100;

impl WeighMultiAssets for MultiAssetFilter {
	fn weigh_multi_assets(&self, weight: Weight) -> XCMWeight {
		let weight = match self {
			Self::Definite(assets) =>
				weight.saturating_mul(assets.inner().into_iter().count() as u64),
			Self::Wild(_) => weight.saturating_mul(MAX_ASSETS as u64),
		};
		weight
	}
}

impl WeighMultiAssets for MultiAssets {
	fn weigh_multi_assets(&self, weight: Weight) -> XCMWeight {
		weight.saturating_mul(self.inner().into_iter().count() as u64)
	}
}

pub struct XcmWeight<Runtime, Call>(core::marker::PhantomData<(Runtime, Call)>);
impl<Runtime, Call> XcmWeightInfo<Call> for XcmWeight<Runtime, Call> where Runtime: frame_system::Config {
	fn withdraw_asset(assets: &MultiAssets) -> XCMWeight {
		assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::withdraw_asset())
	}
	// Currently there is no trusted reserve
	fn reserve_asset_deposited(_assets: &MultiAssets) -> XCMWeight {
		unimplemented!()
	}
	fn receive_teleported_asset(assets: &MultiAssets) -> XCMWeight {
		assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::receive_teleported_asset())
	}
	fn query_response(_query_id: &u64, _response: &Response, _max_weight: &u64) -> XCMWeight {
		XcmGeneric::<Runtime>::query_response()
	}
	fn transfer_asset(assets: &MultiAssets, _dest: &MultiLocation) -> XCMWeight {
		assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::transfer_asset())
	}
	fn transfer_reserve_asset(
		assets: &MultiAssets,
		_dest: &MultiLocation,
		_xcm: &Xcm<()>,
	) -> XCMWeight {
		assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::transfer_reserve_asset())
	}
	fn transact(
		_origin_type: &OriginKind,
		_require_weight_at_most: &u64,
		_call: &DoubleEncoded<Call>,
	) -> XCMWeight {
		XcmGeneric::<Runtime>::transact()
	}
	fn hrmp_new_channel_open_request(
		_sender: &u32,
		_max_message_size: &u32,
		_max_capacity: &u32,
	) -> XCMWeight {
		// XCM Executor does not currently support HRMP channel operations
		Weight::MAX
	}
	fn hrmp_channel_accepted(_recipient: &u32) -> XCMWeight {
		// XCM Executor does not currently support HRMP channel operations
		Weight::MAX
	}
	fn hrmp_channel_closing(_initiator: &u32, _sender: &u32, _recipient: &u32) -> XCMWeight {
		// XCM Executor does not currently support HRMP channel operations
		Weight::MAX
	}
	fn clear_origin() -> XCMWeight {
		XcmGeneric::<Runtime>::clear_origin()
	}
	fn descend_origin(_who: &InteriorMultiLocation) -> XCMWeight {
		XcmGeneric::<Runtime>::descend_origin()
	}
	fn report_error(
		_query_id: &QueryId,
		_dest: &MultiLocation,
		_max_response_weight: &u64,
	) -> XCMWeight {
		XcmGeneric::<Runtime>::report_error()
	}

	fn deposit_asset(
		assets: &MultiAssetFilter,
		_max_assets: &u32,
		_dest: &MultiLocation,
	) -> XCMWeight {
		assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::deposit_asset())
	}
	fn deposit_reserve_asset(
		assets: &MultiAssetFilter,
		_max_assets: &u32,
		_dest: &MultiLocation,
		_xcm: &Xcm<()>,
	) -> XCMWeight {
		assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::deposit_reserve_asset())
	}
	fn exchange_asset(_give: &MultiAssetFilter, _receive: &MultiAssets) -> XCMWeight {
		Weight::MAX
	}
	fn initiate_reserve_withdraw(
		assets: &MultiAssetFilter,
		_reserve: &MultiLocation,
		_xcm: &Xcm<()>,
	) -> XCMWeight {
		assets.weigh_multi_assets(XcmGeneric::<Runtime>::initiate_reserve_withdraw())
	}
	fn initiate_teleport(
		assets: &MultiAssetFilter,
		_dest: &MultiLocation,
		_xcm: &Xcm<()>,
	) -> XCMWeight {
		assets.weigh_multi_assets(XcmFungibleWeight::<Runtime>::initiate_teleport())
	}
	fn query_holding(
		_query_id: &u64,
		_dest: &MultiLocation,
		_assets: &MultiAssetFilter,
		_max_response_weight: &u64,
	) -> XCMWeight {
		XcmGeneric::<Runtime>::query_holding()
	}
	fn buy_execution(_fees: &MultiAsset, _weight_limit: &WeightLimit) -> XCMWeight {
		XcmGeneric::<Runtime>::buy_execution()
	}
	fn refund_surplus() -> XCMWeight {
		XcmGeneric::<Runtime>::refund_surplus()
	}
	fn set_error_handler(_xcm: &Xcm<Call>) -> XCMWeight {
		XcmGeneric::<Runtime>::set_error_handler()
	}
	fn set_appendix(_xcm: &Xcm<Call>) -> XCMWeight {
		XcmGeneric::<Runtime>::set_appendix()
	}
	fn clear_error() -> XCMWeight {
		XcmGeneric::<Runtime>::clear_error()
	}
	fn claim_asset(_assets: &MultiAssets, _ticket: &MultiLocation) -> XCMWeight {
		XcmGeneric::<Runtime>::claim_asset()
	}
	fn trap(_code: &u64) -> XCMWeight {
		XcmGeneric::<Runtime>::trap()
	}
	fn subscribe_version(_query_id: &QueryId, _max_response_weight: &u64) -> XCMWeight {
		XcmGeneric::<Runtime>::subscribe_version()
	}
	fn unsubscribe_version() -> XCMWeight {
		XcmGeneric::<Runtime>::unsubscribe_version()
	}
}