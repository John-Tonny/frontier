// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0
// This file is part of Frontier.
//
// Copyright (c) 2015-2022 Parity Technologies (UK) Ltd.
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

//! Masternode rpc interface.

use jsonrpsee::{core::RpcResult as Result, proc_macros::rpc};

// john
use sp_core::Bytes;
use pallet_masternode::{MasternodeDetails, MasternodeInfo};


/// Web3 rpc interface.
#[rpc(server)]
pub trait MasternodeApi {
    /// Returns masternode status.
    #[method(name = "masternode_status")]
    fn get_status(&self, peer_id: Bytes ) -> Result<Option<MasternodeDetails>>;

    /// Returns masternode info.
    #[method(name = "masternode_info")]
    fn get_info(&self) -> Result<MasternodeInfo>;

}
