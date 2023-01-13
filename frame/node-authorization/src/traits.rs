
use sp_core::OpaquePeerId as PeerId;
use sp_std::collections::btree_set::BTreeSet;

pub trait StorageInterface {
	fn get_nodes() -> BTreeSet<PeerId>;
}
