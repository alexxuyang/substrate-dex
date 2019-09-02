use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, dispatch::Result, ensure, Parameter};
use runtime_primitives::traits::{SimpleArithmetic, Bounded, Member, Zero};
use system::ensure_signed;
use parity_codec::{Encode, Decode};
use runtime_primitives::traits::{Hash};
use rstd::{prelude::*, result, cmp::Ordering};
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

pub trait OrderIt {
    fn get_order() -> Ordering;
}

struct Greater {}
struct Less {}

impl OrderIt for Greater {
    fn get_order() -> Ordering {
        Ordering::Greater
    }
}

impl OrderIt for Less {
    fn get_order() -> Ordering {
        Ordering::Less
    }
}

pub struct LinkedList<T, S, K1, K2, O>(rstd::marker::PhantomData<(T, S, K1, K2, O)>);

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
impl<T, S, K1, K2, O> LinkedList<T, S, K1, K2, O> where
	T: Trait,
	K1: Encode + Decode + Clone + rstd::borrow::Borrow<<T as system::Trait>::Hash> + Copy,
	K2: Parameter + Default + Member + Bounded + SimpleArithmetic + Copy,
	S: StorageMap<(K1, Option<K2>), LinkedItem<K1, K2>, Query = Option<LinkedItem<K1, K2>>>,
	O: OrderIt,
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
                    // if key2 > price {
                    if key2.cmp(&price) == O::get_order() {
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

	pub fn remove_all(key1: K1) -> Result {
		let mut it = Self::read_head(key1);

		while let Some(key2) = it.next {
			Self::remove_value(key1, key2)?;
			it = Self::read_head(key1);
		}

		Ok(())
	}

    // when the order is canceled, it should be remove from Sell / Buy orders
    pub fn remove_value(key1: K1, key2: K2) -> Result {
        let mut it = S::get((key1, Some(key2)));

		while let Some(mut item) = it {

			let mut item_orders_length = 0;

			if item.orders.len() > 0 {
				let order_hash = item.orders.get(0);
				ensure!(order_hash.is_some(), "can not get order from index 0 when we want to remove it");

				let order = <Orders<T>>::get(order_hash.unwrap().borrow());
				ensure!(order.is_some(), "can not get order from index 0 when we want to remove it");
				
				let order = order.unwrap();
				ensure!(order.is_finished(), "try to remove not finished order");

				item.orders.remove(0);
				item_orders_length = item.orders.len();

				Self::write(key1, Some(key2), item);
			}

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
				}
				
				break;
			}

			it = S::get((key1, Some(key2)));
		}

		Ok(())
    }
}

type OrderLinkedItem<T> = LinkedItem<<T as system::Trait>::Hash, <T as Trait>::Price>;
type SellOrdersList<T> = LinkedList<T, SellOrders<T>, <T as system::Trait>::Hash, <T as Trait>::Price, Greater>;
type BuyOrdersList<T> = LinkedList<T, BuyOrders<T>, <T as system::Trait>::Hash, <T as Trait>::Price, Less>;

decl_storage! {
	trait Store for Module<T: Trait> as trade {
		//	TradePairHash => TradePair
		TradePairsByHash get(trade_pair_by_hash): map T::Hash => Option<TradePair<T::Hash>>;
		// (BaseTokenHash, quoteTokenHash) => TradePairHash
		TradePairsHashByBasequote get(trade_pair_hash_by_base_quote): map (T::Hash, T::Hash) => Option<T::Hash>;

		// OrderHash => Order
		Orders get(order): map T::Hash => Option<LimitOrder<T>>;
		// (AccoundId, Index) => OrderHash
		OwnedOrders get(owned_order): map (T::AccountId, u64) => Option<T::Hash>;
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
	fn trade_basic_test_case() {
		with_externalities(&mut new_test_ext(), || {
			let ALICE = 10;
			let BOB = 20;
			let CHARLIE = 30;

			// token1
			assert_ok!(TokenModule::issue(Origin::signed(ALICE), b"66".to_vec(), 21000000));
			let token1_hash = TokenModule::owned_token((ALICE, 0)).unwrap();
			let token1 = TokenModule::token(token1_hash).unwrap();

			// token2
			assert_ok!(TokenModule::issue(Origin::signed(BOB), b"77".to_vec(), 10000000));
			let token2_hash = TokenModule::owned_token((BOB, 0)).unwrap();
			let token2 = TokenModule::token(token2_hash).unwrap();

			// tradepair
			let base = token1.hash;
			let quote = token2.hash;
			assert_ok!(TradeModule::create_trade_pair(Origin::signed(ALICE), base, quote));
			let tp_hash = TradeModule::trade_pair_hash_by_base_quote((base, quote)).unwrap();
			let tp = TradeModule::trade_pair_by_hash(tp_hash).unwrap();

			// limit order
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, 
				OrderType::Sell, 18, 100));
			let order1_hash = TradeModule::owned_order((BOB, 0)).unwrap();
			let order1 = TradeModule::order(order1_hash).unwrap();
			assert_eq!(order1.amount, 100);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, 
				OrderType::Sell, 10, 50));
			let order2_hash = TradeModule::owned_order((BOB, 1)).unwrap();
			let mut order2 = TradeModule::order(order2_hash).unwrap();
			assert_eq!(order2.amount, 50);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 5, 10));
			let order3_hash = TradeModule::owned_order((BOB, 2)).unwrap();
			let mut order3 = TradeModule::order(order3_hash).unwrap();
			
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 5, 20));
			let order4_hash = TradeModule::owned_order((BOB, 3)).unwrap();
			let mut order4 = TradeModule::order(order4_hash).unwrap();
			
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 12, 10));
			let order5_hash = TradeModule::owned_order((BOB, 4)).unwrap();
			let order5 = TradeModule::order(order5_hash).unwrap();
			
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 12, 30));
			let order6_hash = TradeModule::owned_order((BOB, 5)).unwrap();
			let order6 = TradeModule::order(order6_hash).unwrap();
			
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 12, 20));
			let order7_hash = TradeModule::owned_order((BOB, 6)).unwrap();
			let order7 = TradeModule::order(order7_hash).unwrap();
			assert_eq!(order7.amount, 20);

			output(tp_hash);

			// Some(5), Some(18), None, 0: 
			// Some(10), None, Some(5), 2: (0x6de6…98b4 : 10, 10), (0x895b…0377 : 20, 20), 
			// Some(12), Some(5), Some(10), 1: (0xc10f…32e3 : 50, 50), 
			// Some(18), Some(10), Some(12), 3: (0xefbf…d851 : 10, 10), (0xe71e…8be1 : 30, 30), (0xbbe2…36b9 : 20, 20), 
			// None, Some(12), Some(18), 1: (0x8439…5abc : 100, 100), 
			let a = 10;

			// price = 5
			order3.remained_amount = Zero::zero();
			order3.status = OrderStatus::Filled;
			<Orders<Test>>::insert(order3.hash, order3);

			order4.status = OrderStatus::Canceled;
			<Orders<Test>>::insert(order4.hash, order4);

			// price = 10
			order2.remained_amount = Zero::zero();
			order2.status = OrderStatus::Filled;
			<Orders<Test>>::insert(order2.hash, order2);

			<SellOrdersList<Test>>::remove_all(tp_hash);

			output(tp_hash);
		});
	}
}
