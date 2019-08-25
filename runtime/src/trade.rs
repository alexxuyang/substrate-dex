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

#[derive(Encode, Decode, Clone, PartialEq, Eq)]
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
#[cfg_attr(feature = "std", derive(Debug))]
pub enum OrderStatus {
	Created,
	PartialFilled,
	Filled,
	Canceled,
}

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct LimitOrder<T> where T: Trait {
	hash: T::Hash,
	base: T::Hash,
	quoto: T::Hash,
	owner: T::AccountId,
	price: T::Price,
	amount: T::Balance,
	remained_amount: T::Balance,
	otype: OrderType,
	status: OrderStatus,
}

pub trait LimitOrderT<T> where T: Trait {
	fn is_filled(&self) -> bool;
}

impl<T> LimitOrderT<T> for LimitOrder<T> where T: Trait {
	fn is_filled(&self) -> bool {
		(self.remained_amount == Zero::zero() && self.status == OrderStatus::Filled) || self.status == OrderStatus::PartialFilled
	}
}

impl<T> LimitOrder<T> where T: Trait {
	fn new(base: T::Hash, quoto: T::Hash, owner: T::AccountId, price: T::Price, amount: T::Balance, otype: OrderType) -> Self {
		let hash = (<system::Module<T>>::random_seed(), 
					<system::Module<T>>::block_number(), 
					base, quoto, owner.clone(), price, otype)
			.using_encoded(<T as system::Trait>::Hashing::hash);

		LimitOrder {
			hash, base, quoto, owner, price, otype, amount, status: OrderStatus::Created, remained_amount: amount,
		}
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as trade {
		TradePairsByHash get(trade_pair_by_hash): map T::Hash => Option<TradePair<T::Hash>>;	///	TradePairHash => TradePair
		TradePairsHashByBaseQuoto get(trade_pair_hash_by_base_quoto): map (T::Hash, T::Hash) => Option<T::Hash>;	/// (BaseTokenHash, QuotoTokenHash) => TradePairHash

		Orders get(order): map T::Hash => Option<LimitOrder<T>>;	/// OrderHash => Order
		OwnedOrders get(owned_orders): map (T::AccountId, u64) => Option<T::Hash>;	/// (AccoundId, Index) => OrderHash
		OwnedOrdersIndex get(owned_orders_index): map T::AccountId => u64;	///	AccountId => Index

		TradePairOwnedOrders get(trade_pair_owned_order): map (T::Hash, u64) => Option<T::Hash>;	///	(TradePairHash, Index) => OrderHash
		TradePairOwnedOrdersIndex get(trade_pair_owned_order_index): map T::Hash => u64;	///	TradePairHash => Index

		SellOrders get(sell_order): map (T::Hash, Option<T::Price>) => Option<LinkedItem<T>>;	/// (TradePairHash, Price) => LinkedItem

		Nonce: u64;
	}
}

#[derive(Encode, Decode, Clone)]
#[cfg_attr(feature="std", derive(PartialEq, Eq, Debug))]
pub struct LinkedItem<T> where T: Trait
{
    pub prev: Option<T::Price>,
    pub next: Option<T::Price>,
    pub price: Option<T::Price>,
    pub orders: Vec<T::Hash>, // remove the item at 0 index will caused performance issue, should be optimized
}

///             LinkedItem              LinkedItem              LinkedItem              LinkedItem
///             Head                    Item1                   Item2                   Item3
///             Price:None  <--------   Prev                    Next       -------->    Price 20
///             Next        -------->   Price: 5   <--------    Prev                    Next        -------->   Price: None
///   20 <----  Prev                    Next       -------->    Price 10   <--------    Prev
///                                     Orders
///                                     o1: Hash -> sell 20 A at price 5
///                                     o2: Hash -> sell 1 A at price 5
///                                     o3: Hash -> sell 5 A at price 5
///                                     o4: Hash -> sell 200 A at price 5
///                                     o5: Hash -> sell 10 A at price 5
///                                     when do order matching, o1 will match before o2 and so on

/// Self: StorageMap, Key1: TradePairHash, Key2: Price, Value: OrderHash
impl<T: Trait> SellOrders<T> {
    pub fn read_head(key: T::Hash) -> LinkedItem<T> {
        Self::read(key, None)
    }

    pub fn read(key1: T::Hash, key2: Option<T::Price>) -> LinkedItem<T> {
        Self::get(&(key1, key2)).unwrap_or_else(|| LinkedItem {
            prev: None,
            next: None,
            price: None,
            orders: Vec::new(),
        })
    }

    pub fn write(key1: T::Hash, key2: Option<T::Price>, item: LinkedItem<T>) {
        Self::insert(&(key1, key2), item);
    }

    pub fn append(key1: T::Hash, key2: T::Price, value: T::Hash) {

        let item = Self::get(&(key1, Some(key2)));
        match item {
            Some(mut item) => {
                item.orders.push(value);
                return
            },
            None => {
                let mut item = Self::read_head(key1);
                while let Some(price) = item.next {
                    if key2 < price {
                        item = Self::read(key1, item.next);
                    }
                }

                // add key2 after item
                // item(new_prev) -> key2 -> item.next(new_next)

                // update new_prev
                let new_prev = LinkedItem {
                    next: Some(key2), 
                    ..item
                };
                Self::write(key1, new_prev.price, new_prev.clone());

                // update new_next
                let next = Self::read(key1, item.next);
                let new_next = LinkedItem {
                    prev: Some(key2),
                    ..next
                };
                Self::write(key1, new_next.price, new_next.clone());

                // update key2
                let mut v = Vec::new();
                v.push(value);
                let item = LinkedItem {
                    prev: new_prev.price,
                    next: new_next.price,
                    orders: v,
                    price: Some(key2),
                };
                Self::write(key1, Some(key2), item);
            }
        };
    }

    pub fn remove_key2(key1: T::Hash, key2: T::Price) {
        let item = Self::get(&(key1.clone(), Some(key2)));
        match item {
            Some(item) => {
                if item.orders.len() != 0 {
                    return
                }
            },
            None => return,
        };

        if let Some(item) = Self::take(&(key1.clone(), Some(key2))) {
            Self::mutate((key1.clone(), item.prev), |x| {
                if let Some(x) = x {
                    x.next = item.next;
                }
            });

            Self::mutate((key1.clone(), item.next), |x| {
                if let Some(x) = x {
                    x.prev = item.prev;
                }
            });
        }
    }

    /// when we do order match, the first order in LinkedItem will match first
    /// and if this order's remained_amount is zero, then it should be remove from the list
    pub fn remove_value(key1: T::Hash, key2: T::Price) -> Result {
        let item = Self::get(&(key1.clone(), Some(key2)));
        match item {
            Some(mut item) => {
                ensure!(item.orders.len() > 0, "there is no order when we want to remove it");

                let order_hash = item.orders.get(0);
                ensure!(order_hash.is_some(), "can not get order from index 0 when we want to remove it");

                let order_hash = order_hash.unwrap();
				let order = <Orders<T>>::get(order_hash);
				ensure!(order.is_some(), "can not get order from index 0 when we want to remove it");
				
				let order = order.unwrap();
                ensure!(order.is_filled(), "try to remove not filled order");

                item.orders.remove(0);

                Self::write(key1, Some(key2), item);

                Ok(())
            },
            None => Err("try to remove order but the order list is NOT FOUND")
        }
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

			let order = LimitOrder::new(base, quoto, sender.clone(), price, amount, otype);
			let hash  = order.hash;

			<token::Module<T>>::do_freeze(sender.clone(), op_token_hash, amount)?;
			<token::Module<T>>::ensure_free_balance(sender.clone(), op_token_hash, amount)?;
			<Orders<T>>::insert(hash, order);

			let owned_index = Self::owned_orders_index(sender.clone());
			<OwnedOrders<T>>::insert((sender.clone(), owned_index), hash);
			<OwnedOrdersIndex<T>>::insert(sender.clone(), owned_index + 1);

			let tp_owned_index = Self::trade_pair_owned_order_index(tp.hash);
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