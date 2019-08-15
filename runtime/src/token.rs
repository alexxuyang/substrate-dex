use support::{decl_storage, decl_module, StorageValue, StorageMap, decl_event, dispatch::Result, ensure, Parameter};
use system::ensure_signed;
use runtime_primitives::traits::{CheckedSub, CheckedAdd, Member, SimpleArithmetic, As, Hash};
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
		Issued(AccountId, Balance, Hash),
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

        Self::deposit_event(RawEvent::Issued(sender, total_supply, hash.clone()));

        Ok(())
    }
}