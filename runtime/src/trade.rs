use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, dispatch::Result, ensure, Parameter};
use runtime_primitives::{Perbill, traits::{SimpleArithmetic, Bounded, Member, Zero, CheckedSub}};
use system::ensure_signed;
use parity_codec::{Encode, Decode};
use runtime_primitives::traits::{Hash, As};
use rstd::{prelude::*, result, ops::Not};
use crate::token;
use crate::types;

pub trait Trait: token::Trait + system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type Price: Parameter + Default + Member + Bounded + SimpleArithmetic + Copy + From<u64> + Into<u64>;
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

const PRICE_FACTOR: u64 = 100_000_000;

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

	pub fn is_finished(&self) -> bool {
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

type OrderLinkedItem<T> = types::LinkedItem<<T as system::Trait>::Hash, <T as Trait>::Price>;
type OrderLinkedItemList<T> = types::LinkedList<T, LinkedItemList<T>, <T as system::Trait>::Hash, <T as Trait>::Price>;

decl_storage! {
	trait Store for Module<T: Trait> as trade {
		//	TradePairHash => TradePair
		TradePairsByHash get(trade_pair_by_hash): map T::Hash => Option<TradePair<T>>;
		// (BaseTokenHash, quoteTokenHash) => TradePairHash
		TradePairsHashByBaseQuote get(trade_pair_hash_by_base_quote): map (T::Hash, T::Hash) => Option<T::Hash>;

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

impl<T: Trait> OrderOwnedTrades<T> {
	fn add_trade(order_hash: T::Hash, trade_hash: T::Hash) {
		let mut trades;
		if let Some(ts) = Self::get(order_hash) {
			trades = ts;
		} else {
			trades = Vec::<T::Hash>::new();
		}

		trades.push(trade_hash);
		<OrderOwnedTrades<T>>::insert(order_hash, trades);
	}
}

impl<T: Trait> OwnedTrades<T> {
	fn add_trade(account_id: T::AccountId, trade_hash: T::Hash) {
		let mut trades;
		if let Some(ts) = Self::get(&account_id) {
			trades = ts;
		} else {
			trades = Vec::<T::Hash>::new();
		}

		trades.push(trade_hash);
		<OwnedTrades<T>>::insert(account_id, trades);
	}
}

impl<T: Trait> TradePairOwnedTrades<T> {
	fn add_trade(tp_hash: T::Hash, trade_hash: T::Hash) {
		let mut trades;
		if let Some(ts) = Self::get(tp_hash) {
			trades = ts;
		} else {
			trades = Vec::<T::Hash>::new();
		}

		trades.push(trade_hash);
		<TradePairOwnedTrades<T>>::insert(tp_hash, trades);
	}
}

impl<T: Trait> OwnedTPTrades<T> {
	fn add_trade(account_id: T::AccountId, tp_hash: T::Hash, trade_hash: T::Hash) {
		// save to trade pair owned trade store
		let mut trades;
		if let Some(ts) = Self::get((account_id.clone(), tp_hash)) {
			trades = ts;
		} else {
			trades = Vec::<T::Hash>::new();
		}

		trades.push(trade_hash);
		<OwnedTPTrades<T>>::insert((account_id, tp_hash), trades);
	}
}

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
	fn ensure_under_bound(otype: OrderType, price:T::Price, amount: T::Balance) -> Result {
		let r: u64;

		match otype {
			OrderType::Buy => {
				r = amount.as_() * PRICE_FACTOR / price.as_();
			},
			OrderType::Sell => {
				r = amount.as_() * price.as_() / PRICE_FACTOR;
			},
		}

		ensure!(r != 0, "under bound check failed");

		Ok(())
	}

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
		<TradePairsHashByBaseQuote<T>>::insert((base, quote), hash);

		Self::deposit_event(RawEvent::TradePairCreated(sender, hash, tp));

		Ok(())
	}

	fn do_create_limit_order(origin: T::Origin, base: T::Hash, quote: T::Hash, otype: OrderType, price: T::Price, amount: T::Balance) -> Result {
		let sender = ensure_signed(origin)?;

		let tp_hash = Self::ensure_trade_pair(base, quote)?;

		ensure!(price > Zero::zero() && price < T::Price::max_value(), "price is out of bound");
		
		Self::ensure_under_bound(otype, price, amount)?;

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

				let (base_qty, quote_qty) = Self::calculate_ex_amount(&o, &order)?;

				let give_qty: T::Balance;
				let have_qty: T::Balance;
				match otype {
					OrderType::Buy => {
						give_qty = base_qty;
						have_qty = quote_qty;
					},
					OrderType::Sell => {
						give_qty = quote_qty;
						have_qty = base_qty;
					},
				};

				if order.remained_amount == order.amount {
					order.status = OrderStatus::PartialFilled;
				}

				if o.remained_amount == o.amount {
					o.status = OrderStatus::PartialFilled;
				}

				<token::Module<T>>::do_unfreeze(order.owner.clone(), give, give_qty)?;
				<token::Module<T>>::do_unfreeze(o.owner.clone(), have, have_qty)?;

				<token::Module<T>>::do_transfer(order.owner.clone(), give, o.owner.clone(), give_qty)?;
				<token::Module<T>>::do_transfer(o.owner.clone(), have, order.owner.clone(), have_qty)?;

				order.remained_amount = order.remained_amount.checked_sub(&give_qty).ok_or("substract error")?;
				o.remained_amount = o.remained_amount.checked_sub(&have_qty).ok_or("substract error")?;

				if order.remained_amount == Zero::zero() {
					order.status = OrderStatus::Filled;
				}

				if o.remained_amount == Zero::zero() {
					o.status = OrderStatus::Filled;
				}

				<Orders<T>>::insert(order.hash.clone(), order.clone());
				<Orders<T>>::insert(o.hash.clone(), o.clone());

				// save the trade data
				let trade = Trade::new(tp.base, tp.quote, &o, &order, base_qty, quote_qty);
				<Trades<T>>::insert(trade.hash, trade.clone());

				// save trade reference data to store
				<OrderOwnedTrades<T>>::add_trade(order.hash, trade.hash);
				<OrderOwnedTrades<T>>::add_trade(o.hash, trade.hash);

				<OwnedTrades<T>>::add_trade(order.owner.clone(), trade.hash);
				<OwnedTrades<T>>::add_trade(o.owner.clone(), trade.hash);

				<OwnedTPTrades<T>>::add_trade(order.owner.clone(), tp_hash, trade.hash);
				<OwnedTPTrades<T>>::add_trade(o.owner.clone(), tp_hash, trade.hash);

				<TradePairOwnedTrades<T>>::add_trade(tp_hash, trade.hash);

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

	fn calculate_ex_amount(maker_order: &LimitOrder<T>, taker_order: &LimitOrder<T>) -> result::Result<(T::Balance, T::Balance), &'static str> {
		let buyer_order;
		let seller_order;
		if taker_order.otype == OrderType::Buy {
			buyer_order = taker_order;
			seller_order = maker_order;
		} else {
			buyer_order = maker_order;
			seller_order = taker_order;
		}

		// todo: overflow checked need
		// todo: optimization need, 
		let mut base_qty: u64 = seller_order.remained_amount.as_() * maker_order.price.into() / PRICE_FACTOR;
		if buyer_order.remained_amount.as_() >= base_qty {
			let remainder = buyer_order.remained_amount.as_() - base_qty;
			if remainder != 0 {
				match Self::ensure_under_bound(OrderType::Buy, maker_order.price, <T::Balance as As<u64>>::sa(remainder)) {
					Ok(()) => {},
					Err(_) => {
						base_qty = base_qty + remainder;
					},
				}
			}

			return Ok((<T::Balance as As<u64>>::sa(base_qty), seller_order.remained_amount))
		}

		let mut quote_qty: u64 = buyer_order.remained_amount.as_() * PRICE_FACTOR / maker_order.price.into();
		if seller_order.remained_amount.as_() >= quote_qty {
			let remainder = seller_order.remained_amount.as_() - quote_qty;
			if remainder != 0 {
				match Self::ensure_under_bound(OrderType::Sell, maker_order.price, <T::Balance as As<u64>>::sa(remainder)) {
					Ok(()) => {},
					Err(_) => {
						quote_qty = quote_qty + remainder;
					},
				}
			}

			return Ok((buyer_order.remained_amount, <T::Balance as As<u64>>::sa(quote_qty)))
		}

		// should never executed here
		return Err("should never executed here")
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

		println!("[Market Orders]");

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

		println!("[Market Trades]");

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

			let bottom = OrderLinkedItem::<Test> {
				prev: max,
				next: None,
				price: min,
				orders: Vec::new(),
			};

			let top = OrderLinkedItem::<Test> {
				prev: None,
				next: min,
				price: max,
				orders: Vec::new(),
			};

			let head = OrderLinkedItem::<Test> {
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
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 180000000, 100));
			let order1_hash = TradeModule::owned_order((BOB, 0)).unwrap();
			let mut order1 = TradeModule::order(order1_hash).unwrap();
			assert_eq!(order1.amount, 100);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 100000000, 50));
			let order2_hash = TradeModule::owned_order((BOB, 1)).unwrap();
			let mut order2 = TradeModule::order(order2_hash).unwrap();
			assert_eq!(order2.amount, 50);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 50000000, 10));
			let order3_hash = TradeModule::owned_order((BOB, 2)).unwrap();
			let mut order3 = TradeModule::order(order3_hash).unwrap();
			assert_eq!(order3.amount, 10);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 50000000, 20));
			let order4_hash = TradeModule::owned_order((BOB, 3)).unwrap();
			let mut order4 = TradeModule::order(order4_hash).unwrap();
			assert_eq!(order4.amount, 20);
			
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 120000000, 10));
			let order5_hash = TradeModule::owned_order((BOB, 4)).unwrap();
			let mut order5 = TradeModule::order(order5_hash).unwrap();
			assert_eq!(order5.amount, 10);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 120000000, 30));
			let order6_hash = TradeModule::owned_order((BOB, 5)).unwrap();
			let mut order6 = TradeModule::order(order6_hash).unwrap();
			assert_eq!(order6.amount, 30);
			
			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 120000000, 20));
			let order7_hash = TradeModule::owned_order((BOB, 6)).unwrap();
			let mut order7 = TradeModule::order(order7_hash).unwrap();
			assert_eq!(order7.amount, 20);

			// buy limit order
			assert_ok!(TradeModule::create_limit_order(Origin::signed(ALICE), base, quote, OrderType::Buy, 20000000, 5));
			let order101_hash = TradeModule::owned_order((ALICE, 0)).unwrap();
			let mut order101 = TradeModule::order(order101_hash).unwrap();
			assert_eq!(order101.amount, 5);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(ALICE), base, quote, OrderType::Buy, 10000000, 12));
			let order102_hash = TradeModule::owned_order((ALICE, 1)).unwrap();
			let mut order102 = TradeModule::order(order102_hash).unwrap();
			assert_eq!(order102.amount, 12);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(ALICE), base, quote, OrderType::Buy, 40000000, 100));
			let order103_hash = TradeModule::owned_order((ALICE, 2)).unwrap();
			let mut order103 = TradeModule::order(order103_hash).unwrap();
			assert_eq!(order103.amount, 100);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(ALICE), base, quote, OrderType::Buy, 20000000, 1000000));
			let order104_hash = TradeModule::owned_order((ALICE, 3)).unwrap();
			let mut order104 = TradeModule::order(order104_hash).unwrap();
			assert_eq!(order104.amount, 1000000);

			// head
			let mut item = OrderLinkedItem::<Test> {
				next: Some(50000000),
				prev: Some(40000000),
				price: None,
				orders: Vec::new(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_head(tp_hash), item);

			// item1
			let mut curr = item.next;

			let mut v = Vec::new();
			v.push(order3_hash);
			v.push(order4_hash);

			item = OrderLinkedItem::<Test> {
				next: Some(100000000),
				prev: None,
				price: Some(50000000),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item2
			curr = item.next;

			v = Vec::new();
			v.push(order2_hash);

			item = OrderLinkedItem::<Test> {
				next: Some(120000000),
				prev: Some(50000000),
				price: Some(100000000),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item3
			curr = item.next;
			
			v = Vec::new();
			v.push(order5_hash);
			v.push(order6_hash);
			v.push(order7_hash);

			item = OrderLinkedItem::<Test> {
				next: Some(180000000),
				prev: Some(100000000),
				price: Some(120000000),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item4
			curr = item.next;

			v = Vec::new();
			v.push(order1_hash);

			item = OrderLinkedItem::<Test> {
				next: max,
				prev: Some(120000000),
				price: Some(180000000),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// top
			item = OrderLinkedItem::<Test> {
				next: min,
				prev: Some(180000000),
				price: max,
				orders: Vec::new(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_top(tp_hash), item);

			// bottom
			item = OrderLinkedItem::<Test> {
				next: Some(10000000),
				prev: max,
				price: min,
				orders: Vec::new(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_bottom(tp_hash), item);

			// item1
			let mut curr = item.next;

			let mut v = Vec::new();
			v.push(order102_hash);

			item = OrderLinkedItem::<Test> {
				next: Some(20000000),
				prev: min,
				price: Some(10000000),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item2
			curr = item.next;

			v = Vec::new();
			v.push(order101_hash);
			v.push(order104_hash);

			item = OrderLinkedItem::<Test> {
				next: Some(40000000),
				prev: Some(10000000),
				price: Some(20000000),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item3
			curr = item.next;
			
			v = Vec::new();
			v.push(order103_hash);

			item = OrderLinkedItem::<Test> {
				next: None,
				prev: Some(20000000),
				price: Some(40000000),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// remove sell orders
			OrderLinkedItemList::<Test>::remove_items(tp_hash, OrderType::Sell);
			OrderLinkedItemList::<Test>::remove_items(tp_hash, OrderType::Buy);

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(Some(10000000)), Prev(Some(18446744073709551615)), Orders(0): 
			// Price(Some(10000000)), Next(Some(20000000)), Prev(Some(0)), Orders(1): (0x89fb…7391@[Created]: 12, 12), 
			// Price(Some(20000000)), Next(Some(40000000)), Prev(Some(10000000)), Orders(2): (0x7f72…98d7@[Created]: 5, 5), (0x8f83…7345@[Created]: 1000000, 1000000), 
			// Price(Some(40000000)), Next(None), Prev(Some(20000000)), Orders(1): (0xefa4…3711@[Created]: 100, 100), 
			// Head ==> Price(None), Next(Some(50000000)), Prev(Some(40000000)), Orders(0): 
			// Price(Some(50000000)), Next(Some(100000000)), Prev(None), Orders(2): (0xde34…f782@[Created]: 10, 10), (0x028e…cd82@[Created]: 20, 20), 
			// Price(Some(100000000)), Next(Some(120000000)), Prev(Some(50000000)), Orders(1): (0xee57…8e5b@[Created]: 50, 50), 
			// Price(Some(120000000)), Next(Some(180000000)), Prev(Some(100000000)), Orders(3): (0x13a1…4a5d@[Created]: 10, 10), (0x32e4…0c9a@[Created]: 30, 30), (0x40a2…c87d@[Created]: 20, 20), 
			// Price(Some(180000000)), Next(Some(18446744073709551615)), Prev(Some(120000000)), Orders(1): (0xb990…c35a@[Created]: 100, 100), 
			// Top ==> Price(Some(18446744073709551615)), Next(Some(0)), Prev(Some(180000000)), Orders(0): 
			// [Market Trades]
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
			item = OrderLinkedItem::<Test> {
				next: Some(120000000),
				prev: Some(40000000),
				price: None,
				orders: Vec::new(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_head(tp_hash), item);

			// item1
			curr = item.next;
			
			v = Vec::new();
			v.push(order6_hash);
			v.push(order7_hash);

			item = OrderLinkedItem::<Test> {
				next: Some(180000000),
				prev: None,
				price: Some(120000000),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item2
			curr = item.next;

			v = Vec::new();
			v.push(order1_hash);

			item = OrderLinkedItem::<Test> {
				next: max,
				prev: Some(120000000),
				price: Some(180000000),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			<OrderLinkedItemList<Test>>::remove_items(tp_hash, OrderType::Sell);

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(Some(10000000)), Prev(Some(18446744073709551615)), Orders(0): 
			// Price(Some(10000000)), Next(Some(20000000)), Prev(Some(0)), Orders(1): (0x89fb…7391@[Created]: 12, 12), 
			// Price(Some(20000000)), Next(Some(40000000)), Prev(Some(10000000)), Orders(2): (0x7f72…98d7@[Created]: 5, 5), (0x8f83…7345@[Created]: 1000000, 1000000), 
			// Price(Some(40000000)), Next(None), Prev(Some(20000000)), Orders(1): (0xefa4…3711@[Created]: 100, 100), 
			// Head ==> Price(None), Next(Some(120000000)), Prev(Some(40000000)), Orders(0): 
			// Price(Some(120000000)), Next(Some(180000000)), Prev(None), Orders(2): (0x32e4…0c9a@[PartialFilled]: 30, 29), (0x40a2…c87d@[Created]: 20, 20), 
			// Price(Some(180000000)), Next(Some(18446744073709551615)), Prev(Some(120000000)), Orders(1): (0xb990…c35a@[Created]: 100, 100), 
			// Top ==> Price(Some(18446744073709551615)), Next(Some(0)), Prev(Some(180000000)), Orders(0): 
			// [Market Trades]
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
			item = OrderLinkedItem::<Test> {
				next: max,
				prev: Some(40000000),
				price: None,
				orders: Vec::new(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_head(tp_hash), item);

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(Some(10000000)), Prev(Some(18446744073709551615)), Orders(0): 
			// Price(Some(10000000)), Next(Some(20000000)), Prev(Some(0)), Orders(1): (0x89fb…7391@[Created]: 12, 12), 
			// Price(Some(20000000)), Next(Some(40000000)), Prev(Some(10000000)), Orders(2): (0x7f72…98d7@[Created]: 5, 5), (0x8f83…7345@[Created]: 1000000, 1000000), 
			// Price(Some(40000000)), Next(None), Prev(Some(20000000)), Orders(1): (0xefa4…3711@[Created]: 100, 100), 
			// Head ==> Price(None), Next(Some(18446744073709551615)), Prev(Some(40000000)), Orders(0): 
			// Top ==> Price(Some(18446744073709551615)), Next(Some(0)), Prev(None), Orders(0): 
			// [Market Trades]
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
			item = OrderLinkedItem::<Test> {
				next: Some(10000000),
				prev: max,
				price: min,
				orders: Vec::new(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_bottom(tp_hash), item);

			// item1
			let mut curr = item.next;

			let mut v = Vec::new();
			v.push(order102_hash);

			item = OrderLinkedItem::<Test> {
				next: Some(20000000),
				prev: min,
				price: Some(10000000),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item2
			curr = item.next;

			v = Vec::new();
			v.push(order104_hash);

			item = OrderLinkedItem::<Test> {
				next: None,
				prev: Some(10000000),
				price: Some(20000000),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(Some(10000000)), Prev(Some(18446744073709551615)), Orders(0): 
			// Price(Some(10000000)), Next(Some(20000000)), Prev(Some(0)), Orders(1): (0x89fb…7391@[Created]: 12, 12), 
			// Price(Some(20000000)), Next(None), Prev(Some(10000000)), Orders(1): (0x8f83…7345@[PartialFilled]: 1000000, 999900), 
			// Head ==> Price(None), Next(Some(18446744073709551615)), Prev(Some(20000000)), Orders(0): 
			// Top ==> Price(Some(18446744073709551615)), Next(Some(0)), Prev(None), Orders(0): 
			// [Market Trades]
			output_order(tp_hash);

			// price = 2
			order104.status = OrderStatus::Canceled;
			<Orders<Test>>::insert(order104.hash, order104);

			// price = 1
			order102.remained_amount = Zero::zero();
			order102.status = OrderStatus::Filled;
			<Orders<Test>>::insert(order102.hash, order102);

			<OrderLinkedItemList<Test>>::remove_items(tp_hash, OrderType::Buy);

			let bottom = OrderLinkedItem::<Test> {
				prev: max,
				next: None,
				price: min,
				orders: Vec::new(),
			};

			let top = OrderLinkedItem::<Test> {
				prev: None,
				next: min,
				price: max,
				orders: Vec::new(),
			};

			let head = OrderLinkedItem::<Test> {
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
	fn order_match_linked_list_test_case() {
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

			let bottom = OrderLinkedItem::<Test> {
				prev: max,
				next: None,
				price: min,
				orders: Vec::new(),
			};

			let top = OrderLinkedItem::<Test> {
				prev: None,
				next: min,
				price: max,
				orders: Vec::new(),
			};

			let head = OrderLinkedItem::<Test> {
				prev: min,
				next: max,
				price: None,
				orders: Vec::new(),
			};

			assert_eq!(head, <OrderLinkedItemList<Test>>::read_head(tp_hash));
			assert_eq!(bottom, <OrderLinkedItemList<Test>>::read_bottom(tp_hash));
			assert_eq!(top, <OrderLinkedItemList<Test>>::read_top(tp_hash));	

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(None), Prev(Some(18446744073709551615)), Orders(0): 
			// Head ==> Price(None), Next(Some(18446744073709551615)), Prev(Some(0)), Orders(0): 
			// Top ==> Price(Some(18446744073709551615)), Next(Some(0)), Prev(None), Orders(0): 
			// [Market Trades]
			output_order(tp_hash);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 18000000, 200));
			let order1_hash = TradeModule::owned_order((BOB, 0)).unwrap();
			let mut order1 = TradeModule::order(order1_hash).unwrap();
			assert_eq!(order1.amount, 200);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 10000000, 11));
			let order2_hash = TradeModule::owned_order((BOB, 1)).unwrap();
			let mut order2 = TradeModule::order(order2_hash).unwrap();
			assert_eq!(order2.amount, 11);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 11000000, 10));
			let order3_hash = TradeModule::owned_order((BOB, 2)).unwrap();
			let mut order3 = TradeModule::order(order3_hash).unwrap();
			assert_eq!(order3.amount, 10);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(BOB), base, quote, OrderType::Sell, 11000000, 10000));
			let order4_hash = TradeModule::owned_order((BOB, 3)).unwrap();
			let mut order4 = TradeModule::order(order4_hash).unwrap();
			assert_eq!(order4.amount, 10000);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(ALICE), base, quote, OrderType::Buy, 6000000, 50));
			let order101_hash = TradeModule::owned_order((ALICE, 0)).unwrap();
			let mut order101 = TradeModule::order(order101_hash).unwrap();
			assert_eq!(order101.amount, 50);

			// bottom
			let mut item = OrderLinkedItem::<Test> {
				next: Some(6000000),
				prev: max,
				price: min,
				orders: Vec::new(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_bottom(tp_hash), item);

			// item1
			let mut curr = item.next;

			let mut v = Vec::new();
			v.push(order101_hash);

			item = OrderLinkedItem::<Test> {
				next: None,
				prev: Some(0),
				price: Some(6000000),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item2
			curr = item.next;

			v = Vec::new();

			item = OrderLinkedItem::<Test> {
				next: Some(10000000),
				prev: Some(6000000),
				price: None,
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item3
			curr = item.next;
			
			v = Vec::new();
			v.push(order2_hash);

			item = OrderLinkedItem::<Test> {
				next: Some(11000000),
				prev: None,
				price: Some(10000000),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item4
			curr = item.next;

			v = Vec::new();
			v.push(order3_hash);
			v.push(order4_hash);

			item = OrderLinkedItem::<Test> {
				next: Some(18000000),
				prev: Some(10000000),
				price: Some(11000000),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item5
			curr = item.next;

			v = Vec::new();
			v.push(order1_hash);

			item = OrderLinkedItem::<Test> {
				next: max,
				prev: Some(11000000),
				price: Some(18000000),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// top
			item = OrderLinkedItem::<Test> {
				next: min,
				prev: Some(18000000),
				price: max,
				orders: Vec::new(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_top(tp_hash), item);

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(Some(6000000)), Prev(Some(18446744073709551615)), Orders(0): 
			// Price(Some(6000000)), Next(None), Prev(Some(0)), Orders(1): (0x3762…e537@[Created]: 50, 50), 
			// Head ==> Price(None), Next(Some(10000000)), Prev(Some(6000000)), Orders(0): 
			// Price(Some(10000000)), Next(Some(11000000)), Prev(None), Orders(1): (0xbe2b…9cf5@[Created]: 11, 11), 
			// Price(Some(11000000)), Next(Some(18000000)), Prev(Some(10000000)), Orders(2): (0xb383…6092@[Created]: 10, 10), (0x2a56…7abe@[Created]: 10000, 10000), 
			// Price(Some(18000000)), Next(Some(18446744073709551615)), Prev(Some(11000000)), Orders(1): (0x8f2a…03ed@[Created]: 200, 200), 
			// Top ==> Price(Some(18446744073709551615)), Next(Some(0)), Prev(Some(18000000)), Orders(0): 
			// [Market Trades]
			output_order(tp_hash);
	
			assert_ok!(TradeModule::create_limit_order(Origin::signed(ALICE), base, quote, OrderType::Buy, 11000000, 51));

			let order102_hash = TradeModule::owned_order((ALICE, 1)).unwrap();
			let mut order102 = TradeModule::order(order102_hash).unwrap();
			assert_eq!(order102.amount, 51);
			assert_eq!(order102.remained_amount, 0);
			assert_eq!(order102.status, OrderStatus::Filled);

			order2 = TradeModule::order(order2_hash).unwrap();
			assert_eq!(order2.amount, 11);
			assert_eq!(order2.remained_amount, 0);
			assert_eq!(order2.status, OrderStatus::Filled);

			order3 = TradeModule::order(order3_hash).unwrap();
			assert_eq!(order3.amount, 10);
			assert_eq!(order3.remained_amount, 0);
			assert_eq!(order3.status, OrderStatus::Filled);

			order4 = TradeModule::order(order4_hash).unwrap();
			assert_eq!(order4.amount, 10000);
			assert_eq!(order4.remained_amount, 10000 - ((51 - 1 - 1) as f64 / 0.11 as f64) as u128);
			assert_eq!(order4.status, OrderStatus::PartialFilled);

			// bottom
			let mut item = OrderLinkedItem::<Test> {
				next: Some(6000000),
				prev: max,
				price: min,
				orders: Vec::new(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_bottom(tp_hash), item);

			// item1
			let mut curr = item.next;

			let mut v = Vec::new();
			v.push(order101_hash);

			item = OrderLinkedItem::<Test> {
				next: None,
				prev: Some(0),
				price: Some(6000000),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item2
			curr = item.next;

			v = Vec::new();

			item = OrderLinkedItem::<Test> {
				next: Some(11000000),
				prev: Some(6000000),
				price: None,
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item4
			curr = item.next;

			v = Vec::new();
			v.push(order4_hash);

			item = OrderLinkedItem::<Test> {
				next: Some(18000000),
				prev: None,
				price: Some(11000000),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item5
			curr = item.next;

			v = Vec::new();
			v.push(order1_hash);

			item = OrderLinkedItem::<Test> {
				next: max,
				prev: Some(11000000),
				price: Some(18000000),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// top
			item = OrderLinkedItem::<Test> {
				next: min,
				prev: Some(18000000),
				price: max,
				orders: Vec::new(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_top(tp_hash), item);

			let trades = <TradePairOwnedTrades<Test>>::get(tp_hash).unwrap();
			let mut v = Vec::new();
			v.push(trades[0]);
			v.push(trades[1]);
			v.push(trades[2]);

			assert_eq!(<OwnedTrades<Test>>::get(ALICE), Some(v.clone()));
			assert_eq!(<OwnedTrades<Test>>::get(BOB), Some(v.clone()));

			assert_eq!(<OwnedTPTrades<Test>>::get((ALICE, tp_hash)), Some(v.clone()));
			assert_eq!(<OwnedTPTrades<Test>>::get((BOB, tp_hash)), Some(v.clone()));

			assert_eq!(<TradePairOwnedTrades<Test>>::get(tp_hash), Some(v.clone()));

			assert_eq!(<OrderOwnedTrades<Test>>::get(order102_hash).unwrap().len(), 3);
			assert_eq!(<OrderOwnedTrades<Test>>::get(order102_hash), Some(v));

			let mut v = Vec::new();
			v.push(trades[0]);
			assert_eq!(<OrderOwnedTrades<Test>>::get(order2_hash).unwrap().len(), 1);
			assert_eq!(<OrderOwnedTrades<Test>>::get(order2_hash), Some(v));

			let mut v = Vec::new();
			v.push(trades[1]);
			assert_eq!(<OrderOwnedTrades<Test>>::get(order3_hash).unwrap().len(), 1);
			assert_eq!(<OrderOwnedTrades<Test>>::get(order3_hash), Some(v));

			let mut v = Vec::new();
			v.push(trades[2]);
			assert_eq!(<OrderOwnedTrades<Test>>::get(order4_hash).unwrap().len(), 1);
			assert_eq!(<OrderOwnedTrades<Test>>::get(order4_hash), Some(v));

			assert_eq!(trades.len(), 3);
			let t1 = <Trades<Test>>::get(trades[0]).unwrap();
			let trade1 = Trade::<Test> {
				base: base,
				quote: quote,
				buyer: ALICE,
				seller: BOB,
				maker: BOB,
				taker: ALICE,
				otype: OrderType::Buy,
				price: 10000000,
				base_amount: 1,
				quote_amount: 11,
				..t1
			};
			assert_eq!(t1, trade1);

			let t2 = <Trades<Test>>::get(trades[1]).unwrap();
			let trade2 = Trade::<Test> {
				base: base,
				quote: quote,
				buyer: ALICE,
				seller: BOB,
				maker: BOB,
				taker: ALICE,
				otype: OrderType::Buy,
				price: 11000000,
				base_amount: 1,
				quote_amount: 10,
				..t2
			};
			assert_eq!(t2, trade2);

			let t3 = <Trades<Test>>::get(trades[2]).unwrap();
			let trade3 = Trade::<Test> {
				base: base,
				quote: quote,
				buyer: ALICE,
				seller: BOB,
				maker: BOB,
				taker: ALICE,
				otype: OrderType::Buy,
				price: 11000000,
				base_amount: 49,
				quote_amount: 445,
				..t3
			};
			assert_eq!(t3, trade3);

			assert_eq!(TokenModule::balance_of((ALICE, quote)), 466);
			assert_eq!(TokenModule::balance_of((BOB, base)), 51);

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(Some(6000000)), Prev(Some(18446744073709551615)), Orders(0): 
			// Price(Some(6000000)), Next(None), Prev(Some(0)), Orders(1): (0x3762…e537@[Created]: 50, 50), 
			// Head ==> Price(None), Next(Some(11000000)), Prev(Some(6000000)), Orders(0): 
			// Price(Some(11000000)), Next(Some(18000000)), Prev(None), Orders(1): (0x2a56…7abe@[PartialFilled]: 10000, 9555), 
			// Price(Some(18000000)), Next(Some(18446744073709551615)), Prev(Some(11000000)), Orders(1): (0x8f2a…03ed@[Created]: 200, 200), 
			// Top ==> Price(Some(18446744073709551615)), Next(Some(0)), Prev(Some(18000000)), Orders(0): 
			// [Market Trades]
			// [0x72bb…80c0/0x8a33…f642] - 0x770b…84d0@10000000[Buy]: [Buyer,Seller][10,20], [Maker,Taker][20,10], [Base,Quote][1, 11]
			// [0x72bb…80c0/0x8a33…f642] - 0x1c17…cb70@11000000[Buy]: [Buyer,Seller][10,20], [Maker,Taker][20,10], [Base,Quote][1, 10]
			// [0x72bb…80c0/0x8a33…f642] - 0x5c6a…3256@11000000[Buy]: [Buyer,Seller][10,20], [Maker,Taker][20,10], [Base,Quote][49, 445]
			output_order(tp_hash);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(ALICE), base, quote, OrderType::Buy, 18000000, 13211));
			let order103_hash = TradeModule::owned_order((ALICE, 2)).unwrap();
			let mut order103 = TradeModule::order(order103_hash).unwrap();
			assert_eq!(order103.amount, 13211);
			assert_eq!(order103.remained_amount, 12124);
			assert_eq!(order103.status, OrderStatus::PartialFilled);

			order4 = TradeModule::order(order4_hash).unwrap();
			assert_eq!(order4.amount, 10000);
			assert_eq!(order4.remained_amount, 0);
			assert_eq!(order4.status, OrderStatus::Filled);

			order1 = TradeModule::order(order1_hash).unwrap();
			assert_eq!(order1.amount, 200);
			assert_eq!(order1.remained_amount, 0);
			assert_eq!(order1.status, OrderStatus::Filled);

			// bottom
			let mut item = OrderLinkedItem::<Test> {
				next: Some(6000000),
				prev: max,
				price: min,
				orders: Vec::new(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_bottom(tp_hash), item);

			// item1
			let mut curr = item.next;

			let mut v = Vec::new();
			v.push(order101_hash);

			item = OrderLinkedItem::<Test> {
				next: Some(18000000),
				prev: Some(0),
				price: Some(6000000),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item2
			curr = item.next;

			v = Vec::new();
			v.push(order103_hash);

			item = OrderLinkedItem::<Test> {
				next: None,
				prev: Some(6000000),
				price: Some(18000000),
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item4
			curr = item.next;

			v = Vec::new();

			item = OrderLinkedItem::<Test> {
				next: max,
				prev: Some(18000000),
				price: None,
				orders: v,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// top
			item = OrderLinkedItem::<Test> {
				next: min,
				prev: None,
				price: max,
				orders: Vec::new(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_top(tp_hash), item);

			let trades = <TradePairOwnedTrades<Test>>::get(tp_hash).unwrap();
			let mut v = Vec::new();
			v.push(trades[0]);
			v.push(trades[1]);
			v.push(trades[2]);
			v.push(trades[3]);
			v.push(trades[4]);

			assert_eq!(<OwnedTrades<Test>>::get(ALICE), Some(v.clone()));
			assert_eq!(<OwnedTrades<Test>>::get(BOB), Some(v.clone()));

			assert_eq!(<OwnedTPTrades<Test>>::get((ALICE, tp_hash)), Some(v.clone()));
			assert_eq!(<OwnedTPTrades<Test>>::get((BOB, tp_hash)), Some(v.clone()));

			assert_eq!(<TradePairOwnedTrades<Test>>::get(tp_hash), Some(v.clone()));

			let mut v = Vec::new();
			v.push(trades[3]);
			v.push(trades[4]);
			assert_eq!(<OrderOwnedTrades<Test>>::get(order103_hash).unwrap().len(), 2);
			assert_eq!(<OrderOwnedTrades<Test>>::get(order103_hash), Some(v));

			let mut v = Vec::new();
			v.push(trades[2]);
			v.push(trades[3]);
			assert_eq!(<OrderOwnedTrades<Test>>::get(order4_hash).unwrap().len(), 2);
			assert_eq!(<OrderOwnedTrades<Test>>::get(order4_hash), Some(v));

			let mut v = Vec::new();
			v.push(trades[4]);
			assert_eq!(<OrderOwnedTrades<Test>>::get(order1_hash).unwrap().len(), 1);
			assert_eq!(<OrderOwnedTrades<Test>>::get(order1_hash), Some(v));

			let trades = <TradePairOwnedTrades<Test>>::get(tp_hash).unwrap();
			assert_eq!(trades.len(), 5);
			let t1 = <Trades<Test>>::get(trades[0]).unwrap();
			let trade1 = Trade::<Test> {
				base: base,
				quote: quote,
				buyer: ALICE,
				seller: BOB,
				maker: BOB,
				taker: ALICE,
				otype: OrderType::Buy,
				price: 10000000,
				base_amount: 1,
				quote_amount: 11,
				..t1
			};
			assert_eq!(t1, trade1);

			let t2 = <Trades<Test>>::get(trades[1]).unwrap();
			let trade2 = Trade::<Test> {
				base: base,
				quote: quote,
				buyer: ALICE,
				seller: BOB,
				maker: BOB,
				taker: ALICE,
				otype: OrderType::Buy,
				price: 11000000,
				base_amount: 1,
				quote_amount: 10,
				..t2
			};
			assert_eq!(t2, trade2);

			let t3 = <Trades<Test>>::get(trades[2]).unwrap();
			let trade3 = Trade::<Test> {
				base: base,
				quote: quote,
				buyer: ALICE,
				seller: BOB,
				maker: BOB,
				taker: ALICE,
				otype: OrderType::Buy,
				price: 11000000,
				base_amount: 49,
				quote_amount: 445,
				..t3
			};
			assert_eq!(t3, trade3);

			let t4 = <Trades<Test>>::get(trades[3]).unwrap();
			let trade4 = Trade::<Test> {
				base: base,
				quote: quote,
				buyer: ALICE,
				seller: BOB,
				maker: BOB,
				taker: ALICE,
				otype: OrderType::Buy,
				price: 11000000,
				base_amount: 1051,
				quote_amount: 9555,
				..t4
			};
			assert_eq!(t4, trade4);

			let t5 = <Trades<Test>>::get(trades[4]).unwrap();
			let trade5 = Trade::<Test> {
				base: base,
				quote: quote,
				buyer: ALICE,
				seller: BOB,
				maker: BOB,
				taker: ALICE,
				otype: OrderType::Buy,
				price: 18000000,
				base_amount: 36,
				quote_amount: 200,
				..t5
			};
			assert_eq!(t5, trade5);

			assert_eq!(TokenModule::balance_of((ALICE, quote)), 466 + 9555 + 200);
			assert_eq!(TokenModule::balance_of((BOB, base)), 51 + 1051 + 36);

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(Some(6000000)), Prev(Some(18446744073709551615)), Orders(0): 
			// Price(Some(6000000)), Next(Some(18000000)), Prev(Some(0)), Orders(1): (0x3762…e537@[Created]: 50, 50), 
			// Price(Some(18000000)), Next(None), Prev(Some(6000000)), Orders(1): (0x036d…71be@[PartialFilled]: 13211, 12124), 
			// Head ==> Price(None), Next(Some(18446744073709551615)), Prev(Some(18000000)), Orders(0): 
			// Top ==> Price(Some(18446744073709551615)), Next(Some(0)), Prev(None), Orders(0): 
			// [Market Trades]
			// [0x72bb…80c0/0x8a33…f642] - 0x770b…84d0@10000000[Buy]: [Buyer,Seller][10,20], [Maker,Taker][20,10], [Base,Quote][1, 11]
			// [0x72bb…80c0/0x8a33…f642] - 0x1c17…cb70@11000000[Buy]: [Buyer,Seller][10,20], [Maker,Taker][20,10], [Base,Quote][1, 10]
			// [0x72bb…80c0/0x8a33…f642] - 0x5c6a…3256@11000000[Buy]: [Buyer,Seller][10,20], [Maker,Taker][20,10], [Base,Quote][49, 445]
			// [0x72bb…80c0/0x8a33…f642] - 0xf264…1328@11000000[Buy]: [Buyer,Seller][10,20], [Maker,Taker][20,10], [Base,Quote][1051, 9555]
			// [0x72bb…80c0/0x8a33…f642] - 0xb6b0…acc4@18000000[Buy]: [Buyer,Seller][10,20], [Maker,Taker][20,10], [Base,Quote][36, 200]
			output_order(tp_hash);
		});
	}

	#[test]
	fn calculate_ex_amount() {
		with_externalities(&mut new_test_ext(), || {
			let ALICE = 10;
			let BOB = 20;

			let mut order1 = LimitOrder::<Test> {
				hash: H256::from_low_u64_be(0),
				base: H256::from_low_u64_be(0),
				quote: H256::from_low_u64_be(0),
				owner: BOB,
				price: <Test as Trait>::Price::sa(135000000),
				amount: 11,
				remained_amount: 11,
				otype: OrderType::Sell,
				status: OrderStatus::Created,
			};

			let mut order2 = LimitOrder::<Test> {
				hash: H256::from_low_u64_be(0),
				base: H256::from_low_u64_be(0),
				quote: H256::from_low_u64_be(0),
				owner: ALICE,
				price: <Test as Trait>::Price::sa(140000000),
				amount: 12,
				remained_amount: 12,
				otype: OrderType::Buy,
				status: OrderStatus::Created,
			};

			// base_amount=12, quote_amount=11, maker_price=135_000_000
			let result = TradeModule::calculate_ex_amount(&order1, &order2).unwrap();
			assert_eq!(result.0, 12);
			assert_eq!(result.1, 8);

			// base_amount=120, quote_amount=105, maker_price=110_000_000
			order1.remained_amount = 105;
			order1.price = <Test as Trait>::Price::sa(110_000_000);
			order2.remained_amount = 120;
			order2.price = <Test as Trait>::Price::sa(110_000_000);
			let result = TradeModule::calculate_ex_amount(&order1, &order2).unwrap();
			assert_eq!(result.0, 115);
			assert_eq!(result.1, 105);

			// base_amount=960, quote_amount=244, maker_price=330_000_000
			order1.remained_amount = 244;
			order1.price = <Test as Trait>::Price::sa(330_000_000);
			order2.remained_amount = 960;
			order2.price = <Test as Trait>::Price::sa(330_000_000);
			let result = TradeModule::calculate_ex_amount(&order1, &order2).unwrap();
			assert_eq!(result.0, 805);
			assert_eq!(result.1, 244);

			// base_amount=10, quote_amount=12, maker_price=99_000_000
			order1.remained_amount = 12;
			order1.price = <Test as Trait>::Price::sa(99_000_000);
			order2.remained_amount = 10;
			order2.price = <Test as Trait>::Price::sa(99_000_000);
			let result = TradeModule::calculate_ex_amount(&order1, &order2).unwrap();
			assert_eq!(result.0, 10);
			assert_eq!(result.1, 10);

			// base_amount=5, quote_amount=120, maker_price=77_700_000
			order1.remained_amount = 120;
			order1.price = <Test as Trait>::Price::sa(77_700_000);
			order2.remained_amount = 5;
			order2.price = <Test as Trait>::Price::sa(77_700_000);
			let result = TradeModule::calculate_ex_amount(&order1, &order2).unwrap();
			assert_eq!(result.0, 5);
			assert_eq!(result.1, 6);

			// remainder is not enough to exchange
			// base_amount=102, quote_amount=100, maker_price=101_000_000
			order1.remained_amount = 100;
			order1.price = <Test as Trait>::Price::sa(101_000_000);
			order2.remained_amount = 102;
			order2.price = <Test as Trait>::Price::sa(101_000_000);
			let result = TradeModule::calculate_ex_amount(&order1, &order2).unwrap();
			assert_eq!(result.0, 102);
			assert_eq!(result.1, 100);

			// remainder is not enough to exchange
			// base_amount=100, quote_amount=102, maker_price=99_009_900
			order1.remained_amount = 102;
			order1.price = <Test as Trait>::Price::sa(99_009_900);
			order2.remained_amount = 100;
			order2.price = <Test as Trait>::Price::sa(99_009_900);
			let result = TradeModule::calculate_ex_amount(&order1, &order2).unwrap();
			assert_eq!(result.0, 100);
			assert_eq!(result.1, 102);

			// ferfect match
			// base_amount=101, quote_amount=100, maker_price=101_000_000
			order1.remained_amount = 100;
			order1.price = <Test as Trait>::Price::sa(101_000_000);
			order2.remained_amount = 101;
			order2.price = <Test as Trait>::Price::sa(101_000_000);
			let result = TradeModule::calculate_ex_amount(&order1, &order2).unwrap();
			assert_eq!(result.0, 101);
			assert_eq!(result.1, 100);
		});
	}
}
