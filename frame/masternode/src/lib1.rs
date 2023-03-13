// This file is part of Substrate.

// Copyright (C) 2019-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Node authorization pallet
//!
//! This pallet manages a configurable set of nodes for a permissioned network.
//! Each node is dentified by a PeerId (i.e. Vec<u8>). It provides two ways to
//! authorize a node,
//!
//! - a set of well known nodes across different organizations in which the
//! connections are allowed.
//! - users can claim the ownership for each node, then manage the connections of
//! the node.
//!
//! A node must have an owner. The owner can additionally change the connections
//! for the node. Only one user is allowed to claim a specific node. To eliminate
//! false claim, the maintainer of the node should claim it before even starting the
//! node. This pallet uses offchain worker to set reserved nodes, if the node is not
//! an authority, make sure to enable offchain worker with the right CLI flag. The
//! node can be lagged with the latest block, in this case you need to disable offchain
//! worker and manually set reserved nodes when starting it.

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

pub mod weights;

use sp_core::crypto::KeyTypeId;
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"mnod");

use frame_support::{
	pallet_prelude::*,
};

pub mod crypto {
	use super::KEY_TYPE;
	use sp_runtime::app_crypto::{app_crypto, sr25519};
	app_crypto!(sr25519, KEY_TYPE);
}

pub type AuthorityId = crypto::Public;

pub use pallet::*;

use codec::{Decode, Encode};
use frame_support::{dispatch::DispatchResult, RuntimeDebug, traits::{Currency, ReservableCurrency, UnixTime}};
use frame_system::{self as system};
use sp_core::OpaquePeerId;
pub use sp_std::vec::Vec;
use sp_std::{collections::btree_set::BTreeSet, iter::FromIterator, prelude::*, string::String};
pub use weights::WeightInfo;

use sp_std::serde_json;
use sp_std::serde::{Deserialize, Serialize};
use sp_std::bs58;

/// The balance type of this pallet.
type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo, Serialize, Deserialize)]
pub enum MasternodeStatus {
	OnLine,
	OffLine,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo, Serialize, Deserialize)]
pub struct MasternodeDetails {
	pub created_block_number: u32,
	pub updated_block_number: u32,
	pub status: MasternodeStatus,
}
/*
pub struct MasternodeDetails<AccountId, BlockNumber> {
	pub owner: AccountId,
	pub created_block_number: BlockNumber,
	pub updated_block_number: BlockNumber,
	pub status: MasternodeStatus,
}
*/

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug,  TypeInfo,  Serialize)]
pub struct MasternodeInfo {
	pub total_nums: u16,
	pub online_nums: u16,
	pub infos: Vec<(String, MasternodeDetails)>,
}


#[frame_support::pallet]
pub mod pallet {
	use super::*;
	// use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use pallet_node_authorization::traits::StorageInterface;

	use frame_system::offchain::{
		AppCrypto, CreateSignedTransaction, SendUnsignedTransaction, SignedPayload, Signer,
		SigningTypes,
	};

	use sp_core::offchain::OpaqueNetworkState;
	use sp_runtime::traits::IdentifyAccount;
	use sp_runtime::offchain::{http, Duration};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
	pub struct HeartBeatPayload<Public, BlockNumber> {
		block_number: BlockNumber,
		public: Public,
		next_block_number: u32,
		local_peer_id: OpaquePeerId,
		peer_id_vec: Vec<OpaquePeerId>,
	}

	#[derive(Serialize, Deserialize)]
	pub struct RpcCall {
		pub id: u32,
		pub jsonrpc: String,
		pub method: String,
		// pub params: Vec<String>,
	}

	#[derive(Serialize, Deserialize)]
	pub struct RpcResult {
		jsonrpc:String,
		result:Vec<PeerInfo>,
		id: u32,
	}

	#[derive(Serialize, Deserialize)]
	pub struct PeerInfo {
		peerId: String,
		roles: String,
		bestHash: String,
		bestNumber: u32,
	}

	impl<T: SigningTypes> SignedPayload<T> for HeartBeatPayload<T::Public, T::BlockNumber> {
		fn public(&self) -> T::Public {
			self.public.clone()
		}
	}

	/// The module configuration trait
	#[pallet::config]
	pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The currency used for deposits.
		type Currency: ReservableCurrency<Self::AccountId>;

		/// Just the `Currency::Balance` type; we have this item to allow us to constrain it to
		/// `From<u64>`.
		type CurrencyBalance: sp_runtime::traits::AtLeast32BitUnsigned
			+ codec::FullCodec
			+ Copy
			+ MaybeSerializeDeserialize
			+ sp_std::fmt::Debug
			+ Default
			+ From<u64>
			+ TypeInfo
			+ MaxEncodedLen;

		/// Time used for computing era duration.
		type UnixTime: UnixTime;

		/// The deposit required for masternode.
		#[pallet::constant]
		type MasternodeDeposit: Get<BalanceOf<Self>>;

		/// The maximum number of masternodes that are allowed to set
		#[pallet::constant]
		type MaxMasternodes: Get<u32>;

		/// The maximum length in bytes of PeerId
		#[pallet::constant]
		type MaxPeerIdLength: Get<u32>;

		/// The origin which can remove a masternode.
		type RemoveOrigin: EnsureOrigin<Self::Origin>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		type MyStorage: StorageInterface;

	}

	/// The set of well known nodes. This is stored sorted (just by value).
	#[pallet::storage]
	#[pallet::getter(fn register_masternodes)]
	pub type RegisterMasternodes<T> = StorageValue<_, BTreeSet<OpaquePeerId>, ValueQuery>;

	/// A map that maintains the ownership of each node.
	#[pallet::storage]
	#[pallet::getter(fn masternodes)]
	pub type Masternodes<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		OpaquePeerId,
		MasternodeDetails,
		// MasternodeDetails<T::AccountId, T::BlockNumber>,
	>;

	#[pallet::storage]
	#[pallet::getter(fn heartbeat_after)]
	pub(crate) type HeartbeatAfter<T: Config> = StorageMap<_, Blake2_128Concat, OpaquePeerId, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The given masternode was registered.
		MasternodeRegistered {
			peer_id: OpaquePeerId,
			who: T::AccountId,
		},
		/// The given masternode was unregistered.
		MasternodeUnregistered {
			peer_id: OpaquePeerId,
		},
		/// The given masternode was removed.
		MasternodeRemoved {
			peer_id: OpaquePeerId,
		},
        
        /// masternode heartbeat
		MasternodeHeartBeat {
            peer_id: OpaquePeerId, 
            block_number: T::BlockNumber
        },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The PeerId is too long.
		PeerIdTooLong,
		/// You are not masternode.
		NotMasternode,
		/// Too many well known nodes.
		TooManyNodes,
		/// The node is already joined in the list.
		AlreadyRegistered,
		/// The node doesn't exist in the list.
		NotExist,
		/// You are not the owner of the node.
		NotOwner,
		/// No permisson to perform specific operation.
		PermissionDenied,
		/// Balance is insufficient for the required deposit.
		InsufficientFunds,

		OffchainUnsignedTxError,

	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
            let isvalidator = sp_io::offchain::is_validator();
            log::info!("my offchain roles: {}", isvalidator);

            if isvalidator {

			    let number: u32 = block_number.try_into().unwrap_or(0);

			    let network_state: OpaqueNetworkState = sp_io::offchain::network_state().unwrap();
			    let mut peer_id = network_state.peer_id.clone();
			    peer_id.0.remove(0);

			    let heartbeat_after = <HeartbeatAfter<T>>::get(&peer_id).unwrap_or(0);
			    if number >= heartbeat_after {
				    if let Ok(peers) = Self::get_peers(network_state.rpc_http_port) {
					    let mut peer_id_vec = Vec::new();
					    peer_id_vec.push(peer_id.clone());
					    for peer in peers.iter() {
						    let peer_id1 = OpaquePeerId(bs58::decode(&peer.peerId).into_vec().unwrap());
						    if let Some(_) = Masternodes::<T>::get(&peer_id1) {
							    peer_id_vec.push(peer_id1);
						    }
					    }
					    let rand = sp_io::offchain::random_range();
					    let _ = Signer::<T, T::AuthorityId>::any_account().send_unsigned_transaction(
						    |account| HeartBeatPayload {
							    block_number:block_number,
							    public: account.public.clone(),
							    next_block_number: number + rand,
							    local_peer_id: peer_id.clone(),
							    peer_id_vec: peer_id_vec.clone(),
						    },
						    |payload, signature| Call::send_masternode_heartbeat {
							    heartbeat_payload: payload,
							    signature,
						    },
					    );
				    }
                }
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight((T::WeightInfo::register_masternode(), DispatchClass::Operational))]
		pub fn register_masternode(origin: OriginFor<T>, peer_id: OpaquePeerId) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(
				peer_id.0.len() < T::MaxPeerIdLength::get() as usize,
				Error::<T>::PeerIdTooLong
			);

			let nodes = T::MyStorage::get_nodes();
			ensure!(nodes.contains(&peer_id), Error::<T>::NotMasternode);

			let mut nodes = RegisterMasternodes::<T>::get();
			ensure!(
				nodes.len() < T::MaxMasternodes::get() as usize,
				Error::<T>::TooManyNodes
			);
			ensure!(!nodes.contains(&peer_id), Error::<T>::AlreadyRegistered);

			/*
			let deposit = T::MasternodeDeposit::get();
			T::Currency::reserve(&sender, deposit).map_err(|_| Error::<T>::InsufficientFunds)?;
			*/

			nodes.insert(peer_id.clone());
			RegisterMasternodes::<T>::put(&nodes);

			let block_number:u32 = system::Pallet::<T>::block_number().try_into().unwrap_or(0);
			Masternodes::<T>::insert(
				&peer_id,
				MasternodeDetails {
					//owner: sender.clone(),
					created_block_number: block_number,
					updated_block_number: block_number,
					status: MasternodeStatus::OnLine,
				},
			);

			Self::deposit_event(Event::MasternodeRegistered {
				peer_id: peer_id,
				who: sender,
			});
			Ok(())
		}

		#[pallet::weight((T::WeightInfo::unregister_masternode(), DispatchClass::Operational))]
		pub fn unregister_masternode(
			origin: OriginFor<T>,
			peer_id: OpaquePeerId,
		) -> DispatchResult {
			let _sender = ensure_signed(origin)?;

			ensure!(
				peer_id.0.len() < T::MaxPeerIdLength::get() as usize,
				Error::<T>::PeerIdTooLong
			);

			let mut nodes = RegisterMasternodes::<T>::get();
			ensure!(nodes.contains(&peer_id), Error::<T>::NotExist);

			/*
			let masternode_details = Masternodes::<T>::get(&peer_id).ok_or(Error::<T>::NotExist)?;
			ensure!(masternode_details.owner == sender, Error::<T>::NotOwner);
			*/

			/*
			let deposit = T::MasternodeDeposit::get();
			T::Currency::unreserve(&sender, deposit);
			*/

			nodes.remove(&peer_id);
			RegisterMasternodes::<T>::put(&nodes);

			Masternodes::<T>::get(&peer_id).ok_or(Error::<T>::NotExist)?;
			<Masternodes<T>>::remove(&peer_id);

			Self::deposit_event(Event::MasternodeUnregistered { peer_id });
			Ok(())
		}

		#[pallet::weight((T::WeightInfo::remove_masternode(), DispatchClass::Operational))]
		pub fn remove_masternode(origin: OriginFor<T>, peer_id: OpaquePeerId) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin)?;
			ensure!(
				peer_id.0.len() < T::MaxPeerIdLength::get() as usize,
				Error::<T>::PeerIdTooLong
			);

			let mut nodes = RegisterMasternodes::<T>::get();
			ensure!(nodes.contains(&peer_id), Error::<T>::NotExist);

			nodes.remove(&peer_id);
			RegisterMasternodes::<T>::put(&nodes);

			/*
			let masternode_details = Masternodes::<T>::get(&peer_id).ok_or(Error::<T>::NotExist)?;
			let deposit = T::MasternodeDeposit::get();
			T::Currency::unreserve(&masternode_details.owner, deposit);
			*/

			<Masternodes<T>>::remove(&peer_id);

			Self::deposit_event(Event::MasternodeRemoved { peer_id });
			Ok(())
		}

		#[pallet::weight((T::WeightInfo::send_masternode_heartbeat(), DispatchClass::Operational))]
		pub fn send_masternode_heartbeat(
			origin: OriginFor<T>,
			heartbeat_payload: HeartBeatPayload<T::Public, T::BlockNumber>,
			_signature: T::Signature,
		) -> DispatchResultWithPostInfo {
			ensure_none(origin)?;

			for peer_id in heartbeat_payload.peer_id_vec.iter() {
				if let Some(masternode_details) = Masternodes::<T>::get(&peer_id) {
					Masternodes::<T>::insert(
						&peer_id,
						MasternodeDetails {
							// owner: masternode_details.owner.clone(),
							created_block_number: masternode_details.created_block_number,
							updated_block_number: heartbeat_payload.block_number.try_into().unwrap_or(0),
							status: MasternodeStatus::OnLine,
						},
					);
				}
			}
			// Self::update_masternode_state();

			<HeartbeatAfter<T>>::insert(&heartbeat_payload.local_peer_id, heartbeat_payload.next_block_number );
		    /*
            Self::deposit_event(Event::MasternodeHeartBeat{
				peer_id: heartbeat_payload.local_peer_id,
				block_number: heartbeat_payload.block_number,
            });
            */
			Ok(().into())
		}
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			if let Call::send_masternode_heartbeat {
				heartbeat_payload: ref payload,
				ref signature,
			} = call
			{
				let signature_valid =
					SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone());
				if !signature_valid {
					return InvalidTransaction::BadProof.into();
				}

				ValidTransaction::with_tag_prefix("MasternodeHeartBeat")
					.priority(TransactionPriority::max_value())
					.longevity(1)
					.propagate(true)
					.build()
			} else {
				InvalidTransaction::Call.into()
			}
		}
	}

    impl<T: Config> Pallet<T> {
        fn get_peers(rpc_http_port: u16) -> Result<Vec<PeerInfo>, http::Error> {
            let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));
			/*
			let rpc_call = serde_json::json!({
        		"method": "system_peers",
        		"id": 1,
        		"jsonrpc": "2.0",
				"params": []
    		});
			 */

			let rpc_call = RpcCall{
				method: "system_peers".to_owned(),
				id: 1,
				jsonrpc: "2.0".to_owned()
			};

			let body = serde_json::to_string(&rpc_call).unwrap();
			let body = vec![body.as_bytes()];

			let str_port = Self::get_port(rpc_http_port);
			let url = String::from("http://127.0.0.1:") + &str_port;
			let request = http::Request::post(
				url.as_str(),
				body
			);

			let request = request.add_header("Content-Type", "application/json");
			let pending = request.deadline(deadline).send().map_err(|_| http::Error::IoError)?;
            let response =
                pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
            if response.code != 200 {
                log::warn!("Unexpected status code: {}", response.code);
                return Err(http::Error::Unknown)
            }

            let body = response.body().collect::<Vec<u8>>();
            let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
                log::warn!("No UTF8 body");
                http::Error::Unknown
            })?;
            log::debug!("system_peers: {}", body_str);
            let ret:RpcResult = serde_json::from_str(&body_str)
                .unwrap_or(
                RpcResult{
                    jsonrpc: String::from("2.0"),
                    result: Vec::new(),
                    id: 1
                }
            );
            Ok(ret.result)
        }

		fn get_port(port: u16) -> String {
			let mut v = port;
			let mut bstart = false;
			let ps = vec![10000u16,1000u16,100u16,10u16,1u16];
			let mut ret: Vec<u8> = Vec::new();
			for p in ps.into_iter() {
				let b1 = v / p;
				v = v - b1 * p;
				if b1 == 0 && bstart {
					ret.push(b1 as u8 + 48);
				}
				if b1 >0 {
					bstart = true;
					ret.push(b1 as u8 + 48);
				}
			}
			String::from_utf8(ret).expect("Found invalid UTF-8")
		}

		fn update_masternode_state() {
			let block_number:u32 = system::Pallet::<T>::block_number().try_into().unwrap_or(0);
			let nodes = RegisterMasternodes::<T>::get();
			for node in nodes.into_iter() {
				if let Some(mut masternode_details) = Masternodes::<T>::get(&node) {
					let number:u32 = masternode_details.updated_block_number.try_into().unwrap_or(0);
					if block_number > number + 120 {
						masternode_details.status = MasternodeStatus::OffLine;
						Masternodes::<T>::insert(
							&node,
							masternode_details,
						);
					}
				}
			}
		}

		pub fn get_status(peer_id: OpaquePeerId) -> Option<MasternodeDetails> {
			let block_number: u32 = system::Pallet::<T>::block_number().try_into().unwrap_or(0);
			if let Some(mut masternode_details) = Masternodes::<T>::get(&peer_id) {
				let number: u32 = masternode_details.updated_block_number;
				if block_number > number + 600 {
					masternode_details.status = MasternodeStatus::OffLine;
				}
				Some(masternode_details)
			} else {
				None
			}
		}

		pub fn get_info() -> MasternodeInfo {
			let block_number:u32 = system::Pallet::<T>::block_number().try_into().unwrap_or(0);
			let nodes = RegisterMasternodes::<T>::get();
			let total_nums:u16 = nodes.len() as u16;
			let mut online_nums:u16 = 0;

			let mut infos = Vec::new();
			for node in nodes.into_iter() {
				if let Some(mut masternode_details) = Masternodes::<T>::get(&node) {
					let number:u32 = masternode_details.updated_block_number.try_into().unwrap_or(0);
					if block_number <= number + 600 {
						online_nums += 1;
					}else{
						masternode_details.status = MasternodeStatus::OffLine;
					}
					let peer_id = bs58::encode(&node.0).into_string();
					infos.push((peer_id, masternode_details));
				}
			}
			MasternodeInfo{
				total_nums,
				online_nums,
				infos,
			}
		}
    }
}
