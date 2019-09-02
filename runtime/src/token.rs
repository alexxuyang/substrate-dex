use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, dispatch::Result, ensure};
use system::ensure_signed;
use parity_codec::{Encode, Decode};
use runtime_primitives::traits::{Hash, As};
use rstd::prelude::Vec;

#[derive(Encode, Decode, Default)]
pub struct Token<Hash, Balance> {
	pub hash: Hash,
	pub symbol: Vec<u8>,
	pub total_supply: Balance,
}

pub trait Trait: balances::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_storage! {
	trait Store for Module<T: Trait> as token {
		Tokens get(token): map T::Hash => Option<Token<T::Hash, T::Balance>>;
		Owners get(owner): map T::Hash => Option<T::AccountId>;
		BalanceOf get(balance_of): map (T::AccountId, T::Hash) => T::Balance;
		FreeBalanceOf get(free_balance_of): map (T::AccountId, T::Hash) => T::Balance;
		FreezedBalanceOf get(freezed_balance_of): map (T::AccountId, T::Hash) => T::Balance;

		OwnedTokens get(owned_token): map (T::AccountId, u64) => Option<T::Hash>;
		OwnedTokenIndex get(owned_token_index): map T::AccountId => u64;

		Nonce: u64;
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event<T>() = default;

		pub fn issue(origin, symbol: Vec<u8>, total_supply: T::Balance) -> Result {
			Self::do_issue(origin, symbol, total_supply)
		}

		fn transfer(origin, to: T::AccountId, hash: T::Hash, amount: T::Balance) -> Result {
			Self::do_transfer(origin, to, hash, amount)
		}

		fn freeze(origin, hash: T::Hash, amount: T::Balance) -> Result {
			let sender = ensure_signed(origin)?;

			Self::do_freeze(sender, hash, amount)
		}

		fn unfreeze(origin, hash: T::Hash, amount: T::Balance) -> Result {
			let sender = ensure_signed(origin)?;

			Self::do_unfreeze(sender, hash, amount)
		}
	}
}

impl<T: Trait> Module<T> {
	pub fn do_issue(origin: T::Origin, symbol: Vec<u8>, total_supply: T::Balance) -> Result {
		let sender = ensure_signed(origin)?;

		let nonce = <Nonce<T>>::get();

		let hash = (<system::Module<T>>::random_seed(), nonce).using_encoded(<T as system::Trait>::Hashing::hash);

		runtime_io::print(hash.as_ref());

		let token = Token {
			symbol,
			total_supply,
			hash,
		};

		<Nonce<T>>::mutate(|x| *x += 1);
		<Tokens<T>>::insert(hash, token);
		<BalanceOf<T>>::insert((sender.clone(), hash), total_supply);
		<FreeBalanceOf<T>>::insert((sender.clone(), hash), total_supply);
		<Owners<T>>::insert(hash, sender.clone());

		let owned_token_index = <OwnedTokenIndex<T>>::get(sender.clone());
		<OwnedTokens<T>>::insert((sender.clone(), owned_token_index), hash);
		<OwnedTokenIndex<T>>::insert(sender.clone(), owned_token_index + 1);

		Self::deposit_event(RawEvent::TokenIssued(sender, hash, total_supply));

		Ok(())
	}

	fn do_transfer(origin: T::Origin, to: T::AccountId, hash: T::Hash, amount: T::Balance) -> Result {
		let sender = ensure_signed(origin)?;

		let token = Self::token(hash);
		ensure!(token.is_some(), "no matching token found");

		ensure!(<BalanceOf<T>>::exists((sender.clone(), hash)), "sender does not have the token");

		let from_amount = Self::balance_of((sender.clone(), hash));
		ensure!(from_amount >= amount, "sender does not have enough balance");
		let new_from_amount = from_amount - amount;

		let from_free_amount = Self::free_balance_of((sender.clone(), hash));
		ensure!(from_free_amount >= amount, "sender does not have enough free balance");
		let new_from_free_amount = from_free_amount - amount;

		let to_amount = Self::balance_of((to.clone(), hash));
		let new_to_amount = to_amount + amount;
		ensure!(new_to_amount.as_() <= u64::max_value(), "to amount overflow");

		let to_free_amount = Self::free_balance_of((to.clone(), hash));
		let new_to_free_amount = to_free_amount + amount;
		ensure!(new_to_free_amount.as_() <= u64::max_value(), "to free amount overflow");

		<BalanceOf<T>>::insert((sender.clone(), hash), new_from_amount);
		<FreeBalanceOf<T>>::insert((sender.clone(), hash), new_from_free_amount);
		<BalanceOf<T>>::insert((to.clone(), hash), new_to_amount);
		<FreeBalanceOf<T>>::insert((to.clone(), hash), new_to_free_amount);

		Self::deposit_event(RawEvent::TokenTransfered(sender, to, hash, amount));

		Ok(())
	}

    pub fn do_freeze(sender: T::AccountId, hash: T::Hash, amount: T::Balance) -> Result {
        
        let token = Self::token(hash);
        ensure!(token.is_some(), "no matching token found");

        ensure!(<FreeBalanceOf<T>>::exists((sender.clone(), hash.clone())), "sender does not have the token");

        let old_free_amount = Self::free_balance_of((sender.clone(), hash.clone()));
        ensure!(old_free_amount >= amount, "can not freeze more than available tokens");

        let old_freezed_amount = Self::freezed_balance_of((sender.clone(), hash.clone()));
        ensure!((old_freezed_amount + amount).as_() <= u64::max_value(), "freezed amount overflow");

        <FreeBalanceOf<T>>::insert((sender.clone(), hash.clone()), old_free_amount - amount);
        <FreezedBalanceOf<T>>::insert((sender.clone(), hash.clone()), old_freezed_amount + amount);

        Self::deposit_event(RawEvent::Freezed(sender, hash, amount));

        Ok(())
    }

    fn do_unfreeze(sender: T::AccountId, hash: T::Hash, amount: T::Balance) -> Result {
        
        let token = Self::token(hash);
        ensure!(token.is_some(), "no matching token found");

        ensure!(<FreeBalanceOf<T>>::exists((sender.clone(), hash.clone())), "sender does not have the token");

        let old_freezed_amount = Self::freezed_balance_of((sender.clone(), hash.clone()));
        ensure!(old_freezed_amount >= amount, "can not unfreeze more than available tokens");

        let old_free_amount = Self::free_balance_of((sender.clone(), hash.clone()));
        ensure!((old_free_amount + amount).as_() <= u64::max_value(), "unfreezed amount overflow");

        <FreeBalanceOf<T>>::insert((sender.clone(), hash.clone()), old_free_amount + amount);
        <FreezedBalanceOf<T>>::insert((sender.clone(), hash.clone()), old_freezed_amount - amount);

        Self::deposit_event(RawEvent::UnFreezed(sender, hash, amount));

        Ok(())
    }

	pub fn ensure_free_balance(sender: T::AccountId, hash: T::Hash, amount: T::Balance) -> Result {
		let token = Self::token(hash);
        ensure!(token.is_some(), "no matching token found");

        ensure!(<FreeBalanceOf<T>>::exists((sender.clone(), hash.clone())), "sender does not have the token");

		let free_amount = Self::free_balance_of((sender.clone(), hash));
		ensure!(free_amount >= amount, "does not have enough free balance to freeze");

		Ok(())
	}
}

decl_event!(
	pub enum Event<T> 
	where 
		<T as system::Trait>::AccountId,
		<T as system::Trait>::Hash,
		<T as balances::Trait>::Balance,
	{
		TokenIssued(AccountId, Hash, Balance),
		TokenTransfered(AccountId, AccountId, Hash, Balance),
		Freezed(AccountId, Hash, Balance),
		UnFreezed(AccountId, Hash, Balance),
	}
);

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
	#[derive(Clone, Eq, PartialEq)]
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
		/// The type for recording an account's balance.
		type Balance = u128;
		/// What to do if an account's free balance gets zeroed.
		type OnFreeBalanceZero = ();
		/// What to do if a new account is created.
		type OnNewAccount = ();
		/// The uniquitous event type.
		type Event = ();

		type TransactionPayment = ();
		type DustRemoval = ();
		type TransferPayment = ();
	}

	impl Trait for Test {
		type Event = ();
	}

	type TokenModule = super::Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
	}

	#[test]
	fn token_related_test_case() {
		with_externalities(&mut new_test_ext(), || {
			let ok: Result = Ok(());
			assert_ok!(ok);
			assert_eq!(1, 1);
			assert!(true);
			assert!(1 == 1);
			// assert_err!(Err("i am error"), "i am error");

			let ALICE = 10;
			let BOB = 20;
			let CHARLIE = 30;

			assert_ok!(TokenModule::issue(Origin::signed(ALICE), b"6688".to_vec(), 21000000));
			assert_eq!(TokenModule::owned_token_index(ALICE), 1);

			let token_hash = TokenModule::owned_token((ALICE, 0));
			assert!(token_hash.is_some());
			let token_hash = token_hash.unwrap();
			let token = TokenModule::token(token_hash);
			assert!(token.is_some());
			let token = token.unwrap();

			assert_eq!(TokenModule::balance_of((ALICE, token.hash)), 21000000);
			assert_eq!(TokenModule::free_balance_of((ALICE, token.hash)), 21000000);
			assert_eq!(TokenModule::freezed_balance_of((ALICE, token.hash)), 0);

			assert_ok!(TokenModule::transfer(Origin::signed(ALICE), BOB, token.hash, 100));

			assert_eq!(TokenModule::balance_of((ALICE, token.hash)), 20999900);
			assert_eq!(TokenModule::free_balance_of((ALICE, token.hash)), 20999900);
			assert_eq!(TokenModule::freezed_balance_of((ALICE, token.hash)), 0);

			assert_eq!(TokenModule::balance_of((BOB, token.hash)), 100);
			assert_eq!(TokenModule::free_balance_of((BOB, token.hash)), 100);
			assert_eq!(TokenModule::freezed_balance_of((BOB, token.hash)), 0);

			assert_err!(TokenModule::transfer(Origin::signed(ALICE), BOB, H256::from_low_u64_be(2), 100), "no matching token found");
			assert_err!(TokenModule::transfer(Origin::signed(CHARLIE), ALICE, token.hash, 100), "sender does not have the token");
			assert_err!(TokenModule::transfer(Origin::signed(BOB), ALICE, token.hash, 200), "sender does not have enough balance");
		});
	}
}
