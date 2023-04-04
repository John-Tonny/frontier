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

use jsonrpsee::core::RpcResult as Result;
// Substrate
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;

// john
use sp_runtime::traits::Block as BlockT;

// Frontier
use fc_rpc_core::MasternodeApiServer;
use fp_rpc::EthereumRuntimeRPCApi;

use crate::internal_err;

// john
use pallet_masternode::{MasternodeInfo, MasternodeDetails};
use sp_core::Bytes;


/// Web3 API implementation.
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
	B: BlockT,
	C: ProvideRuntimeApi<B>,
	C::Api: EthereumRuntimeRPCApi<B>,
	C: HeaderBackend<B> + 'static,
{
    fn get_status(&self, peer_id: Bytes) -> Result<Option<MasternodeDetails>> {
        let hash = self.client.info().best_hash;
        let ret = self
            .client
            .runtime_api()
            .get_status(hash, peer_id)
            .map_err(|err| internal_err(format!("fetch runtime masternode status failed: {:?}", err)))?;
        Ok(ret)
    }

    fn get_info(&self) -> Result<MasternodeInfo> {
        let hash = self.client.info().best_hash;
        let ret = self
            .client
            .runtime_api()
            .get_info(hash)
            .map_err(|err| internal_err(format!("fetch runtime masternode info failed: {:?}", err)))?;
        Ok(ret)
    }
}

