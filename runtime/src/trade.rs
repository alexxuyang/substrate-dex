use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, dispatch::Result, ensure, Parameter};
use runtime_primitives::traits::{SimpleArithmetic, Bounded, Member, Zero, CheckedSub};
use system::ensure_signed;
use parity_codec::{Encode, Decode};
use runtime_primitives::traits::{Hash};
use rstd::{prelude::*, result, ops::Not};
use crate::token;

pub trait Trait: token::Trait + system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type Price: Parameter + Default + Member + Bounded + SimpleArithmetic + Copy;
}

#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct TradePair<T> where T: Trait {
	hash: T::Hash,
	base: T::Hash,
	quote: T::Hash,
	buy_one_price: T::Price,
	sell_one_price: T::Price,
}

#[derive(Encode, Decode, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum OrderType {
	Buy,
	Sell,
}

impl Not for OrderType {
	type Output = OrderType;

	fn not(self) -> Self::Output {
		match self {
			OrderType::Sell => OrderType::Buy,
			OrderType::Buy => OrderType::Sell,
		}
	}
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

#[derive(Encode, Decode, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Trade<T> where T: Trait {
	hash: T::Hash,
	base: T::Hash,
	quote: T::Hash,
	buyer: T::AccountId, // have base
	seller: T::AccountId, // have quote
	maker: T::AccountId, // create order first
	taker: T::AccountId, // create order not first
	otype: OrderType, // taker order's type
	price: T::Price, // maker order's price
	base_amount: T::Balance, // base token amount to exchange
	quote_amount: T::Balance, // quote token amount to exchange
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

impl<T> Trade<T> where T: Trait {
	fn new(base: T::Hash, quote: T::Hash, maker_order: &LimitOrder<T>, taker_order: &LimitOrder<T>,
		base_amount: T::Balance, quote_amount: T::Balance) -> Self {
		let nonce = <Nonce<T>>::get();

		let hash = (<system::Module<T>>::random_seed(), <system::Module<T>>::block_number(), nonce,
					maker_order.hash, maker_order.remained_amount, maker_order.owner.clone(),
					taker_order.hash, taker_order.remained_amount, taker_order.owner.clone())
			.using_encoded(<T as system::Trait>::Hashing::hash);

		<Nonce<T>>::mutate(|x| *x += 1);

		let buyer;
		let seller;
		if taker_order.otype == OrderType::Buy {
			buyer = taker_order.owner.clone();
			seller = maker_order.owner.clone();
		} else {
			buyer = maker_order.owner.clone();
			seller = taker_order.owner.clone();
		}

		Trade {
			hash, base, quote, buyer, seller, base_amount, quote_amount,
			maker: maker_order.owner.clone(),
			taker: taker_order.owner.clone(),
			otype: taker_order.otype,
			price: maker_order.price,
		}
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
///                                     
/// when do order matching, o1 will match before o2 and so on

// Self: StorageMap, Key1: TradePairHash, Key2: Price, Value: OrderHash
impl<T, S, K1, K2> LinkedList<T, S, K1, K2> where
	T: Trait,
	K1: Encode + Decode + Clone + rstd::borrow::Borrow<<T as system::Trait>::Hash> + Copy,
	K2: Parameter + Default + Member + Bounded + SimpleArithmetic + Copy,
	S: StorageMap<(K1, Option<K2>), LinkedItem<K1, K2>, Query = Option<LinkedItem<K1, K2>>>,
{
    pub fn read_head(key: K1) -> LinkedItem<K1, K2> {
        Self::read(key, None)
    }

    pub fn read_bottom(key: K1) -> LinkedItem<K1, K2> {
        Self::read(key, Some(K2::min_value()))
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

			let head = LinkedItem {
				prev: Some(K2::min_value()),
				next: Some(K2::max_value()),
				price: None,
				orders: Vec::<K1>::new(),
			};

			Self::write(key1, bottom.price, bottom);
			Self::write(key1, top.price, top);
			Self::write(key1, head.price, head.clone());
			head
		})
    }

    pub fn write(key1: K1, key2: Option<K2>, item: LinkedItem<K1, K2>) {
        S::insert((key1, key2), item);
    }

    pub fn append(key1: K1, key2: K2, value: K1, otype: OrderType) {

        let item = S::get((key1, Some(key2)));
        match item {
            Some(mut item) => {
                item.orders.push(value);
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
					}
				}

                let mut item = Self::read(key1, start_item);

                while item.next != end_item {
					match item.next {
						None => {},
						Some(price) => {
							if key2 < price {
								break;
							}
						},
					}

					item = Self::read(key1, item.next);
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

	pub fn next_match_price(item: &LinkedItem<K1, K2>, otype: OrderType) -> Option<K2> {
		if otype == OrderType::Buy {
			item.prev
		} else {
			item.next
		}
	}

	pub fn remove_items(key1: K1, otype: OrderType) {
		let end_item;

		if otype == OrderType::Buy {
			end_item = Some(K2::min_value());
		} else {
			end_item = Some(K2::max_value());
		}

		let mut head = Self::read_head(key1);

		loop {
			let key2 = Self::next_match_price(&head, otype);
			if key2 == end_item {
				break;
			}

			match Self::remove_item(key1, key2.unwrap()) {
				Ok(_) => {},
				Err(_) => break,
			};
			head = Self::read_head(key1);
		}
	}

    // when the order is canceled, it should be remove from Sell / Buy orders
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
			None => {}
		}

		Ok(())
    }
}

type OrderLinkedItem<T> = LinkedItem<<T as system::Trait>::Hash, <T as Trait>::Price>;
type OrderLinkedItemList<T> = LinkedList<T, LinkedItemList<T>, <T as system::Trait>::Hash, <T as Trait>::Price>;

decl_storage! {
	trait Store for Module<T: Trait> as trade {
		//	TradePairHash => TradePair
		TradePairsByHash get(trade_pair_by_hash): map T::Hash => Option<TradePair<T>>;
		// (BaseTokenHash, quoteTokenHash) => TradePairHash
		TradePairsHashByBasequote get(trade_pair_hash_by_base_quote): map (T::Hash, T::Hash) => Option<T::Hash>;

		// OrderHash => Order
		Orders get(order): map T::Hash => Option<LimitOrder<T>>;
		// (AccoundId, Index) => OrderHash
		OwnedOrders get(owned_order): map (T::AccountId, u64) => Option<T::Hash>;
		//	AccountId => Index
		OwnedOrdersIndex get(owned_orders_index): map T::AccountId => u64;
		// OrderHash => Vec<TradeHash>
		OrderOwnedTrades get(order_owned_trade): map T::Hash => Option<Vec<T::Hash>>;

		//	(TradePairHash, Index) => OrderHash
		TradePairOwnedOrders get(trade_pair_owned_order): map (T::Hash, u64) => Option<T::Hash>;
		//	TradePairHash => Index
		TradePairOwnedOrdersIndex get(trade_pair_owned_order_index): map T::Hash => u64;

		// (TradePairHash, Price) => LinkedItem
		LinkedItemList get(linked_item): map (T::Hash, Option<T::Price>) => Option<OrderLinkedItem<T>>;

		// TradeHash => Trade
		Trades get(trade): map T::Hash => Option<Trade<T>>;

		// AccountId => Vec<TradeHash>
		OwnedTrades get(owned_trade): map T::AccountId => Option<Vec<T::Hash>>;
		// (AccountId, TradePairHash) => Vec<TradeHash>
		OwnedTPTrades get(owned_trade_pair_trade): map (T::AccountId, T::Hash) => Option<Vec<T::Hash>>;

		// TradePairHash => Vec<TradeHash>
		TradePairOwnedTrades get(trade_pair_owned_trade): map T::Hash => Option<Vec<T::Hash>>;

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
		TradePair = TradePair<T>,
	{
		TradePairCreated(AccountId, Hash, TradePair),

		// (accountId, orderHash, baseTokenHash, quoteTokenHash, price, balance)
		OrderCreated(AccountId, Hash, Hash, Hash, Price, Balance),


	}
);

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

		pub fn create_trade_pair(origin, base: T::Hash, quote: T::Hash) -> Result {
			Self::do_create_trade_pair(origin, base, quote)
		}

		pub fn create_limit_order(origin, base: T::Hash, quote: T::Hash, otype: OrderType, price: T::Price, amount: T::Balance) -> Result {
			Self::do_create_limit_order(origin, base, quote, otype, price, amount)
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
			hash, base, quote,
			buy_one_price: Zero::zero(),
			sell_one_price: Zero::zero(),
		};

		<Nonce<T>>::mutate(|n| *n += 1);
		<TradePairsByHash<T>>::insert(hash, tp.clone());
		<TradePairsHashByBasequote<T>>::insert((base, quote), hash);

		Self::deposit_event(RawEvent::TradePairCreated(sender, hash, tp));

		Ok(())
	}

	fn do_create_limit_order(origin: T::Origin, base: T::Hash, quote: T::Hash, otype: OrderType, price: T::Price, amount: T::Balance) -> Result {
		let sender = ensure_signed(origin)?;

		let tp_hash = Self::ensure_trade_pair(base, quote)?;

		ensure!(price > Zero::zero() && price < T::Price::max_value(), "price is out of bound");
		
		let op_token_hash;
		match otype {
			OrderType::Buy => op_token_hash = base,
			OrderType::Sell => op_token_hash = quote,
		};

		let order = LimitOrder::new(base, quote, sender.clone(), price, amount, otype);
		let hash  = order.hash;

		<token::Module<T>>::do_freeze(sender.clone(), op_token_hash, amount)?;
		<token::Module<T>>::ensure_free_balance(sender.clone(), op_token_hash, amount)?;
		<Orders<T>>::insert(hash, order.clone());
		<Nonce<T>>::mutate(|n| *n += 1);
		Self::deposit_event(RawEvent::OrderCreated(sender.clone(), hash, base, quote, price, amount));

		let owned_index = Self::owned_orders_index(sender.clone());
		<OwnedOrders<T>>::insert((sender.clone(), owned_index), hash);
		<OwnedOrdersIndex<T>>::insert(sender.clone(), owned_index + 1);

		let tp_owned_index = Self::trade_pair_owned_order_index(tp_hash);
		<TradePairOwnedOrders<T>>::insert((tp_hash, tp_owned_index), hash);
		<TradePairOwnedOrdersIndex<T>>::insert(tp_hash, tp_owned_index + 1);

		// order match
		let filled = Self::order_match(tp_hash, order.clone())?;

		// add order to the market order list
		if !filled {
			<OrderLinkedItemList<T>>::append(tp_hash, price, hash, otype);
		}

		Ok(())
	}

	fn order_match(tp_hash: T::Hash, mut order: LimitOrder<T>) -> result::Result<bool, &'static str> {
		let mut head = <OrderLinkedItemList<T>>::read_head(tp_hash);

		let end_item_price;
		let otype = order.otype;
		let oprice = order.price;

		if otype == OrderType::Buy {
			end_item_price = Some(T::Price::min_value());
		} else {
			end_item_price = Some(T::Price::max_value());
		}

		let tp = Self::trade_pair_by_hash(tp_hash).ok_or("can not get trade pair")?;
		let give: T::Hash;
		let have: T::Hash;

		match otype {
			OrderType::Buy => {
				give = tp.base;
				have = tp.quote;
			},
			OrderType::Sell => {
				give = tp.quote;
				have = tp.base;
			},
		};

		loop {
			if order.status == OrderStatus::Filled {
				break;
			}

			let item_price = Self::next_match_price(&head, !otype);
			
			if item_price == end_item_price {
				break;
			}

			let item_price = item_price.ok_or("can not unwarp item price")?;

			if !Self::price_matched(oprice, otype, item_price) {
				break
			}

			let item = <LinkedItemList<T>>::get((tp_hash, Some(item_price))).ok_or("can not unwrap linked list item")?;
			for o in item.orders.iter() {

				let mut o = Self::order(o).ok_or("can not get order")?;
				let ex_amount = order.remained_amount.min(o.remained_amount);

				if order.remained_amount == order.amount {
					order.status = OrderStatus::PartialFilled;
				}

				if o.remained_amount == o.amount {
					o.status = OrderStatus::PartialFilled;
				}

				<token::Module<T>>::do_unfreeze(order.owner.clone(), give, ex_amount)?;
				<token::Module<T>>::do_unfreeze(o.owner.clone(), have, ex_amount)?;

				<token::Module<T>>::do_transfer(order.owner.clone(), give, o.owner.clone(), ex_amount)?;
				<token::Module<T>>::do_transfer(o.owner.clone(), have, order.owner.clone(), ex_amount)?;

				order.remained_amount = order.remained_amount.checked_sub(&ex_amount).ok_or("substract error")?;
				o.remained_amount = o.remained_amount.checked_sub(&ex_amount).ok_or("substract error")?;

				if order.remained_amount == Zero::zero() {
					order.status = OrderStatus::Filled;
				}

				if o.remained_amount == Zero::zero() {
					o.status = OrderStatus::Filled;
				}

				<Orders<T>>::insert(order.hash.clone(), order.clone());
				<Orders<T>>::insert(o.hash.clone(), o.clone());

				// save the trade data
				let trade = Trade::new(tp.base, tp.quote, &o, &order, ex_amount, ex_amount);
				<Trades<T>>::insert(trade.hash, trade.clone());

				let mut trades;
				if let Some(ts) = Self::trade_pair_owned_trade(tp_hash) {
					trades = ts;
				} else {
					trades = Vec::<T::Hash>::new();
				}
			
				trades.push(trade.hash);
				<TradePairOwnedTrades<T>>::insert(tp_hash, trades);

				if order.status == OrderStatus::Filled {
					break
				}
			}

			head = <OrderLinkedItemList<T>>::read(tp_hash, Some(item_price));
		}

		// todo: should remove every single item when finish one order match
		<OrderLinkedItemList<T>>::remove_items(tp_hash, !otype);

		if order.status == OrderStatus::Filled {
			Ok(true)
		} else {
			Ok(false)
		}
	}

	fn next_match_price(item: &OrderLinkedItem<T>, otype: OrderType) -> Option<T::Price> {
		if otype == OrderType::Buy {
			item.prev
		} else {
			item.next
		}
	}

	fn price_matched(order_price: T::Price, order_type: OrderType, linked_item_price: T::Price) -> bool {
		match order_type {
			OrderType::Sell => order_price <= linked_item_price,
			OrderType::Buy => order_price >= linked_item_price,
		}
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

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
	}

	fn output_order(tp_hash: <Test as system::Trait>::Hash) {

		let mut item = <OrderLinkedItemList<Test>>::read_bottom(tp_hash);

		println!("[Trade Pair Market Orders]");

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
						print!("({}@[{:?}]: {}, {}), ", order.hash, order.status, order.amount, order.remained_amount);
					},
					None => break,
				}
			}

			println!("");

			if item.next == Some(<Test as Trait>::Price::min_value()) {
				break;
			} else {
				item = OrderLinkedItemList::<Test>::read(tp_hash, item.next);
			}
		}

		println!("[Trade Pair Matched Trades]");

		let trades = TradeModule::trade_pair_owned_trade(tp_hash);
		if let Some(trades) = trades {
			for hash in trades.iter() {
				let trade = <Trades<Test>>::get(hash).unwrap();
				println!("[{}/{}] - {}@{}[{:?}]: [Buyer,Seller][{},{}], [Maker,Taker][{},{}], [Base,Quote][{}, {}]", 
					trade.quote, trade.base, hash, trade.price, trade.otype, trade.buyer, trade.seller, trade.maker, 
					trade.taker, trade.base_amount, trade.quote_amount);
			}
		}

		println!();
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
			let tp_hash = TradeModule::trade_pair_hash_by_base_quote((base, quote)).unwrap();
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

	#[test]
	fn order_match_test_case() {
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
			let tp_hash = TradeModule::trade_pair_hash_by_base_quote((base, quote)).unwrap();
			let tp = TradeModule::trade_pair_by_hash(tp_hash).unwrap();

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 18, 100));
			let order1_hash = TradeModule::owned_order((BOB, 0)).unwrap();
			let mut order1 = TradeModule::order(order1_hash).unwrap();
			assert_eq!(order1.amount, 100);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 10, 10));
			let order2_hash = TradeModule::owned_order((BOB, 1)).unwrap();
			let mut order2 = TradeModule::order(order2_hash).unwrap();
			assert_eq!(order2.amount, 10);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 11, 10));
			let order3_hash = TradeModule::owned_order((BOB, 2)).unwrap();
			let mut order3 = TradeModule::order(order3_hash).unwrap();
			assert_eq!(order3.amount, 10);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 11, 100));
			let order4_hash = TradeModule::owned_order((BOB, 3)).unwrap();
			let mut order4 = TradeModule::order(order4_hash).unwrap();
			assert_eq!(order4.amount, 100);

			output_order(tp_hash);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(ALICE), base, quote, OrderType::Buy, 9, 15));
			let order101_hash = TradeModule::owned_order((ALICE, 0)).unwrap();
			let mut order101 = TradeModule::order(order101_hash).unwrap();
			assert_eq!(order101.amount, 15);

			output_order(tp_hash);
			
			assert_ok!(TradeModule::create_limit_order(Origin::signed(ALICE), base, quote, OrderType::Buy, 12, 50));
			let order102_hash = TradeModule::owned_order((ALICE, 1)).unwrap();
			let mut order102 = TradeModule::order(order102_hash).unwrap();
			assert_eq!(order102.amount, 50);

			output_order(tp_hash);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(ALICE), base, quote, OrderType::Buy, 13, 2000));
			let order103_hash = TradeModule::owned_order((ALICE, 2)).unwrap();
			let mut order103 = TradeModule::order(order103_hash).unwrap();
			assert_eq!(order103.amount, 2000);

			output_order(tp_hash);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 9, 5000));
			let order5_hash = TradeModule::owned_order((BOB, 4)).unwrap();
			let mut order5 = TradeModule::order(order5_hash).unwrap();
			assert_eq!(order5.amount, 5000);

			output_order(tp_hash);
		});
	}
}
