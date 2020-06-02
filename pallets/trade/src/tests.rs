use super::*;

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_err, traits::{OnFinalize, OnInitialize}};
use sp_core::H256;

type System = system::Module<Test>;
type Balances = balances::Module<Test>;
type TokenModule = token::Module<Test>;
type TradeModule = super::Module<Test>;

fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default()
		.build_storage::<Test>()
		.unwrap()
		.into()
}

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
	new_test_ext().execute_with(|| {
		assert_eq!(System::block_number(), 0);
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

	println!("[Trade Pair Data]");
	let tp = TradeModule::trade_pair(tp_hash).unwrap();
	println!("latest matched price: {:?}", tp.latest_matched_price);

	println!();
}

#[test]
fn linked_list_test_case() {
	new_test_ext().execute_with(|| {
		run_to_block(10);

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
	new_test_ext().execute_with(|| {
        run_to_block(10);

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
	new_test_ext().execute_with(|| {
		run_to_block(10);

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
		assert_err!(TradeModule::cancel_limit_order(Origin::signed(alice), order101.hash), Error::<Test>::CanOnlyCancelNotFinishedOrder);

		assert_err!(TradeModule::cancel_limit_order(Origin::signed(alice), order1.hash), Error::<Test>::CanOnlyCancelOwnOrder);

		assert_err!(TradeModule::cancel_limit_order(Origin::signed(alice), H256::from_low_u64_be(0)), Error::<Test>::NoMatchingOrder);

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
	new_test_ext().execute_with(|| {
		run_to_block(10);

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
	new_test_ext().execute_with(|| {
        run_to_block(1);

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
	new_test_ext().execute_with(|| {
		let alice = 10;
		let bob = 20;

		let order1 = LimitOrder::<Test> {
			hash: H256::from_low_u64_be(0),
			base: H256::from_low_u64_be(0),
			quote: H256::from_low_u64_be(0),
			owner: alice,
			price: TradeModule::from_128(390000000).unwrap(),
			sell_amount: 3120,
			remained_sell_amount: 1590,
			buy_amount: 800,
			remained_buy_amount: 300,
			otype: OrderType::Buy,
			status: OrderStatus::PartialFilled,
		};

		let order2 = LimitOrder::<Test> {
			hash: H256::from_low_u64_be(0),
			base: H256::from_low_u64_be(0),
			quote: H256::from_low_u64_be(0),
			owner: bob,
			price: TradeModule::from_128(342000000).unwrap(),
			sell_amount: 400,
			remained_sell_amount: 400,
			buy_amount: 1368,
			remained_buy_amount: 1368,
			otype: OrderType::Sell,
			status: OrderStatus::Created,
		};

		let result = TradeModule::calculate_ex_amount(&order2, &order1).unwrap();
		assert_eq!(result.0, 1026);
		assert_eq!(result.1, 300);

		let order1 = LimitOrder::<Test> {
			hash: H256::from_low_u64_be(0),
			base: H256::from_low_u64_be(0),
			quote: H256::from_low_u64_be(0),
			owner: alice,
			price: TradeModule::from_128(390000000).unwrap(),
			sell_amount: 1170,
			remained_sell_amount: 594,
			buy_amount: 300,
			remained_buy_amount: 134,
			otype: OrderType::Buy,
			status: OrderStatus::PartialFilled,
		};

		let order2 = LimitOrder::<Test> {
			hash: H256::from_low_u64_be(0),
			base: H256::from_low_u64_be(0),
			quote: H256::from_low_u64_be(0),
			owner: bob,
			price: TradeModule::from_128(369000000).unwrap(),
			sell_amount: 1000,
			remained_sell_amount: 200,
			buy_amount: 3690,
			remained_buy_amount: 498,
			otype: OrderType::Sell,
			status: OrderStatus::PartialFilled,
		};

		let result = TradeModule::calculate_ex_amount(&order2, &order1).unwrap();
		assert_eq!(result.0, 498);
		assert_eq!(result.1, 134);

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
	new_test_ext().execute_with(|| {
		let price = TradeModule::from_128(25_010_000).unwrap();
		let amount = TradeModule::from_128(2501).unwrap();
		let otype = OrderType::Buy;
		assert_ok!(TradeModule::ensure_counterparty_amount_bounds(otype, price, amount), 10000u128);

		let price = TradeModule::from_128(25_010_000).unwrap(); // 0.2501
		let amount = TradeModule::from_128(2500).unwrap();
		let otype = OrderType::Buy;
		assert_err!(TradeModule::ensure_counterparty_amount_bounds(otype, price, amount), Error::<Test>::BoundsCheckFailed);

		let price = TradeModule::from_128(25_000_000).unwrap(); // 0.25
		let amount = TradeModule::from_128(24).unwrap();
		let otype = OrderType::Sell;
		assert_ok!(TradeModule::ensure_counterparty_amount_bounds(otype, price, amount), 6u128);

		let price = TradeModule::from_128(25_000_000).unwrap(); // 0.25
		let amount = TradeModule::from_128(21).unwrap();
		let otype = OrderType::Sell;
		assert_err!(TradeModule::ensure_counterparty_amount_bounds(otype, price, amount), Error::<Test>::BoundsCheckFailed);

		let price = TradeModule::from_128(200_000_000).unwrap(); // 2.0
		let amount = TradeModule::from_128(u128::max_value() - 1).unwrap();
		let otype = OrderType::Sell;
		assert_err!(TradeModule::ensure_counterparty_amount_bounds(otype, price, amount), Error::<Test>::BoundsCheckFailed);
	});
}

#[test]
fn price_as_vec_u8_to_x_by_100m_test_case() {
	new_test_ext().execute_with(|| {
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
                Error::<Test>::PriceLengthCheckFailed);

		let _price_v1 = 3.111222333f64;
		let price_v1_vec_u8: [u8; 8] = [255, 142, 214, 136, 200, 227, 8, 64];
		assert_err!(TradeModule::price_as_vec_u8_to_x_by_100m(price_v1_vec_u8.to_vec()),
                Error::<Test>::PriceLengthCheckFailed);

		let _price_v1 = 3.000011112222f64;
		let price_v1_vec_u8: [u8; 8] = [101, 10, 117, 211, 5, 0, 8, 64];
		assert_err!(TradeModule::price_as_vec_u8_to_x_by_100m(price_v1_vec_u8.to_vec()),
                Error::<Test>::PriceLengthCheckFailed);
	});
}
