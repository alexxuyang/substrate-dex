use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, dispatch::Result, Parameter, ensure};
use runtime_primitives::traits::{Member, Bounded, SimpleArithmetic, Hash, Zero};
use system::ensure_signed;
use parity_codec::{Encode, Decode};
use rstd::result;
use crate::token;

pub trait Trait: token::Trait + system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type Price: Parameter + Member + Default + Bounded + SimpleArithmetic + Copy;
}

#[derive(Debug, Encode, Decode, Clone, PartialEq, Eq)]
pub struct TradePair<Hash> {
	hash: Hash,
	base: Hash,
	quote: Hash,
}

#[derive(Debug, Encode, Decode, Clone, PartialEq)]
pub enum OrderType {
	Buy,
	Sell,
}

#[derive(Encode, Decode)]
pub enum OrderStatus {
	Created,
	PartialFilled,
	Filled,
	Canceled,
}

#[derive(Encode, Decode)]
pub struct LimitOrder<T> where T: Trait {
	hash: T::Hash,
	base: T::Hash,
	quote: T::Hash,
	owner: T::AccountId,
	price: T::Price,
	amount: T::Balance,
	remained_amount: T::Balance,
	otype: OrderType,
	status: OrderStatus,
}

impl<T> LimitOrder<T> where T: Trait {
	pub fn new(base: T::Hash, quote: T::Hash, owner: T::AccountId, price: T::Price, amount: T::Balance, otype: OrderType) -> Self {
		let hash = (base, quote, price, amount, owner.clone(), <system::Module<T>>::random_seed()).using_encoded(<T as system::Trait>::Hashing::hash);

		LimitOrder {
			hash, base, quote, owner, price, otype, amount,
			status: OrderStatus::Created,
			remained_amount: amount,
		}
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as trade {
		// TradePairHash => TradePair
		TradePairsByHash get(trade_pair_by_hash): map T::Hash => Option<TradePair<T::Hash>>;

		// (BaseTokenHash, QuoteTokenHash) => TradePairHash
		TradePairHashByBaseQuote get(get_trade_pair_hash_by_base_quote): map (T::Hash, T::Hash) => Option<T::Hash>;

		// OrderHash => Order
		Orders get(order): map T::Hash => Option<LimitOrder<T>>;

		// (AccountId, Index) => OrderHash
		OwnedOrders get(owned_order): map (T::AccountId, u64) => Option<T::Hash>;
		// AccountId => Index
		OwnedOrdersIndex get(owned_orders_index): map T::AccountId => u64;

		// (TradePairHash, Index) => OrderHash
		TradePairOwnedOrders get(trade_pair_owned_order): map (T::Hash, u64) => Option<T::Hash>;
		// TradePairHash => Index
		TradePairOwnedOrdersIndex get(trade_pair_owned_orders_index): map T::Hash => u64;

		Nonce: u64;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

		pub fn create_trade_pair(origin, base: T::Hash, quote: T::Hash) -> Result {
			Self::do_create_trade_pair(origin, base, quote)
		}

		pub fn create_limit_order(origin, base: T::Hash, quote: T::Hash, otype: OrderType,
		price: T::Price, amount: T::Balance) -> Result {
			let sender = ensure_signed(origin)?;

			let tp = Self::ensure_trade_pair(base, quote)?;

			ensure!(price > Zero::zero(), "price must be positive");

			let op_token_hash;
			match otype {
				OrderType::Buy => op_token_hash = base,
				OrderType::Sell => op_token_hash = quote,
			};

			let order = LimitOrder::<T>::new(base, quote, sender.clone(), price, amount, otype);

			let hash = order.hash;

			<token::Module<T>>::ensure_free_balance(sender.clone(), op_token_hash, amount)?;
			<token::Module<T>>::do_freeze(sender.clone(), op_token_hash, amount)?;

			<Orders<T>>::insert(hash, order);

			let owned_index = Self::owned_orders_index(sender.clone());
			<OwnedOrders<T>>::insert((sender.clone(), owned_index), hash);
			<OwnedOrdersIndex<T>>::insert(sender.clone(), owned_index + 1);

			let tp_owned_index = Self::trade_pair_owned_orders_index(tp);
			<TradePairOwnedOrders<T>>::insert((tp, tp_owned_index), hash);
			<TradePairOwnedOrdersIndex<T>>::insert(tp, tp_owned_index + 1);

			Self::deposit_event(RawEvent::OrderCreated(sender, base, quote, hash, price, amount));

			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
	pub fn ensure_trade_pair(base: T::Hash, quote: T::Hash) -> result::Result<T::Hash, &'static str> {
		let tp = Self::get_trade_pair_hash_by_base_quote((base, quote));
		ensure!(tp.is_some(), "");

		match tp {
			Some(tp) => Ok(tp),
			None => Err(""),
		}
	}

	pub fn do_create_trade_pair(origin: T::Origin, base: T::Hash, quote: T::Hash) -> Result {
		let sender = ensure_signed(origin)?;

		ensure!(base != quote, "base can not equal to quote");

		let base_owner = <token::Module<T>>::owner(base);
		let quote_owner = <token::Module<T>>::owner(quote);

		ensure!(base_owner.is_some() && quote_owner.is_some(), "");

		let base_owner = base_owner.unwrap();
		let quote_owner = quote_owner.unwrap();

		ensure!(sender == base_owner || sender == quote_owner, "");

		let bq = Self::get_trade_pair_hash_by_base_quote((base, quote));
		let qb = Self::get_trade_pair_hash_by_base_quote((quote, base));

		ensure!(!bq.is_some() && !qb.is_some(), "");

		let nonce = <Nonce<T>>::get();

		let hash = (base, quote, nonce, sender.clone(), <system::Module<T>>::random_seed()).using_encoded(<T as system::Trait>::Hashing::hash);

		let tp = TradePair {
			hash, base, quote
		};

		<Nonce<T>>::mutate(|n| *n += 1);
		<TradePairsByHash<T>>::insert(hash, tp.clone());
		<TradePairHashByBaseQuote<T>>::insert((base, quote), hash);

		Self::deposit_event(RawEvent::TradePairCreated(sender, hash, base, quote, tp));

		Ok(())
	}
}

decl_event!(
	pub enum Event<T> 
	where 
		<T as system::Trait>::AccountId,
		<T as system::Trait>::Hash,
		<T as Trait>::Price,
		<T as balances::Trait>::Balance,
		TradePair = TradePair<<T as system::Trait>::Hash>,
	{
		TradePairCreated(AccountId, Hash, Hash, Hash, TradePair),
		OrderCreated(AccountId, Hash, Hash, Hash, Price, Balance),
	}
);

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok};
	use runtime_primitives::{
		BuildStorage,
		traits::{BlakeTwo256, IdentityLookup},
		testing::{Digest, DigestItem, Header}
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	impl system::Trait for Test {
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type Digest = Digest;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type Event = ();
		type Log = DigestItem;
	}
	impl Trait for Test {
		type Event = ();
	}
	type trade = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
	}

	#[test]
	fn it_works_for_default_value() {
		with_externalities(&mut new_test_ext(), || {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			assert_ok!(trade::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			assert_eq!(trade::something(), Some(42));
		});
	}
}
