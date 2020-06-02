use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_err, traits::{OnFinalize, OnInitialize}};
use sp_core::H256;

fn run_to_block(n: u64) {
	while System::block_number() < n {
		Balances::on_finalize(System::block_number());
		System::on_finalize(System::block_number());
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		Balances::on_initialize(System::block_number());
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

#[test]
fn token_tests() {
	new_test_ext().execute_with(|| {
		run_to_block(1);

		let alice = 10u64;
		let bob = 20u64;
		let charlie = 30u64;

		assert_ok!(TokenModule::issue(
			Origin::signed(alice),
			b"6688".to_vec(),
			21000000
		));
		assert_eq!(TokenModule::owned_token_index(alice), 1);

		let token_hash = TokenModule::owned_token((alice, 0));
		assert!(token_hash.is_some());
		let token_hash = token_hash.unwrap();
		let token = TokenModule::token(token_hash);
		assert!(token.is_some());
		let token = token.unwrap();

		assert_eq!(TokenModule::balance_of((alice, token.hash)), 21000000);
		assert_eq!(TokenModule::free_balance_of((alice, token.hash)), 21000000);
		assert_eq!(TokenModule::freezed_balance_of((alice, token.hash)), 0);

		assert_ok!(TokenModule::transfer(
			Origin::signed(alice),
			token.hash,
			bob,
			100,
            None
		));
		assert_eq!(TokenModule::balance_of((alice, token.hash)), 20999900);
		assert_eq!(TokenModule::free_balance_of((alice, token.hash)), 20999900);
		assert_eq!(TokenModule::freezed_balance_of((alice, token.hash)), 0);
		assert_eq!(TokenModule::balance_of((bob, token.hash)), 100);
		assert_eq!(TokenModule::free_balance_of((bob, token.hash)), 100);
		assert_eq!(TokenModule::freezed_balance_of((bob, token.hash)), 0);

		assert_err!(
			TokenModule::transfer(Origin::signed(bob), H256::from_low_u64_be(0), charlie, 101, None),
			Error::<Test>::NoMatchingToken
		);
		assert_err!(
			TokenModule::transfer(Origin::signed(charlie), token.hash, bob, 101, None),
			Error::<Test>::SenderHaveNoToken
		);
		assert_err!(
			TokenModule::transfer(Origin::signed(bob), token.hash, charlie, 101, None),
			Error::<Test>::BalanceNotEnough
		);
	});
}
