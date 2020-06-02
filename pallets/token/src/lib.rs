#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use sp_std::prelude::Vec;
use sp_runtime::{traits::{Bounded, Hash}};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, ensure, dispatch, StorageMap, StorageValue, traits::Randomness
};

use frame_system::ensure_signed;

use pallet_balances as balances;
use frame_system as system;
use pallet_randomness_collective_flip as randomness_collective_flip;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Token<Hash, Balance> {
    pub hash: Hash,
    pub symbol: Vec<u8>,
    pub total_supply: Balance,
}

pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

decl_error! {
	/// Error for the token module.
	pub enum Error for Module<T: Trait> {
		/// There is no match token
		NoMatchingToken,
		/// The balance is not enough
		BalanceNotEnough,
		/// Amount overflow
		AmountOverflow,
		/// Sender does not have token
		SenderHaveNoToken,
		/// Memo length exceed limitation
		MemoLengthExceedLimitation,
	}
}

decl_event!(
	pub enum Event<T> 
    where 
        <T as system::Trait>::AccountId,
        <T as system::Trait>::Hash,
        <T as balances::Trait>::Balance,
    {
		Issued(AccountId, Hash, Balance),
        Transferd(AccountId, AccountId, Hash, Balance),
        Freezed(AccountId, Hash, Balance),
        UnFreezed(AccountId, Hash, Balance),
	}
);

decl_storage! {
    trait Store for Module<T: Trait> as TokenModule {
        Tokens get(fn token): map hasher(blake2_128_concat) T::Hash => Option<Token<T::Hash, T::Balance>>;
        Owners get(fn owner): map hasher(blake2_128_concat) T::Hash => Option<T::AccountId>;
        BalanceOf get(fn balance_of): map hasher(blake2_128_concat) (T::AccountId, T::Hash) => T::Balance;
        FreeBalanceOf get(fn free_balance_of): map hasher(blake2_128_concat) (T::AccountId, T::Hash) => T::Balance;
        FreezedBalanceOf get(fn freezed_balance_of): map hasher(blake2_128_concat) (T::AccountId, T::Hash) => T::Balance;

        OwnedTokens get(fn owned_token): map hasher(blake2_128_concat) (T::AccountId, u64) => Option<T::Hash>;
        OwnedTokensIndex get(fn owned_token_index): map hasher(blake2_128_concat) T::AccountId => u64;

        Nonce get(fn nonce): u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        type Error = Error<T>;

		#[weight = 200_000]
        pub fn issue(origin, symbol: Vec<u8>, total_supply: T::Balance) -> dispatch::DispatchResult {
            Self::do_issue(origin, symbol, total_supply)
        }

		#[weight = 200_000]
        pub fn transfer(origin, token_hash: T::Hash, to: T::AccountId, amount: T::Balance, memo: Option<Vec<u8>>)
            -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?;
            Self::do_transfer(sender.clone(), token_hash, to.clone(), amount, memo)?;
            Self::deposit_event(RawEvent::Transferd(sender, to, token_hash, amount));

            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn do_issue(origin: T::Origin, symbol: Vec<u8>, total_supply: T::Balance) -> dispatch::DispatchResult {
        let sender = ensure_signed(origin)?;

        let nonce = Nonce::get();

        let random_seed = <randomness_collective_flip::Module<T>>::random_seed();
        let hash = (random_seed, sender.clone(), nonce)
            .using_encoded(<T as system::Trait>::Hashing::hash);

        let token = Token::<T::Hash, T::Balance> {
            hash: hash.clone(),
            total_supply,
            symbol: symbol.clone(),
        };

        Nonce::mutate(|n| *n += 1);
        Tokens::<T>::insert(hash.clone(), token);
        Owners::<T>::insert(hash.clone(), sender.clone());
        BalanceOf::<T>::insert((sender.clone(), hash.clone()), total_supply);
        FreeBalanceOf::<T>::insert((sender.clone(), hash.clone()), total_supply);

        let owned_token_index = OwnedTokensIndex::<T>::get(sender.clone());
        OwnedTokens::<T>::insert((sender.clone(), owned_token_index), hash);
        OwnedTokensIndex::<T>::insert(sender.clone(), owned_token_index + 1);

        Self::deposit_event(RawEvent::Issued(sender, hash.clone(), total_supply));

        Ok(())
    }

    pub fn do_transfer(
        sender: T::AccountId,
        hash: T::Hash,
        to: T::AccountId,
        amount: T::Balance,
        memo: Option<Vec<u8>>,
    ) -> dispatch::DispatchResult {
        let token = Self::token(hash);
        ensure!(token.is_some(), Error::<T>::NoMatchingToken);

        if let Some(memo) = memo {
            ensure!(memo.len() <= 512, Error::<T>::MemoLengthExceedLimitation);
        }

        ensure!(
            <FreeBalanceOf<T>>::contains_key((sender.clone(), hash)),
            Error::<T>::SenderHaveNoToken 
        );

        let from_amount = Self::balance_of((sender.clone(), hash.clone()));
        ensure!(from_amount >= amount, Error::<T>::BalanceNotEnough);
        let new_from_amount = from_amount - amount;

        let from_free_amount = Self::free_balance_of((sender.clone(), hash.clone()));
        ensure!(
            from_free_amount >= amount,
            Error::<T>::BalanceNotEnough
        );
        let new_from_free_amount = from_free_amount - amount;

        let to_amount = Self::balance_of((to.clone(), hash.clone()));
        let new_to_amount = to_amount + amount;
        ensure!(
            new_to_amount <= T::Balance::max_value(),
            Error::<T>::AmountOverflow
        );

        let to_free_amount = Self::free_balance_of((to.clone(), hash.clone()));
        let new_to_free_amount = to_free_amount + amount;
        ensure!(
            new_to_free_amount <= T::Balance::max_value(),
            Error::<T>::AmountOverflow
        );

        BalanceOf::<T>::insert((sender.clone(), hash.clone()), new_from_amount);
        FreeBalanceOf::<T>::insert((sender.clone(), hash.clone()), new_from_free_amount);
        BalanceOf::<T>::insert((to.clone(), hash.clone()), new_to_amount);
        FreeBalanceOf::<T>::insert((to.clone(), hash.clone()), new_to_free_amount);

        Ok(())
    }

    pub fn do_freeze(sender: T::AccountId, hash: T::Hash, amount: T::Balance) -> dispatch::DispatchResult {
        let token = Self::token(hash);
        ensure!(token.is_some(), Error::<T>::NoMatchingToken);

        ensure!(
            FreeBalanceOf::<T>::contains_key((sender.clone(), hash)),
            Error::<T>::SenderHaveNoToken
        );

        let old_free_amount = Self::free_balance_of((sender.clone(), hash.clone()));
        ensure!(
            old_free_amount >= amount,
            Error::<T>::BalanceNotEnough
        );

        let old_freezed_amount = Self::freezed_balance_of((sender.clone(), hash.clone()));
        ensure!(
            old_freezed_amount + amount <= T::Balance::max_value(),
            Error::<T>::AmountOverflow
        );

        FreeBalanceOf::<T>::insert((sender.clone(), hash.clone()), old_free_amount - amount);
        FreezedBalanceOf::<T>::insert((sender.clone(), hash.clone()), old_freezed_amount + amount);

        Self::deposit_event(RawEvent::Freezed(sender, hash, amount));

        Ok(())
    }

    pub fn do_unfreeze(sender: T::AccountId, hash: T::Hash, amount: T::Balance) -> dispatch::DispatchResult {
        let token = Self::token(hash);
        ensure!(token.is_some(), Error::<T>::NoMatchingToken);

        ensure!(
            FreeBalanceOf::<T>::contains_key((sender.clone(), hash)),
            Error::<T>::SenderHaveNoToken
        );

        let old_freezed_amount = Self::freezed_balance_of((sender.clone(), hash.clone()));
        ensure!(
            old_freezed_amount >= amount,
            Error::<T>::BalanceNotEnough
        );

        let old_free_amount = Self::free_balance_of((sender.clone(), hash.clone()));
        ensure!(
            old_free_amount + amount <= T::Balance::max_value(),
            Error::<T>::AmountOverflow
        );

        FreeBalanceOf::<T>::insert((sender.clone(), hash.clone()), old_free_amount + amount);
        FreezedBalanceOf::<T>::insert((sender.clone(), hash.clone()), old_freezed_amount - amount);

        Self::deposit_event(RawEvent::UnFreezed(sender, hash, amount));

        Ok(())
    }

    pub fn ensure_free_balance(sender: T::AccountId, hash: T::Hash, amount: T::Balance) -> dispatch::DispatchResult {
        let token = Self::token(hash);
        ensure!(token.is_some(), Error::<T>::NoMatchingToken);

        ensure!(
            FreeBalanceOf::<T>::contains_key((sender.clone(), hash.clone())),
            Error::<T>::SenderHaveNoToken
        );

        let free_amount = Self::free_balance_of((sender.clone(), hash.clone()));
        ensure!(
            free_amount >= amount,
            Error::<T>::BalanceNotEnough
        );

        Ok(())
    }
}
