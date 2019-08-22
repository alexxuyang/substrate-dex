use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, dispatch::Result, ensure, Parameter};
use runtime_primitives::traits::{SimpleArithmetic, Bounded, Member, Zero};
use system::ensure_signed;
use parity_codec::{Encode, Decode};
use runtime_primitives::traits::{Hash};
use rstd::result;
use rstd::prelude::*;
use crate::token;

pub trait Trait: token::Trait + system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type Price: Parameter + Default + Member + Bounded + SimpleArithmetic + Copy;
}

#[derive(Encode, Decode, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct TradePair<Hash> {
	hash: Hash,
	base: Hash,
	quoto: Hash,
}

#[derive(Encode, Decode, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum OrderType {
	Buy,
	Sell,
}

#[derive(Encode, Decode, Clone, PartialEq)]
pub enum OrderStatus {
	Created,
	PartialFilled,
	Filled,
	Canceled,
}

static PriceDecimal: u32 = 8;

#[derive(Encode, Decode)]
pub struct LimitOrder<T> where T: Trait {
	hash: T::Hash,
	base: T::Hash,
	quoto: T::Hash,
	owner: T::AccountId,
	price: T::Price,
	otype: OrderType,
	status: OrderStatus,
}

impl<T> LimitOrder<T> where T: Trait {
	fn new(base: T::Hash, quoto: T::Hash, owner: T::AccountId, price: T::Price, otype: OrderType) -> Self {
		let hash = (<system::Module<T>>::random_seed(), 
					<system::Module<T>>::block_number(), 
					base, quoto, owner.clone(), price, otype)
			.using_encoded(<T as system::Trait>::Hashing::hash);

		LimitOrder {
			hash, base, quoto, owner, price, otype, status: OrderStatus::Created
		}
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as trade {
		TradePairsByHash get(trade_pair_by_hash): map T::Hash => Option<TradePair<T::Hash>>;
		TradePairsHashByBaseQuoto get(trade_pair_hash_by_base_quoto): map (T::Hash, T::Hash) => Option<T::Hash>;

		Orders get(order): map T::Hash => Option<LimitOrder<T>>;
		OwnedOrders get(owned_orders): map (T::AccountId, u64) => Option<T::Hash>;
		OwnedOrdersIndex get(owned_orders_index): map T::AccountId => u64;

		TradePairOwnedOrders get(trade_pair_owned_orders): map (T::Hash, u64) => Option<T::Hash>;
		TradePairOwnedOrdersIndex get(trade_pair_owned_orders_index): map T::Hash => u64;

		Nonce: u64;
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
		TradePairCreated(AccountId, Hash, TradePair),
		OrderCreated(AccountId, Hash, Hash, Hash, Price, Balance), // (alice, orderHash, baseTokenHash, quotoTokenHash, price, balance)
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

		pub fn create_trade_pair(origin, base: T::Hash, quoto: T::Hash) -> Result {
			Self::do_create_trade_pair(origin, base, quoto)
		}

		pub fn create_limit_order(origin, base: T::Hash, quoto: T::Hash, otype: OrderType, price: T::Price, amount: T::Balance) -> Result {
			let sender = ensure_signed(origin)?;
			let tp = Self::ensure_trade_pair(base, quoto)?;

			ensure!(price > Zero::zero(), "price should be great than zero");
			
			let op_token_hash;
			match otype {
				OrderType::Buy => op_token_hash = base,
				OrderType::Sell => op_token_hash = quoto,
			};

			let order = LimitOrder::new(base, quoto, sender.clone(), price, otype);
			let hash  = order.hash;

			<token::Module<T>>::do_freeze(sender.clone(), op_token_hash, amount)?;
			<token::Module<T>>::ensure_free_balance(sender.clone(), op_token_hash, amount)?;
			<Orders<T>>::insert(hash, order);

			let owned_index = Self::owned_orders_index(sender.clone());
			<OwnedOrders<T>>::insert((sender.clone(), owned_index), hash);
			<OwnedOrdersIndex<T>>::insert(sender.clone(), owned_index + 1);

			let tp_owned_index = Self::trade_pair_owned_orders_index(tp.hash);
			<TradePairOwnedOrders<T>>::insert((tp.hash, tp_owned_index), hash);
			<TradePairOwnedOrdersIndex<T>>::insert(tp.hash, tp_owned_index + 1);

			Self::deposit_event(RawEvent::OrderCreated(sender.clone(), hash, base, quoto, price, amount));

			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
	fn get_trade_pair_by_base_quoto(base: T::Hash, quoto: T::Hash) -> Option<TradePair<T::Hash>> {
		let hash = Self::trade_pair_hash_by_base_quoto((base, quoto));

		match hash {
			Some(h) => Self::trade_pair_by_hash(h),
			None => None,
		}
	}

	fn ensure_trade_pair(base: T::Hash, quoto: T::Hash) -> result::Result<TradePair<T::Hash>, &'static str> {
		let bq = Self::get_trade_pair_by_base_quoto(base, quoto);
		ensure!(bq.is_some(), "not trade pair with base & quoto");

		match bq {
			Some(bq) => {
				return Ok(bq)
			},
			None => {
				return Err("not trade pair with base & quoto")
			},
		}
	}

	fn do_create_trade_pair(origin: T::Origin, base: T::Hash, quoto: T::Hash) -> Result {
		let sender = ensure_signed(origin)?;
		
		ensure!(base != quoto, "base and quoto can not be the same token");

		let base_owner = <token::Module<T>>::owner(base);
		let quoto_owner = <token::Module<T>>::owner(quoto);

		ensure!(base_owner.is_some() && quoto_owner.is_some(), "can't find owner of base or quoto token");

		let base_owner = base_owner.unwrap();
		let quoto_owner = quoto_owner.unwrap();
		
		ensure!(sender == base_owner || sender == quoto_owner, "sender should be equal to owner of base or quoto token");

		let bq = Self::get_trade_pair_by_base_quoto(base, quoto);
		let qb = Self::get_trade_pair_by_base_quoto(quoto, base);

		ensure!(!bq.is_some() && !qb.is_some(), "the same trade pair already exists");

		let nonce = <Nonce<T>>::get();

		let hash = (<system::Module<T>>::random_seed(), <system::Module<T>>::block_number(), sender.clone(), base, quoto, nonce)
			.using_encoded(<T as system::Trait>::Hashing::hash);

		let tp = TradePair {
			hash, base, quoto
		};

		<Nonce<T>>::mutate(|n| *n += 1);
		<TradePairsByHash<T>>::insert(hash, tp.clone());
		<TradePairsHashByBaseQuoto<T>>::insert((base, quoto), hash);

		Self::deposit_event(RawEvent::TradePairCreated(sender, hash, tp));

		Ok(())
	}
}