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

	fn is_finished(&self) -> bool {
		(self.remained_amount == Zero::zero() && self.status == OrderStatus::Filled) || self.status == OrderStatus::Canceled
	}
}

#[derive(Encode, Decode, Clone)]
#[cfg_attr(feature="std", derive(PartialEq, Eq, Debug))]
pub struct LinkedItem<K1, K2>
{
    pub prev: Option<K2>,
    pub next: Option<K2>,
    pub price: Option<K2>,
    pub orders: Vec<K1>, // remove the item at 0 index will caused performance issue, should be optimized
}

pub struct LinkedList<T, S, K1, K2>(rstd::marker::PhantomData<(T, S, K1, K2)>);

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
impl<T, S, K1, K2> LinkedList<T, S, K1, K2> where
	T: Trait,
	K1: Encode + Decode + Clone + rstd::borrow::Borrow<<T as system::Trait>::Hash> + Copy,
	K2: Parameter + Default + Member + Bounded + SimpleArithmetic + Copy,
	S: StorageMap<(K1, Option<K2>), LinkedItem<K1, K2>, Query = Option<LinkedItem<K1, K2>>>,
{
    pub fn read_head(key: K1) -> LinkedItem<K1, K2> {
        Self::read(key, None)
    }

    pub fn read(key1: K1, key2: Option<K2>) -> LinkedItem<K1, K2> {

        S::get((key1, key2)).unwrap_or_else(|| {
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

    pub fn write(key1: K1, key2: Option<K2>, item: LinkedItem<K1, K2>) {
        S::insert((key1, key2), item);
    }

    pub fn append(key1: K1, key2: K2, value: K1) {

        let item = S::get((key1, Some(key2)));
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

    // when the order is canceled, it should be remove from Sell / Buy orders
    pub fn remove_value(key1: K1, key2: K2) -> Result {
        let mut it = S::get((key1, Some(key2)));

		loop {
			match it {
				Some(mut item) => {
					ensure!(item.orders.len() > 0, "there is no order when we want to remove it");

					let order_hash = item.orders.get(0);
					ensure!(order_hash.is_some(), "can not get order from index 0 when we want to remove it");

					let order = <Orders<T>>::get(order_hash.unwrap().borrow());
					ensure!(order.is_some(), "can not get order from index 0 when we want to remove it");
					
					let order = order.unwrap();
					ensure!(order.is_finished(), "try to remove not finished order");

					item.orders.remove(0);
					let item_orders_length = item.orders.len();

					Self::write(key1, Some(key2), item);

					if item_orders_length == 0 {
						if let Some(item) = S::take((key1, Some(key2))) {
							S::mutate((key1.clone(), item.prev), |x| {
								if let Some(x) = x {
									x.next = item.next;
								}
							});

							S::mutate((key1.clone(), item.next), |x| {
								if let Some(x) = x {
									x.prev = item.prev;
								}
							});

							return Ok(());
						}
					}

					it = S::get((key1, Some(key2)));
				},
				None => return Err("try to remove order but the order list is NOT FOUND")
			}
		}
    }
}

type OrderLinkedItem<T> = LinkedItem<<T as system::Trait>::Hash, <T as Trait>::Price>;
type SellOrdersList<T> = LinkedList<T, SellOrders<T>, <T as system::Trait>::Hash, <T as Trait>::Price>;
type BuyOrdersList<T> = LinkedList<T, BuyOrders<T>, <T as system::Trait>::Hash, <T as Trait>::Price>;

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
		SellOrders get(sell_order): map (T::Hash, Option<T::Price>) => Option<OrderLinkedItem<T>>;
		BuyOrders get(buy_order): map (T::Hash, Option<T::Price>) => Option<OrderLinkedItem<T>>;

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

			let tp_owned_index = Self::trade_pair_owned_order_index(tp);
			<TradePairOwnedOrders<T>>::insert((tp, tp_owned_index), hash);
			<TradePairOwnedOrdersIndex<T>>::insert(tp, tp_owned_index + 1);

			if otype == OrderType::Buy {
				<BuyOrdersList<T>>::append(tp, price, hash);
			} else {
				<SellOrdersList<T>>::append(tp, price, hash);
			}

			<Nonce<T>>::mutate(|n| *n += 1);

			Self::deposit_event(RawEvent::OrderCreated(sender.clone(), hash, base, quote, price, amount));

			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
	fn ensure_trade_pair(base: T::Hash, quote: T::Hash) -> result::Result<T::Hash, &'static str> {
		let bq = Self::trade_pair_hash_by_base_quote((base, quote));
		ensure!(bq.is_some(), "not trade pair with base & quote");

		match bq {
			Some(bq) => Ok(bq),
			None => Err("not trade pair with base & quote"),
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

		let bq = Self::trade_pair_hash_by_base_quote((base, quote));
		let qb = Self::trade_pair_hash_by_base_quote((quote, base));

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
	#[derive(Clone, Eq, PartialEq, Debug)]
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

	fn output(key: <Test as system::Trait>::Hash) {

		let mut item = <SellOrdersList<Test>>::read_head(key);

		loop {
			print!("{:?}, {:?}, {:?}, {}: ", item.next, item.prev, item.price, item.orders.len());

			let mut orders = item.orders.iter();
			loop {
				match orders.next() {
					Some(order_hash) => {
						let order = <Orders<Test>>::get(order_hash).unwrap();
						print!("({} : {}, {}), ", order.hash, order.amount, order.remained_amount);
					},
					None => break,
				}
			}

			println!("");

			if item.next == None {
				break;
			} else {
				item = SellOrdersList::<Test>::read(key, item.next);
			}
		}

		println!("");
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
			let head = SellOrdersList::<Test>::read_head(tp_hash);
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
			let head = SellOrdersList::<Test>::read_head(tp_hash);
			assert!(head.prev != None);
			assert!(head.next != None);
			assert!(head.price == None);
			assert!(head.orders.len() == 0);

			let item1 = SellOrdersList::<Test>::read(tp_hash, Some(18u64));
			assert!(head.prev == item1.price);
			assert!(head.next == item1.price);
			assert!(item1.price == Some(18u64));
			assert!(item1.next == None);
			assert!(item1.prev == None);
			assert!(item1.orders.len() == 1);
			assert!(item1.orders[0] == order1.hash);

			output(tp_hash);

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

			// have two sell orders
			let head = SellOrdersList::<Test>::read_head(tp_hash);
			assert!(head.price == None);
			assert!(head.orders.len() == 0);

			let item1 = SellOrdersList::<Test>::read(tp_hash, Some(10u64));
			assert!(head.next == item1.price);
			assert!(item1.price == Some(10u64));
			assert!(item1.prev == None);
			assert!(item1.orders.len() == 1);
			assert!(item1.orders[0] == order2.hash);

			let item2 = SellOrdersList::<Test>::read(tp_hash, Some(18u64));
			assert!(head.prev == item2.price);
			assert!(item1.next == item2.price);
			assert!(item2.next == None);
			assert!(item2.prev == item1.price);
			assert!(item2.price == Some(18u64));
			assert!(item2.orders.len() == 1);
			assert!(item2.orders[0] == order1.hash);

			output(tp_hash);

			// assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 18, 100));
			// assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 10, 50));

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 5, 10));
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 5, 20));

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 12, 10));
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 12, 30));
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 12, 20));

			// assert order is the same
			let bob_sell_order_0_18_100_hash = OwnedOrders::<Test>::get((BOB, 0)).unwrap();

			let bob_sell_order_1_10_50_hash = OwnedOrders::<Test>::get((BOB, 1)).unwrap();

			let bob_sell_order_2_5_10_hash = OwnedOrders::<Test>::get((BOB, 2)).unwrap();
			let bob_sell_order_3_5_20_hash = OwnedOrders::<Test>::get((BOB, 3)).unwrap();

			let bob_sell_order_4_12_10_hash = OwnedOrders::<Test>::get((BOB, 4)).unwrap();
			let bob_sell_order_5_12_30_hash = OwnedOrders::<Test>::get((BOB, 5)).unwrap();
			let bob_sell_order_6_12_20_hash = OwnedOrders::<Test>::get((BOB, 6)).unwrap();

			// 0	bob_sell_order_0_18_100_hash
			let mut bob_sell_order_0_18_100_order = Orders::<Test>::get(bob_sell_order_0_18_100_hash).unwrap();
			let order = LimitOrder {
				hash: bob_sell_order_0_18_100_hash,
				base,
				quote,
				owner: BOB,
				price: 18,
				amount: 100,
				remained_amount: 100,
				otype: OrderType::Sell,
				status: OrderStatus::Created,
			};
			assert!(bob_sell_order_0_18_100_order == order);

			// 1	bob_sell_order_1_10_50_hash
			let mut bob_sell_order_1_10_50_order = Orders::<Test>::get(bob_sell_order_1_10_50_hash).unwrap();
			let order = LimitOrder {
				hash: bob_sell_order_1_10_50_hash,
				base,
				quote,
				owner: BOB,
				price: 10,
				amount: 50,
				remained_amount: 50,
				otype: OrderType::Sell,
				status: OrderStatus::Created,
			};
			assert!(bob_sell_order_1_10_50_order == order);

			// 2	bob_sell_order_2_5_10_hash
			let mut bob_sell_order_2_5_10_order = Orders::<Test>::get(bob_sell_order_2_5_10_hash).unwrap();
			let order = LimitOrder {
				hash: bob_sell_order_2_5_10_hash,
				base,
				quote,
				owner: BOB,
				price: 5,
				amount: 10,
				remained_amount: 10,
				otype: OrderType::Sell,
				status: OrderStatus::Created,
			};
			assert!(bob_sell_order_2_5_10_order == order);

			// 3	bob_sell_order_3_5_20_hash
			let mut bob_sell_order_3_5_20_order = Orders::<Test>::get(bob_sell_order_3_5_20_hash).unwrap();
			let order = LimitOrder {
				hash: bob_sell_order_3_5_20_hash,
				base,
				quote,
				owner: BOB,
				price: 5,
				amount: 20,
				remained_amount: 20,
				otype: OrderType::Sell,
				status: OrderStatus::Created,
			};
			assert!(bob_sell_order_3_5_20_order == order);

			// 4	bob_sell_order_4_12_10_hash
			let mut bob_sell_order_4_12_10_order = Orders::<Test>::get(bob_sell_order_4_12_10_hash).unwrap();
			let order = LimitOrder {
				hash: bob_sell_order_4_12_10_hash,
				base,
				quote,
				owner: BOB,
				price: 12,
				amount: 10,
				remained_amount: 10,
				otype: OrderType::Sell,
				status: OrderStatus::Created,
			};
			assert!(bob_sell_order_4_12_10_order == order);

			// 5	bob_sell_order_5_12_30_hash
			let mut bob_sell_order_5_12_30_order = Orders::<Test>::get(bob_sell_order_5_12_30_hash).unwrap();
			let order = LimitOrder {
				hash: bob_sell_order_5_12_30_hash,
				base,
				quote,
				owner: BOB,
				price: 12,
				amount: 30,
				remained_amount: 30,
				otype: OrderType::Sell,
				status: OrderStatus::Created,
			};
			assert!(bob_sell_order_5_12_30_order == order);

			// 6	bob_sell_order_6_12_20_hash
			let mut bob_sell_order_6_12_20_order = Orders::<Test>::get(bob_sell_order_6_12_20_hash).unwrap();
			let order = LimitOrder {
				hash: bob_sell_order_6_12_20_hash,
				base,
				quote,
				owner: BOB,
				price: 12,
				amount: 20,
				remained_amount: 20,
				otype: OrderType::Sell,
				status: OrderStatus::Created,
			};
			assert!(bob_sell_order_6_12_20_order == order);

			// assert LinkedItem is the same
			let head = LinkedItem {
				next: Some(5),
				prev: Some(18),
				price: None,
				orders: Vec::new(),
			};
			assert!(SellOrdersList::<Test>::read_head(tp_hash) == head);

			// price == 5 item
			let mut orders = Vec::new();
			orders.push(bob_sell_order_2_5_10_hash);
			orders.push(bob_sell_order_3_5_20_hash);

			let item1 = LinkedItem {
				next: Some(10),
				prev: None,
				price: Some(5),
				orders: orders,
			};
			assert!(SellOrdersList::<Test>::read(tp_hash, Some(5)) == item1);

			// price == 10 item
			let mut orders = Vec::new();
			orders.push(bob_sell_order_1_10_50_hash);

			let item2 = LinkedItem {
				next: Some(12),
				prev: Some(5),
				price: Some(10),
				orders: orders,
			};
			assert!(SellOrdersList::<Test>::read(tp_hash, Some(10)) == item2);

			// price == 12 item
			let mut orders = Vec::new();
			orders.push(bob_sell_order_4_12_10_hash);
			orders.push(bob_sell_order_5_12_30_hash);
			orders.push(bob_sell_order_6_12_20_hash);

			let item3 = LinkedItem {
				next: Some(18),
				prev: Some(10),
				price: Some(12),
				orders: orders,
			};
			assert!(SellOrdersList::<Test>::read(tp_hash, Some(12)) == item3);

			// price == 18 item
			let mut orders = Vec::new();
			orders.push(bob_sell_order_0_18_100_hash);

			let item4 = LinkedItem {
				next: None,
				prev: Some(12),
				price: Some(18),
				orders: orders,
			};
			assert!(SellOrdersList::<Test>::read(tp_hash, Some(18)) == item4);

			// None(0), 5(2: 10, 20), 10(1: 50), 12(3: 10, 30, 20), 18(1: 100)
			output(tp_hash);

			// set: 5(2: 10(F), 20(F))
			println!("after match price 5");
			bob_sell_order_2_5_10_order.remained_amount = Zero::zero();
			bob_sell_order_2_5_10_order.status = OrderStatus::Filled;
			<Orders<Test>>::insert(bob_sell_order_2_5_10_order.hash, bob_sell_order_2_5_10_order);

			bob_sell_order_3_5_20_order.remained_amount = Zero::zero();
			bob_sell_order_3_5_20_order.status = OrderStatus::Filled;
			<Orders<Test>>::insert(bob_sell_order_3_5_20_order.hash, bob_sell_order_3_5_20_order);
			<SellOrdersList<Test>>::remove_value(tp_hash, 5);

			// None(0), 10(1: 50), 12(3: 10, 30, 20), 18(1: 100)
			output(tp_hash);

			// set: 10(1: 50(C))
			// bob_sell_order_1_10_50_order
			println!("after match price 10");
			bob_sell_order_1_10_50_order.remained_amount = bob_sell_order_1_10_50_order.remained_amount.checked_sub(25).unwrap();
			bob_sell_order_1_10_50_order.status = OrderStatus::Canceled;
			<Orders<Test>>::insert(bob_sell_order_1_10_50_order.hash, bob_sell_order_1_10_50_order);
			<SellOrdersList<Test>>::remove_value(tp_hash, 10);
			
			// None(0), 12(3: 10, 30, 20), 18(1: 100)
			output(tp_hash);

			// set: 12(3: 10(C), 30(F), 20(PF))
			// bob_sell_order_1_10_50_order
			println!("after match price 12");

			bob_sell_order_4_12_10_order.remained_amount = bob_sell_order_4_12_10_order.remained_amount.checked_sub(5).unwrap();
			bob_sell_order_4_12_10_order.status = OrderStatus::Canceled;
			<Orders<Test>>::insert(bob_sell_order_4_12_10_order.hash, bob_sell_order_4_12_10_order);

			bob_sell_order_5_12_30_order.remained_amount = Zero::zero();
			bob_sell_order_5_12_30_order.status = OrderStatus::Filled;
			<Orders<Test>>::insert(bob_sell_order_5_12_30_order.hash, bob_sell_order_5_12_30_order);

			bob_sell_order_6_12_20_order.remained_amount = bob_sell_order_6_12_20_order.remained_amount.checked_sub(15).unwrap();
			bob_sell_order_6_12_20_order.status = OrderStatus::PartialFilled;
			<Orders<Test>>::insert(bob_sell_order_6_12_20_order.hash, bob_sell_order_6_12_20_order.clone());

			<SellOrdersList<Test>>::remove_value(tp_hash, 12);
			
			// None(0), 12(3: 10(C) - removed, 30(F) - removed, 20(PF) - remained: 5), 18(1: 100)
			output(tp_hash);

			// match all and nothing left
			// None(0), 12(20(C)), 18(1: 100(F))
			println!("after match all");
			bob_sell_order_6_12_20_order.status = OrderStatus::Canceled;
			<Orders<Test>>::insert(bob_sell_order_6_12_20_order.hash, bob_sell_order_6_12_20_order);

			bob_sell_order_0_18_100_order.remained_amount = Zero::zero();
			bob_sell_order_0_18_100_order.status = OrderStatus::Filled;
			<Orders<Test>>::insert(bob_sell_order_0_18_100_order.hash, bob_sell_order_0_18_100_order);
			<SellOrdersList<Test>>::remove_value(tp_hash, 12);
			<SellOrdersList<Test>>::remove_value(tp_hash, 18);
			
			// None(0)
			output(tp_hash);
		});
	}
}
