use support::{decl_module, decl_storage, decl_event, StorageValue, StorageMap, dispatch::Result, ensure};
use system::ensure_signed;
use parity_codec::{Encode, Decode};
use runtime_primitives::traits::{Hash, As};
use rstd::prelude::Vec;

#[derive(Encode, Decode, Default)]
pub struct Token<Hash, Balance> {
	hash: Hash,
	symbol: Vec<u8>,
	total_supply: Balance,
}

pub trait Trait: balances::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

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

			Self::deposit_event(RawEvent::TokenIssued(sender, hash, total_supply));

			Ok(())
		}

		fn transfer(origin, to: T::AccountId, hash: T::Hash, amount: T::Balance) -> Result {
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
	}
);
