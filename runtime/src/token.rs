use support::{decl_storage, decl_module, StorageValue, StorageMap, decl_event, dispatch::Result, ensure, Parameter};
use system::ensure_signed;
use runtime_primitives::traits::{CheckedSub, CheckedAdd, Member, SimpleArithmetic, As, Hash, Zero};
use parity_codec::{Encode, Decode, Codec};
use rstd::prelude::Vec;
use runtime_io::print;

#[derive(Encode, Decode, Default, Clone, PartialEq)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Token<Hash, Balance> {
    hash: Hash,
    symbol: Vec<u8>,
    total_supply: Balance,
}

pub trait Trait: balances::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
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
	}
);

decl_storage! {
    trait Store for Module<T: Trait> as token {
        Tokens get(token): map T::Hash => Option<Token<T::Hash, T::Balance>>;
        BalanceOf get(balance_of): map (T::AccountId, T::Hash) => T::Balance;
        FreeBalanceOf get(free_balance_of): map (T::AccountId, T::Hash) => T::Balance;
        FreezedBalanceOf get(freezed_balance_of): map (T::AccountId, T::Hash) => T::Balance;

        Nonce: u64;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event<T>() = default;

        pub fn issue(origin, symbol: Vec<u8>, total_supply: T::Balance) -> Result {
            Self::do_issue(origin, symbol, total_supply)
        }

        pub fn transfer(origin, token_hash: T::Hash, to: T::AccountId, amount: T::Balance) -> Result {
            Self::do_transfer(origin, token_hash, to, amount)
        }
    }
}

impl<T: Trait> Module<T> {
    fn do_issue(origin: T::Origin, symbol: Vec<u8>, total_supply: T::Balance) -> Result {
        let sender = ensure_signed(origin)?;

        let nonce = <Nonce<T>>::get();

        let hash = (<system::Module<T>>::random_seed(), sender.clone(), nonce).using_encoded(<T as system::Trait>::Hashing::hash);

        runtime_io::print("hash");
        runtime_io::print(hash.as_ref());        

        let token = Token {
            hash: hash.clone(),
            total_supply,
            symbol: symbol.clone(),
        };

        <Nonce<T>>::mutate(|n| *n += 1);
        <Tokens<T>>::insert(hash.clone(), token);
        <BalanceOf<T>>::insert((sender.clone(), hash.clone()), total_supply);
        <FreeBalanceOf<T>>::insert((sender.clone(), hash.clone()), total_supply);

        Self::deposit_event(RawEvent::Issued(sender, hash.clone(), total_supply));

        Ok(())
    }

    fn do_transfer(origin: T::Origin, hash: T::Hash, to: T::AccountId, amount: T::Balance) -> Result {

        let token = Self::token(hash);
        ensure!(token.is_some(), "no matching token found");

        let sender = ensure_signed(origin)?;
        ensure!(<FreeBalanceOf<T>>::exists((sender.clone(), hash.clone())), "sender does not have the token");

        let from_amount = Self::balance_of((sender.clone(), hash.clone()));
        ensure!(from_amount >= amount, "sender does not have enough balance");
        let new_from_amount = from_amount - amount;

        let from_free_amount = Self::free_balance_of((sender.clone(), hash.clone()));
        ensure!(from_free_amount >= amount, "sender does not have enough free balance");
        let new_from_free_amount = from_free_amount - amount;

        let to_amount = Self::balance_of((to.clone(), hash.clone()));
        let new_to_amount = to_amount + amount;
        ensure!(new_to_amount.as_() <= u64::max_value(), "to amount overflow");

        let to_free_amount = Self::free_balance_of((to.clone(), hash.clone()));
        let new_to_free_amount = to_free_amount + amount;
        ensure!(new_to_free_amount.as_() <= u64::max_value(), "to free amount overflow");

        <BalanceOf<T>>::insert((sender.clone(), hash.clone()), new_from_amount);
        <FreeBalanceOf<T>>::insert((sender.clone(), hash.clone()), new_from_free_amount);
        <BalanceOf<T>>::insert((to.clone(), hash.clone()), new_to_amount);
        <FreeBalanceOf<T>>::insert((to.clone(), hash.clone()), new_to_free_amount);

        Self::deposit_event(RawEvent::Transferd(sender, to, hash, amount));

        Ok(())
    }
}
