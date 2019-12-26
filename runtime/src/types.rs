use codec::{Decode, Encode};
use rstd::prelude::*;
use sr_primitives::traits::{Bounded, Member, SimpleArithmetic};
use support::{dispatch::Result, ensure, Parameter, StorageMap};

use crate::trade::{self, *};

#[derive(Encode, Decode, Clone)]
#[cfg_attr(feature = "std", derive(PartialEq, Eq, Debug))]
pub struct LinkedItem<K1, K2, K3> {
	pub prev: Option<K2>,
	pub next: Option<K2>,
	pub price: Option<K2>,
	pub buy_amount: K3,
	pub sell_amount: K3,
	pub orders: Vec<K1>, // remove the item at 0 index will caused performance issue, should be optimized
}

pub struct LinkedList<T, S, K1, K2, K3>(rstd::marker::PhantomData<(T, S, K1, K2, K3)>);

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
impl<T, S, K1, K2, K3> LinkedList<T, S, K1, K2, K3>
where
	T: trade::Trait,
	K1: Encode
		+ Decode
		+ Clone
		+ rstd::borrow::Borrow<<T as system::Trait>::Hash>
		+ Copy
		+ PartialEq
        + AsRef<[u8]>,
	K2: Parameter + Default + Member + Bounded + SimpleArithmetic + Copy,
	K3: Parameter + Default + Member + Bounded + SimpleArithmetic + Copy,
	S: StorageMap<(K1, Option<K2>), LinkedItem<K1, K2, K3>, Query = Option<LinkedItem<K1, K2, K3>>>,
{
	pub fn read_head(key: K1) -> LinkedItem<K1, K2, K3> {
		Self::read(key, None)
	}

	pub fn read_bottom(key: K1) -> LinkedItem<K1, K2, K3> {
		Self::read(key, Some(K2::min_value()))
	}

	pub fn read_top(key: K1) -> LinkedItem<K1, K2, K3> {
		Self::read(key, Some(K2::max_value()))
	}

	pub fn read(key1: K1, key2: Option<K2>) -> LinkedItem<K1, K2, K3> {
		S::get((key1, key2)).unwrap_or_else(|| {
			let bottom = LinkedItem {
				prev: Some(K2::max_value()),
				next: None,
				price: Some(K2::min_value()),
				orders: Vec::<K1>::new(),
				buy_amount: Default::default(),
				sell_amount: Default::default(),
			};

			let top = LinkedItem {
				prev: None,
				next: Some(K2::min_value()),
				price: Some(K2::max_value()),
				orders: Vec::<K1>::new(),
				buy_amount: Default::default(),
				sell_amount: Default::default(),
			};

			let head = LinkedItem {
				prev: Some(K2::min_value()),
				next: Some(K2::max_value()),
				price: None,
				orders: Vec::<K1>::new(),
				buy_amount: Default::default(),
				sell_amount: Default::default(),
			};

			Self::write(key1, bottom.price, bottom);
			Self::write(key1, top.price, top);
			Self::write(key1, head.price, head.clone());
			head
		})
	}

	pub fn write(key1: K1, key2: Option<K2>, item: LinkedItem<K1, K2, K3>) {
		S::insert((key1, key2), item);
	}

	pub fn append(key1: K1, key2: K2, value: K1, sell_amount: K3, buy_amount: K3, otype: OrderType) {
        #[cfg(feature = "std")] 
        eprintln!("order book append item begin");

        #[cfg(feature = "std")] 
        eprintln!("order book append item: tp_hash[{:#?}], price[{:#?}], order_hash[{:#?}], order_type:[{:#?}], sell_amount[{:#?}], buy_amount[{:#?}]", key1.as_ref(), key2, value.as_ref(), otype, sell_amount, buy_amount);

		let item = S::get((key1, Some(key2)));
		match item {
			Some(mut item) => {
				item.orders.push(value);
				item.buy_amount = item.buy_amount + buy_amount;
				item.sell_amount = item.sell_amount + sell_amount;
				Self::write(key1, Some(key2), item);
				return;
			}
			None => {
				let start_item;
				let end_item;

				match otype {
					OrderType::Buy => {
						start_item = Some(K2::min_value());
						end_item = None;
					}
					OrderType::Sell => {
						start_item = None;
						end_item = Some(K2::max_value());
					}
				}

				let mut item = Self::read(key1, start_item);

				while item.next != end_item {
					match item.next {
						None => {}
						Some(price) => {
							if key2 < price {
								break;
							}
						}
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
					buy_amount,
					sell_amount,
					orders: v,
					price: Some(key2),
				};
				Self::write(key1, Some(key2), item);
			}
		};

        #[cfg(feature = "std")] 
        eprintln!("order book append item end");
	}

	pub fn next_match_price(item: &LinkedItem<K1, K2, K3>, otype: OrderType) -> Option<K2> {
		if otype == OrderType::Buy {
			item.prev
		} else {
			item.next
		}
	}

	pub fn update_amount(key1: K1, key2: K2, sell_amount: K3, buy_amount: K3) {
		let mut item = Self::read(key1, Some(key2));
		item.buy_amount = item.buy_amount - buy_amount;
		item.sell_amount = item.sell_amount - sell_amount;
		Self::write(key1, Some(key2), item);
	}

	pub fn remove_all(key1: K1, otype: OrderType) {
        #[cfg(feature = "std")] 
        eprintln!("order book remove all items begin");

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

			match Self::remove_orders_in_one_item(key1, key2.unwrap()) {
				Err(_) => break,
				_ => {}
			};

			head = Self::read_head(key1);
		}

        #[cfg(feature = "std")] 
        eprintln!("order book remove all items end");
	}

	pub fn remove_order(key1: K1, key2: K2, order_hash: K1, sell_amount: K3, buy_amount: K3) -> Result {
        #[cfg(feature = "std")] 
        eprintln!("order book cancel order begin");

		match S::get((key1, Some(key2))) {
			Some(mut item) => {
				ensure!(
					item.orders.contains(&order_hash),
					"cancel the order but not in market order list"
				);

				item.orders.retain(|&x| x != order_hash);
				item.buy_amount = item.buy_amount - buy_amount;
				item.sell_amount = item.sell_amount - sell_amount;
				Self::write(key1, Some(key2), item.clone());

                #[cfg(feature = "std")] 
                eprintln!("order book cancel order: tp_hash[{:#?}], price[{:#?}], order_hash[{:#?}]", key1.as_ref(), key2, order_hash.as_ref());

				if item.orders.len() == 0 {
					Self::remove_item(key1, key2);
				}
			}
			None => {}
		}

        #[cfg(feature = "std")] 
        eprintln!("order book cancel order end");

		Ok(())
	}

	pub fn remove_item(key1: K1, key2: K2) {
        #[cfg(feature = "std")] 
        eprintln!("order book remove item begin");

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

            #[cfg(feature = "std")] 
            eprintln!("order book remove item: tp_hash[{:#?}], price[{:#?}]", key1.as_ref(), key2);
		}

        #[cfg(feature = "std")] 
        eprintln!("order book remove itme end");
	}

	// when the order is canceled, it should be remove from Sell / Buy orders
	pub fn remove_orders_in_one_item(key1: K1, key2: K2) -> Result {
        #[cfg(feature = "std")] 
        eprintln!("order book remove order begin");

		match S::get((key1, Some(key2))) {
			Some(mut item) => {
				while item.orders.len() > 0 {
					let order_hash = item.orders.get(0).ok_or("can not get order hash")?;

					let order = <trade::Module<T>>::order(order_hash.borrow())
						.ok_or("can not get order")?;
					ensure!(order.is_finished(), "try to remove not finished order");

					item.orders.remove(0);

					Self::write(key1, Some(key2), item.clone());

                    #[cfg(feature = "std")] 
                    eprintln!("order book remove order: tp_hash[{:#?}], price[{:#?}], Owner[{:#?}], Type[{:#?}], Status[{:#?}], SellAmount[{:#?}], RemainedSellAmount[{:#?}], BuyAmount[{:#?}], RemainBuyAmount[{:#?}]", key1.as_ref(), key2, order.owner, order.otype, order.status, order.sell_amount, order.remained_sell_amount, order.buy_amount, order.remained_buy_amount);

				}

				if item.orders.len() == 0 {
					Self::remove_item(key1, key2);
				}
			}
			None => {}
		}

        #[cfg(feature = "std")] 
        eprintln!("order book remove order end");

		Ok(())
	}
}
