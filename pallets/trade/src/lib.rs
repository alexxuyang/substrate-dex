#![cfg_attr(not(feature = "std"), no_std)]

use core::convert::{TryInto, TryFrom};

use sp_core::U256;
use sp_std::{prelude::*, if_std, fmt::Debug, result, ops::Not};
use sp_runtime::{traits::{Bounded, Member, Zero, CheckedSub, Hash, AtLeast32Bit}};

use frame_support::{decl_module, decl_storage, decl_event, decl_error, StorageValue, StorageMap,
                    ensure, Parameter, dispatch, traits::{Get, Randomness},
                    weights::{Weight}};

use system::ensure_signed;
use codec::{Encode, Decode};
use byteorder::{ByteOrder, LittleEndian};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod types;

pub trait Trait: token::Trait + system::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type Price: Parameter + Default + Member + Bounded + AtLeast32Bit + Copy + From<u128> + Into<u128>;
    type PriceFactor: Get<u128>;
    type BlocksPerDay: Get<u32>;
    type OpenedOrdersArrayCap: Get<u8>;
    type ClosedOrdersArrayCap: Get<u8>;
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

#[derive(Encode, Decode, Debug, Clone, Copy, PartialEq, Eq)]
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

        let random_seed = <randomness_collective_flip::Module<T>>::random_seed();
        let hash = (random_seed,
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

        let random_seed = <randomness_collective_flip::Module<T>>::random_seed();
        let hash = (random_seed, <system::Module<T>>::block_number(), nonce,
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

decl_error! {
	/// Error for the trade module.
	pub enum Error for Module<T: Trait> {
		/// Price bounds check failed
		BoundsCheckFailed,
		/// Price length check failed
		PriceLengthCheckFailed,
		/// Number cast error
		NumberCastError,
		/// Overflow error
		OverflowError,
        /// No matching trade pair
        NoMatchingTradePair,
        /// Base equals to quote
        BaseEqualQuote,
        /// Token owner not found
        TokenOwnerNotFound,
        /// Sender not equal to base or quote owner
        SenderNotEqualToBaseOrQuoteOwner,
        /// Same trade pair with the given base and quote was already exist
        TradePairExisted,
        /// Get price error
        OrderMatchGetPriceError,
        /// Get linked list item error
        OrderMatchGetLinkedListItemError,
        /// Get order error
        OrderMatchGetOrderError,
        /// Order match substract error
        OrderMatchSubstractError,
        /// Order match order is not finish
        OrderMatchOrderIsNotFinished,
        /// No matching order
        NoMatchingOrder,
        /// Can only cancel own order
        CanOnlyCancelOwnOrder,
        /// can only cancel not finished order
        CanOnlyCancelNotFinishedOrder,
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as TradeModule {
		///	TradePairHash => TradePair
		TradePairs get(fn trade_pair): map hasher(blake2_128_concat) T::Hash => Option<TradePair<T>>;
		/// (BaseTokenHash, quoteTokenHash) => TradePairHash
		TradePairsHashByBaseQuote get(fn trade_pair_hash_by_base_quote): map hasher(blake2_128_concat) (T::Hash, T::Hash) => Option<T::Hash>;
		/// Index => TradePairHash
		TradePairsHashByIndex get(fn trade_pair_hash_by_index): map hasher(blake2_128_concat) u64 => Option<T::Hash>;
		/// Index
		TradePairsIndex get(fn trade_pair_index): u64;

		/// OrderHash => Order
		Orders get(fn order): map hasher(blake2_128_concat) T::Hash => Option<LimitOrder<T>>;
		/// (AccoundId, Index) => OrderHash
		OwnedOrders get(fn owned_order): map hasher(blake2_128_concat) (T::AccountId, u64) => Option<T::Hash>;
		///	AccountId => Index
		OwnedOrdersIndex get(fn owned_orders_index): map hasher(blake2_128_concat) T::AccountId => u64;
		/// (OrderHash, u64) => TradeHash
		OrderOwnedTrades get(fn order_owned_trades): map hasher(blake2_128_concat) (T::Hash, u64) => Option<T::Hash>;
		/// (OrderHash, u64) => TradeHash
		OrderOwnedTradesIndex get(fn order_owned_trades_index): map hasher(blake2_128_concat) T::Hash => u64;

		/// (TradePairHash, Index) => OrderHash
		TradePairOwnedOrders get(fn trade_pair_owned_order): map hasher(blake2_128_concat) (T::Hash, u64) => Option<T::Hash>;
		/// TradePairHash => Index
		TradePairOwnedOrdersIndex get(fn trade_pair_owned_order_index): map hasher(blake2_128_concat) T::Hash => u64;

		/// (TradePairHash, Price) => LinkedItem
		LinkedItemList get(fn linked_item): map hasher(blake2_128_concat) (T::Hash, Option<T::Price>) => Option<OrderLinkedItem<T>>;

		/// TradeHash => Trade
		Trades get(fn trade): map hasher(blake2_128_concat) T::Hash => Option<Trade<T>>;

		/// (AccountId, u64) => TradeHash
		OwnedTrades get(fn owned_trades): map hasher(blake2_128_concat) (T::AccountId, u64) => Option<T::Hash>;
		/// AccountId => u64
		OwnedTradesIndex get(fn owned_trades_index): map hasher(blake2_128_concat) T::AccountId => u64;

		/// (AccountId, TradePairHash, u64) => TradeHash
		OwnedTPTrades get(fn owned_tp_trades): map hasher(blake2_128_concat) (T::AccountId, T::Hash, u64) => Option<T::Hash>;
		/// (AccountId, TradePairHash) => u64
		OwnedTPTradesIndex get(fn owned_tp_trades_index): map hasher(blake2_128_concat) (T::AccountId, T::Hash) => u64;

        /// (AccountId, TradePairHash) => Vec<OrderHash>
        OwnedTPOpenedOrders get(fn owned_tp_opened_orders): map hasher(blake2_128_concat) (T::AccountId, T::Hash) => Option<Vec<T::Hash>>;

        /// (AccountId, TradePairHash) => Vec<OrderHash>
        OwnedTPClosedOrders get(fn owned_tp_closed_orders): map hasher(blake2_128_concat) (T::AccountId, T::Hash) => Option<Vec<T::Hash>>;

		/// (TradePairHash, u64) => TradeHash
		TradePairOwnedTrades get(fn trade_pair_owned_trades): map hasher(blake2_128_concat) (T::Hash, u64) => Option<T::Hash>;
		/// TradePairHash => u64
		TradePairOwnedTradesIndex get(fn trade_pair_owned_trades_index): map hasher(blake2_128_concat) T::Hash => u64;

		/// (TradePairHash, BlockNumber) => (Sum_of_Trade_Volume, Highest_Price, Lowest_Price)
		TPTradeDataBucket get(fn trade_pair_trade_data_bucket): map hasher(blake2_128_concat) (T::Hash, T::BlockNumber) => (T::Balance, Option<T::Price>, Option<T::Price>);
		/// store the trade pair's H/L price within last day
		/// TradePairHash => (Vec<Highest_Price>, Vec<Lowest_Price>)
		TPTradePriceBucket get(fn trade_pair_trade_price_bucket): map hasher(blake2_128_concat) T::Hash => (Vec<Option<T::Price>>, Vec<Option<T::Price>>);

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

impl<T: Trait> OwnedTPOpenedOrders<T> {
    fn add_order(account_id: T::AccountId, tp_hash: T::Hash, order_hash: T::Hash) {

        let mut orders;
        if let Some(ts) = Self::get((account_id.clone(), tp_hash)) {
            orders = ts;
        } else {
            orders = Vec::<T::Hash>::new();
        }

        match orders.iter().position(|&x| x == order_hash) {
            Some(_) => return,
            None => {
                orders.insert(0, order_hash);
                if orders.len() == T::OpenedOrdersArrayCap::get() as usize {
                    orders.pop();
                }

                <OwnedTPOpenedOrders<T>>::insert((account_id, tp_hash), orders);
            }
        }
    }

    fn remove_order(account_id: T::AccountId, tp_hash: T::Hash, order_hash: T::Hash) {

        let mut orders;
        if let Some(ts) = Self::get((account_id.clone(), tp_hash)) {
            orders = ts;
        } else {
            orders = Vec::<T::Hash>::new();
        }

        orders.retain(|&x| x != order_hash);
        <OwnedTPOpenedOrders<T>>::insert((account_id, tp_hash), orders);
    }
}

impl<T: Trait> OwnedTPClosedOrders<T> {
    fn add_order(account_id: T::AccountId, tp_hash: T::Hash, order_hash: T::Hash) {

        let mut orders;
        if let Some(ts) = Self::get((account_id.clone(), tp_hash)) {
            orders = ts;
        } else {
            orders = Vec::<T::Hash>::new();
        }

        match orders.iter().position(|&x| x == order_hash) {
            Some(_) => return,
            None => {
                orders.insert(0, order_hash);
                if orders.len() == T::ClosedOrdersArrayCap::get() as usize {
                    orders.pop();
                }

                <OwnedTPClosedOrders<T>>::insert((account_id, tp_hash), orders);
            }
        }
    }
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

        type Error = Error<T>;

		#[weight = 1_000_000]
		pub fn create_trade_pair(origin, base: T::Hash, quote: T::Hash) -> Result<(), dispatch::DispatchError> {
			let sender = ensure_signed(origin)?;

			Self::do_create_trade_pair(sender, base, quote)
		}

		#[weight = 1_000_000]
		pub fn create_limit_order(origin, base: T::Hash, quote: T::Hash, otype: OrderType, price: T::Price, sell_amount: T::Balance) -> Result<(), dispatch::DispatchError> {
			let sender = ensure_signed(origin)?;

			Self::do_create_limit_order(sender, base, quote, otype, price, sell_amount)
		}

		#[weight = 1_000_000]
		pub fn create_limit_order_with_le_float(origin, base: T::Hash, quote: T::Hash, otype: OrderType, price: Vec<u8>, sell_amount: T::Balance) -> Result<(), dispatch::DispatchError> {
			let sender = ensure_signed(origin)?;

			let price = Self::price_as_vec_u8_to_x_by_100m(price)?;
			Self::do_create_limit_order(sender, base, quote, otype, price, sell_amount)
		}

		#[weight = 1_000_000]
		pub fn cancel_limit_order(origin, order_hash: T::Hash) -> Result<(), dispatch::DispatchError> {
			let sender = ensure_signed(origin)?;

			Self::do_cancel_limit_order(sender, order_hash)
		}

		fn on_initialize(block_number: T::BlockNumber) -> Weight {
			let days: T::BlockNumber = <<T as system::Trait>::BlockNumber as From<_>>::from(T::BlocksPerDay::get());

			if block_number <= days {
				return 1000
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

			500_000
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
    fn ensure_bounds(price: T::Price, sell_amount: T::Balance) -> dispatch::DispatchResult {
        ensure!(price > Zero::zero() && price <= T::Price::max_value(), Error::<T>::BoundsCheckFailed);
        ensure!(sell_amount > Zero::zero() && sell_amount <= T::Balance::max_value(), Error::<T>::BoundsCheckFailed);
        Ok(())
    }

    fn price_as_vec_u8_to_x_by_100m(price: Vec<u8>) -> Result<T::Price, dispatch::DispatchError> {

        ensure!(price.len() >= 8, Error::<T>::PriceLengthCheckFailed);

        let price = LittleEndian::read_f64(price.as_slice());

        let price_v2 = (T::PriceFactor::get() as f64 * price) as u128;
        let price_v3 = price_v2 as f64 / T::PriceFactor::get() as f64;

        ensure!(price == price_v3, Error::<T>::PriceLengthCheckFailed);

        TryFrom::try_from(price_v2).map_err(|_| Error::<T>::NumberCastError.into())
    }

    fn ensure_counterparty_amount_bounds(otype: OrderType, price:T::Price, amount: T::Balance)
                                         -> result::Result<T::Balance, dispatch::DispatchError> {

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

        ensure!(amount_u256 == amount_v2, Error::<T>::BoundsCheckFailed);
        ensure!(counterparty_amount != 0.into() && counterparty_amount <= max_balance_u256, Error::<T>::BoundsCheckFailed);

        // todo: change to u128
        let result: u128 = counterparty_amount.try_into().map_err(|_| Error::<T>::OverflowError)?;

        Self::from_128(result)
    }

    fn ensure_trade_pair(base: T::Hash, quote: T::Hash) -> result::Result<T::Hash, dispatch::DispatchError> {
        let bq = Self::trade_pair_hash_by_base_quote((base, quote));
        ensure!(bq.is_some(), Error::<T>::NoMatchingTradePair);

        match bq {
            Some(bq) => Ok(bq),
            None => Err(Error::<T>::NoMatchingTradePair.into()),
        }
    }

    fn do_create_trade_pair(sender: T::AccountId, base: T::Hash, quote: T::Hash) -> dispatch::DispatchResult {

        ensure!(base != quote, Error::<T>::BaseEqualQuote);

        let base_owner = <token::Module<T>>::owner(base);
        let quote_owner = <token::Module<T>>::owner(quote);

        ensure!(base_owner.is_some() && quote_owner.is_some(), Error::<T>::TokenOwnerNotFound);

        let base_owner = base_owner.unwrap();
        let quote_owner = quote_owner.unwrap();

        ensure!(sender == base_owner || sender == quote_owner, Error::<T>::SenderNotEqualToBaseOrQuoteOwner);

        let bq = Self::trade_pair_hash_by_base_quote((base, quote));
        let qb = Self::trade_pair_hash_by_base_quote((quote, base));

        ensure!(!bq.is_some() && !qb.is_some(), Error::<T>::TradePairExisted);

        let nonce = Nonce::get();

        let random_seed = <randomness_collective_flip::Module<T>>::random_seed();
        let hash = (random_seed, <system::Module<T>>::block_number(), sender.clone(), base, quote, nonce)
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
                             sell_amount: T::Balance) -> dispatch::DispatchResult {

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
        <OwnedTPOpenedOrders<T>>::add_order(sender.clone(), tp_hash, order.hash);

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
        } else {
            <OwnedTPOpenedOrders<T>>::remove_order(sender.clone(), tp_hash, order.hash);
            <OwnedTPClosedOrders<T>>::add_order(sender.clone(), tp_hash, order.hash);
        }

        Ok(())
    }

    fn order_match(tp_hash: T::Hash, order: &mut LimitOrder<T>) -> result::Result<bool, dispatch::DispatchError> {
        let mut head = <OrderLinkedItemList<T>>::read_head(tp_hash);

        let end_item_price;
        let otype = order.otype;
        let oprice = order.price;

        if otype == OrderType::Buy {
            end_item_price = Some(T::Price::min_value());
        } else {
            end_item_price = Some(T::Price::max_value());
        }

        let tp = Self::trade_pair(tp_hash).ok_or(Error::<T>::NoMatchingTradePair)?;
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

            let item_price = item_price.ok_or(Error::<T>::OrderMatchGetPriceError)?;

            if !Self::price_matched(oprice, otype, item_price) {
                break
            }

            let item = <LinkedItemList<T>>::get((tp_hash, Some(item_price))).ok_or(Error::<T>::OrderMatchGetLinkedListItemError)?;
            for o in item.orders.iter() {

                let mut o = Self::order(o).ok_or(Error::<T>::OrderMatchGetOrderError)?;

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

                <token::Module<T>>::do_transfer(order.owner.clone(), give, o.owner.clone(), give_qty, None)?;
                <token::Module<T>>::do_transfer(o.owner.clone(), have, order.owner.clone(), have_qty, None)?;

                order.remained_sell_amount = order.remained_sell_amount.checked_sub(&give_qty).ok_or(Error::<T>::OrderMatchSubstractError)?;
                order.remained_buy_amount = order.remained_buy_amount.checked_sub(&have_qty).ok_or(Error::<T>::OrderMatchSubstractError)?;

                o.remained_sell_amount = o.remained_sell_amount.checked_sub(&have_qty).ok_or(Error::<T>::OrderMatchSubstractError)?;
                o.remained_buy_amount = o.remained_buy_amount.checked_sub(&give_qty).ok_or(Error::<T>::OrderMatchSubstractError)?;

                if order.remained_buy_amount == Zero::zero() {
                    order.status = OrderStatus::Filled;
                    if order.remained_sell_amount != Zero::zero() {
                        <token::Module<T>>::do_unfreeze(order.owner.clone(), give, order.remained_sell_amount)?;
                        order.remained_sell_amount = Zero::zero();
                    }

                    <OwnedTPOpenedOrders<T>>::remove_order(order.owner.clone(), tp_hash, order.hash);
                    <OwnedTPClosedOrders<T>>::add_order(order.owner.clone(), tp_hash, order.hash);

                    ensure!(order.is_finished(), Error::<T>::OrderMatchOrderIsNotFinished);
                }

                if o.remained_buy_amount == Zero::zero() {
                    o.status = OrderStatus::Filled;
                    if o.remained_sell_amount != Zero::zero() {
                        <token::Module<T>>::do_unfreeze(o.owner.clone(), have, o.remained_sell_amount)?;
                        o.remained_sell_amount = Zero::zero();
                    }

                    <OwnedTPOpenedOrders<T>>::remove_order(o.owner.clone(), tp_hash, o.hash);
                    <OwnedTPClosedOrders<T>>::add_order(o.owner.clone(), tp_hash, o.hash);

                    ensure!(o.is_finished(), Error::<T>::OrderMatchOrderIsNotFinished);
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

        if order.status == OrderStatus::Filled {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn into_128<A: TryInto<u128>>(i: A) -> Result<u128, dispatch::DispatchError> {
        TryInto::<u128>::try_into(i).map_err(|_| Error::<T>::NumberCastError.into())
    }

    fn from_128<A: TryFrom<u128>>(i: u128) -> Result<A, dispatch::DispatchError> {
        TryFrom::<u128>::try_from(i).map_err(|_| Error::<T>::NumberCastError.into())
    }

    fn calculate_ex_amount(maker_order: &LimitOrder<T>, taker_order: &LimitOrder<T>) -> result::Result<(T::Balance, T::Balance), dispatch::DispatchError> {
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
        let mut seller_order_filled = true;
        if seller_order.remained_buy_amount <= buyer_order.remained_sell_amount { // seller_order is Filled
            let quote_qty: u128 =
                Self::into_128(seller_order.remained_buy_amount)? * T::PriceFactor::get() / maker_order.price.into();
            if Self::into_128(buyer_order.remained_buy_amount)? < quote_qty {
                seller_order_filled = false;
            }
        } else {
            let base_qty: u128 =
                Self::into_128(buyer_order.remained_buy_amount)? * maker_order.price.into() / T::PriceFactor::get();
            if Self::into_128(seller_order.remained_buy_amount)? >= base_qty {
                seller_order_filled = false;
            }
        }

        // if seller_order.remained_buy_amount <= buyer_order.remained_sell_amount { // seller_order is Filled
        if seller_order_filled {
            let mut quote_qty: u128 =
                Self::into_128(seller_order.remained_buy_amount)? * T::PriceFactor::get() / maker_order.price.into();
            let buy_amount_v2 = quote_qty * Self::into_128(maker_order.price)? / T::PriceFactor::get();
            if buy_amount_v2 != Self::into_128(seller_order.remained_buy_amount)? &&
                Self::into_128(buyer_order.remained_buy_amount)? > quote_qty // have fraction, seller(Filled) give more to align
            {
                quote_qty = quote_qty + 1;
            }

            return Ok
                ((
                    seller_order.remained_buy_amount,
                    Self::from_128(quote_qty)?
                ))
        } else { // buyer_order is Filled
            let mut base_qty: u128 =
                Self::into_128(buyer_order.remained_buy_amount)? * maker_order.price.into() / T::PriceFactor::get();
            let buy_amount_v2 = base_qty * T::PriceFactor::get() / maker_order.price.into();
            if buy_amount_v2 != Self::into_128(buyer_order.remained_buy_amount)? &&
                Self::into_128(seller_order.remained_buy_amount)? > base_qty // have fraction, buyer(Filled) give more to align
            {
                base_qty = base_qty + 1;
            }

            return Ok
                ((
                    Self::from_128(base_qty)?,
                    buyer_order.remained_buy_amount
                ))
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

    pub fn set_tp_market_data(tp_hash: T::Hash, price: T::Price, amount: T::Balance) -> dispatch::DispatchResult {

        let mut tp = <TradePairs<T>>::get(tp_hash).ok_or(Error::<T>::NoMatchingTradePair)?;

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

    fn do_cancel_limit_order(sender: T::AccountId, order_hash: T::Hash) -> dispatch::DispatchResult {
        let mut order = Self::order(order_hash).ok_or(Error::<T>::NoMatchingOrder)?;

        ensure!(order.owner == sender, Error::<T>::CanOnlyCancelOwnOrder);

        ensure!(!order.is_finished(), Error::<T>::CanOnlyCancelNotFinishedOrder);

        let tp_hash = Self::ensure_trade_pair(order.base, order.quote)?;

        <OrderLinkedItemList<T>>::remove_order(tp_hash, order.price, order.hash, order.sell_amount, order.buy_amount)?;

        order.status = OrderStatus::Canceled;
        <Orders<T>>::insert(order_hash, order.clone());

        <OwnedTPOpenedOrders<T>>::remove_order(sender.clone(), tp_hash, order_hash);
        <OwnedTPClosedOrders<T>>::add_order(sender.clone(), tp_hash, order_hash);

        let sell_hash = match order.otype {
            OrderType::Buy => order.base,
            OrderType::Sell => order.quote,
        };

        <token::Module<T>>::do_unfreeze(sender.clone(), sell_hash, order.remained_sell_amount)?;

        Self::deposit_event(RawEvent::OrderCanceled(sender, order_hash));

        Ok(())
    }

    fn debug_log_market(tp_hash: T::Hash) {
        if_std! {
            let mut item = <OrderLinkedItemList<T>>::read_bottom(tp_hash);

            eprintln!("[Market Orders]");

            loop {
                if item.price == Some(T::Price::min_value()) {
                    eprint!("Bottom ==> ");
                } else if item.price == Some(T::Price::max_value()) {
                    eprint!("Top ==> ");
                } else if item.price == None {
                    eprint!("Head ==> ");
                }

                eprint!("Price({:?}), Next({:?}), Prev({:?}), Sell_Amount({:?}), Buy_Amount({:?}), Orders({}): ", 
                    item.price, item.next, item.prev, item.sell_amount, item.buy_amount, item.orders.len());

                let mut orders = item.orders.iter();
                loop {
                    match orders.next() {
                        Some(order_hash) => {
                            let order = <Orders<T>>::get(order_hash).unwrap();
                            eprint!("({}@[{:?}]: Sell[{:?}, {:?}], Buy[{:?}, {:?}]), ", order.hash, order.status, 
                                order.sell_amount, order.remained_sell_amount, order.buy_amount, order.remained_buy_amount);
                        },
                        None => break,
                    }
                }

                eprintln!("");

                if item.next == Some(T::Price::min_value()) {
                    break;
                } else {
                    item = OrderLinkedItemList::<T>::read(tp_hash, item.next);
                }
            }

            eprintln!("[Market Trades]");

            let index_end = Self::trade_pair_owned_trades_index(tp_hash);
            for i in 0..index_end {
                let hash = Self::trade_pair_owned_trades((tp_hash, i));
                if let Some(hash) = hash {
                    let trade = <Trades<T>>::get(hash).unwrap();
                    eprintln!("[{}/{}] - {}@{:?}[{:?}]: [Buyer,Seller][{},{}], [Maker,Taker][{},{}], [Base,Quote][{:?}, {:?}]", 
                        trade.quote, trade.base, hash, trade.price, trade.otype, trade.buyer, trade.seller, trade.maker, 
                        trade.taker, trade.base_amount, trade.quote_amount);
                }
            }

            eprintln!("[Trade Pair Data]");
            let tp = Self::trade_pair(tp_hash).unwrap();
            eprintln!("latest matched price: {:?}", tp.latest_matched_price);

            eprintln!();
        }
    }
}
