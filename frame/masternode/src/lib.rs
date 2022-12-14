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

pub use pallet::*;

use frame_support::traits::UnixTime;
use sp_core::OpaquePeerId as PeerId;
// use sp_std::{collections::btree_set::BTreeSet, iter::FromIterator, prelude::*};
pub use weights::WeightInfo;

/*
/// The balance type of this pallet.
pub type BalanceOf<T> = <T as Config>::CurrencyBalance;
*/

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// The module configuration trait
	#[pallet::config]
	pub trait Config: frame_system::Config {
        
        /// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/*
        /// The staking balance.
        type Currency: LockableCurrency<
            Self::AccountId,
            Moment = Self::BlockNumber,
            Balance = Self::CurrencyBalance,
        >;
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
        ///
        /// It is guaranteed to start being called from the first `on_finalize`. Thus value at
        /// genesis is not used.
        */
        type UnixTime: UnixTime;


		/// The maximum number of well known nodes that are allowed to set
		#[pallet::constant]
		type MaxMasternodes: Get<u32>;

		/// The maximum length in bytes of PeerId
		#[pallet::constant]
		type MaxPeerId1Length: Get<u32>;

		/// The origin which can remove a well known node.
		type RemoveOrigin: EnsureOrigin<Self::Origin>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	/// A map that maintains the ownership of each node.
	#[pallet::storage]
	#[pallet::getter(fn masternodes)]
	pub type Masternodes<T: Config> = StorageMap<_, Blake2_128Concat, PeerId, T::AccountId>;


    #[pallet::storage]
    #[pallet::getter(fn something)]
    pub type Something<T> = StorageValue<_, u32>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// The given well known node was added.
		MasternodeRegistered { peer_id: PeerId, who: T::AccountId },
		/// The given well known node was removed.
		MasternodeUnregistered { peer_id: PeerId },
		/// The given well known node was removed.
		MasternodeRemoved { peer_id: PeerId },
    
        SomethingStored(u32, T::AccountId),

    }

	#[pallet::error]
	pub enum Error<T> {
		/// The PeerId is too long.
		PeerIdTooLong,
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
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

        #[pallet::weight((T::WeightInfo::do_something(), DispatchClass::Operational))]
        pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
            // Check that the extrinsic was signed and get the signer.
            // This function will return an error if the extrinsic is not signed.
            // https://docs.substrate.io/main-docs/build/origins/
            let who = ensure_signed(origin)?;
         
            // let _ = T::UnixTime::now().as_millis().saturated_into::<u64>();

            // Update storage.
            <Something<T>>::put(something);

            // Emit an event.
            Self::deposit_event(Event::SomethingStored(something, who));
            // Return a successful DispatchResultWithPostInfo
            Ok(())
        }

        #[pallet::weight((T::WeightInfo::register_masternode(), DispatchClass::Operational))]
		pub fn register_masternode(
			origin: OriginFor<T>,
			node: PeerId,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!(node.0.len() < T::MaxPeerId1Length::get() as usize, Error::<T>::PeerIdTooLong);

            if let Some(_who) =  Masternodes::<T>::get(&node) {
                ensure!(false ,Error::<T>::AlreadyRegistered);
            } else {
			    <Masternodes<T>>::insert(&node, &sender);

			    Self::deposit_event(Event::MasternodeRegistered { peer_id: node, who: sender });
            }
			Ok(())
		}

        #[pallet::weight((T::WeightInfo::unregister_masternode(), DispatchClass::Operational))]
        pub fn unregister_masternode(origin: OriginFor<T>, node: PeerId) -> DispatchResult {
            let sender = ensure_signed(origin)?;

            ensure!(node.0.len() < T::MaxPeerId1Length::get() as usize, Error::<T>::PeerIdTooLong);

            let owner = Masternodes::<T>::get(&node).ok_or(Error::<T>::NotExist)?;
            ensure!(owner == sender, Error::<T>::NotOwner);

            <Masternodes<T>>::remove(&node);

            Self::deposit_event(Event::MasternodeUnregistered { peer_id: node });
            Ok(())
        }

		#[pallet::weight((T::WeightInfo::remove_masternode(), DispatchClass::Operational))]
		pub fn remove_masternode(origin: OriginFor<T>, node: PeerId) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin)?;
			ensure!(node.0.len() < T::MaxPeerId1Length::get() as usize, Error::<T>::PeerIdTooLong);

            let _ = Masternodes::<T>::get(&node).ok_or(Error::<T>::NotExist)?;

			<Masternodes<T>>::remove(&node);

			Self::deposit_event(Event::MasternodeRemoved { peer_id: node });
			Ok(())
		}

	}
}

