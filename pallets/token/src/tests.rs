// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_err};
use sp_core::H256;

#[test]
fn token_tests() {
	new_test_ext().execute_with(|| {
		assert_eq!(1, 1);

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
			100
		));
		assert_eq!(TokenModule::balance_of((alice, token.hash)), 20999900);
		assert_eq!(TokenModule::free_balance_of((alice, token.hash)), 20999900);
		assert_eq!(TokenModule::freezed_balance_of((alice, token.hash)), 0);
		assert_eq!(TokenModule::balance_of((bob, token.hash)), 100);
		assert_eq!(TokenModule::free_balance_of((bob, token.hash)), 100);
		assert_eq!(TokenModule::freezed_balance_of((bob, token.hash)), 0);

		assert_err!(
			TokenModule::transfer(Origin::signed(bob), H256::from_low_u64_be(0), charlie, 101),
			Error::<Test>::NoMatchingToken
		);
		assert_err!(
			TokenModule::transfer(Origin::signed(charlie), token.hash, bob, 101),
			Error::<Test>::SenderHaveNoToken
		);
		assert_err!(
			TokenModule::transfer(Origin::signed(bob), token.hash, charlie, 101),
			Error::<Test>::BalanceNotEnough
		);
	});
}
