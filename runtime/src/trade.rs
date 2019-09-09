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

#[derive(Debug, Encode, Decode, Clone, PartialEq, Copy)]
pub enum OrderType {
	Buy,
	Sell,
}

#[derive(Encode, Decode, PartialEq, Clone, Copy)]
pub enum OrderStatus {
	Created,
	PartialFilled,
	Filled,
	Canceled,
}

#[derive(Encode, Decode, Clone)]
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

	fn is_finished(&self) -> bool {
		(self.remained_amount == Zero::zero() && self.status == OrderStatus::Filled)
		|| self.status == OrderStatus::Canceled
	}
}

///             LinkedItem          LinkedItem			LinkedItem          LinkedItem          LinkedItem
///             Bottom              Buy Order			Head                Sell Order          Top
///   			Next	    ---->   Price: 8	<----	Prev                Next       ---->    Price: max
///   max <---- Prev				Next		---->	Price:None  <----   Prev                Next        ---->   Price: 0
///         	Price:0		<----   Prev     			Next        ---->   Price 10   <----    Prev
///                                 Orders									Orders
///                                 o1: Hash -> buy 1@5						o101: Hash -> sell 100@10
///                                 o2: Hash -> buy 5@5						o102: Hash -> sell 100@5000
///                                 o3: Hash -> buy 100@5					
///                                 o4: Hash -> buy 40@5
///                                 o5: Hash -> buy 1000@5
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
pub struct LinkedItem<K1, K2> {
	pub price: Option<K2>,
	pub next: Option<K2>,
	pub prev: Option<K2>,
	pub orders: Vec<K1>,
}

pub struct LinkedList<T, S, K1, K2>(rstd::marker::PhantomData<(T, S, K1, K2)>);

// (TradePairHash, Price) => LinkedItem
impl<T, S, K1, K2> LinkedList<T, S, K1, K2> where
	T: Trait,
	K1: Parameter + Member + Copy + Clone + rstd::borrow::Borrow<<T as system::Trait>::Hash>,
	K2: Parameter + Member + Default + Bounded + SimpleArithmetic + Copy,
	S: StorageMap<(K1, Option<K2>), LinkedItem<K1, K2>, Query = Option<LinkedItem<K1, K2>>>,
{
	pub fn read_head(key: K1) -> LinkedItem<K1, K2> {
		Self::read(key, None)
	}

	pub fn read_bottom(key: K1) -> LinkedItem<K1, K2> {
		Self::read(key, Some(K2::min_value()))
	}

	pub fn read_top(key: K1) -> LinkedItem<K1, K2> {
		Self::read(key, Some(K2::max_value()))
	}

	pub fn read(key1: K1, key2: Option<K2>) -> LinkedItem<K1, K2> {
		S::get((key1, key2)).unwrap_or_else(|| {
			let bottom = LinkedItem {
				prev: Some(K2::max_value()),
				next: None,
				price: Some(K2::min_value()),
				orders: Vec::<K1>::new(),
			};
			
			let top = LinkedItem {
				prev: None,
				next: Some(K2::min_value()),
				price: Some(K2::max_value()),
				orders: Vec::<K1>::new(),
			};

			let item = LinkedItem {
				prev: Some(K2::min_value()),
				next: Some(K2::max_value()),
				price: None,
				orders: Vec::<K1>::new(),
			};
			Self::write(key1, key2, item.clone());
			Self::write(key1, bottom.price, bottom);
			Self::write(key1, top.price, top);
			
			item
		})
	}

	pub fn write(key1: K1, key2: Option<K2>, item: LinkedItem<K1, K2>) {
		S::insert((key1, key2), item);
	}

	pub fn append(key1: K1, key2: K2, order_hash: K1, otype: OrderType) {
		let item = S::get((key1, Some(key2)));
		match item {
			Some(mut item) => {
				item.orders.push(order_hash);
				Self::write(key1, Some(key2), item);
				return
			},
			None => {
				let start_item;
				let end_item;

				match otype {
					OrderType::Buy => {
						start_item = Some(K2::min_value());
						end_item = None;
					},
					OrderType::Sell => {
						start_item = None;
						end_item = Some(K2::max_value());
					},
				}

				let mut item = Self::read(key1, start_item);
				while item.next != end_item {
					match item.next {
						None => {},
						Some(price) => {
							if key2 < price {
								break;
							}
						}
					}

					item = Self::read(key1, item.next);
				}

				// add key2 after item

				// update new_prev
				let new_prev = LinkedItem {
					next: Some(key2),
					..item
				};
				Self::write(key1, new_prev.price, new_prev.clone());

				// update new next
				let next = Self::read(key1, item.next);
				let new_next = LinkedItem {
					prev: Some(key2),
					..next
				};
				Self::write(key1, new_next.price, new_next.clone());

				// insert new item
				let mut v = Vec::new();
				v.push(order_hash);
				let item = LinkedItem {
					prev: new_prev.price,
					next: new_next.price,
					price: Some(key2),
					orders: v,
				};
				Self::write(key1, Some(key2), item);
			},
		}
	}

	pub fn remove_items(key1: K1, otype: OrderType) -> Result {
		let end_item;

		if otype == OrderType::Buy {
			end_item = Some(K2::min_value());
		} else {
			end_item = Some(K2::max_value());
		}

		let mut head = Self::read_head(key1);

		loop {
			let key2;
			if otype == OrderType::Buy {
				key2 = head.prev;
			} else {
				key2 = head.next;
			}

			if key2 == end_item {
				break;
			}

			Self::remove_item(key1, key2.unwrap())?;
			head = Self::read_head(key1);
		}

		Ok(())
	}

	pub fn remove_item(key1: K1, key2: K2) -> Result {

		match S::get((key1, Some(key2))) {
			Some(mut item) => {
				while item.orders.len() > 0 {
					let order_hash = item.orders.get(0).ok_or("can not get order hash")?;

					let order = <Orders<T>>::get(order_hash.borrow()).ok_or("can not get order")?;

					ensure!(order.is_finished(), "try to remove not finished order");

					item.orders.remove(0);

					Self::write(key1, Some(key2), item.clone());
				}

				if item.orders.len() == 0 {
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
				}
			},
			None => {},
		}

		Ok(())
	}
}

type OrderLinkedItem<T> = LinkedItem<<T as system::Trait>::Hash, <T as Trait>::Price>;
type OrderLinkedItemList<T> = LinkedList<T, LinkedItemList<T>, <T as system::Trait>::Hash, <T as Trait>::Price>;

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

		// (TradePairHash, Price) => LinkedItem
		LinkedItemList get(sell_order): map (T::Hash, Option<T::Price>) => Option<OrderLinkedItem<T>>;

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

			<OrderLinkedItemList<T>>::append(tp, price, hash, otype);

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

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
	}

	fn output_order(key: <Test as system::Trait>::Hash) {

		let mut item = <OrderLinkedItemList<Test>>::read_bottom(key);

		loop {
			if item.price == Some(<Test as Trait>::Price::min_value()) {
				print!("Bottom ==> ");
			} else if item.price == Some(<Test as Trait>::Price::max_value()) {
				print!("Top ==> ");
			} else if item.price == None {
				print!("Head ==> ");
			}

			print!("Price({:?}), Next({:?}), Prev({:?}), Orders({}): ", item.price, item.next, item.prev, item.orders.len());

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

			if item.next == Some(<Test as Trait>::Price::min_value()) {
				break;
			} else {
				item = OrderLinkedItemList::<Test>::read(key, item.next);
			}
		}

		println!("");
	}

	#[test]
	fn linked_list_test_case() {
		with_externalities(&mut new_test_ext(), || {
			let ALICE = 10;
			let BOB = 20;
			let CHARLIE = 30;

			let max = Some(<Test as Trait>::Price::max_value());
			let min = Some(<Test as Trait>::Price::min_value());

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
			let tp_hash = TradeModule::get_trade_pair_hash_by_base_quote((base, quote)).unwrap();
			let tp = TradeModule::trade_pair_by_hash(tp_hash).unwrap();

			let bottom = LinkedItem {
				prev: max,
				next: None,
				price: min,
				orders: Vec::new(),
			};

			let top = LinkedItem {
				prev: None,
				next: min,
				price: max,
				orders: Vec::new(),
			};

			let head = LinkedItem {
				prev: min,
				next: max,
				price: None,
				orders: Vec::new(),
			};

			assert_eq!(head, <OrderLinkedItemList<Test>>::read_head(tp_hash));
			assert_eq!(bottom, <OrderLinkedItemList<Test>>::read_bottom(tp_hash));
			assert_eq!(top, <OrderLinkedItemList<Test>>::read_top(tp_hash));

			output_order(tp_hash);

			// sell limit order
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 18, 100));
			let order1_hash = TradeModule::owned_order((BOB, 0)).unwrap();
			let mut order1 = TradeModule::order(order1_hash).unwrap();
			assert_eq!(order1.amount, 100);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 10, 50));
			let order2_hash = TradeModule::owned_order((BOB, 1)).unwrap();
			let mut order2 = TradeModule::order(order2_hash).unwrap();
			assert_eq!(order2.amount, 50);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 5, 10));
			let order3_hash = TradeModule::owned_order((BOB, 2)).unwrap();
			let mut order3 = TradeModule::order(order3_hash).unwrap();
			assert_eq!(order3.amount, 10);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 5, 20));
			let order4_hash = TradeModule::owned_order((BOB, 3)).unwrap();
			let mut order4 = TradeModule::order(order4_hash).unwrap();
			assert_eq!(order4.amount, 20);
			
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 12, 10));
			let order5_hash = TradeModule::owned_order((BOB, 4)).unwrap();
			let mut order5 = TradeModule::order(order5_hash).unwrap();
			assert_eq!(order5.amount, 10);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 12, 30));
			let order6_hash = TradeModule::owned_order((BOB, 5)).unwrap();
			let mut order6 = TradeModule::order(order6_hash).unwrap();
			assert_eq!(order6.amount, 30);
			
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 12, 20));
			let order7_hash = TradeModule::owned_order((BOB, 6)).unwrap();
			let mut order7 = TradeModule::order(order7_hash).unwrap();
			assert_eq!(order7.amount, 20);

			// buy limit order
			assert_ok!(TradeModule::create_limit_order(Origin::signed(ALICE), base, quote, OrderType::Buy, 2, 5));
			let order101_hash = TradeModule::owned_order((ALICE, 0)).unwrap();
			let mut order101 = TradeModule::order(order101_hash).unwrap();
			assert_eq!(order101.amount, 5);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(ALICE), base, quote, OrderType::Buy, 1, 12));
			let order102_hash = TradeModule::owned_order((ALICE, 1)).unwrap();
			let mut order102 = TradeModule::order(order102_hash).unwrap();
			assert_eq!(order102.amount, 12);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(ALICE), base, quote, OrderType::Buy, 4, 100));
			let order103_hash = TradeModule::owned_order((ALICE, 2)).unwrap();
			let mut order103 = TradeModule::order(order103_hash).unwrap();
			assert_eq!(order103.amount, 100);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(ALICE), base, quote, OrderType::Buy, 2, 1000000));
			let order104_hash = TradeModule::owned_order((ALICE, 3)).unwrap();
			let mut order104 = TradeModule::order(order104_hash).unwrap();
			assert_eq!(order104.amount, 1000000);

			// head
			let mut item = LinkedItem {
				next: Some(5),
				prev: Some(4),
				price: None,
				orders: Vec::new(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_head(tp_hash), item);

			// item1
			let mut curr = item.next;

			let mut v = Vec::new();
			v.push(order3_hash);
			v.push(order4_hash);

			item = LinkedItem {
				next: Some(10),
				prev: None,
				price: Some(5),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item2
			curr = item.next;

			v = Vec::new();
			v.push(order2_hash);

			item = LinkedItem {
				next: Some(12),
				prev: Some(5),
				price: Some(10),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item3
			curr = item.next;
			
			v = Vec::new();
			v.push(order5_hash);
			v.push(order6_hash);
			v.push(order7_hash);

			item = LinkedItem {
				next: Some(18),
				prev: Some(10),
				price: Some(12),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item4
			curr = item.next;

			v = Vec::new();
			v.push(order1_hash);

			item = LinkedItem {
				next: max,
				prev: Some(12),
				price: Some(18),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// top
			item = LinkedItem {
				next: min,
				prev: Some(18),
				price: max,
				orders: Vec::new(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_top(tp_hash), item);

			// bottom
			item = LinkedItem {
				next: Some(1),
				prev: max,
				price: min,
				orders: Vec::new(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_bottom(tp_hash), item);

			// item1
			let mut curr = item.next;

			let mut v = Vec::new();
			v.push(order102_hash);

			item = LinkedItem {
				next: Some(2),
				prev: min,
				price: Some(1),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item2
			curr = item.next;

			v = Vec::new();
			v.push(order101_hash);
			v.push(order104_hash);

			item = LinkedItem {
				next: Some(4),
				prev: Some(1),
				price: Some(2),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item3
			curr = item.next;
			
			v = Vec::new();
			v.push(order103_hash);

			item = LinkedItem {
				next: None,
				prev: Some(2),
				price: Some(4),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// remove sell orders
			OrderLinkedItemList::<Test>::remove_items(tp_hash, OrderType::Sell);
			OrderLinkedItemList::<Test>::remove_items(tp_hash, OrderType::Buy);

			// Bottom ==> Price(Some(0)), Next(Some(1)), Prev(Some(18446744073709551615)), Orders(0): 
			// Price(Some(1)), Next(Some(2)), Prev(Some(0)), Orders(1): (0x2063…669c : 12, 12), 
			// Price(Some(2)), Next(Some(4)), Prev(Some(1)), Orders(2): (0x5fe7…c31c : 5, 5), (0xb0a8…fb1a : 1000000, 1000000), 
			// Price(Some(4)), Next(None), Prev(Some(2)), Orders(1): (0x4293…b948 : 100, 100), 
			// Head ==> Price(None), Next(Some(5)), Prev(Some(4)), Orders(0): 
			// Price(Some(5)), Next(Some(10)), Prev(None), Orders(2): (0x6de6…98b4 : 10, 10), (0x895b…0377 : 20, 20), 
			// Price(Some(10)), Next(Some(12)), Prev(Some(5)), Orders(1): (0xc10f…32e3 : 50, 50), 
			// Price(Some(12)), Next(Some(18)), Prev(Some(10)), Orders(3): (0xefbf…d851 : 10, 10), (0xe71e…8be1 : 30, 30), (0xbbe2…36b9 : 20, 20), 
			// Price(Some(18)), Next(Some(18446744073709551615)), Prev(Some(12)), Orders(1): (0x8439…5abc : 100, 100), 
			// Top ==> Price(Some(18446744073709551615)), Next(Some(0)), Prev(Some(18)), Orders(0): 
			output_order(tp_hash);

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

			// price = 12
			order5.status = OrderStatus::Canceled;
			<Orders<Test>>::insert(order5.hash, order5);

			order6.remained_amount = order6.remained_amount.checked_sub(1).unwrap();
			order6.status = OrderStatus::PartialFilled;
			<Orders<Test>>::insert(order6.hash, order6.clone());

			OrderLinkedItemList::<Test>::remove_items(tp_hash, OrderType::Sell);

			// head
			item = LinkedItem {
				next: Some(12),
				prev: Some(4),
				price: None,
				orders: Vec::new(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_head(tp_hash), item);

			// item1
			curr = item.next;
			
			v = Vec::new();
			v.push(order6_hash);
			v.push(order7_hash);

			item = LinkedItem {
				next: Some(18),
				prev: None,
				price: Some(12),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item2
			curr = item.next;

			v = Vec::new();
			v.push(order1_hash);

			item = LinkedItem {
				next: max,
				prev: Some(12),
				price: Some(18),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			<OrderLinkedItemList<Test>>::remove_items(tp_hash, OrderType::Sell);

			// Bottom ==> Price(Some(0)), Next(Some(1)), Prev(Some(18446744073709551615)), Orders(0): 
			// Price(Some(1)), Next(Some(2)), Prev(Some(0)), Orders(1): (0x2063…669c : 12, 12), 
			// Price(Some(2)), Next(Some(4)), Prev(Some(1)), Orders(2): (0x5fe7…c31c : 5, 5), (0xb0a8…fb1a : 1000000, 1000000), 
			// Price(Some(4)), Next(None), Prev(Some(2)), Orders(1): (0x4293…b948 : 100, 100), 
			// Head ==> Price(None), Next(Some(12)), Prev(Some(4)), Orders(0): 
			// Price(Some(12)), Next(Some(18)), Prev(None), Orders(2): (0xe71e…8be1 : 30, 29), (0xbbe2…36b9 : 20, 20), 
			// Price(Some(18)), Next(Some(18446744073709551615)), Prev(Some(12)), Orders(1): (0x8439…5abc : 100, 100), 
			// Top ==> Price(Some(18446744073709551615)), Next(Some(0)), Prev(Some(18)), Orders(0): 
			output_order(tp_hash);

			// price = 18
			order1.status = OrderStatus::Canceled;
			<Orders<Test>>::insert(order1.hash, order1);

			// price = 12
			order6.remained_amount = Zero::zero();
			order6.status = OrderStatus::Filled;
			<Orders<Test>>::insert(order6.hash, order6);

			order7.remained_amount = Zero::zero();
			order7.status = OrderStatus::Filled;
			<Orders<Test>>::insert(order7.hash, order7);

			<OrderLinkedItemList<Test>>::remove_items(tp_hash, OrderType::Sell);

			// head
			item = LinkedItem {
				next: max,
				prev: Some(4),
				price: None,
				orders: Vec::new(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_head(tp_hash), item);

			// Bottom ==> Price(Some(0)), Next(Some(1)), Prev(Some(18446744073709551615)), Orders(0): 
			// Price(Some(1)), Next(Some(2)), Prev(Some(0)), Orders(1): (0x2063…669c : 12, 12), 
			// Price(Some(2)), Next(Some(4)), Prev(Some(1)), Orders(2): (0x5fe7…c31c : 5, 5), (0xb0a8…fb1a : 1000000, 1000000), 
			// Price(Some(4)), Next(None), Prev(Some(2)), Orders(1): (0x4293…b948 : 100, 100), 
			// Head ==> Price(None), Next(Some(18446744073709551615)), Prev(Some(4)), Orders(0): 
			// Top ==> Price(Some(18446744073709551615)), Next(Some(0)), Prev(None), Orders(0): 
			output_order(tp_hash);

			// remove buy orders
			// price = 4
			order103.remained_amount = Zero::zero();
			order103.status = OrderStatus::Filled;
			<Orders<Test>>::insert(order103.hash, order103);

			// price = 2
			order101.status = OrderStatus::Canceled;
			<Orders<Test>>::insert(order101.hash, order101);

			order104.remained_amount = order104.remained_amount.checked_sub(100).unwrap();
			order104.status = OrderStatus::PartialFilled;
			<Orders<Test>>::insert(order104.hash, order104.clone());

			<OrderLinkedItemList<Test>>::remove_items(tp_hash, OrderType::Buy);

			// bottom
			item = LinkedItem {
				next: Some(1),
				prev: max,
				price: min,
				orders: Vec::new(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_bottom(tp_hash), item);

			// item1
			let mut curr = item.next;

			let mut v = Vec::new();
			v.push(order102_hash);

			item = LinkedItem {
				next: Some(2),
				prev: min,
				price: Some(1),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item2
			curr = item.next;

			v = Vec::new();
			v.push(order104_hash);

			item = LinkedItem {
				next: None,
				prev: Some(1),
				price: Some(2),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// Bottom ==> Price(Some(0)), Next(Some(1)), Prev(Some(18446744073709551615)), Orders(0): 
			// Price(Some(1)), Next(Some(2)), Prev(Some(0)), Orders(1): (0x2063…669c : 12, 12), 
			// Price(Some(2)), Next(None), Prev(Some(1)), Orders(1): (0xb0a8…fb1a : 1000000, 999900), 
			// Head ==> Price(None), Next(Some(18446744073709551615)), Prev(Some(2)), Orders(0): 
			// Top ==> Price(Some(18446744073709551615)), Next(Some(0)), Prev(None), Orders(0):
			output_order(tp_hash);

			// price = 2
			order104.status = OrderStatus::Canceled;
			<Orders<Test>>::insert(order104.hash, order104);

			// price = 1
			order102.remained_amount = Zero::zero();
			order102.status = OrderStatus::Filled;
			<Orders<Test>>::insert(order102.hash, order102);

			<OrderLinkedItemList<Test>>::remove_items(tp_hash, OrderType::Buy);

			let bottom = LinkedItem {
				prev: max,
				next: None,
				price: min,
				orders: Vec::new(),
			};

			let top = LinkedItem {
				prev: None,
				next: min,
				price: max,
				orders: Vec::new(),
			};

			let head = LinkedItem {
				prev: min,
				next: max,
				price: None,
				orders: Vec::new(),
			};

			assert_eq!(head, <OrderLinkedItemList<Test>>::read_head(tp_hash));
			assert_eq!(bottom, <OrderLinkedItemList<Test>>::read_bottom(tp_hash));
			assert_eq!(top, <OrderLinkedItemList<Test>>::read_top(tp_hash));			

			// Bottom ==> Price(Some(0)), Next(None), Prev(Some(18446744073709551615)), Orders(0): 
			// Head ==> Price(None), Next(Some(18446744073709551615)), Prev(Some(0)), Orders(0): 
			// Top ==> Price(Some(18446744073709551615)), Next(Some(0)), Prev(None), Orders(0):
			output_order(tp_hash);
		});
	}
}