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

	fn is_finished(&self) -> bool {
		(self.remained_amount == Zero::zero() && self.status == OrderStatus::Filled)
		|| self.status == OrderStatus::Canceled
	}
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
#[derive(Encode, Decode, Clone)]
pub struct LinkedItem<T> where T: Trait {
	pub price: Option<T::Price>,
	pub next: Option<T::Price>,
	pub prev: Option<T::Price>,
	pub orders: Vec<T::Hash>,
}

// (TradePairHash, Price) => LinkedItem
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

	pub fn append(key1: T::Hash, key2: T::Price, order_hash: T::Hash) {
		let item = Self::get((key1, Some(key2)));
		match item {
			Some(mut item) => {
				item.orders.push(order_hash);
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

	pub fn remove_value(key1: T::Hash, key2: T::Price) -> Result {
		let mut it = Self::get((key1, Some(key2)));

		loop {
			match it {
				Some(mut item) => {
					ensure!(item.orders.len() > 0, "there is no order when we want to remove it");

					let order_hash = item.orders.get(0);
					ensure!(order_hash.is_some(), "can not get order from index 0 when we want to remove it");

					let order = <Orders<T>>::get(order_hash.unwrap());
					ensure!(order.is_some(), "can not get order from index 0 when we want to remove it");
					
					let order = order.unwrap();
					ensure!(order.is_finished(), "try to remove not finished order");

					item.orders.remove(0);
					let length = item.orders.len();

					Self::write(key1, Some(key2), item);

					if length == 0 {
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

					it = Self::get((key1, Some(key2)));
				},
				None => return Err("try to remove non-exist order")
			}
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

		// (TradePairHash, Price) => LinkedItem
		SellOrders get(sell_order): map (T::Hash, Option<T::Price>) => Option<LinkedItem<T>>;

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

			if otype == OrderType::Buy {
				// <BuyOrders<T>>::append(tp, price, hash);
			} else {
				<SellOrders<T>>::append(tp, price, hash);
			}

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
		/// The type for recording an account's balance.
		type Balance = u128;
		/// What to do if an account's free balance gets zeroed.
		type OnFreeBalanceZero = ();
		/// What to do if a new account is created.
		type OnNewAccount = ();
		/// The uniquitous event type.
		type Event = ();

		type TransactionPayment = ();
		type DustRemoval = ();
		type TransferPayment = ();
	}

	impl token::Trait for Test {
		type Event = ();
	}

	impl Trait for Test {
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

	fn output(key: <Test as system::Trait>::Hash) {

		let mut item = <SellOrders<Test>>::read_head(key);
		loop {
			print!("{:?}, {:?}, {:?}, {}: ", item.prev, item.next, item.price, item.orders.len());

			let mut orders_iter = item.orders.iter();
			loop {
				match orders_iter.next() {
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
				item = SellOrders::<Test>::read(key, item.next);
			}
		}
	}

	#[test]
	fn trade_related_test_case() {
		with_externalities(&mut new_test_ext(), || {
			assert!(1 == 1);

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
			let tp_hash = TradeModule::get_trade_pair_hash_by_base_quote((base, quote)).unwrap();
			let tp = TradeModule::trade_pair_by_hash(tp_hash).unwrap();

			// limit order
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, 
				OrderType::Sell, 18, 100));
			let order_hash = TradeModule::owned_order((BOB, 0)).unwrap();
			let order1 = TradeModule::order(order_hash).unwrap();
			assert_eq!(order1.amount, 100);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, 
				OrderType::Sell, 10, 50));
			let order_hash = TradeModule::owned_order((BOB, 1)).unwrap();
			let mut order2 = TradeModule::order(order_hash).unwrap();
			assert_eq!(order2.amount, 50);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 5, 10));
			let order_hash = TradeModule::owned_order((BOB, 2)).unwrap();
			let mut order3 = TradeModule::order(order_hash).unwrap();
			
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 5, 20));
			let order_hash = TradeModule::owned_order((BOB, 3)).unwrap();
			let mut order4 = TradeModule::order(order_hash).unwrap();
			
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 12, 10));
			let order_hash = TradeModule::owned_order((BOB, 4)).unwrap();
			let order5 = TradeModule::order(order_hash).unwrap();
			
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 12, 30));
			let order_hash = TradeModule::owned_order((BOB, 5)).unwrap();
			let order6 = TradeModule::order(order_hash).unwrap();
			
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 12, 20));
			let order_hash = TradeModule::owned_order((BOB, 6)).unwrap();
			let order7 = TradeModule::order(order_hash).unwrap();
			assert_eq!(order7.amount, 20);

			output(tp_hash);

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

			<SellOrders<Test>>::remove_value(tp_hash, 5);
			<SellOrders<Test>>::remove_value(tp_hash, 10);

			output(tp_hash);
		});
	}
}
