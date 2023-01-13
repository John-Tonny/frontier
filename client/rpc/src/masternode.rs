// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0
// This file is part of Frontier.
//
// Copyright (c) 2020-2022 Parity Technologies (UK) Ltd.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::{marker::PhantomData, sync::Arc};

use sp_core::OpaquePeerId;

use ethereum_types::H256;
use jsonrpsee::core::RpcResult as Result;
use sp_api::{Core, ProvideRuntimeApi};
use sp_blockchain::HeaderBackend;
use sp_core::keccak_256;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};

use fc_rpc_core::{types::Bytes, MasternodeApiServer};
use fp_rpc::EthereumRuntimeRPCApi;
//use fp_masternode::MasternodeRuntimeRPCApi;

use crate::internal_err;

use pallet_masternode::{MasternodeInfo};

/// Masternode API implementation.
pub struct Masternode<B, C> {
	client: Arc<C>,
	_marker: PhantomData<B>,
}

impl<B, C> Masternode<B, C> {
	pub fn new(client: Arc<C>) -> Self {
		Self {
			client,
			_marker: PhantomData,
		}
	}
}

impl<B, C> MasternodeApiServer for Masternode<B, C>
where
	B: BlockT<Hash = H256> + Send + Sync + 'static,
	C: HeaderBackend<B> + ProvideRuntimeApi<B> + Send + Sync + 'static,
	C::Api: EthereumRuntimeRPCApi<B>,
{
	fn get_status(&self, peer_id: OpaquePeerId) -> Result<u16> {
		let hash = self.client.info().best_hash;
		let ret = self
			.client
			.runtime_api()
			.get_status(&BlockId::Hash(hash), peer_id)
			.map_err(|err| internal_err(format!("fetch runtime masternode status failed: {:?}", err)))?;
		Ok(ret)
	}

	fn get_info(&self) -> Result<MasternodeInfo> {
		let hash = self.client.info().best_hash;
		let ret = self
			.client
			.runtime_api()
			.get_info(&BlockId::Hash(hash))
			.map_err(|err| internal_err(format!("fetch runtime masternode info failed: {:?}", err)))?;
		Ok(ret)
	}


}
