use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, dispatch::Result, ensure};
use system::ensure_signed;
use parity_codec::{Encode, Decode};
use runtime_primitives::traits::{Hash};
use crate::token;

pub trait Trait: token::Trait + system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct TradePair<Hash> {
	hash: Hash,
	base: Hash,
	quoto: Hash,
}

decl_storage! {
	trait Store for Module<T: Trait> as trade {
		TradePairsByHash get(trade_pair_by_hash): map T::Hash => Option<TradePair<T::Hash>>;
		TradePairsByBaseQuoto get(trade_pair_by_base_quoto): map (T::Hash, T::Hash) => Option<TradePair<T::Hash>>;

		Nonce: u64;
	}
}

decl_event!(
	pub enum Event<T> 
	where
		<T as system::Trait>::AccountId,
		<T as system::Trait>::Hash,
		TradePair = TradePair<<T as system::Trait>::Hash>,
	{
		TradePairCreated(AccountId, Hash, TradePair),
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

		pub fn create_trade_pair(origin, base: T::Hash, quoto: T::Hash) -> Result {
			Self::do_create_trade_pair(origin, base, quoto)
		}
	}
}

impl<T: Trait> Module<T> {
	fn do_create_trade_pair(origin: T::Origin, base: T::Hash, quoto: T::Hash) -> Result {
		let sender = ensure_signed(origin)?;
		
		let base_owner = <token::Module<T>>::owner(base);
		let quoto_owner = <token::Module<T>>::owner(quoto);

		ensure!(base_owner.is_some() && quoto_owner.is_some(), "can't find owner of base or quoto token");

		let base_owner = base_owner.unwrap();
		let quoto_owner = quoto_owner.unwrap();
		
		ensure!(sender == base_owner || sender == quoto_owner, "sender should be equal to owner of base or quoto token");

		let bq = Self::trade_pair_by_base_quoto((base, quoto));
		let qb = Self::trade_pair_by_base_quoto((quoto, base));

		ensure!(!bq.is_some() && !qb.is_some(), "the same trade pair already exists");

		let nonce = <Nonce<T>>::get();

		let hash = (<system::Module<T>>::random_seed(), <system::Module<T>>::block_number(), sender.clone(), base, quoto, nonce)
			.using_encoded(<T as system::Trait>::Hashing::hash);

		let tp = TradePair {
			hash, base, quoto
		};

		<Nonce<T>>::mutate(|n| *n += 1);
		<TradePairsByHash<T>>::insert(hash, tp.clone());
		<TradePairsByBaseQuoto<T>>::insert((base, quoto), tp.clone());

		Self::deposit_event(RawEvent::TradePairCreated(sender, hash, tp));

		Ok(())
	}
}