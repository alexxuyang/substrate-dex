use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, dispatch::Result, ensure, Parameter};
use runtime_primitives::traits::{SimpleArithmetic, Bounded, Member, Zero};
use system::ensure_signed;
use parity_codec::{Encode, Decode};
use runtime_primitives::traits::{Hash};
use rstd::result;
use rstd::fmt::Display;
use rstd::prelude::*;
use crate::token;

pub trait Trait: token::Trait + system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type Price: Parameter + Default + Member + Bounded + SimpleArithmetic + Copy + Display;
}

#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct TradePair<Hash> {
	hash: Hash,
	base: Hash,
	quote: Hash,
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
	quote: T::Hash,
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
	fn new(base: T::Hash, quote: T::Hash, owner: T::AccountId, price: T::Price, amount: T::Balance, otype: OrderType) -> Self {
		let nonce = <Nonce<T>>::get();

		let hash = (<system::Module<T>>::random_seed(), 
					<system::Module<T>>::block_number(), 
					base, quote, owner.clone(), price, amount, otype, nonce)
			.using_encoded(<T as system::Trait>::Hashing::hash);

		LimitOrder {
			hash, base, quote, owner, price, otype, amount, status: OrderStatus::Created, remained_amount: amount,
		}
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as trade {
		//	TradePairHash => TradePair
		TradePairsByHash get(trade_pair_by_hash): map T::Hash => Option<TradePair<T::Hash>>;
		// (BaseTokenHash, quoteTokenHash) => TradePairHash
		TradePairsHashByBasequote get(trade_pair_hash_by_base_quote): map (T::Hash, T::Hash) => Option<T::Hash>;

		// OrderHash => Order
		Orders get(order): map T::Hash => Option<LimitOrder<T>>;
		// (AccoundId, Index) => OrderHash
		OwnedOrders get(owned_orders): map (T::AccountId, u64) => Option<T::Hash>;
		//	AccountId => Index
		OwnedOrdersIndex get(owned_orders_index): map T::AccountId => u64;
		
		//	(TradePairHash, Index) => OrderHash
		TradePairOwnedOrders get(trade_pair_owned_order): map (T::Hash, u64) => Option<T::Hash>;
		//	TradePairHash => Index
		TradePairOwnedOrdersIndex get(trade_pair_owned_order_index): map T::Hash => u64;

		// (TradePairHash, Price) => LinkedItem
		SellOrders get(sell_order): map (T::Hash, Option<T::Price>) => Option<LinkedItem<T>>;

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
        Self::get((key1, key2)).unwrap_or_else(|| {
			let item = LinkedItem {
				prev: None,
				next: None,
				price: None,
				orders: Vec::new(),
			};
			Self::write(key1, key2, item.clone());
			item
		})
    }

    pub fn write(key1: T::Hash, key2: Option<T::Price>, item: LinkedItem<T>) {
        Self::insert((key1, key2), item);
    }

	pub fn output(key: T::Hash) {
		let mut item = Self::read_head(key);
		println!("");

		loop {
			print!("{:?}, {:?}, {:?}, {}: ", item.next, item.prev, item.price, item.orders.len());

			let mut orders = item.orders.iter();
			loop {
				match orders.next() {
					Some(order_hash) => {
						let order = <Orders<T>>::get(order_hash).unwrap();
						print!("({} : {:?}), ", order.hash, order.amount);
					},
					None => break,
				}
			}

			println!("");

			if item.next == None {
				break;
			} else {
				item = Self::read(key, item.next);
			}
		}
	}

    pub fn append(key1: T::Hash, key2: T::Price, value: T::Hash) {

        let item = Self::get((key1, Some(key2)));
        match item {
            Some(mut item) => {
                item.orders.push(value);
				Self::write(key1, Some(key2), item);
                return
            },
            None => {
                let mut item = Self::read_head(key1);
                while let Some(price) = item.next {
                    if key2 > price {
                        item = Self::read(key1, item.next);
                    } else {
						break;
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
        let item = Self::get((key1, Some(key2)));
        match item {
            Some(item) => {
                if item.orders.len() != 0 {
                    return
                }
            },
            None => return,
        };

        if let Some(item) = Self::take((key1, Some(key2))) {
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
        let item = Self::get((key1, Some(key2)));
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
		OrderCreated(AccountId, Hash, Hash, Hash, Price, Balance), // (alice, orderHash, baseTokenHash, quoteTokenHash, price, balance)
	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

		pub fn create_trade_pair(origin, base: T::Hash, quote: T::Hash) -> Result {
			Self::do_create_trade_pair(origin, base, quote)
		}

		pub fn create_limit_order(origin, base: T::Hash, quote: T::Hash, otype: OrderType, price: T::Price, amount: T::Balance) -> Result {
			let sender = ensure_signed(origin)?;
			let tp = Self::ensure_trade_pair(base, quote)?;

			ensure!(price > Zero::zero(), "price should be great than zero");
			
			let op_token_hash;
			match otype {
				OrderType::Buy => op_token_hash = base,
				OrderType::Sell => op_token_hash = quote,
			};

			let order = LimitOrder::new(base, quote, sender.clone(), price, amount, otype);
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

			if otype == OrderType::Buy {
				<SellOrders<T>>::append(tp.hash, price, hash);
			} else {
				<SellOrders<T>>::append(tp.hash, price, hash);
			}

			<Nonce<T>>::mutate(|n| *n += 1);

			Self::deposit_event(RawEvent::OrderCreated(sender.clone(), hash, base, quote, price, amount));

			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
	fn get_trade_pair_by_base_quote(base: T::Hash, quote: T::Hash) -> Option<TradePair<T::Hash>> {
		let hash = Self::trade_pair_hash_by_base_quote((base, quote));

		match hash {
			Some(h) => Self::trade_pair_by_hash(h),
			None => None,
		}
	}

	fn ensure_trade_pair(base: T::Hash, quote: T::Hash) -> result::Result<TradePair<T::Hash>, &'static str> {
		let bq = Self::get_trade_pair_by_base_quote(base, quote);
		ensure!(bq.is_some(), "not trade pair with base & quote");

		match bq {
			Some(bq) => {
				return Ok(bq)
			},
			None => {
				return Err("not trade pair with base & quote")
			},
		}
	}

	fn do_create_trade_pair(origin: T::Origin, base: T::Hash, quote: T::Hash) -> Result {
		let sender = ensure_signed(origin)?;
		
		ensure!(base != quote, "base and quote can not be the same token");

		let base_owner = <token::Module<T>>::owner(base);
		let quote_owner = <token::Module<T>>::owner(quote);

		ensure!(base_owner.is_some() && quote_owner.is_some(), "can't find owner of base or quote token");

		let base_owner = base_owner.unwrap();
		let quote_owner = quote_owner.unwrap();
		
		ensure!(sender == base_owner || sender == quote_owner, "sender should be equal to owner of base or quote token");

		let bq = Self::get_trade_pair_by_base_quote(base, quote);
		let qb = Self::get_trade_pair_by_base_quote(quote, base);

		ensure!(!bq.is_some() && !qb.is_some(), "the same trade pair already exists");

		let nonce = <Nonce<T>>::get();

		let hash = (<system::Module<T>>::random_seed(), <system::Module<T>>::block_number(), sender.clone(), base, quote, nonce)
			.using_encoded(<T as system::Trait>::Hashing::hash);

		let tp = TradePair {
			hash, base, quote
		};

		<Nonce<T>>::mutate(|n| *n += 1);
		<TradePairsByHash<T>>::insert(hash, tp.clone());
		<TradePairsHashByBasequote<T>>::insert((base, quote), hash);

		Self::deposit_event(RawEvent::TradePairCreated(sender, hash, tp));

		Ok(())
	}
}

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok, assert_err};
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

	impl balances::Trait for Test {
		type Balance = u128;

		type OnFreeBalanceZero = ();

		type OnNewAccount = ();

		type Event = ();

		type TransactionPayment = ();
		type DustRemoval = ();
		type TransferPayment = ();
	}

	impl token::Trait for Test {
		type Event = ();
	}

	impl super::Trait for Test {
		type Event = ();
		type Price = u64;
	}

	type TokenModule = token::Module<Test>;
	type TradeModule = super::Module<Test>;
	// type SellOrders = super::Module<Test>::SellOrders;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
	}

	#[test]
	fn trade_related_test_case() {
		with_externalities(&mut new_test_ext(), || {
			let ALICE = 10u64;
			let BOB = 20u64;
			let CHARLIE = 30u64;

			// token1
			assert_ok!(TokenModule::issue(Origin::signed(ALICE), b"6677".to_vec(), 21000000));
			assert_eq!(TokenModule::owned_token_index(ALICE), 1);

			let token_hash = TokenModule::owned_token((ALICE, 0));
			assert!(token_hash.is_some());
			let token_hash = token_hash.unwrap();
			let token1 = TokenModule::token(token_hash);
			assert!(token1.is_some());
			let token1 = token1.unwrap();

			assert_eq!(TokenModule::balance_of((ALICE, token1.hash)), 21000000);
			assert_eq!(TokenModule::free_balance_of((ALICE, token1.hash)), 21000000);
			assert_eq!(TokenModule::freezed_balance_of((ALICE, token1.hash)), 0);

			// token2
			assert_ok!(TokenModule::issue(Origin::signed(BOB), b"8899".to_vec(), 10000000));
			assert_eq!(TokenModule::owned_token_index(BOB), 1);

			let token_hash = TokenModule::owned_token((BOB, 0));
			assert!(token_hash.is_some());
			let token_hash = token_hash.unwrap();
			let token2 = TokenModule::token(token_hash);
			assert!(token2.is_some());
			let token2 = token2.unwrap();

			assert_eq!(TokenModule::balance_of((BOB, token2.hash)), 10000000);
			assert_eq!(TokenModule::free_balance_of((BOB, token2.hash)), 10000000);
			assert_eq!(TokenModule::freezed_balance_of((BOB, token2.hash)), 0);

			// trade pair
			let base = token1.hash;
			let quote = token2.hash;

			assert_ok!(TradeModule::create_trade_pair(Origin::signed(ALICE), base, quote));
			let tp_hash = TradeModule::trade_pair_hash_by_base_quote((base, quote));
			assert!(tp_hash.is_some());
			let tp_hash = tp_hash.unwrap();
			let tp = TradeModule::trade_pair_by_hash(tp_hash);
			assert!(tp.is_some());
			let tp = tp.unwrap();

			assert!(tp.base == base);
			assert!(tp.quote == quote);

			// // sell orders is empty
			let head = SellOrders::<Test>::read_head(tp_hash);
			assert!(head.prev == None);
			assert!(head.next == None);
			assert!(head.price == None);
			assert!(head.orders.len() == 0);

			// add one sell limit order
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 18, 100));
			let index = TradeModule::owned_orders_index(BOB);
			let order_hash = TradeModule::owned_orders((BOB, index - 1));
			assert!(order_hash.is_some());
			let order_hash = order_hash.unwrap();
			let order = TradeModule::order(order_hash);
			assert!(order.is_some());
			let order = order.unwrap();
			let order1 = order.clone();
			let o = LimitOrder {
				base, quote, 
				amount: 100,
				otype: OrderType::Sell,
				price: 18,
				remained_amount: 100,
				status: OrderStatus::Created,
				owner: BOB,
				hash: order_hash,
			};
			assert!(order == o);

			let index = TradeModule::trade_pair_owned_order_index(tp.hash);
			let order_hash = TradeModule::trade_pair_owned_order((tp.hash, index - 1));
			assert!(order_hash.is_some());
			let order_hash = order_hash.unwrap();
			let order = TradeModule::order(order_hash);
			assert!(order.is_some());
			let order = order.unwrap();
			assert!(order == o);

			// have one sell orders
			let head = SellOrders::<Test>::read_head(tp_hash);
			assert!(head.prev != None);
			assert!(head.next != None);
			assert!(head.price == None);
			assert!(head.orders.len() == 0);

			let item1 = SellOrders::<Test>::read(tp_hash, Some(18u64));
			assert!(head.prev == item1.price);
			assert!(head.next == item1.price);
			assert!(item1.price == Some(18u64));
			assert!(item1.next == None);
			assert!(item1.prev == None);
			assert!(item1.orders.len() == 1);
			assert!(item1.orders[0] == order1.hash);

			SellOrders::<Test>::output(tp_hash);

			// add another sell limit order
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 10, 50));
			let index = TradeModule::owned_orders_index(BOB);
			let order_hash = TradeModule::owned_orders((BOB, index - 1));
			assert!(order_hash.is_some());
			let order_hash = order_hash.unwrap();
			let order = TradeModule::order(order_hash);
			assert!(order.is_some());
			let order = order.unwrap();
			let order2 = order.clone();
			let o = LimitOrder {
				base, quote, 
				amount: 50,
				otype: OrderType::Sell,
				price: 10,
				remained_amount: 50,
				status: OrderStatus::Created,
				owner: BOB,
				hash: order_hash,
			};
			assert!(order == o);

			let index = TradeModule::trade_pair_owned_order_index(tp.hash);
			let order_hash = TradeModule::trade_pair_owned_order((tp.hash, index - 1));
			assert!(order_hash.is_some());
			let order_hash = order_hash.unwrap();
			let order = TradeModule::order(order_hash);
			assert!(order.is_some());
			let order = order.unwrap();
			assert!(order == o);

			// have one sell orders
			let head = SellOrders::<Test>::read_head(tp_hash);
			assert!(head.price == None);
			assert!(head.orders.len() == 0);

			let item1 = SellOrders::<Test>::read(tp_hash, Some(10u64));
			assert!(head.next == item1.price);
			assert!(item1.price == Some(10u64));
			assert!(item1.prev == None);
			assert!(item1.orders.len() == 1);
			assert!(item1.orders[0] == order2.hash);

			let item2 = SellOrders::<Test>::read(tp_hash, Some(18u64));
			assert!(head.prev == item2.price);
			assert!(item1.next == item2.price);
			assert!(item2.next == None);
			assert!(item2.prev == item1.price);
			assert!(item2.price == Some(18u64));
			assert!(item2.orders.len() == 1);
			assert!(item2.orders[0] == order1.hash);

			SellOrders::<Test>::output(tp_hash);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 5, 10));
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 5, 20));

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 12, 10));
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 12, 30));
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 12, 20));

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 20, 10));
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 20, 40));
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 20, 20));
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 20, 30));

			// None(0), 5(2: 10, 20), 10(1: 50), 12(3: 10, 30, 20), 18(1: 100), 20(4: 10, 40, 20, 30)
			SellOrders::<Test>::output(tp_hash);
		});
	}
}
