use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, dispatch::Result};
use system::ensure_signed;
use parity_codec::{Encode, Decode};

pub trait Trait: system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[derive(Encode, Decode, Default)]
pub struct TradePair<Hash> {
	hash: Hash,
	base: Hash,
	quoto: Hash,
}

decl_storage! {
	trait Store for Module<T: Trait> as trade {
		TradePairs get(trade_pair): map T::Hash => TradePair<T::Hash>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		TradePairCreated(u32, AccountId),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

		pub fn create_trade_pair(origin, base: T::Hash, quoto: T::Hash) -> Result {
			let who = ensure_signed(origin)?;

			

			Self::deposit_event(RawEvent::TradePairCreated(something, who));
			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {

}