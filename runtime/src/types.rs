use support::{StorageMap, dispatch::Result, Parameter, ensure};
use runtime_primitives::traits::{SimpleArithmetic, Bounded, Member, As};
use parity_codec::{Compact, CompactAs, Encode, Decode};
use rstd::prelude::*;

use uint::construct_uint;
use core::convert::TryInto;

use crate::trade::{self, *};

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
	T: trade::Trait,
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
                    
					let order = <trade::Module<T>>::order(order_hash.borrow()).ok_or("can not get order")?;
					
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

construct_uint! {
	/// 128-bit unsigned integer.
	pub struct U128(2);
}

construct_uint! {
	/// 256-bit unsigned integer.
	pub struct U256(4);
}

/// FeeRate S.F precision
const SCALE_FACTOR: u128 = 1_000_000;

/// FeeRate (based on Permill), uses a scale factor
/// Inner type is `u128` in order to support compatibility with `generic_asset::Balance` type
#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Encode, Decode, Default, Copy, Clone, PartialEq, Eq)]
pub struct FeeRate(u128);

impl FeeRate {
	/// Create FeeRate as a decimal where `x / 1000`
	pub fn from_milli(x: u128) -> FeeRate {
		FeeRate(x * SCALE_FACTOR / 1000)
	}

	/// Create a FeeRate from a % i.e. `x / 100`
	pub fn from_percent(x: u128) -> FeeRate {
		FeeRate(x * SCALE_FACTOR / 100)
	}

	/// Divide a `As::as_<u128>` supported numeric by a FeeRate
	pub fn div<N: As<u128>>(lhs: N, rhs: FeeRate) -> N {
		N::sa(lhs.as_() * SCALE_FACTOR / rhs.0)
	}

	/// Divide a `u128` supported numeric by a FeeRate
	pub fn safe_div<N: Into<u128>>(lhs: N, rhs: FeeRate) -> rstd::result::Result<u128, &'static str> {
		let lhs = lhs.into();
		let lhs = U256::from(lhs);
		let scale_factor = U256::from(SCALE_FACTOR);
		let rhs = U256::from(rhs.0);
		let res: u128 = (lhs * scale_factor / rhs).try_into().map_err(|_| "Overflow error")?;

		Ok(res)
	}

	//Self - lhs and N - rhs
	pub fn safe_mul<N: Into<u128>>(lhs: FeeRate, rhs: N) -> rstd::result::Result<u128, &'static str> {
		let rhs = U256::from(rhs.into());
		let scale_factor = U256::from(SCALE_FACTOR);
		let lhs = U256::from(lhs.0);
		let res: u128 = (lhs * rhs / scale_factor).try_into().map_err(|_| "Overflow error")?;

		Ok(res)
	}

	/// Returns the equivalent of 1 or 100%
	pub fn one() -> FeeRate {
		FeeRate(SCALE_FACTOR)
	}
}

impl As<u128> for FeeRate {
	fn as_(self) -> u128 {
		self.0
	}
	/// Convert `u128` into a FeeRate
	fn sa(x: u128) -> Self {
		FeeRate(x)
	}
}

impl rstd::ops::Add<Self> for FeeRate {
	type Output = Self;
	fn add(self, rhs: FeeRate) -> Self::Output {
		FeeRate(self.0 + rhs.0)
	}
}

impl CompactAs for FeeRate {
	type As = u128;
	fn encode_as(&self) -> &u128 {
		&self.0
	}
	fn decode_from(x: u128) -> FeeRate {
		FeeRate(x)
	}
}

impl From<Compact<FeeRate>> for FeeRate {
	fn from(x: Compact<FeeRate>) -> FeeRate {
		x.0
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use support::{assert_ok, assert_err};

	#[test]
	fn div_works() {
		let fee_rate = FeeRate::from_percent(110);
		assert_eq!(FeeRate::div(10, fee_rate), 9); // Float value would be 9.0909

		let fee_rate = FeeRate::from_percent(10);
		assert_eq!(FeeRate::div(10, fee_rate), 100);
	}

	#[test]
	fn safe_div_works() {
		let fee_rate = FeeRate::from_percent(110);
		let lhs: u128 = 10;
		assert_ok!(FeeRate::safe_div(lhs, fee_rate), 9 as u128); // Float value would be 9.0909

		let fee_rate = FeeRate::from_percent(10);
		assert_ok!(FeeRate::safe_div(lhs, fee_rate), 100 as u128);
	}

	#[test]
	fn safe_div_overflow_works() {
		let fee_rate = FeeRate::from_percent(10);
		let lhs: u128 = u128::max_value();
		assert_err!(FeeRate::safe_div(lhs, fee_rate), "Overflow error");
	}

	#[test]
	fn add_works() {
		let fee_rate = FeeRate::from_percent(50) + FeeRate::from_percent(12);
		assert_eq!(fee_rate, FeeRate::from_percent(62));
	}

	#[test]
	fn safe_mul_works() {
		let fee_rate = FeeRate::from_percent(50);
		let rhs: u128 = 2;
		assert_ok!(FeeRate::safe_mul(fee_rate, rhs), 1 as u128);
	}

	#[test]
	fn safe_mul_overflow_works() {
		let fee_rate = FeeRate::from_percent(200);
		let rhs: u128 = u128::max_value();
		assert_err!(FeeRate::safe_mul(fee_rate, rhs), "Overflow error");
	}
}