use rstd::if_std;
use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, dispatch::Result, ensure, Parameter, traits::{Get}};
use sr_primitives::traits::{SimpleArithmetic, Bounded, Member, Zero, CheckedSub, Hash};
use primitives::{U256};
use system::ensure_signed;
use codec::{Encode, Decode};
use rstd::{prelude::*, result, ops::Not};
use core::convert::{TryInto, TryFrom};
use byteorder::{ByteOrder, LittleEndian};

use crate::token;
use crate::types;
use crate::utils;

pub trait Trait: token::Trait + system::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type Price: Parameter + Default + Member + Bounded + SimpleArithmetic + Copy + From<u128> + Into<u128>;
	type PriceFactor: Get<u128>;
	type BlocksPerDay: Get<u32>;
}

#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct TradePair<T> where T: Trait {
	hash: T::Hash,
	base: T::Hash,
	quote: T::Hash,

	latest_matched_price: Option<T::Price>,
	
	one_day_trade_volume: T::Balance, // sum of quote qty
	one_day_highest_price: Option<T::Price>,
	one_day_lowest_price: Option<T::Price>,
}

#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq)]
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

#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub enum OrderStatus {
	Created,
	PartialFilled,
	Filled,
	Canceled,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct LimitOrder<T> where T: Trait {
	pub hash: T::Hash,
	pub base: T::Hash,
	pub quote: T::Hash,
	pub owner: T::AccountId,
	pub price: T::Price,
	pub sell_amount: T::Balance,
	pub buy_amount: T::Balance,
	pub remained_sell_amount: T::Balance,
	pub remained_buy_amount: T::Balance,
	pub otype: OrderType,
	pub status: OrderStatus,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq)]
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
	fn new(base: T::Hash, quote: T::Hash, owner: T::AccountId, price: T::Price, sell_amount: T::Balance, 
		buy_amount: T::Balance, otype: OrderType) -> Self {
		let nonce = Nonce::get();

		let hash = (<system::Module<T>>::random_seed(), 
					<system::Module<T>>::block_number(), 
					base, quote, owner.clone(), price, sell_amount, buy_amount, otype, nonce)
			.using_encoded(<T as system::Trait>::Hashing::hash);

		LimitOrder {
			hash, base, quote, owner, price, otype, sell_amount, buy_amount, 
			remained_buy_amount: buy_amount,
			remained_sell_amount: sell_amount,
			status: OrderStatus::Created, 
		}
	}

	pub fn is_finished(&self) -> bool {
		(self.remained_buy_amount == Zero::zero() && self.status == OrderStatus::Filled) || self.status == OrderStatus::Canceled
	}
}

impl<T> Trade<T> where T: Trait {
	fn new(base: T::Hash, quote: T::Hash, maker_order: &LimitOrder<T>, taker_order: &LimitOrder<T>,
		base_amount: T::Balance, quote_amount: T::Balance) -> Self {
		let nonce = Nonce::get();

		let hash = (<system::Module<T>>::random_seed(), <system::Module<T>>::block_number(), nonce,
					maker_order.hash, maker_order.remained_sell_amount, maker_order.owner.clone(),
					taker_order.hash, taker_order.remained_sell_amount, taker_order.owner.clone())
			.using_encoded(<T as system::Trait>::Hashing::hash);

		Nonce::mutate(|x| *x += 1);

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

type OrderLinkedItem<T> = types::LinkedItem<<T as system::Trait>::Hash, <T as Trait>::Price, <T as balances::Trait>::Balance>;
type OrderLinkedItemList<T> = types::LinkedList<T, LinkedItemList<T>, <T as system::Trait>::Hash, <T as Trait>::Price, <T as balances::Trait>::Balance>;

decl_storage! {
	trait Store for Module<T: Trait> as TradeModule {
		///	TradePairHash => TradePair
		TradePairs get(trade_pair): map T::Hash => Option<TradePair<T>>;
		/// (BaseTokenHash, quoteTokenHash) => TradePairHash
		TradePairsHashByBaseQuote get(trade_pair_hash_by_base_quote): map (T::Hash, T::Hash) => Option<T::Hash>;
		/// Index => TradePairHash
		TradePairsHashByIndex get(trade_pair_hash_by_index): map u32 => Option<T::Hash>;
		/// Index
		TradePairsIndex get(trade_pair_index): u32;

		/// OrderHash => Order
		Orders get(order): map T::Hash => Option<LimitOrder<T>>;
		/// (AccoundId, Index) => OrderHash
		OwnedOrders get(owned_order): map (T::AccountId, u64) => Option<T::Hash>;
		///	AccountId => Index
		OwnedOrdersIndex get(owned_orders_index): map T::AccountId => u64;
		/// (OrderHash, u64) => TradeHash
		OrderOwnedTrades get(order_owned_trades): map (T::Hash, u64) => Option<T::Hash>;
		/// (OrderHash, u64) => TradeHash
		OrderOwnedTradesIndex get(order_owned_trades_index): map T::Hash => u64;

		/// (TradePairHash, Index) => OrderHash
		TradePairOwnedOrders get(trade_pair_owned_order): map (T::Hash, u64) => Option<T::Hash>;
		/// TradePairHash => Index
		TradePairOwnedOrdersIndex get(trade_pair_owned_order_index): map T::Hash => u64;

		/// (TradePairHash, Price) => LinkedItem
		LinkedItemList get(linked_item): map (T::Hash, Option<T::Price>) => Option<OrderLinkedItem<T>>;

		/// TradeHash => Trade
		Trades get(trade): map T::Hash => Option<Trade<T>>;

		/// (AccountId, u64) => TradeHash
		OwnedTrades get(owned_trades): map (T::AccountId, u64) => Option<T::Hash>;
		/// AccountId => u64
		OwnedTradesIndex get(owned_trades_index): map T::AccountId => u64;

		/// (AccountId, TradePairHash, u64) => TradeHash
		OwnedTPTrades get(owned_tp_trades): map (T::AccountId, T::Hash, u64) => Option<T::Hash>;
		/// (AccountId, TradePairHash) => u64
		OwnedTPTradesIndex get(owned_tp_trades_index): map (T::AccountId, T::Hash) => u64;

		/// (TradePairHash, u64) => TradeHash
		TradePairOwnedTrades get(trade_pair_owned_trades): map (T::Hash, u64) => Option<T::Hash>;
		/// TradePairHash => u64
		TradePairOwnedTradesIndex get(trade_pair_owned_trades_index): map T::Hash => u64;

		/// (TradePairHash, BlockNumber) => (Sum_of_Trade_Volume, Highest_Price, Lowest_Price)
		TPTradeDataBucket get(trade_pair_trade_data_bucket): map (T::Hash, T::BlockNumber) => (T::Balance, Option<T::Price>, Option<T::Price>);
		/// store the trade pair's H/L price within last day
		/// TradePairHash => (Vec<Highest_Price>, Vec<Lowest_Price>)
		TPTradePriceBucket get(trade_pair_trade_price_bucket): map T::Hash => (Vec<Option<T::Price>>, Vec<Option<T::Price>>);

		Nonce: u64;
	}
}

decl_event!(
	pub enum Event<T> 
	where
		<T as system::Trait>::AccountId,
		<T as system::Trait>::Hash,
		TradePair = TradePair<T>,
		LimitOrder = LimitOrder<T>,
		Trade = Trade<T>,
	{
		TradePairCreated(AccountId, Hash, TradePair),

		// (accountId, baseTokenHash, quoteTokenHash, orderHash, LimitOrder)
		OrderCreated(AccountId, Hash, Hash, Hash, LimitOrder),

		// (accountId, baseTokenHash, quoteTokenHash, tradeHash, Trade)
		TradeCreated(AccountId, Hash, Hash, Hash, Trade),

		// (accountId, orderHash)
		OrderCanceled(AccountId, Hash),
	}
);

impl<T: Trait> OrderOwnedTrades<T> {
	fn add_trade(order_hash: T::Hash, trade_hash: T::Hash) {
		let index = OrderOwnedTradesIndex::<T>::get(&order_hash);
		Self::insert((order_hash.clone(), index), trade_hash);
		OrderOwnedTradesIndex::<T>::insert(order_hash, index + 1);
	}
}

impl<T: Trait> OwnedTrades<T> {
	fn add_trade(account_id: T::AccountId, trade_hash: T::Hash) {
		let index = OwnedTradesIndex::<T>::get(&account_id);
		Self::insert((account_id.clone(), index), trade_hash);
		OwnedTradesIndex::<T>::insert(account_id, index + 1);
	}
}

impl<T: Trait> TradePairOwnedTrades<T> {
	fn add_trade(tp_hash: T::Hash, trade_hash: T::Hash) {
		let index = TradePairOwnedTradesIndex::<T>::get(&tp_hash);
		Self::insert((tp_hash.clone(), index), trade_hash);
		TradePairOwnedTradesIndex::<T>::insert(tp_hash, index + 1);
	}
}

impl<T: Trait> OwnedTPTrades<T> {
	fn add_trade(account_id: T::AccountId, tp_hash: T::Hash, trade_hash: T::Hash) {
		let index = OwnedTPTradesIndex::<T>::get((account_id.clone(), tp_hash));
		Self::insert((account_id.clone(), tp_hash, index), trade_hash);
		OwnedTPTradesIndex::<T>::insert((account_id.clone(), tp_hash), index + 1);
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		pub fn create_trade_pair(origin, base: T::Hash, quote: T::Hash) -> Result {
			let sender = ensure_signed(origin)?;

			Self::do_create_trade_pair(sender, base, quote)
		}

		pub fn create_limit_order(origin, base: T::Hash, quote: T::Hash, otype: OrderType, price: T::Price, sell_amount: T::Balance) -> Result {
			let sender = ensure_signed(origin)?;

			Self::do_create_limit_order(sender, base, quote, otype, price, sell_amount)
		}

		pub fn create_limit_order_with_le_float(origin, base: T::Hash, quote: T::Hash, otype: OrderType, price: Vec<u8>, sell_amount: T::Balance) -> Result {
			let sender = ensure_signed(origin)?;

			let price = Self::price_as_vec_u8_to_x_by_100m(price)?;
			Self::do_create_limit_order(sender, base, quote, otype, price, sell_amount)
		}

		pub fn cancel_limit_order(origin, order_hash: T::Hash) -> Result {
			let sender = ensure_signed(origin)?;

			Self::do_cancel_limit_order(sender, order_hash)
		}

		fn on_initialize(block_number: T::BlockNumber) {
			let days: T::BlockNumber = <<T as system::Trait>::BlockNumber as From<_>>::from(T::BlocksPerDay::get());

			if block_number <= days {
				return
			}

			for index in 0 .. TradePairsIndex::get() {
				let tp_hash = TradePairsHashByIndex::<T>::get(index).unwrap();
				let mut tp = TradePairs::<T>::get(tp_hash).unwrap();
				let (amount, _, _) = TPTradeDataBucket::<T>::get((tp_hash, block_number - days));
				tp.one_day_trade_volume = tp.one_day_trade_volume - amount;
				TradePairs::<T>::insert(tp_hash, tp);

				let mut bucket = TPTradePriceBucket::<T>::get(tp_hash);
				if bucket.0.len() > 0 {
					bucket.0.remove(0);
				}
				if bucket.1.len() > 0 {
					bucket.1.remove(0);
				}
				TPTradePriceBucket::<T>::insert(tp_hash, bucket);
			}
		}

		fn on_finalize(block_number: T::BlockNumber) {
			for index in 0 .. TradePairsIndex::get() {
				let tp_hash = TradePairsHashByIndex::<T>::get(index).unwrap();
				let mut tp = TradePairs::<T>::get(tp_hash).unwrap();

				let data_bucket = TPTradeDataBucket::<T>::get((tp_hash, block_number));
				
				let mut price_bucket = TPTradePriceBucket::<T>::get(tp_hash);
				price_bucket.0.push(data_bucket.1);
				price_bucket.1.push(data_bucket.2);
				TPTradePriceBucket::<T>::insert(tp_hash, &price_bucket);

				let mut h_price = T::Price::min_value();
				for price in price_bucket.0.iter() {
					if let &Some(price) = price {
						if price > h_price {
							h_price = price;
						}
					}
				}

				let mut l_price = T::Price::max_value();
				for price in price_bucket.1.iter() {
					if let &Some(price) = price {
						if price < l_price {
							l_price = price;
						}
					}
				}

				tp.one_day_trade_volume = tp.one_day_trade_volume + data_bucket.0;
				
				if h_price != T::Price::min_value() {
					tp.one_day_highest_price = Some(h_price);
				} else {
					tp.one_day_highest_price = None;
				}

				if l_price != T::Price::max_value() {
					tp.one_day_lowest_price = Some(l_price);
				} else {
					tp.one_day_lowest_price = None;
				}
				
				TradePairs::<T>::insert(tp_hash, tp);
			}
		}
	}
}

impl<T: Trait> Module<T> {
	fn ensure_bounds(price: T::Price, sell_amount: T::Balance) -> Result {
		ensure!(price > Zero::zero() && price <= T::Price::max_value(), "price bounds check failed");
		ensure!(sell_amount > Zero::zero() && sell_amount <= T::Balance::max_value(), "sell amount bound check failed");
		Ok(())
	}

	fn price_as_vec_u8_to_x_by_100m(price: Vec<u8>) -> result::Result<T::Price, &'static str> {
		
		ensure!(price.len() >= 8, "price length is less than 8");

		let price = LittleEndian::read_f64(price.as_slice());

		let price_v2 = (T::PriceFactor::get() as f64 * price) as u128;
		let price_v3 = price_v2 as f64 / T::PriceFactor::get() as f64;

		ensure!(price == price_v3, "price have more digits than required");

		TryFrom::try_from(price_v2).map_err(|_| "number cast error")
	}

	fn ensure_counterparty_amount_bounds(otype: OrderType, price:T::Price, amount: T::Balance) 
		-> result::Result<T::Balance, &'static str> {
		
		let price_u256 = U256::from(Self::into_128(price)?);
		let amount_u256 = U256::from(Self::into_128(amount)?);
		let max_balance_u256 = U256::from(Self::into_128(T::Balance::max_value())?);
		let price_factor_u256 = U256::from(T::PriceFactor::get());

		let amount_v2: U256;
		let counterparty_amount: U256;

		match otype {
			OrderType::Buy => {
				counterparty_amount = amount_u256 * price_factor_u256 / price_u256;
				amount_v2 = counterparty_amount * price_u256 / price_factor_u256;
			},
			OrderType::Sell => {
				counterparty_amount = amount_u256 * price_u256 / price_factor_u256;
				amount_v2 = counterparty_amount * price_factor_u256 / price_u256;
			},
		}

		if amount_u256 != amount_v2 {
			return Err("amount have digits parts")
		}

		if counterparty_amount == 0.into() || counterparty_amount > max_balance_u256 {
			return Err("counterparty bound check failed")
		}

		// todo: change to u128
		let result: u128 = counterparty_amount.try_into().map_err(|_| "Overflow error")?;

		Self::from_128(result)
	}

	fn ensure_trade_pair(base: T::Hash, quote: T::Hash) -> result::Result<T::Hash, &'static str> {
		let bq = Self::trade_pair_hash_by_base_quote((base, quote));
		ensure!(bq.is_some(), "not trade pair with base & quote");

		match bq {
			Some(bq) => Ok(bq),
			None => Err("not trade pair with base & quote"),
		}
	}

	fn do_create_trade_pair(sender: T::AccountId, base: T::Hash, quote: T::Hash) -> Result {
	
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

		let nonce = Nonce::get();

		let hash = (<system::Module<T>>::random_seed(), <system::Module<T>>::block_number(), sender.clone(), base, quote, nonce)
			.using_encoded(<T as system::Trait>::Hashing::hash);

		let tp = TradePair {
			hash, base, quote,
			latest_matched_price: None,
			one_day_trade_volume: Default::default(),
			one_day_highest_price: None,
			one_day_lowest_price: None,
		};

		Nonce::mutate(|n| *n += 1);
		TradePairs::insert(hash, tp.clone());
		TradePairsHashByBaseQuote::<T>::insert((base, quote), hash);

		let index = Self::trade_pair_index();
		TradePairsHashByIndex::<T>::insert(index, hash);
		TradePairsIndex::mutate(|n| *n += 1);

		Self::deposit_event(RawEvent::TradePairCreated(sender, hash, tp));

		Ok(())
	}

	fn do_create_limit_order(sender: T::AccountId, base: T::Hash, quote: T::Hash, otype: OrderType, price: T::Price, 
		sell_amount: T::Balance) -> Result {

        if_std! {
            eprintln!("create limit order begin");
            let hex_string = hex::encode("Hello world!");
            println!("{}", hex_string);
        }

		Self::ensure_bounds(price, sell_amount)?;
		let buy_amount = Self::ensure_counterparty_amount_bounds(otype, price, sell_amount)?;

		let tp_hash = Self::ensure_trade_pair(base, quote)?;
		
		let op_token_hash;
		match otype {
			OrderType::Buy => op_token_hash = base,
			OrderType::Sell => op_token_hash = quote,
		};

		let mut order = LimitOrder::new(base, quote, sender.clone(), price, sell_amount, buy_amount, otype);
		let hash  = order.hash;

		<token::Module<T>>::ensure_free_balance(sender.clone(), op_token_hash, sell_amount)?;
		<token::Module<T>>::do_freeze(sender.clone(), op_token_hash, sell_amount)?;
		Orders::insert(hash, order.clone());
		Nonce::mutate(|n| *n += 1);
		Self::deposit_event(RawEvent::OrderCreated(sender.clone(), base, quote, hash, order.clone()));

        if_std! {
            eprintln!("[order1]: Base[{:02x}], Quote[{:02x}], Owner[{:#?}], Type[{:#?}], Status[{:#?}], SellAmount[{:#?}], RemainedSellAmount[{:#?}], BuyAmount[{:#?}], RemainBuyAmount[{:#?}]", utils::ByteBuf(base.as_ref()), utils::ByteBuf(quote.as_ref()), order.owner, order.otype, order.status, order.sell_amount, order.remained_sell_amount, order.buy_amount, order.remained_buy_amount);
        }

		let owned_index = Self::owned_orders_index(sender.clone());
		OwnedOrders::<T>::insert((sender.clone(), owned_index), hash);
		OwnedOrdersIndex::<T>::insert(sender.clone(), owned_index + 1);

		let tp_owned_index = Self::trade_pair_owned_order_index(tp_hash);
		TradePairOwnedOrders::<T>::insert((tp_hash, tp_owned_index), hash);
		TradePairOwnedOrdersIndex::<T>::insert(tp_hash, tp_owned_index + 1);

		// order match
		let filled = Self::order_match(tp_hash, &mut order)?;

		// add order to the market order list
		if !filled {
			<OrderLinkedItemList<T>>::append(tp_hash, price, hash, order.remained_sell_amount, order.remained_buy_amount, otype);
		}

        if_std! {
            eprintln!("create limit order end");
        }

		Ok(())
	}

	fn order_match(tp_hash: T::Hash, order: &mut LimitOrder<T>) -> result::Result<bool, &'static str> {
        if_std! {
            eprintln!("order match begin");
        }

		let mut head = <OrderLinkedItemList<T>>::read_head(tp_hash);

		let end_item_price;
		let otype = order.otype;
		let oprice = order.price;

		if otype == OrderType::Buy {
			end_item_price = Some(T::Price::min_value());
		} else {
			end_item_price = Some(T::Price::max_value());
		}

		let tp = Self::trade_pair(tp_hash).ok_or("can not get trade pair")?;
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

			let item_price = item_price.ok_or("can not unwrap item price")?;

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

				if order.remained_sell_amount == order.sell_amount {
					order.status = OrderStatus::PartialFilled;
				}

				if o.remained_sell_amount == o.sell_amount {
					o.status = OrderStatus::PartialFilled;
				}

				<token::Module<T>>::do_unfreeze(order.owner.clone(), give, give_qty)?;
				<token::Module<T>>::do_unfreeze(o.owner.clone(), have, have_qty)?;

				<token::Module<T>>::do_transfer(order.owner.clone(), give, o.owner.clone(), give_qty)?;
				<token::Module<T>>::do_transfer(o.owner.clone(), have, order.owner.clone(), have_qty)?;

				order.remained_sell_amount = order.remained_sell_amount.checked_sub(&give_qty).ok_or("substract error")?;
				order.remained_buy_amount = order.remained_buy_amount.checked_sub(&have_qty).ok_or("substract error")?;

				o.remained_sell_amount = o.remained_sell_amount.checked_sub(&have_qty).ok_or("substract error")?;
				o.remained_buy_amount = o.remained_buy_amount.checked_sub(&give_qty).ok_or("substract error")?;

				if order.remained_buy_amount == Zero::zero() {
					order.status = OrderStatus::Filled;
					if order.remained_sell_amount != Zero::zero() {
						<token::Module<T>>::do_unfreeze(order.owner.clone(), give, order.remained_sell_amount)?;
						order.remained_sell_amount = Zero::zero();
					}

					ensure!(order.is_finished(), "order is not finished");
				}

				if o.remained_buy_amount == Zero::zero() {
					o.status = OrderStatus::Filled;
					if o.remained_sell_amount != Zero::zero() {
						<token::Module<T>>::do_unfreeze(o.owner.clone(), have, o.remained_sell_amount)?;
						o.remained_sell_amount = Zero::zero();
					}

					ensure!(o.is_finished(), "order is not finished");
				}

				Orders::insert(order.hash.clone(), order.clone());
				Orders::insert(o.hash.clone(), o.clone());

				// save the trade pair market data
				Self::set_tp_market_data(tp_hash, o.price, quote_qty)?;

				// update maker order's amount in market
				<OrderLinkedItemList<T>>::update_amount(tp_hash, o.price, have_qty, give_qty);

				// remove the matched order
				<OrderLinkedItemList<T>>::remove_all(tp_hash, !otype);
				
				// save the trade data
				let trade = Trade::new(tp.base, tp.quote, &o, &order, base_qty, quote_qty);
				Trades::insert(trade.hash, trade.clone());

		        Self::deposit_event(RawEvent::TradeCreated(order.owner.clone(), tp.base, tp.quote, trade.hash, trade.clone()));

                if_std! {
                    eprintln!("[order1]: Base[{:02x}], Quote[{:02x}], Owner[{:#?}], Type[{:#?}], Status[{:#?}], SellAmount[{:#?}], RemainedSellAmount[{:#?}], BuyAmount[{:#?}], RemainBuyAmount[{:#?}]", utils::ByteBuf(tp.base.as_ref()), utils::ByteBuf(tp.quote.as_ref()), order.owner, order.otype, order.status, order.sell_amount, order.remained_sell_amount, order.buy_amount, order.remained_buy_amount);
                    eprintln!("[order2]: Base[{:02x}], Quote[{:02x}], Owner[{:#?}], Type[{:#?}], Status[{:#?}], SellAmount[{:#?}], RemainedSellAmount[{:#?}], BuyAmount[{:#?}], RemainBuyAmount[{:#?}]", utils::ByteBuf(tp.base.as_ref()), utils::ByteBuf(tp.quote.as_ref()), o.owner, o.otype, order.status, o.sell_amount, o.remained_sell_amount, o.buy_amount, o.remained_buy_amount);
                    eprintln!("[trade]: Hash[{:02x}], BaseQty[{:#?}], QuoteQty[{:#?}]", utils::ByteBuf(trade.hash.as_ref()), base_qty, quote_qty);
                }

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

			head = <OrderLinkedItemList<T>>::read_head(tp_hash);
		}

        if_std! {
            eprintln!("order match end");
        }

		if order.status == OrderStatus::Filled {
			Ok(true)
		} else {
			Ok(false)
		}
	}

	fn into_128<A: TryInto<u128>>(i: A) -> result::Result<u128, &'static str> {
		TryInto::<u128>::try_into(i).map_err(|_| "number cast error")
	}

	fn from_128<A: TryFrom<u128>>(i: u128) -> result::Result<A, &'static str> {
		TryFrom::<u128>::try_from(i).map_err(|_| "number cast error")
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
		if seller_order.remained_buy_amount <= buyer_order.remained_sell_amount { // seller_order is Filled
			let mut quote_qty: u128 = 
				Self::into_128(seller_order.remained_buy_amount)? * T::PriceFactor::get() / maker_order.price.into();
			let buy_amount_v2 = quote_qty * Self::into_128(maker_order.price)? / T::PriceFactor::get();
			if buy_amount_v2 != Self::into_128(seller_order.remained_buy_amount)? { // have fraction, seller(Filled) give more to align
				quote_qty = quote_qty + 1;
			}

			return Ok
			((
				seller_order.remained_buy_amount, 
				Self::from_128(quote_qty)?
			))
		} else if buyer_order.remained_buy_amount <= seller_order.remained_sell_amount { // buyer_order is Filled
			let mut base_qty: u128 = 
				Self::into_128(buyer_order.remained_buy_amount)? * maker_order.price.into() / T::PriceFactor::get();
			let buy_amount_v2 = base_qty * T::PriceFactor::get() / maker_order.price.into();
			if buy_amount_v2 != Self::into_128(buyer_order.remained_buy_amount)? { // have fraction, buyer(Filled) give more to align
				base_qty = base_qty + 1;
			}

			return Ok
			((
				Self::from_128(base_qty)?,
				buyer_order.remained_buy_amount
			))
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

	pub fn set_tp_market_data(tp_hash: T::Hash, price: T::Price, amount: T::Balance) -> Result {

		let mut tp = <TradePairs<T>>::get(tp_hash).ok_or("can not get trade pair")?;
		
		tp.latest_matched_price = Some(price);
		
		let mut bucket = <TPTradeDataBucket<T>>::get((tp_hash, <system::Module<T>>::block_number()));
		bucket.0 = bucket.0 + amount;
		
		match bucket.1 {
			Some(tp_h_price) => {
				if price > tp_h_price {
					bucket.1 = Some(price);
				}
			},
			None => {
				bucket.1 = Some(price);
			},
		}

		match bucket.2 {
			Some(tp_l_price) => {
				if price < tp_l_price {
					bucket.2 = Some(price);
				}
			},
			None => {
				bucket.2 = Some(price);
			},
		}

		<TPTradeDataBucket<T>>::insert((tp_hash, <system::Module<T>>::block_number()), bucket);
		<TradePairs<T>>::insert(tp_hash, tp);

		Ok(())
	}

	fn do_cancel_limit_order(sender: T::AccountId, order_hash: T::Hash) -> Result {
        if_std! {
            eprintln!("cancel limit order begin");
            eprintln!("[order]: hash[{:02x}]", utils::ByteBuf(order_hash.as_ref()));
        }

		let mut order = Self::order(order_hash).ok_or("can not get order")?;

		ensure!(order.owner == sender, "can only cancel your owned order");

		ensure!(!order.is_finished(), "can not cancel finished order");

		let tp_hash = Self::ensure_trade_pair(order.base, order.quote)?;

		<OrderLinkedItemList<T>>::remove_order(tp_hash, order.price, order.hash, order.sell_amount, order.buy_amount)?;

		order.status = OrderStatus::Canceled;
		<Orders<T>>::insert(order_hash, order.clone());

        if_std! {
            eprintln!("[order canceled]: hash[{:02x}], status[{:#?}]", utils::ByteBuf(order_hash.as_ref()), order.status);
        }

        let sell_hash = match order.otype {
            OrderType::Buy => order.base,
            OrderType::Sell => order.quote,
        };

        <token::Module<T>>::do_unfreeze(sender.clone(), sell_hash, order.remained_sell_amount)?;

		Self::deposit_event(RawEvent::OrderCanceled(sender, order_hash));

        if_std! {
            eprintln!("cancel limit order end");
        }

		Ok(())
	}
}

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use primitives::{Blake2Hasher, H256};
	use runtime_io::with_externalities;
	use sr_primitives::weights::Weight;
	use sr_primitives::Perbill;
	use sr_primitives::{
		testing::Header,
		traits::{BlakeTwo256, IdentityLookup, OnInitialize, OnFinalize}
	};
    use std::cell::RefCell;
    use support::{assert_err, assert_ok, impl_outer_origin, parameter_types, traits::Get};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq, Debug)]
	pub struct Test;
	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub const MaximumBlockWeight: Weight = 1024;
		pub const MaximumBlockLength: u32 = 2 * 1024;
		pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
		pub const BalancesTransactionBaseFee: u64 = 0;
		pub const BalancesTransactionByteFee: u64 = 0;
		pub const PriceFactor: u128 = 100_000_000;
		pub const BlocksPerDay: u32 = 10;
	}

	impl system::Trait for Test {
		type Origin = Origin;
		type Call = ();
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = u64;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type WeightMultiplierUpdate = ();
		type Event = ();
		type BlockHashCount = BlockHashCount;
		type MaximumBlockWeight = MaximumBlockWeight;
		type MaximumBlockLength = MaximumBlockLength;
		type AvailableBlockRatio = AvailableBlockRatio;
		type Version = ();
	}

    thread_local! {
        static EXISTENTIAL_DEPOSIT: RefCell<u128> = RefCell::new(0);
        static TRANSFER_FEE: RefCell<u128> = RefCell::new(0);
        static CREATION_FEE: RefCell<u128> = RefCell::new(0);
        static BLOCK_GAS_LIMIT: RefCell<u128> = RefCell::new(0);
    }

    pub struct ExistentialDeposit;
    impl Get<u128> for ExistentialDeposit {
        fn get() -> u128 {
            EXISTENTIAL_DEPOSIT.with(|v| *v.borrow())
        }
    }

    pub struct TransferFee;
    impl Get<u128> for TransferFee {
        fn get() -> u128 {
            TRANSFER_FEE.with(|v| *v.borrow())
        }
    }

    pub struct CreationFee;
    impl Get<u128> for CreationFee {
        fn get() -> u128 {
            CREATION_FEE.with(|v| *v.borrow())
        }
    }

    impl balances::Trait for Test {
        type Balance = u128;

        type OnFreeBalanceZero = ();

        type OnNewAccount = ();

        type Event = ();

        type TransactionPayment = ();
        type DustRemoval = ();
        type TransferPayment = ();

        type ExistentialDeposit = ExistentialDeposit;
        type TransferFee = TransferFee;
        type CreationFee = CreationFee;
        type TransactionBaseFee = BalancesTransactionBaseFee;
        type TransactionByteFee = BalancesTransactionByteFee;
        type WeightToFee = ();
    }

	impl token::Trait for Test {
		type Event = ();
	}

	impl super::Trait for Test {
		type Event = ();
		type Price = u128;
		type PriceFactor = PriceFactor;
		type BlocksPerDay = BlocksPerDay;
	}

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::default()
			.build_storage::<Test>()
			.unwrap()
			.into()
	}

	type System = system::Module<Test>;
	type Balances = balances::Module<Test>;
	type TokenModule = token::Module<Test>;
	type TradeModule = super::Module<Test>;

	fn run_to_block(n: u64) {
		while System::block_number() < n {
			TradeModule::on_finalize(System::block_number());
			Balances::on_finalize(System::block_number());
			System::on_finalize(System::block_number());
			System::set_block_number(System::block_number() + 1);
			System::on_initialize(System::block_number());
			Balances::on_initialize(System::block_number());
			TradeModule::on_initialize(System::block_number());
		}
	}

	#[test]
	fn run_to_block_works() {
		with_externalities(&mut new_test_ext(), || {
			assert_eq!(System::block_number(), 1);
			run_to_block(10);
			assert_eq!(System::block_number(), 10);
		});
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

			print!("Price({:?}), Next({:?}), Prev({:?}), Sell_Amount({:?}), Buy_Amount({:?}), Orders({}): ", 
				item.price, item.next, item.prev, item.sell_amount, item.buy_amount, item.orders.len());

			let mut orders = item.orders.iter();
			loop {
				match orders.next() {
					Some(order_hash) => {
						let order = <Orders<Test>>::get(order_hash).unwrap();
						print!("({}@[{:?}]: Sell[{}, {}], Buy[{}, {}]), ", order.hash, order.status, 
							order.sell_amount, order.remained_sell_amount, order.buy_amount, order.remained_buy_amount);
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

		let index_end = TradeModule::trade_pair_owned_trades_index(tp_hash);
		for i in 0..index_end {
			let hash = TradeModule::trade_pair_owned_trades((tp_hash, i));
			if let Some(hash) = hash {
				let trade = <Trades<Test>>::get(hash).unwrap();
				println!("[{}/{}] - {}@{}[{:?}]: [Buyer,Seller][{},{}], [Maker,Taker][{},{}], [Base,Quote][{}, {}]", 
					trade.quote, trade.base, hash, trade.price, trade.otype, trade.buyer, trade.seller, trade.maker, 
					trade.taker, trade.base_amount, trade.quote_amount);
			}
		}

		// let trades = TradeModule::trade_pair_owned_trades(tp_hash);
		// if let Some(trades) = trades {
		// 	for hash in trades.iter() {
		// 		let trade = <Trades<Test>>::get(hash).unwrap();
		// 		println!("[{}/{}] - {}@{}[{:?}]: [Buyer,Seller][{},{}], [Maker,Taker][{},{}], [Base,Quote][{}, {}]", 
		// 			trade.quote, trade.base, hash, trade.price, trade.otype, trade.buyer, trade.seller, trade.maker, 
		// 			trade.taker, trade.base_amount, trade.quote_amount);
		// 	}
		// }

		println!("[Trade Pair Data]");
		let tp = TradeModule::trade_pair(tp_hash).unwrap();
		println!("latest matched price: {:?}", tp.latest_matched_price);

		println!();
	}

	#[test]
	fn linked_list_test_case() {
		with_externalities(&mut new_test_ext(), || {
			let alice = 10;
			let bob = 20;

			let max = Some(<Test as Trait>::Price::max_value());
			let min = Some(<Test as Trait>::Price::min_value());

			// token1
			assert_ok!(TokenModule::issue(Origin::signed(alice), b"66".to_vec(), 21000000));
			let token1_hash = TokenModule::owned_token((alice, 0)).unwrap();
			let token1 = TokenModule::token(token1_hash).unwrap();

			// token2
			assert_ok!(TokenModule::issue(Origin::signed(bob), b"77".to_vec(), 10000000));
			let token2_hash = TokenModule::owned_token((bob, 0)).unwrap();
			let token2 = TokenModule::token(token2_hash).unwrap();

			// tradepair
			let base = token1.hash;
			let quote = token2.hash;
			assert_ok!(TradeModule::create_trade_pair(Origin::signed(alice), base, quote));
			let tp_hash = TradeModule::trade_pair_hash_by_base_quote((base, quote)).unwrap();

			let bottom = OrderLinkedItem::<Test> {
				prev: max,
				next: None,
				price: min,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};

			let top = OrderLinkedItem::<Test> {
				prev: None,
				next: min,
				price: max,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};

			let head = OrderLinkedItem::<Test> {
				prev: min,
				next: max,
				price: None,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};

			assert_eq!(head, <OrderLinkedItemList<Test>>::read_head(tp_hash));
			assert_eq!(bottom, <OrderLinkedItemList<Test>>::read_bottom(tp_hash));
			assert_eq!(top, <OrderLinkedItemList<Test>>::read_top(tp_hash));

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(None), Prev(Some(340282366920938463463374607431768211455)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Head ==> Price(None), Next(Some(340282366920938463463374607431768211455)), Prev(Some(0)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Top ==> Price(Some(340282366920938463463374607431768211455)), Next(Some(0)), Prev(None), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// [Market Trades]
			// [Trade Pair Data]
			// buy one: None, sell one: None, latest matched price: None
			output_order(tp_hash);

			// sell limit order
			assert_ok!(TradeModule::create_limit_order(Origin::signed(bob), base, quote, OrderType::Sell, 180_000_000, 100));
			let order1_hash = TradeModule::owned_order((bob, 0)).unwrap();
			let mut order1 = TradeModule::order(order1_hash).unwrap();
			assert_eq!(order1.sell_amount, 100);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(bob), base, quote, OrderType::Sell, 100_000_000, 50));
			let order2_hash = TradeModule::owned_order((bob, 1)).unwrap();
			let mut order2 = TradeModule::order(order2_hash).unwrap();
			assert_eq!(order2.sell_amount, 50);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(bob), base, quote, OrderType::Sell, 50_000_000, 10));
			let order3_hash = TradeModule::owned_order((bob, 2)).unwrap();
			let mut order3 = TradeModule::order(order3_hash).unwrap();
			assert_eq!(order3.sell_amount, 10);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(bob), base, quote, OrderType::Sell, 50_000_000, 20));
			let order4_hash = TradeModule::owned_order((bob, 3)).unwrap();
			let mut order4 = TradeModule::order(order4_hash).unwrap();
			assert_eq!(order4.sell_amount, 20);
			
			assert_ok!(TradeModule::create_limit_order(Origin::signed(bob), base, quote, OrderType::Sell, 120_000_000, 10));
			let order5_hash = TradeModule::owned_order((bob, 4)).unwrap();
			let mut order5 = TradeModule::order(order5_hash).unwrap();
			assert_eq!(order5.sell_amount, 10);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(bob), base, quote, OrderType::Sell, 120_000_000, 30));
			let order6_hash = TradeModule::owned_order((bob, 5)).unwrap();
			let mut order6 = TradeModule::order(order6_hash).unwrap();
			assert_eq!(order6.sell_amount, 30);
			
			assert_ok!(TradeModule::create_limit_order(Origin::signed(bob), base, quote, OrderType::Sell, 120_000_000, 20));
			let order7_hash = TradeModule::owned_order((bob, 6)).unwrap();
			let mut order7 = TradeModule::order(order7_hash).unwrap();
			assert_eq!(order7.sell_amount, 20);

			// buy limit order
			assert_ok!(TradeModule::create_limit_order(Origin::signed(alice), base, quote, OrderType::Buy, 20_000_000, 5));
			let order101_hash = TradeModule::owned_order((alice, 0)).unwrap();
			let mut order101 = TradeModule::order(order101_hash).unwrap();
			assert_eq!(order101.sell_amount, 5);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(alice), base, quote, OrderType::Buy, 10_000_000, 12));
			let order102_hash = TradeModule::owned_order((alice, 1)).unwrap();
			let mut order102 = TradeModule::order(order102_hash).unwrap();
			assert_eq!(order102.sell_amount, 12);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(alice), base, quote, OrderType::Buy, 40_000_000, 100));
			let order103_hash = TradeModule::owned_order((alice, 2)).unwrap();
			let mut order103 = TradeModule::order(order103_hash).unwrap();
			assert_eq!(order103.sell_amount, 100);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(alice), base, quote, OrderType::Buy, 20_000_000, 1000000));
			let order104_hash = TradeModule::owned_order((alice, 3)).unwrap();
			let mut order104 = TradeModule::order(order104_hash).unwrap();
			assert_eq!(order104.sell_amount, 1000000);

			// head
			let mut item = OrderLinkedItem::<Test> {
				next: Some(50000000),
				prev: Some(40000000),
				price: None,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
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
				sell_amount: 10 + 20,
				buy_amount: 15,
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
				sell_amount: 50,
				buy_amount: 50,
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
				sell_amount: 60,
				buy_amount: 72,
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
				sell_amount: 100,
				buy_amount: 180,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// top
			item = OrderLinkedItem::<Test> {
				next: min,
				prev: Some(180000000),
				price: max,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_top(tp_hash), item);

			// bottom
			item = OrderLinkedItem::<Test> {
				next: Some(10000000),
				prev: max,
				price: min,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
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
				sell_amount: 12,
				buy_amount: 120,
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
				sell_amount: 1000005,
				buy_amount: 5000025,
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
				sell_amount: 100,
				buy_amount: 250,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(Some(10000000)), Prev(Some(340282366920938463463374607431768211455)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Price(Some(10000000)), Next(Some(20000000)), Prev(Some(0)), Sell_Amount(12), Buy_Amount(120), Orders(1): (0x3958…0b00@[Created]: Sell[12, 12], Buy[120, 120]),
			// Price(Some(20000000)), Next(Some(40000000)), Prev(Some(10000000)), Sell_Amount(1000005), Buy_Amount(5000025), Orders(2): (0xfa29…43a7@[Created]: Sell[5, 5], Buy[25, 25]), (0xdc67…0657@[Created]: Sell[1000000, 1000000], Buy[5000000, 5000000]),
			// Price(Some(40000000)), Next(None), Prev(Some(20000000)), Sell_Amount(100), Buy_Amount(250), Orders(1): (0xcb35…8889@[Created]: Sell[100, 100], Buy[250, 250]),
			// Head ==> Price(None), Next(Some(50000000)), Prev(Some(40000000)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Price(Some(50000000)), Next(Some(100000000)), Prev(None), Sell_Amount(30), Buy_Amount(15), Orders(2): (0xa2df…a11b@[Created]: Sell[10, 10], Buy[5, 5]), (0x7cf2…7de9@[Created]: Sell[20, 20], Buy[10, 10]),
			// Price(Some(100000000)), Next(Some(120000000)), Prev(Some(50000000)), Sell_Amount(50), Buy_Amount(50), Orders(1): (0x0053…0cf2@[Created]: Sell[50, 50], Buy[50, 50]),
			// Price(Some(120000000)), Next(Some(180000000)), Prev(Some(100000000)), Sell_Amount(60), Buy_Amount(72), Orders(3): (0x84fc…0d6c@[Created]: Sell[10, 10], Buy[12, 12]), (0x4d63…b992@[Created]: Sell[30, 30], Buy[36, 36]), (0x6203…87ce@[Created]: Sell[20, 20], Buy[24, 24]),
			// Price(Some(180000000)), Next(Some(340282366920938463463374607431768211455)), Prev(Some(120000000)), Sell_Amount(100), Buy_Amount(180), Orders(1): (0xa118…233c@[Created]: Sell[100, 100], Buy[180, 180]),
			// Top ==> Price(Some(340282366920938463463374607431768211455)), Next(Some(0)), Prev(Some(180000000)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// [Market Trades]
			// [Trade Pair Data]
			// buy one: Some(40000000), sell one: Some(50000000), latest matched price: None
			output_order(tp_hash);

			// price = 5
			order3.remained_buy_amount = Zero::zero();
			order3.status = OrderStatus::Filled;
			<Orders<Test>>::insert(order3.hash, order3);

			order4.status = OrderStatus::Canceled;
			<Orders<Test>>::insert(order4.hash, order4);

			// price = 10
			order2.remained_buy_amount = Zero::zero();
			order2.status = OrderStatus::Filled;
			<Orders<Test>>::insert(order2.hash, order2);

			// price = 12
			order5.status = OrderStatus::Canceled;
			<Orders<Test>>::insert(order5.hash, order5);

			order6.remained_buy_amount = order6.remained_buy_amount.checked_sub(1).unwrap();
			order6.status = OrderStatus::PartialFilled;
			<Orders<Test>>::insert(order6.hash, order6.clone());

			OrderLinkedItemList::<Test>::remove_all(tp_hash, OrderType::Sell);

			// head
			item = OrderLinkedItem::<Test> {
				next: Some(120000000),
				prev: Some(40000000),
				price: None,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
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
				sell_amount: 60,
				buy_amount: 72,
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
				sell_amount: 100,
				buy_amount: 180,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			<OrderLinkedItemList<Test>>::remove_all(tp_hash, OrderType::Sell);

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(Some(10000000)), Prev(Some(340282366920938463463374607431768211455)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Price(Some(10000000)), Next(Some(20000000)), Prev(Some(0)), Sell_Amount(12), Buy_Amount(120), Orders(1): (0x3958…0b00@[Created]: Sell[12, 12], Buy[120, 120]),
			// Price(Some(20000000)), Next(Some(40000000)), Prev(Some(10000000)), Sell_Amount(1000005), Buy_Amount(5000025), Orders(2): (0xfa29…43a7@[Created]: Sell[5, 5], Buy[25, 25]), (0xdc67…0657@[Created]: Sell[1000000, 1000000], Buy[5000000, 5000000]),
			// Price(Some(40000000)), Next(None), Prev(Some(20000000)), Sell_Amount(100), Buy_Amount(250), Orders(1): (0xcb35…8889@[Created]: Sell[100, 100], Buy[250, 250]),
			// Head ==> Price(None), Next(Some(120000000)), Prev(Some(40000000)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Price(Some(120000000)), Next(Some(180000000)), Prev(None), Sell_Amount(60), Buy_Amount(72), Orders(2): (0x4d63…b992@[PartialFilled]: Sell[30, 30], Buy[36, 35]), (0x6203…87ce@[Created]: Sell[20, 20], Buy[24, 24]),
			// Price(Some(180000000)), Next(Some(340282366920938463463374607431768211455)), Prev(Some(120000000)), Sell_Amount(100), Buy_Amount(180), Orders(1): (0xa118…233c@[Created]: Sell[100, 100], Buy[180, 180]),
			// Top ==> Price(Some(340282366920938463463374607431768211455)), Next(Some(0)), Prev(Some(180000000)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// [Market Trades]
			// [Trade Pair Data]
			// buy one: Some(40000000), sell one: Some(50000000), latest matched price: None
			output_order(tp_hash);

			// price = 18
			order1.status = OrderStatus::Canceled;
			<Orders<Test>>::insert(order1.hash, order1);

			// price = 12
			order6.remained_buy_amount = Zero::zero();
			order6.status = OrderStatus::Filled;
			<Orders<Test>>::insert(order6.hash, order6);

			order7.remained_buy_amount = Zero::zero();
			order7.status = OrderStatus::Filled;
			<Orders<Test>>::insert(order7.hash, order7);

			<OrderLinkedItemList<Test>>::remove_all(tp_hash, OrderType::Sell);

			// head
			item = OrderLinkedItem::<Test> {
				next: max,
				prev: Some(40000000),
				price: None,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_head(tp_hash), item);

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(Some(10000000)), Prev(Some(340282366920938463463374607431768211455)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Price(Some(10000000)), Next(Some(20000000)), Prev(Some(0)), Sell_Amount(12), Buy_Amount(120), Orders(1): (0x3958…0b00@[Created]: Sell[12, 12], Buy[120, 120]),
			// Price(Some(20000000)), Next(Some(40000000)), Prev(Some(10000000)), Sell_Amount(1000005), Buy_Amount(5000025), Orders(2): (0xfa29…43a7@[Created]: Sell[5, 5], Buy[25, 25]), (0xdc67…0657@[Created]: Sell[1000000, 1000000], Buy[5000000, 5000000]),
			// Price(Some(40000000)), Next(None), Prev(Some(20000000)), Sell_Amount(100), Buy_Amount(250), Orders(1): (0xcb35…8889@[Created]: Sell[100, 100], Buy[250, 250]),
			// Head ==> Price(None), Next(Some(340282366920938463463374607431768211455)), Prev(Some(40000000)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Top ==> Price(Some(340282366920938463463374607431768211455)), Next(Some(0)), Prev(None), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// [Market Trades]
			// [Trade Pair Data]
			// buy one: Some(40000000), sell one: Some(50000000), latest matched price: None
			output_order(tp_hash);

			// remove buy orders
			// price = 4
			order103.remained_buy_amount = Zero::zero();
			order103.status = OrderStatus::Filled;
			<Orders<Test>>::insert(order103.hash, order103);

			// price = 2
			order101.status = OrderStatus::Canceled;
			<Orders<Test>>::insert(order101.hash, order101);

			order104.remained_buy_amount = order104.remained_buy_amount.checked_sub(100).unwrap();
			order104.status = OrderStatus::PartialFilled;
			<Orders<Test>>::insert(order104.hash, order104.clone());

			<OrderLinkedItemList<Test>>::remove_all(tp_hash, OrderType::Buy);

			// bottom
			item = OrderLinkedItem::<Test> {
				next: Some(10000000),
				prev: max,
				price: min,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
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
				sell_amount: 12,
				buy_amount: 120,
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
				sell_amount: 1000005,
				buy_amount: 5000025,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(Some(10000000)), Prev(Some(340282366920938463463374607431768211455)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Price(Some(10000000)), Next(Some(20000000)), Prev(Some(0)), Sell_Amount(12), Buy_Amount(120), Orders(1): (0x3958…0b00@[Created]: Sell[12, 12], Buy[120, 120]),
			// Price(Some(20000000)), Next(None), Prev(Some(10000000)), Sell_Amount(1000005), Buy_Amount(5000025), Orders(1): (0xdc67…0657@[PartialFilled]: Sell[1000000, 1000000], Buy[5000000, 4999900]),
			// Head ==> Price(None), Next(Some(340282366920938463463374607431768211455)), Prev(Some(20000000)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Top ==> Price(Some(340282366920938463463374607431768211455)), Next(Some(0)), Prev(None), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// [Market Trades]
			// [Trade Pair Data]
			// buy one: Some(40000000), sell one: Some(50000000), latest matched price: None
			output_order(tp_hash);

			// price = 2
			order104.status = OrderStatus::Canceled;
			<Orders<Test>>::insert(order104.hash, order104);

			// price = 1
			order102.remained_buy_amount = Zero::zero();
			order102.status = OrderStatus::Filled;
			<Orders<Test>>::insert(order102.hash, order102);

			<OrderLinkedItemList<Test>>::remove_all(tp_hash, OrderType::Buy);

			let bottom = OrderLinkedItem::<Test> {
				prev: max,
				next: None,
				price: min,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};

			let top = OrderLinkedItem::<Test> {
				prev: None,
				next: min,
				price: max,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};

			let head = OrderLinkedItem::<Test> {
				prev: min,
				next: max,
				price: None,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};

			assert_eq!(head, <OrderLinkedItemList<Test>>::read_head(tp_hash));
			assert_eq!(bottom, <OrderLinkedItemList<Test>>::read_bottom(tp_hash));
			assert_eq!(top, <OrderLinkedItemList<Test>>::read_top(tp_hash));			

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(None), Prev(Some(340282366920938463463374607431768211455)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Head ==> Price(None), Next(Some(340282366920938463463374607431768211455)), Prev(Some(0)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Top ==> Price(Some(340282366920938463463374607431768211455)), Next(Some(0)), Prev(None), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// [Market Trades]
			// [Trade Pair Data]
			// buy one: Some(40000000), sell one: Some(50000000), latest matched price: None
			output_order(tp_hash);
		});
	}

	#[test]
	fn order_match_test_case() {
		with_externalities(&mut new_test_ext(), || {
			let alice = 10;
			let bob = 20;

			let max = Some(<Test as Trait>::Price::max_value());
			let min = Some(<Test as Trait>::Price::min_value());

			// token1
			assert_ok!(TokenModule::issue(Origin::signed(alice), b"66".to_vec(), 21000000));
			let token1_hash = TokenModule::owned_token((alice, 0)).unwrap();
			let token1 = TokenModule::token(token1_hash).unwrap();

			// token2
			assert_ok!(TokenModule::issue(Origin::signed(bob), b"77".to_vec(), 10000000));
			let token2_hash = TokenModule::owned_token((bob, 0)).unwrap();
			let token2 = TokenModule::token(token2_hash).unwrap();

			// tradepair
			let base = token1.hash;
			let quote = token2.hash;
			assert_ok!(TradeModule::create_trade_pair(Origin::signed(alice), base, quote));
			let tp_hash = TradeModule::trade_pair_hash_by_base_quote((base, quote)).unwrap();

			let bottom = OrderLinkedItem::<Test> {
				prev: max,
				next: None,
				price: min,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};

			let top = OrderLinkedItem::<Test> {
				prev: None,
				next: min,
				price: max,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};

			let head = OrderLinkedItem::<Test> {
				prev: min,
				next: max,
				price: None,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};

			assert_eq!(head, <OrderLinkedItemList<Test>>::read_head(tp_hash));
			assert_eq!(bottom, <OrderLinkedItemList<Test>>::read_bottom(tp_hash));
			assert_eq!(top, <OrderLinkedItemList<Test>>::read_top(tp_hash));	

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(None), Prev(Some(340282366920938463463374607431768211455)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Head ==> Price(None), Next(Some(340282366920938463463374607431768211455)), Prev(Some(0)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Top ==> Price(Some(340282366920938463463374607431768211455)), Next(Some(0)), Prev(None), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// [Market Trades]
			// [Trade Pair Data]
			// buy one: None, sell one: None, latest matched price: None
			output_order(tp_hash);

			let p: [u8; 8] = [10, 215, 163, 112, 61, 10, 199, 63]; // 18_000_000
			assert_ok!(TradeModule::create_limit_order_with_le_float(Origin::signed(bob), base, quote, OrderType::Sell, p.to_vec(), 200));

			let order1_hash = TradeModule::owned_order((bob, 0)).unwrap();
			let mut order1 = TradeModule::order(order1_hash).unwrap();
			assert_eq!(order1.sell_amount, 200);
			assert_eq!(order1.remained_sell_amount, 200);
			assert_eq!(order1.buy_amount, 36);
			assert_eq!(order1.remained_buy_amount, 36);

			let p: [u8; 8] = [154, 153, 153, 153, 153, 153, 185, 63]; // 10_000_000
			assert_ok!(TradeModule::create_limit_order_with_le_float(Origin::signed(bob), base, quote, OrderType::Sell, p.to_vec(), 10));
			let order2_hash = TradeModule::owned_order((bob, 1)).unwrap();
			let mut order2 = TradeModule::order(order2_hash).unwrap();
			assert_eq!(order2.sell_amount, 10);
			assert_eq!(order2.remained_sell_amount, 10);
			assert_eq!(order2.buy_amount, 1);
			assert_eq!(order2.remained_buy_amount, 1);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(bob), base, quote, OrderType::Sell, 11_000_000, 100));
			let order3_hash = TradeModule::owned_order((bob, 2)).unwrap();
			let mut order3 = TradeModule::order(order3_hash).unwrap();
			assert_eq!(order3.sell_amount, 100);
			assert_eq!(order3.remained_sell_amount, 100);
			assert_eq!(order3.buy_amount, 11);
			assert_eq!(order3.remained_buy_amount, 11);

			let p: [u8; 8] = [41, 92, 143, 194, 245, 40, 188, 63]; // 11_000_000
			assert_ok!(TradeModule::create_limit_order_with_le_float(Origin::signed(bob), base, quote, OrderType::Sell, p.to_vec(), 10000));
			let order4_hash = TradeModule::owned_order((bob, 3)).unwrap();
			let mut order4 = TradeModule::order(order4_hash).unwrap();
			assert_eq!(order4.sell_amount, 10000);
			assert_eq!(order4.remained_sell_amount, 10000);
			assert_eq!(order4.buy_amount, 1100);
			assert_eq!(order4.remained_buy_amount, 1100);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(alice), base, quote, OrderType::Buy, 6_000_000, 24));
			let order101_hash = TradeModule::owned_order((alice, 0)).unwrap();
			let order101 = TradeModule::order(order101_hash).unwrap();
			assert_eq!(order101.sell_amount, 24);
			assert_eq!(order101.remained_sell_amount, 24);
			assert_eq!(order101.buy_amount, 400);
			assert_eq!(order101.remained_buy_amount, 400);

			// bottom
			let mut item = OrderLinkedItem::<Test> {
				next: Some(6000000),
				prev: max,
				price: min,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
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
				sell_amount: 24,
				buy_amount: 400,
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
				sell_amount: Default::default(),
				buy_amount: Default::default(),
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
				sell_amount: 10,
				buy_amount: 1,
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
				sell_amount: 10100,
				buy_amount: 1111,
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
				sell_amount: 200,
				buy_amount: 36,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// top
			item = OrderLinkedItem::<Test> {
				next: min,
				prev: Some(18000000),
				price: max,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_top(tp_hash), item);

			let block_number = <system::Module<Test>>::block_number();
			let bucket = TPTradeDataBucket::<Test>::get((tp_hash, block_number));
			assert_eq!(bucket, (0, None, None));

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(Some(6000000)), Prev(Some(340282366920938463463374607431768211455)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Price(Some(6000000)), Next(None), Prev(Some(0)), Sell_Amount(24), Buy_Amount(400), Orders(1): (0x5240…b6d0@[Created]: Sell[24, 24], Buy[400, 400]),
			// Head ==> Price(None), Next(Some(10000000)), Prev(Some(6000000)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Price(Some(10000000)), Next(Some(11000000)), Prev(None), Sell_Amount(10), Buy_Amount(1), Orders(1): (0x26d8…e90f@[Created]: Sell[10, 10], Buy[1, 1]),
			// Price(Some(11000000)), Next(Some(18000000)), Prev(Some(10000000)), Sell_Amount(10100), Buy_Amount(1111), Orders(2): (0x5983…7090@[Created]: Sell[100, 100], Buy[11, 11]), (0x0e9b…125e@[Created]: Sell[10000, 10000], Buy[1100, 1100]),
			// Price(Some(18000000)), Next(Some(340282366920938463463374607431768211455)), Prev(Some(11000000)), Sell_Amount(200), Buy_Amount(36), Orders(1): (0xdc46…ffce@[Created]: Sell[200, 200], Buy[36, 36]),
			// Top ==> Price(Some(340282366920938463463374607431768211455)), Next(Some(0)), Prev(Some(18000000)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// [Market Trades]
			// [Trade Pair Data]
			// buy one: Some(6000000), sell one: Some(10000000), latest matched price: None
			output_order(tp_hash);
	
			assert_ok!(TradeModule::create_limit_order(Origin::signed(alice), base, quote, OrderType::Buy, 11_000_000, 55));

			let order102_hash = TradeModule::owned_order((alice, 1)).unwrap();
			let order102 = TradeModule::order(order102_hash).unwrap();
			assert_eq!(order102.sell_amount, 55);
			assert_eq!(order102.remained_sell_amount, 0);
			assert_eq!(order102.buy_amount, 500);
			assert_eq!(order102.remained_buy_amount, 0);
			assert_eq!(order102.status, OrderStatus::Filled);

			order2 = TradeModule::order(order2_hash).unwrap();
			assert_eq!(order2.sell_amount, 10);
			assert_eq!(order2.remained_sell_amount, 0);
			assert_eq!(order2.buy_amount, 1);
			assert_eq!(order2.remained_buy_amount, 0);
			assert_eq!(order2.status, OrderStatus::Filled);

			order3 = TradeModule::order(order3_hash).unwrap();
			assert_eq!(order3.sell_amount, 100);
			assert_eq!(order3.remained_sell_amount, 0);
			assert_eq!(order3.buy_amount, 11);
			assert_eq!(order3.remained_buy_amount, 0);
			assert_eq!(order3.status, OrderStatus::Filled);

			order4 = TradeModule::order(order4_hash).unwrap();
			assert_eq!(order4.sell_amount, 10000);
			assert_eq!(order4.remained_sell_amount, 9610);
			assert_eq!(order4.buy_amount, 1100);
			assert_eq!(order4.remained_buy_amount, 1057);
			assert_eq!(order4.status, OrderStatus::PartialFilled);

			// bottom
			let mut item = OrderLinkedItem::<Test> {
				next: Some(6000000),
				prev: max,
				price: min,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
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
				sell_amount: 24,
				buy_amount: 400,
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
				sell_amount: Default::default(),
				buy_amount: Default::default(),
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
				sell_amount: 9610,
				buy_amount: 1057,
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
				sell_amount: 200,
				buy_amount: 36,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// top
			item = OrderLinkedItem::<Test> {
				next: min,
				prev: Some(18000000),
				price: max,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_top(tp_hash), item);

			let t0_hash = <TradePairOwnedTrades<Test>>::get((tp_hash, 0));
			let t1_hash = <TradePairOwnedTrades<Test>>::get((tp_hash, 1));
			let t2_hash = <TradePairOwnedTrades<Test>>::get((tp_hash, 2));

			assert_eq!(<OwnedTrades<Test>>::get((alice, 0)), t0_hash);
			assert_eq!(<OwnedTrades<Test>>::get((alice, 1)), t1_hash);
			assert_eq!(<OwnedTrades<Test>>::get((alice, 2)), t2_hash);

			assert_eq!(<OwnedTrades<Test>>::get((bob, 0)), t0_hash);
			assert_eq!(<OwnedTrades<Test>>::get((bob, 1)), t1_hash);
			assert_eq!(<OwnedTrades<Test>>::get((bob, 2)), t2_hash);

			assert_eq!(<OwnedTPTrades<Test>>::get((alice, tp_hash, 0)), t0_hash);
			assert_eq!(<OwnedTPTrades<Test>>::get((alice, tp_hash, 1)), t1_hash);
			assert_eq!(<OwnedTPTrades<Test>>::get((alice, tp_hash, 2)), t2_hash);

			assert_eq!(<OwnedTPTrades<Test>>::get((bob, tp_hash, 0)), t0_hash);
			assert_eq!(<OwnedTPTrades<Test>>::get((bob, tp_hash, 1)), t1_hash);
			assert_eq!(<OwnedTPTrades<Test>>::get((bob, tp_hash, 2)), t2_hash);

			assert_eq!(<OrderOwnedTrades<Test>>::get((order102_hash, 0)), t0_hash);
			assert_eq!(<OrderOwnedTrades<Test>>::get((order102_hash, 1)), t1_hash);
			assert_eq!(<OrderOwnedTrades<Test>>::get((order102_hash, 2)), t2_hash);

			assert_eq!(<OrderOwnedTrades<Test>>::get((order2_hash, 0)), t0_hash);
			assert_eq!(<OrderOwnedTrades<Test>>::get((order3_hash, 0)), t1_hash);
			assert_eq!(<OrderOwnedTrades<Test>>::get((order4_hash, 0)), t2_hash);

			let t0 = <Trades<Test>>::get(t0_hash.unwrap()).unwrap();
			let trade0 = Trade::<Test> {
				base: base,
				quote: quote,
				buyer: alice,
				seller: bob,
				maker: bob,
				taker: alice,
				otype: OrderType::Buy,
				price: 10_000_000,
				base_amount: 1,
				quote_amount: 10,
				..t0
			};
			assert_eq!(t0, trade0);

			let t1 = <Trades<Test>>::get(t1_hash.unwrap()).unwrap();
			let trade1 = Trade::<Test> {
				base: base,
				quote: quote,
				buyer: alice,
				seller: bob,
				maker: bob,
				taker: alice,
				otype: OrderType::Buy,
				price: 11000000,
				base_amount: 11,
				quote_amount: 100,
				..t1
			};
			assert_eq!(t1, trade1);

			let t2 = <Trades<Test>>::get(t2_hash.unwrap()).unwrap();
			let trade2 = Trade::<Test> {
				base: base,
				quote: quote,
				buyer: alice,
				seller: bob,
				maker: bob,
				taker: alice,
				otype: OrderType::Buy,
				price: 11000000,
				base_amount: 43,
				quote_amount: 390,
				..t2
			};
			assert_eq!(t2, trade2);

			assert_eq!(TokenModule::balance_of((alice, quote)), 500);
			assert_eq!(TokenModule::balance_of((bob, base)), 55);

			let tp = TradeModule::trade_pair(tp_hash).unwrap();
			assert_eq!(tp.latest_matched_price, Some(11000000));

			let bucket = TPTradeDataBucket::<Test>::get((tp_hash, block_number));
			assert_eq!(bucket, (500, Some(11000000), Some(10000000)));

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(Some(6000000)), Prev(Some(340282366920938463463374607431768211455)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Price(Some(6000000)), Next(None), Prev(Some(0)), Sell_Amount(24), Buy_Amount(400), Orders(1): (0x5240…b6d0@[Created]: Sell[24, 24], Buy[400, 400]),
			// Head ==> Price(None), Next(Some(11000000)), Prev(Some(6000000)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Price(Some(11000000)), Next(Some(18000000)), Prev(None), Sell_Amount(9610), Buy_Amount(1057), Orders(1): (0x0e9b…125e@[PartialFilled]: Sell[10000, 9610], Buy[1100, 1057]),
			// Price(Some(18000000)), Next(Some(340282366920938463463374607431768211455)), Prev(Some(11000000)), Sell_Amount(200), Buy_Amount(36), Orders(1): (0xdc46…ffce@[Created]: Sell[200, 200], Buy[36, 36]),
			// Top ==> Price(Some(340282366920938463463374607431768211455)), Next(Some(0)), Prev(Some(18000000)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// [Market Trades]
			// [0x72bb…80c0/0x8a33…f642] - 0xab39…3add@10000000[Buy]: [Buyer,Seller][10,20], [Maker,Taker][20,10], [Base,Quote][1, 10]
			// [0x72bb…80c0/0x8a33…f642] - 0x6b55…2d58@11000000[Buy]: [Buyer,Seller][10,20], [Maker,Taker][20,10], [Base,Quote][11, 100]
			// [0x72bb…80c0/0x8a33…f642] - 0xa55c…cd59@11000000[Buy]: [Buyer,Seller][10,20], [Maker,Taker][20,10], [Base,Quote][43, 390]
			// [Trade Pair Data]
			// buy one: Some(6000000), sell one: Some(11000000), latest matched price: Some(11000000)
			output_order(tp_hash);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(alice), base, quote, OrderType::Buy, 18_000_000, 13212));
			let order103_hash = TradeModule::owned_order((alice, 2)).unwrap();
			let order103 = TradeModule::order(order103_hash).unwrap();
			assert_eq!(order103.sell_amount, 13212);
			assert_eq!(order103.remained_sell_amount, 13212 - 1057 - 36);
			assert_eq!(order103.buy_amount, 73400);
			assert_eq!(order103.remained_buy_amount, 73400 - 9610 - 200);
			assert_eq!(order103.status, OrderStatus::PartialFilled);

			order4 = TradeModule::order(order4_hash).unwrap();
			assert_eq!(order4.sell_amount, 10000);
			assert_eq!(order4.remained_sell_amount, 0);
			assert_eq!(order4.buy_amount, 1100);
			assert_eq!(order4.remained_buy_amount, 0);
			assert_eq!(order4.status, OrderStatus::Filled);

			order1 = TradeModule::order(order1_hash).unwrap();
			assert_eq!(order1.sell_amount, 200);
			assert_eq!(order1.remained_sell_amount, 0);
			assert_eq!(order1.buy_amount, 36);
			assert_eq!(order1.remained_buy_amount, 0);
			assert_eq!(order1.status, OrderStatus::Filled);

			// bottom
			let mut item = OrderLinkedItem::<Test> {
				next: Some(6000000),
				prev: max,
				price: min,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
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
				sell_amount: 24,
				buy_amount: 400,
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
				sell_amount: 13212 - 1057 - 36,
				buy_amount: 73400 - 9610 - 200,
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
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// top
			item = OrderLinkedItem::<Test> {
				next: min,
				prev: None,
				price: max,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_top(tp_hash), item);

			let t0_hash = <TradePairOwnedTrades<Test>>::get((tp_hash, 0));
			let t1_hash = <TradePairOwnedTrades<Test>>::get((tp_hash, 1));
			let t2_hash = <TradePairOwnedTrades<Test>>::get((tp_hash, 2));
			let t3_hash = <TradePairOwnedTrades<Test>>::get((tp_hash, 3));
			let t4_hash = <TradePairOwnedTrades<Test>>::get((tp_hash, 4));

			assert_eq!(<OwnedTrades<Test>>::get((alice, 0)), t0_hash);
			assert_eq!(<OwnedTrades<Test>>::get((alice, 1)), t1_hash);
			assert_eq!(<OwnedTrades<Test>>::get((alice, 2)), t2_hash);
			assert_eq!(<OwnedTrades<Test>>::get((alice, 3)), t3_hash);
			assert_eq!(<OwnedTrades<Test>>::get((alice, 4)), t4_hash);

			assert_eq!(<OwnedTrades<Test>>::get((bob, 0)), t0_hash);
			assert_eq!(<OwnedTrades<Test>>::get((bob, 1)), t1_hash);
			assert_eq!(<OwnedTrades<Test>>::get((bob, 2)), t2_hash);
			assert_eq!(<OwnedTrades<Test>>::get((bob, 3)), t3_hash);
			assert_eq!(<OwnedTrades<Test>>::get((bob, 4)), t4_hash);

			assert_eq!(<OwnedTPTrades<Test>>::get((alice, tp_hash, 0)), t0_hash);
			assert_eq!(<OwnedTPTrades<Test>>::get((alice, tp_hash, 1)), t1_hash);
			assert_eq!(<OwnedTPTrades<Test>>::get((alice, tp_hash, 2)), t2_hash);
			assert_eq!(<OwnedTPTrades<Test>>::get((alice, tp_hash, 3)), t3_hash);
			assert_eq!(<OwnedTPTrades<Test>>::get((alice, tp_hash, 4)), t4_hash);

			assert_eq!(<OwnedTPTrades<Test>>::get((bob, tp_hash, 0)), t0_hash);
			assert_eq!(<OwnedTPTrades<Test>>::get((bob, tp_hash, 1)), t1_hash);
			assert_eq!(<OwnedTPTrades<Test>>::get((bob, tp_hash, 2)), t2_hash);
			assert_eq!(<OwnedTPTrades<Test>>::get((bob, tp_hash, 3)), t3_hash);
			assert_eq!(<OwnedTPTrades<Test>>::get((bob, tp_hash, 4)), t4_hash);

			assert_eq!(<OrderOwnedTrades<Test>>::get((order102_hash, 0)), t0_hash);
			assert_eq!(<OrderOwnedTrades<Test>>::get((order102_hash, 1)), t1_hash);
			assert_eq!(<OrderOwnedTrades<Test>>::get((order102_hash, 2)), t2_hash);

			assert_eq!(<OrderOwnedTrades<Test>>::get((order103_hash, 0)), t3_hash);
			assert_eq!(<OrderOwnedTrades<Test>>::get((order103_hash, 1)), t4_hash);

			assert_eq!(<OrderOwnedTrades<Test>>::get((order2_hash, 0)), t0_hash);
			assert_eq!(<OrderOwnedTrades<Test>>::get((order3_hash, 0)), t1_hash);
			assert_eq!(<OrderOwnedTrades<Test>>::get((order4_hash, 0)), t2_hash);
			assert_eq!(<OrderOwnedTrades<Test>>::get((order4_hash, 1)), t3_hash);

			let t0 = <Trades<Test>>::get(t0_hash.unwrap()).unwrap();
			let trade0 = Trade::<Test> {
				base: base,
				quote: quote,
				buyer: alice,
				seller: bob,
				maker: bob,
				taker: alice,
				otype: OrderType::Buy,
				price: 10_000_000,
				base_amount: 1,
				quote_amount: 10,
				..t0
			};
			assert_eq!(t0, trade0);

			let t1 = <Trades<Test>>::get(t1_hash.unwrap()).unwrap();
			let trade1 = Trade::<Test> {
				base: base,
				quote: quote,
				buyer: alice,
				seller: bob,
				maker: bob,
				taker: alice,
				otype: OrderType::Buy,
				price: 11000000,
				base_amount: 11,
				quote_amount: 100,
				..t1
			};
			assert_eq!(t1, trade1);

			let t2 = <Trades<Test>>::get(t2_hash.unwrap()).unwrap();
			let trade2 = Trade::<Test> {
				base: base,
				quote: quote,
				buyer: alice,
				seller: bob,
				maker: bob,
				taker: alice,
				otype: OrderType::Buy,
				price: 11000000,
				base_amount: 43,
				quote_amount: 390,
				..t2
			};
			assert_eq!(t2, trade2);

			let t3 = <Trades<Test>>::get(t3_hash.unwrap()).unwrap();
			let trade3 = Trade::<Test> {
				base: base,
				quote: quote,
				buyer: alice,
				seller: bob,
				maker: bob,
				taker: alice,
				otype: OrderType::Buy,
				price: 11000000,
				base_amount: 1057,
				quote_amount: 9610,
				..t3
			};
			assert_eq!(t3, trade3);

			let t4 = <Trades<Test>>::get(t4_hash.unwrap()).unwrap();
			let trade4 = Trade::<Test> {
				base: base,
				quote: quote,
				buyer: alice,
				seller: bob,
				maker: bob,
				taker: alice,
				otype: OrderType::Buy,
				price: 18000000,
				base_amount: 36,
				quote_amount: 200,
				..t4
			};
			assert_eq!(t4, trade4);

			assert_eq!(TokenModule::balance_of((alice, quote)), 10 + 100 + 10000 + 200);
			assert_eq!(TokenModule::balance_of((bob, base)), 1 + 11 + 1100 + 36);

			let tp = TradeModule::trade_pair(tp_hash).unwrap();
			assert_eq!(tp.latest_matched_price, Some(18000000));

			let bucket = TPTradeDataBucket::<Test>::get((tp_hash, block_number));
			assert_eq!(bucket, (10 + 100 + 10000 + 200, Some(18000000), Some(10000000)));

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(Some(6000000)), Prev(Some(340282366920938463463374607431768211455)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Price(Some(6000000)), Next(Some(18000000)), Prev(Some(0)), Sell_Amount(24), Buy_Amount(400), Orders(1): (0x5240…b6d0@[Created]: Sell[24, 24], Buy[400, 400]),
			// Price(Some(18000000)), Next(None), Prev(Some(6000000)), Sell_Amount(12119), Buy_Amount(63590), Orders(1): (0x77e2…c008@[PartialFilled]: Sell[13212, 12119], Buy[73400, 63590]),
			// Head ==> Price(None), Next(Some(340282366920938463463374607431768211455)), Prev(Some(18000000)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Top ==> Price(Some(340282366920938463463374607431768211455)), Next(Some(0)), Prev(None), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// [Market Trades]
			// [0x72bb…80c0/0x8a33…f642] - 0xab39…3add@10000000[Buy]: [Buyer,Seller][10,20], [Maker,Taker][20,10], [Base,Quote][1, 10]
			// [0x72bb…80c0/0x8a33…f642] - 0x6b55…2d58@11000000[Buy]: [Buyer,Seller][10,20], [Maker,Taker][20,10], [Base,Quote][11, 100]
			// [0x72bb…80c0/0x8a33…f642] - 0xa55c…cd59@11000000[Buy]: [Buyer,Seller][10,20], [Maker,Taker][20,10], [Base,Quote][43, 390]
			// [0x72bb…80c0/0x8a33…f642] - 0xb833…a1e0@11000000[Buy]: [Buyer,Seller][10,20], [Maker,Taker][20,10], [Base,Quote][1057, 9610]
			// [0x72bb…80c0/0x8a33…f642] - 0x278b…4ea6@18000000[Buy]: [Buyer,Seller][10,20], [Maker,Taker][20,10], [Base,Quote][36, 200]
			// [Trade Pair Data]
			// buy one: Some(18000000), sell one: None, latest matched price: Some(18000000)
			output_order(tp_hash);
		});
	}

	#[test]
	fn order_cancel_test_case() {
		with_externalities(&mut new_test_ext(), || {
			let alice = 10;
			let bob = 20;

			let max = Some(<Test as Trait>::Price::max_value());
			let min = Some(<Test as Trait>::Price::min_value());

			// token1
			assert_ok!(TokenModule::issue(Origin::signed(alice), b"66".to_vec(), 21000000));
			let token1_hash = TokenModule::owned_token((alice, 0)).unwrap();
			let token1 = TokenModule::token(token1_hash).unwrap();

			// token2
			assert_ok!(TokenModule::issue(Origin::signed(bob), b"77".to_vec(), 10000000));
			let token2_hash = TokenModule::owned_token((bob, 0)).unwrap();
			let token2 = TokenModule::token(token2_hash).unwrap();

			// tradepair
			let base = token1.hash;
			let quote = token2.hash;
			assert_ok!(TradeModule::create_trade_pair(Origin::signed(alice), base, quote));
			let tp_hash = TradeModule::trade_pair_hash_by_base_quote((base, quote)).unwrap();

			let bottom = OrderLinkedItem::<Test> {
				prev: max,
				next: None,
				price: min,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};

			let top = OrderLinkedItem::<Test> {
				prev: None,
				next: min,
				price: max,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};

			let head = OrderLinkedItem::<Test> {
				prev: min,
				next: max,
				price: None,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};

			assert_eq!(head, <OrderLinkedItemList<Test>>::read_head(tp_hash));
			assert_eq!(bottom, <OrderLinkedItemList<Test>>::read_bottom(tp_hash));
			assert_eq!(top, <OrderLinkedItemList<Test>>::read_top(tp_hash));	

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(None), Prev(Some(340282366920938463463374607431768211455)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Head ==> Price(None), Next(Some(340282366920938463463374607431768211455)), Prev(Some(0)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Top ==> Price(Some(340282366920938463463374607431768211455)), Next(Some(0)), Prev(None), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// [Market Trades]
			// [Trade Pair Data]
			// buy one: None, sell one: None, latest matched price: None
			output_order(tp_hash);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(bob), base, quote, OrderType::Sell, 18_000_000, 200));
			let order1_hash = TradeModule::owned_order((bob, 0)).unwrap();
			let order1 = TradeModule::order(order1_hash).unwrap();
			assert_eq!(order1.sell_amount, 200);
			assert_eq!(order1.remained_sell_amount, 200);
			assert_eq!(order1.buy_amount, 36);
			assert_eq!(order1.remained_buy_amount, 36);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(bob), base, quote, OrderType::Sell, 10_000_000, 10));
			let order2_hash = TradeModule::owned_order((bob, 1)).unwrap();
			let order2 = TradeModule::order(order2_hash).unwrap();
			assert_eq!(order2.sell_amount, 10);
			assert_eq!(order2.remained_sell_amount, 10);
			assert_eq!(order2.buy_amount, 1);
			assert_eq!(order2.remained_buy_amount, 1);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(bob), base, quote, OrderType::Sell, 11_000_000, 100));
			let order3_hash = TradeModule::owned_order((bob, 2)).unwrap();
			let mut order3 = TradeModule::order(order3_hash).unwrap();
			assert_eq!(order3.sell_amount, 100);
			assert_eq!(order3.remained_sell_amount, 100);
			assert_eq!(order3.buy_amount, 11);
			assert_eq!(order3.remained_buy_amount, 11);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(bob), base, quote, OrderType::Sell, 11_000_000, 10000));
			let order4_hash = TradeModule::owned_order((bob, 3)).unwrap();
			let order4 = TradeModule::order(order4_hash).unwrap();
			assert_eq!(order4.sell_amount, 10000);
			assert_eq!(order4.remained_sell_amount, 10000);
			assert_eq!(order4.buy_amount, 1100);
			assert_eq!(order4.remained_buy_amount, 1100);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(alice), base, quote, OrderType::Buy, 6_000_000, 24));
			let order101_hash = TradeModule::owned_order((alice, 0)).unwrap();
			let mut order101 = TradeModule::order(order101_hash).unwrap();
			assert_eq!(order101.sell_amount, 24);
			assert_eq!(order101.remained_sell_amount, 24);
			assert_eq!(order101.buy_amount, 400);
			assert_eq!(order101.remained_buy_amount, 400);

			// bottom
			let mut item = OrderLinkedItem::<Test> {
				next: Some(6000000),
				prev: max,
				price: min,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
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
				sell_amount: 24,
				buy_amount: 400,
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
				sell_amount: Default::default(),
				buy_amount: Default::default(),
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
				sell_amount: 10,
				buy_amount: 1,
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
				sell_amount: 10100,
				buy_amount: 1111,
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
				sell_amount: 200,
				buy_amount: 36,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// top
			item = OrderLinkedItem::<Test> {
				next: min,
				prev: Some(18000000),
				price: max,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_top(tp_hash), item);

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(Some(6000000)), Prev(Some(340282366920938463463374607431768211455)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Price(Some(6000000)), Next(None), Prev(Some(0)), Sell_Amount(24), Buy_Amount(400), Orders(1): (0x5240…b6d0@[Created]: Sell[24, 24], Buy[400, 400]),
			// Head ==> Price(None), Next(Some(10000000)), Prev(Some(6000000)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Price(Some(10000000)), Next(Some(11000000)), Prev(None), Sell_Amount(10), Buy_Amount(1), Orders(1): (0x26d8…e90f@[Created]: Sell[10, 10], Buy[1, 1]),
			// Price(Some(11000000)), Next(Some(18000000)), Prev(Some(10000000)), Sell_Amount(10100), Buy_Amount(1111), Orders(2): (0x5983…7090@[Created]: Sell[100, 100], Buy[11, 11]), (0x0e9b…125e@[Created]: Sell[10000, 10000], Buy[1100, 1100]),
			// Price(Some(18000000)), Next(Some(340282366920938463463374607431768211455)), Prev(Some(11000000)), Sell_Amount(200), Buy_Amount(36), Orders(1): (0xdc46…ffce@[Created]: Sell[200, 200], Buy[36, 36]),
			// Top ==> Price(Some(340282366920938463463374607431768211455)), Next(Some(0)), Prev(Some(18000000)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// [Market Trades]
			// [Trade Pair Data]
			// buy one: Some(6000000), sell one: Some(10000000), latest matched price: None
			output_order(tp_hash);
	
			order101.status = OrderStatus::Filled;
			let tmp_amount = order101.remained_buy_amount;
			order101.remained_buy_amount = Zero::zero();
			<Orders<Test>>::insert(order101.hash, order101.clone());
			assert_err!(TradeModule::cancel_limit_order(Origin::signed(alice), order101.hash), "can not cancel finished order");

			assert_err!(TradeModule::cancel_limit_order(Origin::signed(alice), order1.hash), "can only cancel your owned order");

			assert_err!(TradeModule::cancel_limit_order(Origin::signed(alice), H256::from_low_u64_be(0)), "can not get order");

			order101.status = OrderStatus::Created;
			order101.remained_buy_amount = tmp_amount;
			<Orders<Test>>::insert(order101.hash, order101.clone());
			assert_ok!(TradeModule::cancel_limit_order(Origin::signed(alice), order101.hash));

			let o = TradeModule::order(order101_hash).unwrap();
			assert_eq!(o.status, OrderStatus::Canceled);

			order3.status = OrderStatus::PartialFilled;
			order3.remained_buy_amount = order3.remained_buy_amount.checked_sub(1).unwrap();
			<Orders<Test>>::insert(order3.hash, order3.clone());
			assert_ok!(TradeModule::cancel_limit_order(Origin::signed(bob), order3.hash));
			assert_ok!(TradeModule::cancel_limit_order(Origin::signed(bob), order4.hash));

			let o = TradeModule::order(order3_hash).unwrap();
			assert_eq!(o.status, OrderStatus::Canceled);
			let o = TradeModule::order(order4_hash).unwrap();
			assert_eq!(o.status, OrderStatus::Canceled);

			// bottom
			let mut item = OrderLinkedItem::<Test> {
				next: None,
				prev: max,
				price: min,
				orders: Vec::new(),
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read_bottom(tp_hash), item);

			// item1
			let mut curr = item.next;

			let mut v = Vec::new();

			item = OrderLinkedItem::<Test> {
				next: Some(10000000),
				prev: Some(0),
				price: None,
				orders: v,
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item2
			curr = item.next;

			v = Vec::new();
			v.push(order2_hash);

			item = OrderLinkedItem::<Test> {
				next: Some(18000000),
				prev: None,
				price: Some(10000000),
				orders: v,
				sell_amount: 10,
				buy_amount: 1,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item3
			curr = item.next;
			
			v = Vec::new();
			v.push(order1_hash);

			item = OrderLinkedItem::<Test> {
				next: max,
				prev: Some(10000000),
				price: Some(18000000),
				orders: v,
				sell_amount: 200,
				buy_amount: 36,
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// item4
			curr = item.next;

			v = Vec::new();

			item = OrderLinkedItem::<Test> {
				next: min,
				prev: Some(18000000),
				price: max,
				orders: v,
				sell_amount: Default::default(),
				buy_amount: Default::default(),
			};
			assert_eq!(OrderLinkedItemList::<Test>::read(tp_hash, curr), item);

			// [Market Orders]
			// Bottom ==> Price(Some(0)), Next(None), Prev(Some(340282366920938463463374607431768211455)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Head ==> Price(None), Next(Some(10000000)), Prev(Some(0)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// Price(Some(10000000)), Next(Some(18000000)), Prev(None), Sell_Amount(10), Buy_Amount(1), Orders(1): (0x26d8…e90f@[Created]: Sell[10, 10], Buy[1, 1]),
			// Price(Some(18000000)), Next(Some(340282366920938463463374607431768211455)), Prev(Some(10000000)), Sell_Amount(200), Buy_Amount(36), Orders(1): (0xdc46…ffce@[Created]: Sell[200, 200], Buy[36, 36]),
			// Top ==> Price(Some(340282366920938463463374607431768211455)), Next(Some(0)), Prev(Some(18000000)), Sell_Amount(0), Buy_Amount(0), Orders(0):
			// [Market Trades]
			// [Trade Pair Data]
			// buy one: Some(6000000), sell one: Some(10000000), latest matched price: None
			output_order(tp_hash);
		});
	}

	#[test]
	fn order_match_calculation_test_case() {
		with_externalities(&mut new_test_ext(), || {
			let alice = 10;
			let bob = 20;

			// token1
			assert_ok!(TokenModule::issue(Origin::signed(alice), b"66".to_vec(), 21000000));
			let token1_hash = TokenModule::owned_token((alice, 0)).unwrap();
			let token1 = TokenModule::token(token1_hash).unwrap();

			// token2
			assert_ok!(TokenModule::issue(Origin::signed(bob), b"77".to_vec(), 10000000));
			let token2_hash = TokenModule::owned_token((bob, 0)).unwrap();
			let token2 = TokenModule::token(token2_hash).unwrap();

			// tradepair
			let base = token1.hash;
			let quote = token2.hash;
			assert_ok!(TradeModule::create_trade_pair(Origin::signed(alice), base, quote));

			assert_ok!(TradeModule::create_limit_order(Origin::signed(alice), base, quote, OrderType::Buy, 25_010_000, 2501));
			let order101_hash = TradeModule::owned_order((alice, 0)).unwrap();
			let order101 = TradeModule::order(order101_hash).unwrap();
			assert_eq!(order101.sell_amount, 2501);
			assert_eq!(order101.remained_sell_amount, 2501);
			assert_eq!(order101.buy_amount, 10000);
			assert_eq!(order101.remained_buy_amount, 10000);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(bob), base, quote, OrderType::Sell, 25_000_000, 4));
			let order1_hash = TradeModule::owned_order((bob, 0)).unwrap();
			let order1 = TradeModule::order(order1_hash).unwrap();
			assert_eq!(order1.sell_amount, 4);
			assert_eq!(order1.remained_sell_amount, 0);
			assert_eq!(order1.buy_amount, 1);
			assert_eq!(order1.remained_buy_amount, 0);

			assert_eq!(TokenModule::balance_of((alice, quote)), 4);
			assert_eq!(TokenModule::balance_of((alice, base)), 21000000 - 1);
			assert_eq!(TokenModule::balance_of((bob, base)), 1);
			assert_eq!(TokenModule::balance_of((bob, quote)), 10000000 - 4);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(bob), base, quote, OrderType::Sell, 25_000_000, 9996));
			let order1_hash = TradeModule::owned_order((bob, 1)).unwrap();
			let order1 = TradeModule::order(order1_hash).unwrap();
			assert_eq!(order1.sell_amount, 9996);
			assert_eq!(order1.remained_sell_amount, 0);
			assert_eq!(order1.buy_amount, 2499);
			assert_eq!(order1.remained_buy_amount, 0);

			let order101_hash = TradeModule::owned_order((alice, 0)).unwrap();
			let order101 = TradeModule::order(order101_hash).unwrap();
			assert_eq!(order101.sell_amount, 2501);
			assert_eq!(order101.remained_sell_amount, 1);
			assert_eq!(order101.buy_amount, 10000);
			assert_eq!(order101.remained_buy_amount, 3);

			assert_eq!(TokenModule::balance_of((alice, quote)), 4 + 9993);
			assert_eq!(TokenModule::balance_of((alice, base)), 21000000 - 1 - 2499);
			assert_eq!(TokenModule::freezed_balance_of((alice, base)), 1);
			assert_eq!(TokenModule::balance_of((bob, base)), 1 + 2499);
			assert_eq!(TokenModule::balance_of((bob, quote)), 10000000 - 4 - 9993);
			assert_eq!(TokenModule::freezed_balance_of((bob, base)), 0);
		});
	}

	#[test]
	fn trade_pair_bucket_test_case() {
		with_externalities(&mut new_test_ext(), || {
			let alice = 10;
			let bob = 20;

			// token1
			assert_ok!(TokenModule::issue(Origin::signed(alice), b"66".to_vec(), 21000000));
			let token1_hash = TokenModule::owned_token((alice, 0)).unwrap();
			let token1 = TokenModule::token(token1_hash).unwrap();

			// token2
			assert_ok!(TokenModule::issue(Origin::signed(bob), b"77".to_vec(), 10000000));
			let token2_hash = TokenModule::owned_token((bob, 0)).unwrap();
			let token2 = TokenModule::token(token2_hash).unwrap();

			// tradepair
			let base = token1.hash;
			let quote = token2.hash;
			assert_ok!(TradeModule::create_trade_pair(Origin::signed(alice), base, quote));
			let tp_hash = TradeModule::trade_pair_hash_by_base_quote((base, quote)).unwrap();

			assert_ok!(TradeModule::create_limit_order(Origin::signed(alice), base, quote, OrderType::Buy, 25_010_000, 2501));
			let order101_hash = TradeModule::owned_order((alice, 0)).unwrap();
			let order101 = TradeModule::order(order101_hash).unwrap();
			assert_eq!(order101.sell_amount, 2501);
			assert_eq!(order101.remained_sell_amount, 2501);
			assert_eq!(order101.buy_amount, 10000);
			assert_eq!(order101.remained_buy_amount, 10000);

			assert_ok!(TradeModule::create_limit_order(Origin::signed(bob), base, quote, OrderType::Sell, 25_000_000, 4));
			let order1_hash = TradeModule::owned_order((bob, 0)).unwrap();
			let order1 = TradeModule::order(order1_hash).unwrap();
			assert_eq!(order1.sell_amount, 4);
			assert_eq!(order1.remained_sell_amount, 0);
			assert_eq!(order1.buy_amount, 1);
			assert_eq!(order1.remained_buy_amount, 0);

			assert_eq!(TokenModule::balance_of((alice, quote)), 4);
			assert_eq!(TokenModule::balance_of((alice, base)), 21000000 - 1);
			assert_eq!(TokenModule::balance_of((bob, base)), 1);
			assert_eq!(TokenModule::balance_of((bob, quote)), 10000000 - 4);

			run_to_block(1);
			assert_eq!(System::block_number(), 1);
			assert_eq!(TradeModule::trade_pair_trade_data_bucket((tp_hash, 1)), (4, Some(25_010_000), Some(25_010_000)));

			for i in 2..6 {
				run_to_block(i);
				assert_eq!(System::block_number(), i);
				let trade_pair = TradeModule::trade_pair(tp_hash).unwrap();
				assert_eq!(trade_pair.latest_matched_price, Some(25_010_000));
				assert_eq!(trade_pair.one_day_trade_volume, 4);
				assert_eq!(trade_pair.one_day_highest_price, Some(25_010_000));
				assert_eq!(trade_pair.one_day_lowest_price, Some(25_010_000));
				assert_eq!(TradeModule::trade_pair_trade_data_bucket((tp_hash, i)), (0, None, None));
			}

			assert_eq!(TradeModule::trade_pair_trade_data_bucket((tp_hash, 1)), (4, Some(25_010_000), Some(25_010_000)));
			assert_eq!(TradeModule::trade_pair_trade_data_bucket((tp_hash, 2)), (0, None, None));
			assert_eq!(TradeModule::trade_pair_trade_data_bucket((tp_hash, 3)), (0, None, None));
			assert_eq!(TradeModule::trade_pair_trade_data_bucket((tp_hash, 4)), (0, None, None));
			assert_eq!(TradeModule::trade_pair_trade_data_bucket((tp_hash, 5)), (0, None, None));

			assert_ok!(TradeModule::create_limit_order(Origin::signed(bob), base, quote, OrderType::Sell, 25_000_000, 9996));
			let order1_hash = TradeModule::owned_order((bob, 1)).unwrap();
			let order1 = TradeModule::order(order1_hash).unwrap();
			assert_eq!(order1.sell_amount, 9996);
			assert_eq!(order1.remained_sell_amount, 0);
			assert_eq!(order1.buy_amount, 2499);
			assert_eq!(order1.remained_buy_amount, 0);

			let order101_hash = TradeModule::owned_order((alice, 0)).unwrap();
			let order101 = TradeModule::order(order101_hash).unwrap();
			assert_eq!(order101.sell_amount, 2501);
			assert_eq!(order101.remained_sell_amount, 1);
			assert_eq!(order101.buy_amount, 10000);
			assert_eq!(order101.remained_buy_amount, 3);

			assert_eq!(TokenModule::balance_of((alice, quote)), 4 + 9993);
			assert_eq!(TokenModule::balance_of((alice, base)), 21000000 - 1 - 2499);
			assert_eq!(TokenModule::freezed_balance_of((alice, base)), 1);
			assert_eq!(TokenModule::balance_of((bob, base)), 1 + 2499);
			assert_eq!(TokenModule::balance_of((bob, quote)), 10000000 - 4 - 9993);
			assert_eq!(TokenModule::freezed_balance_of((bob, base)), 0);

			for i in 7..11 {
				run_to_block(i);
				assert_eq!(System::block_number(), i);
				let trade_pair = TradeModule::trade_pair(tp_hash).unwrap();
				assert_eq!(trade_pair.latest_matched_price, Some(25_010_000));
				assert_eq!(trade_pair.one_day_trade_volume, 4 + 9993);
				assert_eq!(trade_pair.one_day_highest_price, Some(25_010_000));
				assert_eq!(trade_pair.one_day_lowest_price, Some(25_010_000));
				assert_eq!(TradeModule::trade_pair_trade_data_bucket((tp_hash, i)), (0, None, None));
			}

			run_to_block(12);
			assert_eq!(System::block_number(), 12);
			assert_eq!(TradeModule::trade_pair_trade_data_bucket((tp_hash, 11)), (0, None, None));

			let trade_pair = TradeModule::trade_pair(tp_hash).unwrap();
			assert_eq!(trade_pair.latest_matched_price, Some(25_010_000));
			assert_eq!(trade_pair.one_day_trade_volume, 9993);
			assert_eq!(trade_pair.one_day_highest_price, Some(25_010_000));
			assert_eq!(trade_pair.one_day_lowest_price, Some(25_010_000));

			run_to_block(16);

			let trade_pair = TradeModule::trade_pair(tp_hash).unwrap();
			assert_eq!(trade_pair.latest_matched_price, Some(25_010_000));
			assert_eq!(trade_pair.one_day_trade_volume, 0);
			assert_eq!(trade_pair.one_day_highest_price, None);
			assert_eq!(trade_pair.one_day_lowest_price, None);
		});
	}

	#[test]
	fn calculate_ex_amount() {
		with_externalities(&mut new_test_ext(), || {
			let alice = 10;
			let bob = 20;

			let order1 = LimitOrder::<Test> {
				hash: H256::from_low_u64_be(0),
				base: H256::from_low_u64_be(0),
				quote: H256::from_low_u64_be(0),
				owner: alice,
				price: TradeModule::from_128(25010000).unwrap(),
				sell_amount: 2501,
				remained_sell_amount: 2501,
				buy_amount: 10000,
				remained_buy_amount: 10000,
				otype: OrderType::Buy,
				status: OrderStatus::Created,
			};

			let mut order2 = LimitOrder::<Test> {
				hash: H256::from_low_u64_be(0),
				base: H256::from_low_u64_be(0),
				quote: H256::from_low_u64_be(0),
				owner: bob,
				price: TradeModule::from_128(25000000).unwrap(),
				sell_amount: 4,
				remained_sell_amount: 4,
				buy_amount: 1,
				remained_buy_amount: 1,
				otype: OrderType::Sell,
				status: OrderStatus::Created,
			};

			let result = TradeModule::calculate_ex_amount(&order1, &order2).unwrap();
			assert_eq!(result.0, 1);
			assert_eq!(result.1, 4);

			let result = TradeModule::calculate_ex_amount(&order2, &order1).unwrap();
			assert_eq!(result.0, 1);
			assert_eq!(result.1, 4);

			order2.price = TradeModule::from_128(25_010_000).unwrap();
			let result = TradeModule::calculate_ex_amount(&order1, &order2).unwrap();
			assert_eq!(result.0, 1);
			assert_eq!(result.1, 4);

			let result = TradeModule::calculate_ex_amount(&order2, &order1).unwrap();
			assert_eq!(result.0, 1);
			assert_eq!(result.1, 4);

			order2.price = TradeModule::from_128(33_000_000).unwrap();
			let result = TradeModule::calculate_ex_amount(&order1, &order2).unwrap();
			assert_eq!(result.0, 1);
			assert_eq!(result.1, 4);

			let result = TradeModule::calculate_ex_amount(&order2, &order1).unwrap();
			assert_eq!(result.0, 1);
			assert_eq!(result.1, 4);

			order2.price = TradeModule::from_128(35_000_000).unwrap();
			let result = TradeModule::calculate_ex_amount(&order1, &order2).unwrap();
			assert_eq!(result.0, 1);
			assert_eq!(result.1, 4);

			let result = TradeModule::calculate_ex_amount(&order2, &order1).unwrap();
			assert_eq!(result.0, 1);
			assert_eq!(result.1, 3);
		});
	}

	#[test]
	fn ensure_amount_zero_digits_test_case() {
		with_externalities(&mut new_test_ext(), || {
			assert_eq!(1, 1);

			let price = TradeModule::from_128(25_010_000).unwrap();
			let amount = TradeModule::from_128(2501).unwrap();
			let otype = OrderType::Buy;
			assert_ok!(TradeModule::ensure_counterparty_amount_bounds(otype, price, amount), 10000u128);
			
			let price = TradeModule::from_128(25_010_000).unwrap(); // 0.2501
			let amount = TradeModule::from_128(2500).unwrap();
			let otype = OrderType::Buy;
			assert_err!(TradeModule::ensure_counterparty_amount_bounds(otype, price, amount), "amount have digits parts");

			let price = TradeModule::from_128(25_000_000).unwrap(); // 0.25
			let amount = TradeModule::from_128(24).unwrap();
			let otype = OrderType::Sell;
			assert_ok!(TradeModule::ensure_counterparty_amount_bounds(otype, price, amount), 6u128);
			
			let price = TradeModule::from_128(25_000_000).unwrap(); // 0.25
			let amount = TradeModule::from_128(21).unwrap();
			let otype = OrderType::Sell;
			assert_err!(TradeModule::ensure_counterparty_amount_bounds(otype, price, amount), "amount have digits parts");

			let price = TradeModule::from_128(200_000_000).unwrap(); // 2.0
			let amount = TradeModule::from_128(u128::max_value() - 1).unwrap();
			let otype = OrderType::Sell;
			assert_err!(TradeModule::ensure_counterparty_amount_bounds(otype, price, amount), "counterparty bound check failed");
		});
	}

	#[test]
	fn price_as_vec_u8_to_x_by_100m_test_case() {
		with_externalities(&mut new_test_ext(), || {
			assert_eq!(1, 1);

			let _price_v1 = 3.11122233f64;
			let price_v1_vec_u8: [u8; 8] = [183, 122, 111, 136, 200, 227, 8, 64];
			let price_v2 = 311122233u128;
			assert_ok!(TradeModule::price_as_vec_u8_to_x_by_100m(price_v1_vec_u8.to_vec()), price_v2);

			let _price_v1 = 123.00000000f64;
			let price_v1_vec_u8: [u8; 8] = [0, 0, 0, 0, 0, 192, 94, 64];
			let price_v2 = 12_300_000_000;
			assert_ok!(TradeModule::price_as_vec_u8_to_x_by_100m(price_v1_vec_u8.to_vec()), price_v2);

			let _price_v1 = 999.6789f64;
			let price_v1_vec_u8: [u8; 8] = [9, 138, 31, 99, 110, 61, 143, 64];
			let price_v2 = 99_967_890_000u128;
			assert_ok!(TradeModule::price_as_vec_u8_to_x_by_100m(price_v1_vec_u8.to_vec()), price_v2);

			let _price_v1 = 0.00000001f64;
			let price_v1_vec_u8: [u8; 8] = [58, 140, 48, 226, 142, 121, 69, 62];
			let price_v2 = 1u128;
			assert_ok!(TradeModule::price_as_vec_u8_to_x_by_100m(price_v1_vec_u8.to_vec()), price_v2);

			let price_v1_vec_u8: [u8; 7] = [255, 142, 214, 136, 200, 227, 8];
			assert_err!(TradeModule::price_as_vec_u8_to_x_by_100m(price_v1_vec_u8.to_vec()), 
				"price length is less than 8");

			let _price_v1 = 3.111222333f64;
			let price_v1_vec_u8: [u8; 8] = [255, 142, 214, 136, 200, 227, 8, 64];
			assert_err!(TradeModule::price_as_vec_u8_to_x_by_100m(price_v1_vec_u8.to_vec()), 
				"price have more digits than required");

			let _price_v1 = 3.000011112222f64;
			let price_v1_vec_u8: [u8; 8] = [101, 10, 117, 211, 5, 0, 8, 64];
			assert_err!(TradeModule::price_as_vec_u8_to_x_by_100m(price_v1_vec_u8.to_vec()), 
				"price have more digits than required");
		});
	}
}
