use support::{StorageMap, Parameter, ensure, dispatch::Result};
use runtime_primitives::traits::Member;
use parity_codec::{Encode, Decode};
use rstd::prelude::*;
use crate::trade::{self, LimitOrderT};

#[derive(Encode, Decode, Clone)]
#[cfg_attr(feature="std", derive(PartialEq, Eq, Debug))]
/// K: Price, V: OrderHash
pub struct LinkedItem<K2, V> where
{
    pub prev: Option<K2>,
    pub next: Option<K2>,
    pub price: Option<K2>,
    pub orders: Vec<V>, // remove the item at 0 index will caused performance issue, should be optimized
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

pub struct LinkedList<T, S, K1, K2, V>(rstd::marker::PhantomData<(T, S, K1, K2, V)>);

/// S: StorageMap, K1: TradePairHash, K2: Price, V: OrderHash
impl<T, S, K1, K2, V> LinkedList<T, S, K1, K2, V> where
    T: trade::Trait,
    K1: Parameter + Clone,
    K2: Parameter + Member + Copy + PartialEq + Eq + PartialOrd,
    V: Parameter + Clone + LimitOrderT<T>,
    S: StorageMap<(K1, Option<K2>), LinkedItem<K2, V>, Query = Option<LinkedItem<K2, V>>>,
{
    pub fn read_head(key: &K1) -> LinkedItem<K2, V> {
        Self::read(key, None)
    }

    pub fn read(key1: &K1, key2: Option<K2>) -> LinkedItem<K2, V> {
        S::get(&(key1.clone(), key2)).unwrap_or_else(|| LinkedItem {
            prev: None,
            next: None,
            price: None,
            orders: Vec::new(),
        })
    }

    pub fn write(key1: &K1, key2: Option<K2>, item: LinkedItem<K2, V>) {
        S::insert(&(key1.clone(), key2), item);
    }

    pub fn append(key1: &K1, key2: K2, value: V) {

        let item = S::get(&(key1.clone(), Some(key2)));
        match item {
            Some(mut item) => {
                item.orders.push(value);
                return
            },
            None => {
                let mut item = Self::read_head(key1);
                while let Some(price) = item.next {
                    if key2 < price {
                        item = Self::read(key1, item.next);
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

    pub fn try_remove_K2(key1: &K1, key2: K2) {
        let item = S::get(&(key1.clone(), Some(key2)));
        match item {
            Some(item) => {
                if item.orders.len() != 0 {
                    return
                }
            },
            None => return,
        };

        if let Some(item) = S::take(&(key1.clone(), Some(key2))) {
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

    /// when we do order match, the first order in LinkedItem will match first
    /// and if this order's remained_amount is zero, then it should be remove from the list
    pub fn remove_value(key1: &K1, key2: K2) -> Result {
        let item = S::get(&(key1.clone(), Some(key2)));
        match item {
            Some(mut item) => {
                ensure!(item.orders.len() > 0, "there is no order when we want to remove it");

                let order = item.orders.get(0);
                ensure!(order.is_some(), "can not get order from index 0 when we want to remove it");

                let order = order.unwrap();
                ensure!(order.is_filled(), "try to remove not filled order");

                item.orders.remove(0);

                Self::write(key1, Some(key2), item);

                Ok(())
            },
            None => Err("try to remove order but the order list is NOT FOUND")
        }
    }
}