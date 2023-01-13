use sp_core::OpaquePeerId as PeerId;

use super::*;
use frame_support::{
	pallet_prelude::*,
	traits::{fungible, tokens::BalanceConversion},
};

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub enum MasternodeStatus {
	OnLine,
	OffLine,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct MasternodeDetails<AccountId, BlockNumber> {
	pub owner: AccountId,
	pub created_block_number: BlockNumber,
	pub updated_block_number: BlockNumber,
	pub status: MasternodeStatus,
}

/*
pub trait MasternodeStorage<AccountId, BlockNumber> {
	fn get_masternode(peerId: PeerId) -> Option<MasternodeDetails<AccountId, BlockNumber>>;
}
*/