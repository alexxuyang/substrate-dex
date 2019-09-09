#![feature(prelude_import)]
#![no_std]
//! The Substrate Node Template runtime. This can be compiled with `#[no_std]`, ready for Wasm.
#![recursion_limit = "256"]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
pub use balances::Call as BalancesCall;
use client::{
    block_builder::api::{self as block_builder_api, CheckInherentsResult, InherentData},
    impl_runtime_apis, runtime_api,
};
pub use consensus::Call as ConsensusCall;
use parity_codec::{Decode, Encode};
#[cfg(feature = "std")]
use primitives::bytes;
use primitives::{ed25519, sr25519, OpaqueMetadata};
use rstd::prelude::*;
#[cfg(any(feature = "std", test))]
pub use runtime_primitives::BuildStorage;
use runtime_primitives::{
    create_runtime_str, generic,
    traits::{self, BlakeTwo256, Block as BlockT, NumberFor, StaticLookup, Verify},
    transaction_validity::TransactionValidity,
    ApplyResult,
};
pub use runtime_primitives::{Perbill, Permill};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
pub use support::{construct_runtime, StorageValue};
pub use timestamp::BlockPeriod;
pub use timestamp::Call as TimestampCall;
#[cfg(feature = "std")]
use version::NativeVersion;
use version::RuntimeVersion;
/// The type that is used for identifying authorities.
pub type AuthorityId = <AuthoritySignature as Verify>::Signer;
/// The type used by authorities to prove their ID.
pub type AuthoritySignature = ed25519::Signature;
/// Alias to pubkey that identifies an account on the chain.
pub type AccountId = <AccountSignature as Verify>::Signer;
/// The type used by authorities to prove their ID.
pub type AccountSignature = sr25519::Signature;
/// A hash of some data used by the chain.
pub type Hash = primitives::H256;
/// Index of a block number in the chain.
pub type BlockNumber = u64;
/// Index of an account's extrinsic in the chain.
pub type Nonce = u64;
/// Used for the module template in `./template.rs`
mod template {
    /// A runtime module template with necessary imports
    /// Feel free to remove or edit this file as needed.
    /// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
    /// If you remove this file, you can remove those references
    /// For more guidance on Substrate modules, see the example module
    /// https://github.com/paritytech/substrate/blob/master/srml/example/src/lib.rs
    use support::{decl_event, decl_module, decl_storage, dispatch::Result, StorageValue};
    use system::ensure_signed;
    /// The module's configuration trait.
    pub trait Trait: system::Trait {
        /// The overarching event type.
        type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    }
    #[doc(hidden)]
    mod sr_api_hidden_includes_decl_storage {
        pub extern crate support as hidden_include;
    }
    struct Something<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>,
    );
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < u32 > for Something < T > { type Query = Option < u32 > ; # [ doc = r" Get the storage key." ] fn key ( ) -> & 'static [ u8 ] { "TemplateModule Something" . as_bytes ( ) } # [ doc = r" Load the value from the provided storage instance." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Twox128 > > ( storage : & S ) -> Self :: Query { storage . get ( < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < u32 > > :: key ( ) ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Take a value from storage, removing it afterwards." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Twox128 > > ( storage : & S ) -> Self :: Query { storage . take ( < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < u32 > > :: key ( ) ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key." ] fn mutate < R , F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Twox128 > > ( f : F , storage : & S ) -> R { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < u32 > > :: get ( storage ) ; let ret = f ( & mut val ) ; match val { Some ( ref val ) => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < u32 > > :: put ( & val , storage ) , None => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < u32 > > :: kill ( storage ) , } ; ret } }
    trait Store {
        type Something;
    }
    #[doc(hidden)]
    pub struct __GetByteStructSomething<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_Something:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructSomething<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_Something
                .get_or_init(|| {
                    let def_val: Option<u32> = Default::default();
                    <Option<u32> as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    impl<T: Trait> Store for Module<T> {
        type Something = Something<T>;
    }
    impl<T: 'static + Trait> Module<T> {
        pub fn something() -> Option<u32> {
            < Something < T > as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < u32 > > :: get ( & self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: RuntimeStorage )
        }
        #[doc(hidden)]
        pub fn store_metadata(
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata
        {
            self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageMetadata { functions : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( { & [ self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Something" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Plain ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "u32" ) ) , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructSomething :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } ] } ) , }
        }
        #[doc(hidden)]pub fn store_metadata_functions ( ) -> & 'static [ self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata ]{
            {
                & [ self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Something" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Plain ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "u32" ) ) , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructSomething :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } ]
            }
        }
        #[doc(hidden)]
        pub fn store_metadata_name() -> &'static str {
            "TemplateModule"
        }
    }
    #[cfg(feature = "std")]
    pub struct Module<T: Trait>(::std::marker::PhantomData<(T)>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::clone::Clone + Trait> ::core::clone::Clone for Module<T> {
        #[inline]
        fn clone(&self) -> Module<T> {
            match *self {
                Module(ref __self_0_0) => Module(::core::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::marker::Copy + Trait> ::core::marker::Copy for Module<T> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::PartialEq + Trait> ::core::cmp::PartialEq for Module<T> {
        #[inline]
        fn eq(&self, other: &Module<T>) -> bool {
            match *other {
                Module(ref __self_1_0) => match *self {
                    Module(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Module<T>) -> bool {
            match *other {
                Module(ref __self_1_0) => match *self {
                    Module(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::Eq + Trait> ::core::cmp::Eq for Module<T> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<::std::marker::PhantomData<(T)>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::fmt::Debug + Trait> ::core::fmt::Debug for Module<T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Module(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("Module");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl<T: Trait> ::srml_support::runtime_primitives::traits::OnInitialize<T::BlockNumber>
        for Module<T>
    {
    }
    impl<T: Trait> ::srml_support::runtime_primitives::traits::OnFinalize<T::BlockNumber>
        for Module<T>
    {
    }
    impl<T: Trait> ::srml_support::runtime_primitives::traits::OffchainWorker<T::BlockNumber>
        for Module<T>
    {
    }
    impl<T: Trait> Module<T> {
        fn deposit_event(event: Event<T>) {
            <system::Module<T>>::deposit_event(<T as Trait>::from(event).into());
        }
    }
    /// Can also be called using [`Call`].
    ///
    /// [`Call`]: enum.Call.html
    impl<T: Trait> Module<T> {
        pub fn do_something(origin: T::Origin, something: u32) -> Result {
            let who = ensure_signed(origin)?;
            <Something<T>>::put(something);
            Self::deposit_event(RawEvent::SomethingStored(something, who));
            Ok(())
        }
    }
    #[cfg(feature = "std")]
    /// The module declaration.
    pub enum Call<T: Trait> {
        #[doc(hidden)]
        __PhantomItem(
            ::std::marker::PhantomData<(T)>,
            ::srml_support::dispatch::Never,
        ),
        #[allow(non_camel_case_types)]
        do_something(u32),
    }
    impl<T: Trait> ::srml_support::dispatch::Clone for Call<T> {
        fn clone(&self) -> Self {
            match *self {
                Call::do_something(ref something) => Call::do_something((*something).clone()),
                _ => ::std::rt::begin_panic(
                    "internal error: entered unreachable code",
                    &("runtime/src/template.rs", 32u32, 1u32),
                ),
            }
        }
    }
    impl<T: Trait> ::srml_support::dispatch::PartialEq for Call<T> {
        fn eq(&self, _other: &Self) -> bool {
            match *self {
                Call::do_something(ref something) => {
                    let self_params = (something,);
                    if let Call::do_something(ref something) = *_other {
                        self_params == (something,)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => ::std::rt::begin_panic(
                                "internal error: entered unreachable code",
                                &("runtime/src/template.rs", 32u32, 1u32),
                            ),
                            _ => false,
                        }
                    }
                }
                _ => ::std::rt::begin_panic(
                    "internal error: entered unreachable code",
                    &("runtime/src/template.rs", 32u32, 1u32),
                ),
            }
        }
    }
    impl<T: Trait> ::srml_support::dispatch::Eq for Call<T> {}
    #[cfg(feature = "std")]
    impl<T: Trait> ::srml_support::dispatch::fmt::Debug for Call<T> {
        fn fmt(
            &self,
            _f: &mut ::srml_support::dispatch::fmt::Formatter,
        ) -> ::srml_support::dispatch::result::Result<(), ::srml_support::dispatch::fmt::Error>
        {
            match *self {
                Call::do_something(ref something) => _f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (&"do_something", &(something.clone(),)) {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                        ],
                    },
                )),
                _ => ::std::rt::begin_panic(
                    "internal error: entered unreachable code",
                    &("runtime/src/template.rs", 32u32, 1u32),
                ),
            }
        }
    }
    impl<T: Trait> ::srml_support::dispatch::Decode for Call<T> {
        fn decode<Input: ::srml_support::dispatch::Input>(input: &mut Input) -> Option<Self> {
            let _input_id = input.read_byte()?;
            {
                if _input_id == (0) {
                    let something = ::srml_support::dispatch::Decode::decode(input)?;
                    return Some(Call::do_something(something));
                }
                None
            }
        }
    }
    impl<T: Trait> ::srml_support::dispatch::Encode for Call<T> {
        fn encode_to<W: ::srml_support::dispatch::Output>(&self, _dest: &mut W) {
            {
                if let Call::do_something(ref something) = *self {
                    _dest.push_byte((0) as u8);
                    something.encode_to(_dest);
                }
                {}
            };
        }
    }
    impl<T: Trait> ::srml_support::dispatch::Dispatchable for Call<T> {
        type Trait = T;
        type Origin = T::Origin;
        fn dispatch(self, _origin: Self::Origin) -> ::srml_support::dispatch::Result {
            match self {
                Call::do_something(something) => <Module<T>>::do_something(_origin, something),
                Call::__PhantomItem(_, _) => ::std::rt::begin_panic_fmt(
                    &::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"__PhantomItem should never be used.",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ),
                    &("runtime/src/template.rs", 32u32, 1u32),
                ),
            }
        }
    }
    impl<T: Trait> ::srml_support::dispatch::Callable for Module<T> {
        type Call = Call<T>;
    }
    impl<T: Trait> Module<T> {
        #[doc(hidden)]
        pub fn dispatch<D: ::srml_support::dispatch::Dispatchable<Trait = T>>(
            d: D,
            origin: D::Origin,
        ) -> ::srml_support::dispatch::Result {
            d.dispatch(origin)
        }
    }
    impl<T: Trait> Module<T> {
        #[doc(hidden)]
        pub fn call_functions() -> &'static [::srml_support::dispatch::FunctionMetadata] {
            &[::srml_support::dispatch::FunctionMetadata {
                name: ::srml_support::dispatch::DecodeDifferent::Encode("do_something"),
                arguments: ::srml_support::dispatch::DecodeDifferent::Encode(&[
                    ::srml_support::dispatch::FunctionArgumentMetadata {
                        name: ::srml_support::dispatch::DecodeDifferent::Encode("something"),
                        ty: ::srml_support::dispatch::DecodeDifferent::Encode("u32"),
                    },
                ]),
                documentation: ::srml_support::dispatch::DecodeDifferent::Encode(&[]),
            }]
        }
    }
    /// [`RawEvent`] specialized for the configuration [`Trait`]
    ///
    /// [`RawEvent`]: enum.RawEvent.html
    /// [`Trait`]: trait.Trait.html
    pub type Event<T> = RawEvent<<T as system::Trait>::AccountId>;
    /// Events for this module.
    ///
    pub enum RawEvent<AccountId> {
        SomethingStored(u32, AccountId),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<AccountId: ::core::clone::Clone> ::core::clone::Clone for RawEvent<AccountId> {
        #[inline]
        fn clone(&self) -> RawEvent<AccountId> {
            match (&*self,) {
                (&RawEvent::SomethingStored(ref __self_0, ref __self_1),) => {
                    RawEvent::SomethingStored(
                        ::core::clone::Clone::clone(&(*__self_0)),
                        ::core::clone::Clone::clone(&(*__self_1)),
                    )
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<AccountId: ::core::cmp::PartialEq> ::core::cmp::PartialEq for RawEvent<AccountId> {
        #[inline]
        fn eq(&self, other: &RawEvent<AccountId>) -> bool {
            match (&*self, &*other) {
                (
                    &RawEvent::SomethingStored(ref __self_0, ref __self_1),
                    &RawEvent::SomethingStored(ref __arg_1_0, ref __arg_1_1),
                ) => (*__self_0) == (*__arg_1_0) && (*__self_1) == (*__arg_1_1),
            }
        }
        #[inline]
        fn ne(&self, other: &RawEvent<AccountId>) -> bool {
            match (&*self, &*other) {
                (
                    &RawEvent::SomethingStored(ref __self_0, ref __self_1),
                    &RawEvent::SomethingStored(ref __arg_1_0, ref __arg_1_1),
                ) => (*__self_0) != (*__arg_1_0) || (*__self_1) != (*__arg_1_1),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<AccountId: ::core::cmp::Eq> ::core::cmp::Eq for RawEvent<AccountId> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<u32>;
                let _: ::core::cmp::AssertParamIsEq<AccountId>;
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_RawEvent: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl<AccountId> _parity_codec::Encode for RawEvent<AccountId>
        where
            AccountId: _parity_codec::Encode,
            AccountId: _parity_codec::Encode,
        {
            fn encode_to<EncOut: _parity_codec::Output>(&self, dest: &mut EncOut) {
                match *self {
                    RawEvent::SomethingStored(ref aa, ref ba) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                    }
                    _ => (),
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_RawEvent: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl<AccountId> _parity_codec::Decode for RawEvent<AccountId>
        where
            AccountId: _parity_codec::Decode,
            AccountId: _parity_codec::Decode,
        {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn) -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => Some(RawEvent::SomethingStored(
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                    )),
                    _ => None,
                }
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<AccountId: ::core::fmt::Debug> ::core::fmt::Debug for RawEvent<AccountId> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&RawEvent::SomethingStored(ref __self_0, ref __self_1),) => {
                    let mut debug_trait_builder = f.debug_tuple("SomethingStored");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl<AccountId> From<RawEvent<AccountId>> for () {
        fn from(_: RawEvent<AccountId>) -> () {
            ()
        }
    }
    impl<AccountId> RawEvent<AccountId> {
        #[allow(dead_code)]
        pub fn metadata() -> &'static [::srml_support::event::EventMetadata] {
            &[::srml_support::event::EventMetadata {
                name: ::srml_support::event::DecodeDifferent::Encode("SomethingStored"),
                arguments: ::srml_support::event::DecodeDifferent::Encode(&["u32", "AccountId"]),
                documentation: ::srml_support::event::DecodeDifferent::Encode(&[]),
            }]
        }
    }
}
mod token {
    use parity_codec::{Decode, Encode};
    use rstd::prelude::Vec;
    use runtime_primitives::traits::{As, Hash};
    use support::{
        decl_event, decl_module, decl_storage, dispatch::Result, ensure, StorageMap, StorageValue,
    };
    use system::ensure_signed;
    pub struct Token<Hash, Balance> {
        pub hash: Hash,
        pub symbol: Vec<u8>,
        pub total_supply: Balance,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_Token: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl<Hash, Balance> _parity_codec::Encode for Token<Hash, Balance>
        where
            Hash: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Balance: _parity_codec::Encode,
            Balance: _parity_codec::Encode,
        {
            fn encode_to<EncOut: _parity_codec::Output>(&self, dest: &mut EncOut) {
                dest.push(&self.hash);
                dest.push(&self.symbol);
                dest.push(&self.total_supply);
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_Token: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl<Hash, Balance> _parity_codec::Decode for Token<Hash, Balance>
        where
            Hash: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Balance: _parity_codec::Decode,
            Balance: _parity_codec::Decode,
        {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn) -> Option<Self> {
                Some(Token {
                    hash: _parity_codec::Decode::decode(input)?,
                    symbol: _parity_codec::Decode::decode(input)?,
                    total_supply: _parity_codec::Decode::decode(input)?,
                })
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<Hash: ::core::default::Default, Balance: ::core::default::Default> ::core::default::Default
        for Token<Hash, Balance>
    {
        #[inline]
        fn default() -> Token<Hash, Balance> {
            Token {
                hash: ::core::default::Default::default(),
                symbol: ::core::default::Default::default(),
                total_supply: ::core::default::Default::default(),
            }
        }
    }
    pub trait Trait: balances::Trait {
        type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    }
    #[doc(hidden)]
    mod sr_api_hidden_includes_decl_storage {
        pub extern crate support as hidden_include;
    }
    struct Tokens<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>,
    );
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , Token < T :: Hash , T :: Balance > > for Tokens < T > { type Query = Option < Token < T :: Hash , T :: Balance > > ; type Hasher = self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 ; # [ doc = r" Get the prefix key in storage." ] fn prefix ( ) -> & 'static [ u8 ] { "token Tokens" . as_bytes ( ) } # [ doc = r" Get the storage key used to fetch a value corresponding to a specific key." ] fn key_for ( x : & T :: Hash ) -> self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: vec :: Vec < u8 > { let mut key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , Token < T :: Hash , T :: Balance > > > :: prefix ( ) . to_vec ( ) ; self :: sr_api_hidden_includes_decl_storage :: hidden_include :: codec :: Encode :: encode_to ( x , & mut key ) ; key } # [ doc = r" Load the value associated with the given key from the map." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: Hash , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , Token < T :: Hash , T :: Balance > > > :: key_for ( key ) ; storage . get ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Take the value, reading and removing it." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: Hash , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , Token < T :: Hash , T :: Balance > > > :: key_for ( key ) ; storage . take ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key" ] fn mutate < R , F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: Hash , f : F , storage : & S ) -> R { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , Token < T :: Hash , T :: Balance > > > :: get ( key , storage ) ; let ret = f ( & mut val ) ; match val { Some ( ref val ) => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , Token < T :: Hash , T :: Balance > > > :: insert ( key , & val , storage ) , None => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , Token < T :: Hash , T :: Balance > > > :: remove ( key , storage ) , } ; ret } }
    struct Owners<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>,
    );
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , T :: AccountId > for Owners < T > { type Query = Option < T :: AccountId > ; type Hasher = self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 ; # [ doc = r" Get the prefix key in storage." ] fn prefix ( ) -> & 'static [ u8 ] { "token Owners" . as_bytes ( ) } # [ doc = r" Get the storage key used to fetch a value corresponding to a specific key." ] fn key_for ( x : & T :: Hash ) -> self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: vec :: Vec < u8 > { let mut key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , T :: AccountId > > :: prefix ( ) . to_vec ( ) ; self :: sr_api_hidden_includes_decl_storage :: hidden_include :: codec :: Encode :: encode_to ( x , & mut key ) ; key } # [ doc = r" Load the value associated with the given key from the map." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: Hash , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , T :: AccountId > > :: key_for ( key ) ; storage . get ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Take the value, reading and removing it." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: Hash , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , T :: AccountId > > :: key_for ( key ) ; storage . take ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key" ] fn mutate < R , F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: Hash , f : F , storage : & S ) -> R { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , T :: AccountId > > :: get ( key , storage ) ; let ret = f ( & mut val ) ; match val { Some ( ref val ) => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , T :: AccountId > > :: insert ( key , & val , storage ) , None => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , T :: AccountId > > :: remove ( key , storage ) , } ; ret } }
    struct BalanceOf<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>,
    );
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > for BalanceOf < T > { type Query = T :: Balance ; type Hasher = self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 ; # [ doc = r" Get the prefix key in storage." ] fn prefix ( ) -> & 'static [ u8 ] { "token BalanceOf" . as_bytes ( ) } # [ doc = r" Get the storage key used to fetch a value corresponding to a specific key." ] fn key_for ( x : & ( T :: AccountId , T :: Hash ) ) -> self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: vec :: Vec < u8 > { let mut key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > > :: prefix ( ) . to_vec ( ) ; self :: sr_api_hidden_includes_decl_storage :: hidden_include :: codec :: Encode :: encode_to ( x , & mut key ) ; key } # [ doc = r" Load the value associated with the given key from the map." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: AccountId , T :: Hash ) , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > > :: key_for ( key ) ; storage . get ( & key [ .. ] ) . unwrap_or_else ( | | Default :: default ( ) ) } # [ doc = r" Take the value, reading and removing it." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: AccountId , T :: Hash ) , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > > :: key_for ( key ) ; storage . take ( & key [ .. ] ) . unwrap_or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key" ] fn mutate < R , F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: AccountId , T :: Hash ) , f : F , storage : & S ) -> R { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > > :: get ( key , storage ) ; let ret = f ( & mut val ) ; < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > > :: insert ( key , & val , storage ) ; ret } }
    struct FreeBalanceOf<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>,
    );
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > for FreeBalanceOf < T > { type Query = T :: Balance ; type Hasher = self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 ; # [ doc = r" Get the prefix key in storage." ] fn prefix ( ) -> & 'static [ u8 ] { "token FreeBalanceOf" . as_bytes ( ) } # [ doc = r" Get the storage key used to fetch a value corresponding to a specific key." ] fn key_for ( x : & ( T :: AccountId , T :: Hash ) ) -> self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: vec :: Vec < u8 > { let mut key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > > :: prefix ( ) . to_vec ( ) ; self :: sr_api_hidden_includes_decl_storage :: hidden_include :: codec :: Encode :: encode_to ( x , & mut key ) ; key } # [ doc = r" Load the value associated with the given key from the map." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: AccountId , T :: Hash ) , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > > :: key_for ( key ) ; storage . get ( & key [ .. ] ) . unwrap_or_else ( | | Default :: default ( ) ) } # [ doc = r" Take the value, reading and removing it." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: AccountId , T :: Hash ) , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > > :: key_for ( key ) ; storage . take ( & key [ .. ] ) . unwrap_or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key" ] fn mutate < R , F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: AccountId , T :: Hash ) , f : F , storage : & S ) -> R { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > > :: get ( key , storage ) ; let ret = f ( & mut val ) ; < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > > :: insert ( key , & val , storage ) ; ret } }
    struct FreezedBalanceOf<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>,
    );
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > for FreezedBalanceOf < T > { type Query = T :: Balance ; type Hasher = self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 ; # [ doc = r" Get the prefix key in storage." ] fn prefix ( ) -> & 'static [ u8 ] { "token FreezedBalanceOf" . as_bytes ( ) } # [ doc = r" Get the storage key used to fetch a value corresponding to a specific key." ] fn key_for ( x : & ( T :: AccountId , T :: Hash ) ) -> self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: vec :: Vec < u8 > { let mut key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > > :: prefix ( ) . to_vec ( ) ; self :: sr_api_hidden_includes_decl_storage :: hidden_include :: codec :: Encode :: encode_to ( x , & mut key ) ; key } # [ doc = r" Load the value associated with the given key from the map." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: AccountId , T :: Hash ) , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > > :: key_for ( key ) ; storage . get ( & key [ .. ] ) . unwrap_or_else ( | | Default :: default ( ) ) } # [ doc = r" Take the value, reading and removing it." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: AccountId , T :: Hash ) , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > > :: key_for ( key ) ; storage . take ( & key [ .. ] ) . unwrap_or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key" ] fn mutate < R , F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: AccountId , T :: Hash ) , f : F , storage : & S ) -> R { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > > :: get ( key , storage ) ; let ret = f ( & mut val ) ; < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > > :: insert ( key , & val , storage ) ; ret } }
    struct OwnedTokens<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>,
    );
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , u64 ) , T :: Hash > for OwnedTokens < T > { type Query = Option < T :: Hash > ; type Hasher = self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 ; # [ doc = r" Get the prefix key in storage." ] fn prefix ( ) -> & 'static [ u8 ] { "token OwnedTokens" . as_bytes ( ) } # [ doc = r" Get the storage key used to fetch a value corresponding to a specific key." ] fn key_for ( x : & ( T :: AccountId , u64 ) ) -> self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: vec :: Vec < u8 > { let mut key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , u64 ) , T :: Hash > > :: prefix ( ) . to_vec ( ) ; self :: sr_api_hidden_includes_decl_storage :: hidden_include :: codec :: Encode :: encode_to ( x , & mut key ) ; key } # [ doc = r" Load the value associated with the given key from the map." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: AccountId , u64 ) , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , u64 ) , T :: Hash > > :: key_for ( key ) ; storage . get ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Take the value, reading and removing it." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: AccountId , u64 ) , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , u64 ) , T :: Hash > > :: key_for ( key ) ; storage . take ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key" ] fn mutate < R , F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: AccountId , u64 ) , f : F , storage : & S ) -> R { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , u64 ) , T :: Hash > > :: get ( key , storage ) ; let ret = f ( & mut val ) ; match val { Some ( ref val ) => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , u64 ) , T :: Hash > > :: insert ( key , & val , storage ) , None => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , u64 ) , T :: Hash > > :: remove ( key , storage ) , } ; ret } }
    struct OwnedTokenIndex<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>,
    );
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: AccountId , u64 > for OwnedTokenIndex < T > { type Query = u64 ; type Hasher = self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 ; # [ doc = r" Get the prefix key in storage." ] fn prefix ( ) -> & 'static [ u8 ] { "token OwnedTokenIndex" . as_bytes ( ) } # [ doc = r" Get the storage key used to fetch a value corresponding to a specific key." ] fn key_for ( x : & T :: AccountId ) -> self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: vec :: Vec < u8 > { let mut key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: AccountId , u64 > > :: prefix ( ) . to_vec ( ) ; self :: sr_api_hidden_includes_decl_storage :: hidden_include :: codec :: Encode :: encode_to ( x , & mut key ) ; key } # [ doc = r" Load the value associated with the given key from the map." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: AccountId , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: AccountId , u64 > > :: key_for ( key ) ; storage . get ( & key [ .. ] ) . unwrap_or_else ( | | Default :: default ( ) ) } # [ doc = r" Take the value, reading and removing it." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: AccountId , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: AccountId , u64 > > :: key_for ( key ) ; storage . take ( & key [ .. ] ) . unwrap_or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key" ] fn mutate < R , F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: AccountId , f : F , storage : & S ) -> R { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: AccountId , u64 > > :: get ( key , storage ) ; let ret = f ( & mut val ) ; < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: AccountId , u64 > > :: insert ( key , & val , storage ) ; ret } }
    struct Nonce<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>,
    );
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < u64 > for Nonce < T > { type Query = u64 ; # [ doc = r" Get the storage key." ] fn key ( ) -> & 'static [ u8 ] { "token Nonce" . as_bytes ( ) } # [ doc = r" Load the value from the provided storage instance." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Twox128 > > ( storage : & S ) -> Self :: Query { storage . get ( < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < u64 > > :: key ( ) ) . unwrap_or_else ( | | Default :: default ( ) ) } # [ doc = r" Take a value from storage, removing it afterwards." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Twox128 > > ( storage : & S ) -> Self :: Query { storage . take ( < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < u64 > > :: key ( ) ) . unwrap_or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key." ] fn mutate < R , F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Twox128 > > ( f : F , storage : & S ) -> R { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < u64 > > :: get ( storage ) ; let ret = f ( & mut val ) ; < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < u64 > > :: put ( & val , storage ) ; ret } }
    trait Store {
        type Tokens;
        type Owners;
        type BalanceOf;
        type FreeBalanceOf;
        type FreezedBalanceOf;
        type OwnedTokens;
        type OwnedTokenIndex;
        type Nonce;
    }
    #[doc(hidden)]
    pub struct __GetByteStructTokens<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_Tokens:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructTokens<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_Tokens
                .get_or_init(|| {
                    let def_val: Option<Token<T::Hash, T::Balance>> = Default::default();
                    <Option<Token<T::Hash, T::Balance>> as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructOwners<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_Owners:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructOwners<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_Owners
                .get_or_init(|| {
                    let def_val: Option<T::AccountId> = Default::default();
                    <Option<T::AccountId> as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructBalanceOf<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_BalanceOf:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructBalanceOf<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_BalanceOf
                .get_or_init(|| {
                    let def_val: T::Balance = Default::default();
                    <T::Balance as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructFreeBalanceOf<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_FreeBalanceOf:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructFreeBalanceOf<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_FreeBalanceOf
                .get_or_init(|| {
                    let def_val: T::Balance = Default::default();
                    <T::Balance as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructFreezedBalanceOf<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_FreezedBalanceOf:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructFreezedBalanceOf<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_FreezedBalanceOf
                .get_or_init(|| {
                    let def_val: T::Balance = Default::default();
                    <T::Balance as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructOwnedTokens<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_OwnedTokens:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructOwnedTokens<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_OwnedTokens
                .get_or_init(|| {
                    let def_val: Option<T::Hash> = Default::default();
                    <Option<T::Hash> as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructOwnedTokenIndex<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_OwnedTokenIndex:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructOwnedTokenIndex<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_OwnedTokenIndex
                .get_or_init(|| {
                    let def_val: u64 = Default::default();
                    <u64 as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructNonce<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_Nonce:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructNonce<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_Nonce
                .get_or_init(|| {
                    let def_val: u64 = Default::default();
                    <u64 as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    impl<T: Trait> Store for Module<T> {
        type Tokens = Tokens<T>;
        type Owners = Owners<T>;
        type BalanceOf = BalanceOf<T>;
        type FreeBalanceOf = FreeBalanceOf<T>;
        type FreezedBalanceOf = FreezedBalanceOf<T>;
        type OwnedTokens = OwnedTokens<T>;
        type OwnedTokenIndex = OwnedTokenIndex<T>;
        type Nonce = Nonce<T>;
    }
    impl<T: 'static + Trait> Module<T> {
        pub fn token<
            K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<
                T::Hash,
            >,
        >(
            key: K,
        ) -> Option<Token<T::Hash, T::Balance>> {
            < Tokens < T > as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , Token < T :: Hash , T :: Balance > > > :: get ( key . borrow ( ) , & self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: RuntimeStorage )
        }
        pub fn owner<
            K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<
                T::Hash,
            >,
        >(
            key: K,
        ) -> Option<T::AccountId> {
            < Owners < T > as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , T :: AccountId > > :: get ( key . borrow ( ) , & self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: RuntimeStorage )
        }
        pub fn balance_of<
            K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<(
                T::AccountId,
                T::Hash,
            )>,
        >(
            key: K,
        ) -> T::Balance {
            < BalanceOf < T > as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > > :: get ( key . borrow ( ) , & self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: RuntimeStorage )
        }
        pub fn free_balance_of<
            K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<(
                T::AccountId,
                T::Hash,
            )>,
        >(
            key: K,
        ) -> T::Balance {
            < FreeBalanceOf < T > as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > > :: get ( key . borrow ( ) , & self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: RuntimeStorage )
        }
        pub fn freezed_balance_of<
            K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<(
                T::AccountId,
                T::Hash,
            )>,
        >(
            key: K,
        ) -> T::Balance {
            < FreezedBalanceOf < T > as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , T :: Hash ) , T :: Balance > > :: get ( key . borrow ( ) , & self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: RuntimeStorage )
        }
        pub fn owned_token<
            K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<(
                T::AccountId,
                u64,
            )>,
        >(
            key: K,
        ) -> Option<T::Hash> {
            < OwnedTokens < T > as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , u64 ) , T :: Hash > > :: get ( key . borrow ( ) , & self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: RuntimeStorage )
        }
        pub fn owned_token_index<
            K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<
                T::AccountId,
            >,
        >(
            key: K,
        ) -> u64 {
            < OwnedTokenIndex < T > as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: AccountId , u64 > > :: get ( key . borrow ( ) , & self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: RuntimeStorage )
        }
        #[doc(hidden)]
        pub fn store_metadata(
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata
        {
            self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageMetadata { functions : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( { & [ self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Tokens" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Hash" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Token<T::Hash, T::Balance>" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructTokens :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Owners" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Hash" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::AccountId" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructOwners :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "BalanceOf" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Default , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "(T::AccountId, T::Hash)" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Balance" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructBalanceOf :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "FreeBalanceOf" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Default , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "(T::AccountId, T::Hash)" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Balance" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructFreeBalanceOf :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "FreezedBalanceOf" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Default , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "(T::AccountId, T::Hash)" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Balance" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructFreezedBalanceOf :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "OwnedTokens" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "(T::AccountId, u64)" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Hash" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructOwnedTokens :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "OwnedTokenIndex" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Default , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::AccountId" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "u64" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructOwnedTokenIndex :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Nonce" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Default , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Plain ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "u64" ) ) , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructNonce :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } ] } ) , }
        }
        #[doc(hidden)]pub fn store_metadata_functions ( ) -> & 'static [ self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata ]{
            {
                & [ self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Tokens" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Hash" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Token<T::Hash, T::Balance>" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructTokens :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Owners" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Hash" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::AccountId" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructOwners :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "BalanceOf" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Default , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "(T::AccountId, T::Hash)" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Balance" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructBalanceOf :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "FreeBalanceOf" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Default , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "(T::AccountId, T::Hash)" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Balance" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructFreeBalanceOf :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "FreezedBalanceOf" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Default , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "(T::AccountId, T::Hash)" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Balance" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructFreezedBalanceOf :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "OwnedTokens" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "(T::AccountId, u64)" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Hash" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructOwnedTokens :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "OwnedTokenIndex" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Default , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::AccountId" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "u64" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructOwnedTokenIndex :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Nonce" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Default , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Plain ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "u64" ) ) , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructNonce :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } ]
            }
        }
        #[doc(hidden)]
        pub fn store_metadata_name() -> &'static str {
            "token"
        }
    }
    #[cfg(feature = "std")]
    pub struct Module<T: Trait>(::std::marker::PhantomData<(T)>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::clone::Clone + Trait> ::core::clone::Clone for Module<T> {
        #[inline]
        fn clone(&self) -> Module<T> {
            match *self {
                Module(ref __self_0_0) => Module(::core::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::marker::Copy + Trait> ::core::marker::Copy for Module<T> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::PartialEq + Trait> ::core::cmp::PartialEq for Module<T> {
        #[inline]
        fn eq(&self, other: &Module<T>) -> bool {
            match *other {
                Module(ref __self_1_0) => match *self {
                    Module(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Module<T>) -> bool {
            match *other {
                Module(ref __self_1_0) => match *self {
                    Module(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::Eq + Trait> ::core::cmp::Eq for Module<T> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<::std::marker::PhantomData<(T)>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::fmt::Debug + Trait> ::core::fmt::Debug for Module<T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Module(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("Module");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl<T: Trait> ::srml_support::runtime_primitives::traits::OnInitialize<T::BlockNumber>
        for Module<T>
    {
    }
    impl<T: Trait> ::srml_support::runtime_primitives::traits::OnFinalize<T::BlockNumber>
        for Module<T>
    {
    }
    impl<T: Trait> ::srml_support::runtime_primitives::traits::OffchainWorker<T::BlockNumber>
        for Module<T>
    {
    }
    impl<T: Trait> Module<T> {
        fn deposit_event(event: Event<T>) {
            <system::Module<T>>::deposit_event(<T as Trait>::from(event).into());
        }
    }
    /// Can also be called using [`Call`].
    ///
    /// [`Call`]: enum.Call.html
    impl<T: Trait> Module<T> {
        pub fn issue(origin: T::Origin, symbol: Vec<u8>, total_supply: T::Balance) -> Result {
            Self::do_issue(origin, symbol, total_supply)
        }
        fn transfer(
            origin: T::Origin,
            to: T::AccountId,
            hash: T::Hash,
            amount: T::Balance,
        ) -> Result {
            Self::do_transfer(origin, to, hash, amount)
        }
        fn freeze(origin: T::Origin, hash: T::Hash, amount: T::Balance) -> Result {
            let sender = ensure_signed(origin)?;
            Self::do_freeze(sender, hash, amount)
        }
        fn unfreeze(origin: T::Origin, hash: T::Hash, amount: T::Balance) -> Result {
            let sender = ensure_signed(origin)?;
            Self::do_unfreeze(sender, hash, amount)
        }
    }
    #[cfg(feature = "std")]
    pub enum Call<T: Trait> {
        #[doc(hidden)]
        __PhantomItem(
            ::std::marker::PhantomData<(T)>,
            ::srml_support::dispatch::Never,
        ),
        #[allow(non_camel_case_types)]
        issue(Vec<u8>, T::Balance),
        #[allow(non_camel_case_types)]
        transfer(T::AccountId, T::Hash, T::Balance),
        #[allow(non_camel_case_types)]
        freeze(T::Hash, T::Balance),
        #[allow(non_camel_case_types)]
        unfreeze(T::Hash, T::Balance),
    }
    impl<T: Trait> ::srml_support::dispatch::Clone for Call<T> {
        fn clone(&self) -> Self {
            match *self {
                Call::issue(ref symbol, ref total_supply) => {
                    Call::issue((*symbol).clone(), (*total_supply).clone())
                }
                Call::transfer(ref to, ref hash, ref amount) => {
                    Call::transfer((*to).clone(), (*hash).clone(), (*amount).clone())
                }
                Call::freeze(ref hash, ref amount) => {
                    Call::freeze((*hash).clone(), (*amount).clone())
                }
                Call::unfreeze(ref hash, ref amount) => {
                    Call::unfreeze((*hash).clone(), (*amount).clone())
                }
                _ => ::std::rt::begin_panic(
                    "internal error: entered unreachable code",
                    &("runtime/src/token.rs", 33u32, 1u32),
                ),
            }
        }
    }
    impl<T: Trait> ::srml_support::dispatch::PartialEq for Call<T> {
        fn eq(&self, _other: &Self) -> bool {
            match *self {
                Call::issue(ref symbol, ref total_supply) => {
                    let self_params = (symbol, total_supply);
                    if let Call::issue(ref symbol, ref total_supply) = *_other {
                        self_params == (symbol, total_supply)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => ::std::rt::begin_panic(
                                "internal error: entered unreachable code",
                                &("runtime/src/token.rs", 33u32, 1u32),
                            ),
                            _ => false,
                        }
                    }
                }
                Call::transfer(ref to, ref hash, ref amount) => {
                    let self_params = (to, hash, amount);
                    if let Call::transfer(ref to, ref hash, ref amount) = *_other {
                        self_params == (to, hash, amount)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => ::std::rt::begin_panic(
                                "internal error: entered unreachable code",
                                &("runtime/src/token.rs", 33u32, 1u32),
                            ),
                            _ => false,
                        }
                    }
                }
                Call::freeze(ref hash, ref amount) => {
                    let self_params = (hash, amount);
                    if let Call::freeze(ref hash, ref amount) = *_other {
                        self_params == (hash, amount)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => ::std::rt::begin_panic(
                                "internal error: entered unreachable code",
                                &("runtime/src/token.rs", 33u32, 1u32),
                            ),
                            _ => false,
                        }
                    }
                }
                Call::unfreeze(ref hash, ref amount) => {
                    let self_params = (hash, amount);
                    if let Call::unfreeze(ref hash, ref amount) = *_other {
                        self_params == (hash, amount)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => ::std::rt::begin_panic(
                                "internal error: entered unreachable code",
                                &("runtime/src/token.rs", 33u32, 1u32),
                            ),
                            _ => false,
                        }
                    }
                }
                _ => ::std::rt::begin_panic(
                    "internal error: entered unreachable code",
                    &("runtime/src/token.rs", 33u32, 1u32),
                ),
            }
        }
    }
    impl<T: Trait> ::srml_support::dispatch::Eq for Call<T> {}
    #[cfg(feature = "std")]
    impl<T: Trait> ::srml_support::dispatch::fmt::Debug for Call<T> {
        fn fmt(
            &self,
            _f: &mut ::srml_support::dispatch::fmt::Formatter,
        ) -> ::srml_support::dispatch::result::Result<(), ::srml_support::dispatch::fmt::Error>
        {
            match *self {
                Call::issue(ref symbol, ref total_supply) => {
                    _f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["", ""],
                        &match (&"issue", &(symbol.clone(), total_supply.clone())) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                            ],
                        },
                    ))
                }
                Call::transfer(ref to, ref hash, ref amount) => {
                    _f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["", ""],
                        &match (&"transfer", &(to.clone(), hash.clone(), amount.clone())) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                            ],
                        },
                    ))
                }
                Call::freeze(ref hash, ref amount) => _f.write_fmt(::core::fmt::Arguments::new_v1(
                    &["", ""],
                    &match (&"freeze", &(hash.clone(), amount.clone())) {
                        (arg0, arg1) => [
                            ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                        ],
                    },
                )),
                Call::unfreeze(ref hash, ref amount) => {
                    _f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["", ""],
                        &match (&"unfreeze", &(hash.clone(), amount.clone())) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                            ],
                        },
                    ))
                }
                _ => ::std::rt::begin_panic(
                    "internal error: entered unreachable code",
                    &("runtime/src/token.rs", 33u32, 1u32),
                ),
            }
        }
    }
    impl<T: Trait> ::srml_support::dispatch::Decode for Call<T> {
        fn decode<Input: ::srml_support::dispatch::Input>(input: &mut Input) -> Option<Self> {
            let _input_id = input.read_byte()?;
            {
                if _input_id == (0) {
                    let symbol = ::srml_support::dispatch::Decode::decode(input)?;
                    let total_supply = ::srml_support::dispatch::Decode::decode(input)?;
                    return Some(Call::issue(symbol, total_supply));
                }
                {
                    if _input_id == (0 + 1) {
                        let to = ::srml_support::dispatch::Decode::decode(input)?;
                        let hash = ::srml_support::dispatch::Decode::decode(input)?;
                        let amount = ::srml_support::dispatch::Decode::decode(input)?;
                        return Some(Call::transfer(to, hash, amount));
                    }
                    {
                        if _input_id == (0 + 1 + 1) {
                            let hash = ::srml_support::dispatch::Decode::decode(input)?;
                            let amount = ::srml_support::dispatch::Decode::decode(input)?;
                            return Some(Call::freeze(hash, amount));
                        }
                        {
                            if _input_id == (0 + 1 + 1 + 1) {
                                let hash = ::srml_support::dispatch::Decode::decode(input)?;
                                let amount = ::srml_support::dispatch::Decode::decode(input)?;
                                return Some(Call::unfreeze(hash, amount));
                            }
                            None
                        }
                    }
                }
            }
        }
    }
    impl<T: Trait> ::srml_support::dispatch::Encode for Call<T> {
        fn encode_to<W: ::srml_support::dispatch::Output>(&self, _dest: &mut W) {
            {
                if let Call::issue(ref symbol, ref total_supply) = *self {
                    _dest.push_byte((0) as u8);
                    symbol.encode_to(_dest);
                    total_supply.encode_to(_dest);
                }
                {
                    if let Call::transfer(ref to, ref hash, ref amount) = *self {
                        _dest.push_byte((0 + 1) as u8);
                        to.encode_to(_dest);
                        hash.encode_to(_dest);
                        amount.encode_to(_dest);
                    }
                    {
                        if let Call::freeze(ref hash, ref amount) = *self {
                            _dest.push_byte((0 + 1 + 1) as u8);
                            hash.encode_to(_dest);
                            amount.encode_to(_dest);
                        }
                        {
                            if let Call::unfreeze(ref hash, ref amount) = *self {
                                _dest.push_byte((0 + 1 + 1 + 1) as u8);
                                hash.encode_to(_dest);
                                amount.encode_to(_dest);
                            }
                            {}
                        }
                    }
                }
            };
        }
    }
    impl<T: Trait> ::srml_support::dispatch::Dispatchable for Call<T> {
        type Trait = T;
        type Origin = T::Origin;
        fn dispatch(self, _origin: Self::Origin) -> ::srml_support::dispatch::Result {
            match self {
                Call::issue(symbol, total_supply) => {
                    <Module<T>>::issue(_origin, symbol, total_supply)
                }
                Call::transfer(to, hash, amount) => {
                    <Module<T>>::transfer(_origin, to, hash, amount)
                }
                Call::freeze(hash, amount) => <Module<T>>::freeze(_origin, hash, amount),
                Call::unfreeze(hash, amount) => <Module<T>>::unfreeze(_origin, hash, amount),
                Call::__PhantomItem(_, _) => ::std::rt::begin_panic_fmt(
                    &::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"__PhantomItem should never be used.",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ),
                    &("runtime/src/token.rs", 33u32, 1u32),
                ),
            }
        }
    }
    impl<T: Trait> ::srml_support::dispatch::Callable for Module<T> {
        type Call = Call<T>;
    }
    impl<T: Trait> Module<T> {
        #[doc(hidden)]
        pub fn dispatch<D: ::srml_support::dispatch::Dispatchable<Trait = T>>(
            d: D,
            origin: D::Origin,
        ) -> ::srml_support::dispatch::Result {
            d.dispatch(origin)
        }
    }
    impl<T: Trait> Module<T> {
        #[doc(hidden)]
        pub fn call_functions() -> &'static [::srml_support::dispatch::FunctionMetadata] {
            &[
                ::srml_support::dispatch::FunctionMetadata {
                    name: ::srml_support::dispatch::DecodeDifferent::Encode("issue"),
                    arguments: ::srml_support::dispatch::DecodeDifferent::Encode(&[
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("symbol"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("Vec<u8>"),
                        },
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("total_supply"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("T::Balance"),
                        },
                    ]),
                    documentation: ::srml_support::dispatch::DecodeDifferent::Encode(&[]),
                },
                ::srml_support::dispatch::FunctionMetadata {
                    name: ::srml_support::dispatch::DecodeDifferent::Encode("transfer"),
                    arguments: ::srml_support::dispatch::DecodeDifferent::Encode(&[
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("to"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("T::AccountId"),
                        },
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("hash"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("T::Hash"),
                        },
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("amount"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("T::Balance"),
                        },
                    ]),
                    documentation: ::srml_support::dispatch::DecodeDifferent::Encode(&[]),
                },
                ::srml_support::dispatch::FunctionMetadata {
                    name: ::srml_support::dispatch::DecodeDifferent::Encode("freeze"),
                    arguments: ::srml_support::dispatch::DecodeDifferent::Encode(&[
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("hash"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("T::Hash"),
                        },
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("amount"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("T::Balance"),
                        },
                    ]),
                    documentation: ::srml_support::dispatch::DecodeDifferent::Encode(&[]),
                },
                ::srml_support::dispatch::FunctionMetadata {
                    name: ::srml_support::dispatch::DecodeDifferent::Encode("unfreeze"),
                    arguments: ::srml_support::dispatch::DecodeDifferent::Encode(&[
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("hash"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("T::Hash"),
                        },
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("amount"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("T::Balance"),
                        },
                    ]),
                    documentation: ::srml_support::dispatch::DecodeDifferent::Encode(&[]),
                },
            ]
        }
    }
    impl<T: Trait> Module<T> {
        pub fn do_issue(origin: T::Origin, symbol: Vec<u8>, total_supply: T::Balance) -> Result {
            let sender = ensure_signed(origin)?;
            let nonce = <Nonce<T>>::get();
            let hash = (<system::Module<T>>::random_seed(), nonce)
                .using_encoded(<T as system::Trait>::hash);
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
        fn do_transfer(
            origin: T::Origin,
            to: T::AccountId,
            hash: T::Hash,
            amount: T::Balance,
        ) -> Result {
            let sender = ensure_signed(origin)?;
            let token = Self::token(hash);
            {
                if !token.is_some() {
                    {
                        return Err("no matching token found");
                    };
                }
            };
            {
                if !<BalanceOf<T>>::exists((sender.clone(), hash)) {
                    {
                        return Err("sender does not have the token");
                    };
                }
            };
            let from_amount = Self::balance_of((sender.clone(), hash));
            {
                if !(from_amount >= amount) {
                    {
                        return Err("sender does not have enough balance");
                    };
                }
            };
            let new_from_amount = from_amount - amount;
            let from_free_amount = Self::free_balance_of((sender.clone(), hash));
            {
                if !(from_free_amount >= amount) {
                    {
                        return Err("sender does not have enough free balance");
                    };
                }
            };
            let new_from_free_amount = from_free_amount - amount;
            let to_amount = Self::balance_of((to.clone(), hash));
            let new_to_amount = to_amount + amount;
            {
                if !(new_to_amount.as_() <= u64::max_value()) {
                    {
                        return Err("to amount overflow");
                    };
                }
            };
            let to_free_amount = Self::free_balance_of((to.clone(), hash));
            let new_to_free_amount = to_free_amount + amount;
            {
                if !(new_to_free_amount.as_() <= u64::max_value()) {
                    {
                        return Err("to free amount overflow");
                    };
                }
            };
            <BalanceOf<T>>::insert((sender.clone(), hash), new_from_amount);
            <FreeBalanceOf<T>>::insert((sender.clone(), hash), new_from_free_amount);
            <BalanceOf<T>>::insert((to.clone(), hash), new_to_amount);
            <FreeBalanceOf<T>>::insert((to.clone(), hash), new_to_free_amount);
            Self::deposit_event(RawEvent::TokenTransfered(sender, to, hash, amount));
            Ok(())
        }
        pub fn do_freeze(sender: T::AccountId, hash: T::Hash, amount: T::Balance) -> Result {
            let token = Self::token(hash);
            {
                if !token.is_some() {
                    {
                        return Err("no matching token found");
                    };
                }
            };
            {
                if !<FreeBalanceOf<T>>::exists((sender.clone(), hash.clone())) {
                    {
                        return Err("sender does not have the token");
                    };
                }
            };
            let old_free_amount = Self::free_balance_of((sender.clone(), hash.clone()));
            {
                if !(old_free_amount >= amount) {
                    {
                        return Err("can not freeze more than available tokens");
                    };
                }
            };
            let old_freezed_amount = Self::freezed_balance_of((sender.clone(), hash.clone()));
            {
                if !((old_freezed_amount + amount).as_() <= u64::max_value()) {
                    {
                        return Err("freezed amount overflow");
                    };
                }
            };
            <FreeBalanceOf<T>>::insert((sender.clone(), hash.clone()), old_free_amount - amount);
            <FreezedBalanceOf<T>>::insert(
                (sender.clone(), hash.clone()),
                old_freezed_amount + amount,
            );
            Self::deposit_event(RawEvent::Freezed(sender, hash, amount));
            Ok(())
        }
        fn do_unfreeze(sender: T::AccountId, hash: T::Hash, amount: T::Balance) -> Result {
            let token = Self::token(hash);
            {
                if !token.is_some() {
                    {
                        return Err("no matching token found");
                    };
                }
            };
            {
                if !<FreeBalanceOf<T>>::exists((sender.clone(), hash.clone())) {
                    {
                        return Err("sender does not have the token");
                    };
                }
            };
            let old_freezed_amount = Self::freezed_balance_of((sender.clone(), hash.clone()));
            {
                if !(old_freezed_amount >= amount) {
                    {
                        return Err("can not unfreeze more than available tokens");
                    };
                }
            };
            let old_free_amount = Self::free_balance_of((sender.clone(), hash.clone()));
            {
                if !((old_free_amount + amount).as_() <= u64::max_value()) {
                    {
                        return Err("unfreezed amount overflow");
                    };
                }
            };
            <FreeBalanceOf<T>>::insert((sender.clone(), hash.clone()), old_free_amount + amount);
            <FreezedBalanceOf<T>>::insert(
                (sender.clone(), hash.clone()),
                old_freezed_amount - amount,
            );
            Self::deposit_event(RawEvent::UnFreezed(sender, hash, amount));
            Ok(())
        }
        pub fn ensure_free_balance(
            sender: T::AccountId,
            hash: T::Hash,
            amount: T::Balance,
        ) -> Result {
            let token = Self::token(hash);
            {
                if !token.is_some() {
                    {
                        return Err("no matching token found");
                    };
                }
            };
            {
                if !<FreeBalanceOf<T>>::exists((sender.clone(), hash.clone())) {
                    {
                        return Err("sender does not have the token");
                    };
                }
            };
            let free_amount = Self::free_balance_of((sender.clone(), hash));
            {
                if !(free_amount >= amount) {
                    {
                        return Err("does not have enough free balance to freeze");
                    };
                }
            };
            Ok(())
        }
    }
    /// [`RawEvent`] specialized for the configuration [`Trait`]
    ///
    /// [`RawEvent`]: enum.RawEvent.html
    /// [`Trait`]: trait.Trait.html
    pub type Event<T> = RawEvent<
        <T as system::Trait>::AccountId,
        <T as system::Trait>::Hash,
        <T as balances::Trait>::Balance,
    >;
    /// Events for this module.
    ///
    pub enum RawEvent<AccountId, Hash, Balance> {
        TokenIssued(AccountId, Hash, Balance),
        TokenTransfered(AccountId, AccountId, Hash, Balance),
        Freezed(AccountId, Hash, Balance),
        UnFreezed(AccountId, Hash, Balance),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<
            AccountId: ::core::clone::Clone,
            Hash: ::core::clone::Clone,
            Balance: ::core::clone::Clone,
        > ::core::clone::Clone for RawEvent<AccountId, Hash, Balance>
    {
        #[inline]
        fn clone(&self) -> RawEvent<AccountId, Hash, Balance> {
            match (&*self,) {
                (&RawEvent::TokenIssued(ref __self_0, ref __self_1, ref __self_2),) => {
                    RawEvent::TokenIssued(
                        ::core::clone::Clone::clone(&(*__self_0)),
                        ::core::clone::Clone::clone(&(*__self_1)),
                        ::core::clone::Clone::clone(&(*__self_2)),
                    )
                }
                (&RawEvent::TokenTransfered(
                    ref __self_0,
                    ref __self_1,
                    ref __self_2,
                    ref __self_3,
                ),) => RawEvent::TokenTransfered(
                    ::core::clone::Clone::clone(&(*__self_0)),
                    ::core::clone::Clone::clone(&(*__self_1)),
                    ::core::clone::Clone::clone(&(*__self_2)),
                    ::core::clone::Clone::clone(&(*__self_3)),
                ),
                (&RawEvent::Freezed(ref __self_0, ref __self_1, ref __self_2),) => {
                    RawEvent::Freezed(
                        ::core::clone::Clone::clone(&(*__self_0)),
                        ::core::clone::Clone::clone(&(*__self_1)),
                        ::core::clone::Clone::clone(&(*__self_2)),
                    )
                }
                (&RawEvent::UnFreezed(ref __self_0, ref __self_1, ref __self_2),) => {
                    RawEvent::UnFreezed(
                        ::core::clone::Clone::clone(&(*__self_0)),
                        ::core::clone::Clone::clone(&(*__self_1)),
                        ::core::clone::Clone::clone(&(*__self_2)),
                    )
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<
            AccountId: ::core::cmp::PartialEq,
            Hash: ::core::cmp::PartialEq,
            Balance: ::core::cmp::PartialEq,
        > ::core::cmp::PartialEq for RawEvent<AccountId, Hash, Balance>
    {
        #[inline]
        fn eq(&self, other: &RawEvent<AccountId, Hash, Balance>) -> bool {
            {
                let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi =
                    unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &RawEvent::TokenIssued(ref __self_0, ref __self_1, ref __self_2),
                            &RawEvent::TokenIssued(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                        ) => {
                            (*__self_0) == (*__arg_1_0)
                                && (*__self_1) == (*__arg_1_1)
                                && (*__self_2) == (*__arg_1_2)
                        }
                        (
                            &RawEvent::TokenTransfered(
                                ref __self_0,
                                ref __self_1,
                                ref __self_2,
                                ref __self_3,
                            ),
                            &RawEvent::TokenTransfered(
                                ref __arg_1_0,
                                ref __arg_1_1,
                                ref __arg_1_2,
                                ref __arg_1_3,
                            ),
                        ) => {
                            (*__self_0) == (*__arg_1_0)
                                && (*__self_1) == (*__arg_1_1)
                                && (*__self_2) == (*__arg_1_2)
                                && (*__self_3) == (*__arg_1_3)
                        }
                        (
                            &RawEvent::Freezed(ref __self_0, ref __self_1, ref __self_2),
                            &RawEvent::Freezed(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                        ) => {
                            (*__self_0) == (*__arg_1_0)
                                && (*__self_1) == (*__arg_1_1)
                                && (*__self_2) == (*__arg_1_2)
                        }
                        (
                            &RawEvent::UnFreezed(ref __self_0, ref __self_1, ref __self_2),
                            &RawEvent::UnFreezed(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                        ) => {
                            (*__self_0) == (*__arg_1_0)
                                && (*__self_1) == (*__arg_1_1)
                                && (*__self_2) == (*__arg_1_2)
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &RawEvent<AccountId, Hash, Balance>) -> bool {
            {
                let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi =
                    unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &RawEvent::TokenIssued(ref __self_0, ref __self_1, ref __self_2),
                            &RawEvent::TokenIssued(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                        ) => {
                            (*__self_0) != (*__arg_1_0)
                                || (*__self_1) != (*__arg_1_1)
                                || (*__self_2) != (*__arg_1_2)
                        }
                        (
                            &RawEvent::TokenTransfered(
                                ref __self_0,
                                ref __self_1,
                                ref __self_2,
                                ref __self_3,
                            ),
                            &RawEvent::TokenTransfered(
                                ref __arg_1_0,
                                ref __arg_1_1,
                                ref __arg_1_2,
                                ref __arg_1_3,
                            ),
                        ) => {
                            (*__self_0) != (*__arg_1_0)
                                || (*__self_1) != (*__arg_1_1)
                                || (*__self_2) != (*__arg_1_2)
                                || (*__self_3) != (*__arg_1_3)
                        }
                        (
                            &RawEvent::Freezed(ref __self_0, ref __self_1, ref __self_2),
                            &RawEvent::Freezed(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                        ) => {
                            (*__self_0) != (*__arg_1_0)
                                || (*__self_1) != (*__arg_1_1)
                                || (*__self_2) != (*__arg_1_2)
                        }
                        (
                            &RawEvent::UnFreezed(ref __self_0, ref __self_1, ref __self_2),
                            &RawEvent::UnFreezed(ref __arg_1_0, ref __arg_1_1, ref __arg_1_2),
                        ) => {
                            (*__self_0) != (*__arg_1_0)
                                || (*__self_1) != (*__arg_1_1)
                                || (*__self_2) != (*__arg_1_2)
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
                } else {
                    true
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<AccountId: ::core::cmp::Eq, Hash: ::core::cmp::Eq, Balance: ::core::cmp::Eq>
        ::core::cmp::Eq for RawEvent<AccountId, Hash, Balance>
    {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<AccountId>;
                let _: ::core::cmp::AssertParamIsEq<Hash>;
                let _: ::core::cmp::AssertParamIsEq<Balance>;
                let _: ::core::cmp::AssertParamIsEq<AccountId>;
                let _: ::core::cmp::AssertParamIsEq<AccountId>;
                let _: ::core::cmp::AssertParamIsEq<Hash>;
                let _: ::core::cmp::AssertParamIsEq<Balance>;
                let _: ::core::cmp::AssertParamIsEq<AccountId>;
                let _: ::core::cmp::AssertParamIsEq<Hash>;
                let _: ::core::cmp::AssertParamIsEq<Balance>;
                let _: ::core::cmp::AssertParamIsEq<AccountId>;
                let _: ::core::cmp::AssertParamIsEq<Hash>;
                let _: ::core::cmp::AssertParamIsEq<Balance>;
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_RawEvent: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl<AccountId, Hash, Balance> _parity_codec::Encode for RawEvent<AccountId, Hash, Balance>
        where
            AccountId: _parity_codec::Encode,
            AccountId: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Balance: _parity_codec::Encode,
            Balance: _parity_codec::Encode,
            AccountId: _parity_codec::Encode,
            AccountId: _parity_codec::Encode,
            AccountId: _parity_codec::Encode,
            AccountId: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Balance: _parity_codec::Encode,
            Balance: _parity_codec::Encode,
            AccountId: _parity_codec::Encode,
            AccountId: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Balance: _parity_codec::Encode,
            Balance: _parity_codec::Encode,
            AccountId: _parity_codec::Encode,
            AccountId: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Balance: _parity_codec::Encode,
            Balance: _parity_codec::Encode,
        {
            fn encode_to<EncOut: _parity_codec::Output>(&self, dest: &mut EncOut) {
                match *self {
                    RawEvent::TokenIssued(ref aa, ref ba, ref ca) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                        dest.push(ca);
                    }
                    RawEvent::TokenTransfered(ref aa, ref ba, ref ca, ref da) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                        dest.push(ca);
                        dest.push(da);
                    }
                    RawEvent::Freezed(ref aa, ref ba, ref ca) => {
                        dest.push_byte(2usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                        dest.push(ca);
                    }
                    RawEvent::UnFreezed(ref aa, ref ba, ref ca) => {
                        dest.push_byte(3usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                        dest.push(ca);
                    }
                    _ => (),
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_RawEvent: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl<AccountId, Hash, Balance> _parity_codec::Decode for RawEvent<AccountId, Hash, Balance>
        where
            AccountId: _parity_codec::Decode,
            AccountId: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Balance: _parity_codec::Decode,
            Balance: _parity_codec::Decode,
            AccountId: _parity_codec::Decode,
            AccountId: _parity_codec::Decode,
            AccountId: _parity_codec::Decode,
            AccountId: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Balance: _parity_codec::Decode,
            Balance: _parity_codec::Decode,
            AccountId: _parity_codec::Decode,
            AccountId: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Balance: _parity_codec::Decode,
            Balance: _parity_codec::Decode,
            AccountId: _parity_codec::Decode,
            AccountId: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Balance: _parity_codec::Decode,
            Balance: _parity_codec::Decode,
        {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn) -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => Some(RawEvent::TokenIssued(
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                    )),
                    x if x == 1usize as u8 => Some(RawEvent::TokenTransfered(
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                    )),
                    x if x == 2usize as u8 => Some(RawEvent::Freezed(
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                    )),
                    x if x == 3usize as u8 => Some(RawEvent::UnFreezed(
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                    )),
                    _ => None,
                }
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<AccountId: ::core::fmt::Debug, Hash: ::core::fmt::Debug, Balance: ::core::fmt::Debug>
        ::core::fmt::Debug for RawEvent<AccountId, Hash, Balance>
    {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&RawEvent::TokenIssued(ref __self_0, ref __self_1, ref __self_2),) => {
                    let mut debug_trait_builder = f.debug_tuple("TokenIssued");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    let _ = debug_trait_builder.field(&&(*__self_2));
                    debug_trait_builder.finish()
                }
                (&RawEvent::TokenTransfered(
                    ref __self_0,
                    ref __self_1,
                    ref __self_2,
                    ref __self_3,
                ),) => {
                    let mut debug_trait_builder = f.debug_tuple("TokenTransfered");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    let _ = debug_trait_builder.field(&&(*__self_2));
                    let _ = debug_trait_builder.field(&&(*__self_3));
                    debug_trait_builder.finish()
                }
                (&RawEvent::Freezed(ref __self_0, ref __self_1, ref __self_2),) => {
                    let mut debug_trait_builder = f.debug_tuple("Freezed");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    let _ = debug_trait_builder.field(&&(*__self_2));
                    debug_trait_builder.finish()
                }
                (&RawEvent::UnFreezed(ref __self_0, ref __self_1, ref __self_2),) => {
                    let mut debug_trait_builder = f.debug_tuple("UnFreezed");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    let _ = debug_trait_builder.field(&&(*__self_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl<AccountId, Hash, Balance> From<RawEvent<AccountId, Hash, Balance>> for () {
        fn from(_: RawEvent<AccountId, Hash, Balance>) -> () {
            ()
        }
    }
    impl<AccountId, Hash, Balance> RawEvent<AccountId, Hash, Balance> {
        #[allow(dead_code)]
        pub fn metadata() -> &'static [::srml_support::event::EventMetadata] {
            &[
                ::srml_support::event::EventMetadata {
                    name: ::srml_support::event::DecodeDifferent::Encode("TokenIssued"),
                    arguments: ::srml_support::event::DecodeDifferent::Encode(&[
                        "AccountId",
                        "Hash",
                        "Balance",
                    ]),
                    documentation: ::srml_support::event::DecodeDifferent::Encode(&[]),
                },
                ::srml_support::event::EventMetadata {
                    name: ::srml_support::event::DecodeDifferent::Encode("TokenTransfered"),
                    arguments: ::srml_support::event::DecodeDifferent::Encode(&[
                        "AccountId",
                        "AccountId",
                        "Hash",
                        "Balance",
                    ]),
                    documentation: ::srml_support::event::DecodeDifferent::Encode(&[]),
                },
                ::srml_support::event::EventMetadata {
                    name: ::srml_support::event::DecodeDifferent::Encode("Freezed"),
                    arguments: ::srml_support::event::DecodeDifferent::Encode(&[
                        "AccountId",
                        "Hash",
                        "Balance",
                    ]),
                    documentation: ::srml_support::event::DecodeDifferent::Encode(&[]),
                },
                ::srml_support::event::EventMetadata {
                    name: ::srml_support::event::DecodeDifferent::Encode("UnFreezed"),
                    arguments: ::srml_support::event::DecodeDifferent::Encode(&[
                        "AccountId",
                        "Hash",
                        "Balance",
                    ]),
                    documentation: ::srml_support::event::DecodeDifferent::Encode(&[]),
                },
            ]
        }
    }
}
mod trade {
    use crate::token;
    use parity_codec::{Decode, Encode};
    use rstd::result;
    use runtime_primitives::traits::{Bounded, Hash, Member, SimpleArithmetic, Zero};
    use support::{
        decl_event, decl_module, decl_storage, dispatch::Result, ensure, Parameter, StorageMap,
        StorageValue,
    };
    use system::ensure_signed;
    pub trait Trait: token::Trait + system::Trait {
        type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
        type Price: Parameter + Member + Default + Bounded + SimpleArithmetic + Copy;
    }
    pub struct TradePair<Hash> {
        hash: Hash,
        base: Hash,
        quote: Hash,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<Hash: ::core::fmt::Debug> ::core::fmt::Debug for TradePair<Hash> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                TradePair {
                    hash: ref __self_0_0,
                    base: ref __self_0_1,
                    quote: ref __self_0_2,
                } => {
                    let mut debug_trait_builder = f.debug_struct("TradePair");
                    let _ = debug_trait_builder.field("hash", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("base", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("quote", &&(*__self_0_2));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_TradePair: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl<Hash> _parity_codec::Encode for TradePair<Hash>
        where
            Hash: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
        {
            fn encode_to<EncOut: _parity_codec::Output>(&self, dest: &mut EncOut) {
                dest.push(&self.hash);
                dest.push(&self.base);
                dest.push(&self.quote);
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_TradePair: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl<Hash> _parity_codec::Decode for TradePair<Hash>
        where
            Hash: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
        {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn) -> Option<Self> {
                Some(TradePair {
                    hash: _parity_codec::Decode::decode(input)?,
                    base: _parity_codec::Decode::decode(input)?,
                    quote: _parity_codec::Decode::decode(input)?,
                })
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<Hash: ::core::clone::Clone> ::core::clone::Clone for TradePair<Hash> {
        #[inline]
        fn clone(&self) -> TradePair<Hash> {
            match *self {
                TradePair {
                    hash: ref __self_0_0,
                    base: ref __self_0_1,
                    quote: ref __self_0_2,
                } => TradePair {
                    hash: ::core::clone::Clone::clone(&(*__self_0_0)),
                    base: ::core::clone::Clone::clone(&(*__self_0_1)),
                    quote: ::core::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<Hash: ::core::cmp::PartialEq> ::core::cmp::PartialEq for TradePair<Hash> {
        #[inline]
        fn eq(&self, other: &TradePair<Hash>) -> bool {
            match *other {
                TradePair {
                    hash: ref __self_1_0,
                    base: ref __self_1_1,
                    quote: ref __self_1_2,
                } => match *self {
                    TradePair {
                        hash: ref __self_0_0,
                        base: ref __self_0_1,
                        quote: ref __self_0_2,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &TradePair<Hash>) -> bool {
            match *other {
                TradePair {
                    hash: ref __self_1_0,
                    base: ref __self_1_1,
                    quote: ref __self_1_2,
                } => match *self {
                    TradePair {
                        hash: ref __self_0_0,
                        base: ref __self_0_1,
                        quote: ref __self_0_2,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<Hash: ::core::cmp::Eq> ::core::cmp::Eq for TradePair<Hash> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<Hash>;
                let _: ::core::cmp::AssertParamIsEq<Hash>;
                let _: ::core::cmp::AssertParamIsEq<Hash>;
            }
        }
    }
    pub enum OrderType {
        Buy,
        Sell,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for OrderType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&OrderType::Buy,) => {
                    let mut debug_trait_builder = f.debug_tuple("Buy");
                    debug_trait_builder.finish()
                }
                (&OrderType::Sell,) => {
                    let mut debug_trait_builder = f.debug_tuple("Sell");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_OrderType: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for OrderType {
            fn encode_to<EncOut: _parity_codec::Output>(&self, dest: &mut EncOut) {
                match *self {
                    OrderType::Buy => {
                        dest.push_byte(0usize as u8);
                    }
                    OrderType::Sell => {
                        dest.push_byte(1usize as u8);
                    }
                    _ => (),
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_OrderType: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for OrderType {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn) -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => Some(OrderType::Buy),
                    x if x == 1usize as u8 => Some(OrderType::Sell),
                    _ => None,
                }
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for OrderType {
        #[inline]
        fn clone(&self) -> OrderType {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for OrderType {
        #[inline]
        fn eq(&self, other: &OrderType) -> bool {
            {
                let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi =
                    unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for OrderType {}
    pub enum OrderStatus {
        Created,
        PartialFilled,
        Filled,
        Canceled,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_OrderStatus: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for OrderStatus {
            fn encode_to<EncOut: _parity_codec::Output>(&self, dest: &mut EncOut) {
                match *self {
                    OrderStatus::Created => {
                        dest.push_byte(0usize as u8);
                    }
                    OrderStatus::PartialFilled => {
                        dest.push_byte(1usize as u8);
                    }
                    OrderStatus::Filled => {
                        dest.push_byte(2usize as u8);
                    }
                    OrderStatus::Canceled => {
                        dest.push_byte(3usize as u8);
                    }
                    _ => (),
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_OrderStatus: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for OrderStatus {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn) -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => Some(OrderStatus::Created),
                    x if x == 1usize as u8 => Some(OrderStatus::PartialFilled),
                    x if x == 2usize as u8 => Some(OrderStatus::Filled),
                    x if x == 3usize as u8 => Some(OrderStatus::Canceled),
                    _ => None,
                }
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for OrderStatus {
        #[inline]
        fn eq(&self, other: &OrderStatus) -> bool {
            {
                let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi =
                    unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        _ => true,
                    }
                } else {
                    false
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for OrderStatus {
        #[inline]
        fn clone(&self) -> OrderStatus {
            {
                *self
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for OrderStatus {}
    pub struct LimitOrder<T>
    where
        T: Trait,
    {
        hash: T::Hash,
        base: T::Hash,
        quote: T::Hash,
        owner: T::AccountId,
        price: T::Price,
        amount: T::Balance,
        remained_amount: T::Balance,
        otype: OrderType,
        status: OrderStatus,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_LimitOrder: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl<T> _parity_codec::Encode for LimitOrder<T>
        where
            T: Trait,
            T::Hash: _parity_codec::Encode,
            T::Hash: _parity_codec::Encode,
            T::Hash: _parity_codec::Encode,
            T::Hash: _parity_codec::Encode,
            T::Hash: _parity_codec::Encode,
            T::Hash: _parity_codec::Encode,
            T::AccountId: _parity_codec::Encode,
            T::AccountId: _parity_codec::Encode,
            T::Price: _parity_codec::Encode,
            T::Price: _parity_codec::Encode,
            T::Balance: _parity_codec::Encode,
            T::Balance: _parity_codec::Encode,
            T::Balance: _parity_codec::Encode,
            T::Balance: _parity_codec::Encode,
        {
            fn encode_to<EncOut: _parity_codec::Output>(&self, dest: &mut EncOut) {
                dest.push(&self.hash);
                dest.push(&self.base);
                dest.push(&self.quote);
                dest.push(&self.owner);
                dest.push(&self.price);
                dest.push(&self.amount);
                dest.push(&self.remained_amount);
                dest.push(&self.otype);
                dest.push(&self.status);
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_LimitOrder: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl<T> _parity_codec::Decode for LimitOrder<T>
        where
            T: Trait,
            T::Hash: _parity_codec::Decode,
            T::Hash: _parity_codec::Decode,
            T::Hash: _parity_codec::Decode,
            T::Hash: _parity_codec::Decode,
            T::Hash: _parity_codec::Decode,
            T::Hash: _parity_codec::Decode,
            T::AccountId: _parity_codec::Decode,
            T::AccountId: _parity_codec::Decode,
            T::Price: _parity_codec::Decode,
            T::Price: _parity_codec::Decode,
            T::Balance: _parity_codec::Decode,
            T::Balance: _parity_codec::Decode,
            T::Balance: _parity_codec::Decode,
            T::Balance: _parity_codec::Decode,
        {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn) -> Option<Self> {
                Some(LimitOrder {
                    hash: _parity_codec::Decode::decode(input)?,
                    base: _parity_codec::Decode::decode(input)?,
                    quote: _parity_codec::Decode::decode(input)?,
                    owner: _parity_codec::Decode::decode(input)?,
                    price: _parity_codec::Decode::decode(input)?,
                    amount: _parity_codec::Decode::decode(input)?,
                    remained_amount: _parity_codec::Decode::decode(input)?,
                    otype: _parity_codec::Decode::decode(input)?,
                    status: _parity_codec::Decode::decode(input)?,
                })
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::clone::Clone> ::core::clone::Clone for LimitOrder<T>
    where
        T: Trait,
        T::Hash: ::core::clone::Clone,
        T::Hash: ::core::clone::Clone,
        T::Hash: ::core::clone::Clone,
        T::AccountId: ::core::clone::Clone,
        T::Price: ::core::clone::Clone,
        T::Balance: ::core::clone::Clone,
        T::Balance: ::core::clone::Clone,
    {
        #[inline]
        fn clone(&self) -> LimitOrder<T> {
            match *self {
                LimitOrder {
                    hash: ref __self_0_0,
                    base: ref __self_0_1,
                    quote: ref __self_0_2,
                    owner: ref __self_0_3,
                    price: ref __self_0_4,
                    amount: ref __self_0_5,
                    remained_amount: ref __self_0_6,
                    otype: ref __self_0_7,
                    status: ref __self_0_8,
                } => LimitOrder {
                    hash: ::core::clone::Clone::clone(&(*__self_0_0)),
                    base: ::core::clone::Clone::clone(&(*__self_0_1)),
                    quote: ::core::clone::Clone::clone(&(*__self_0_2)),
                    owner: ::core::clone::Clone::clone(&(*__self_0_3)),
                    price: ::core::clone::Clone::clone(&(*__self_0_4)),
                    amount: ::core::clone::Clone::clone(&(*__self_0_5)),
                    remained_amount: ::core::clone::Clone::clone(&(*__self_0_6)),
                    otype: ::core::clone::Clone::clone(&(*__self_0_7)),
                    status: ::core::clone::Clone::clone(&(*__self_0_8)),
                },
            }
        }
    }
    impl<T> LimitOrder<T>
    where
        T: Trait,
    {
        pub fn new(
            base: T::Hash,
            quote: T::Hash,
            owner: T::AccountId,
            price: T::Price,
            amount: T::Balance,
            otype: OrderType,
        ) -> Self {
            let hash = (
                base,
                quote,
                price,
                amount,
                owner.clone(),
                <system::Module<T>>::random_seed(),
            )
                .using_encoded(<T as system::Trait>::hash);
            LimitOrder {
                hash,
                base,
                quote,
                owner,
                price,
                otype,
                amount,
                status: OrderStatus::Created,
                remained_amount: amount,
            }
        }
        fn is_finished(&self) -> bool {
            (self.remained_amount == Zero::zero() && self.status == OrderStatus::Filled)
                || self.status == OrderStatus::Canceled
        }
    }
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
    pub struct LinkedItem<T>
    where
        T: Trait,
    {
        pub price: Option<T::Price>,
        pub next: Option<T::Price>,
        pub prev: Option<T::Price>,
        pub orders: Vec<T::Hash>,
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_LinkedItem: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl<T> _parity_codec::Encode for LinkedItem<T>
        where
            T: Trait,
            Option<T::Price>: _parity_codec::Encode,
            Option<T::Price>: _parity_codec::Encode,
            Option<T::Price>: _parity_codec::Encode,
            Option<T::Price>: _parity_codec::Encode,
            Option<T::Price>: _parity_codec::Encode,
            Option<T::Price>: _parity_codec::Encode,
            Vec<T::Hash>: _parity_codec::Encode,
            Vec<T::Hash>: _parity_codec::Encode,
        {
            fn encode_to<EncOut: _parity_codec::Output>(&self, dest: &mut EncOut) {
                dest.push(&self.price);
                dest.push(&self.next);
                dest.push(&self.prev);
                dest.push(&self.orders);
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_LinkedItem: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl<T> _parity_codec::Decode for LinkedItem<T>
        where
            T: Trait,
            Option<T::Price>: _parity_codec::Decode,
            Option<T::Price>: _parity_codec::Decode,
            Option<T::Price>: _parity_codec::Decode,
            Option<T::Price>: _parity_codec::Decode,
            Option<T::Price>: _parity_codec::Decode,
            Option<T::Price>: _parity_codec::Decode,
            Vec<T::Hash>: _parity_codec::Decode,
            Vec<T::Hash>: _parity_codec::Decode,
        {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn) -> Option<Self> {
                Some(LinkedItem {
                    price: _parity_codec::Decode::decode(input)?,
                    next: _parity_codec::Decode::decode(input)?,
                    prev: _parity_codec::Decode::decode(input)?,
                    orders: _parity_codec::Decode::decode(input)?,
                })
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::clone::Clone> ::core::clone::Clone for LinkedItem<T>
    where
        T: Trait,
        T::Price: ::core::clone::Clone,
        T::Price: ::core::clone::Clone,
        T::Price: ::core::clone::Clone,
        T::Hash: ::core::clone::Clone,
    {
        #[inline]
        fn clone(&self) -> LinkedItem<T> {
            match *self {
                LinkedItem {
                    price: ref __self_0_0,
                    next: ref __self_0_1,
                    prev: ref __self_0_2,
                    orders: ref __self_0_3,
                } => LinkedItem {
                    price: ::core::clone::Clone::clone(&(*__self_0_0)),
                    next: ::core::clone::Clone::clone(&(*__self_0_1)),
                    prev: ::core::clone::Clone::clone(&(*__self_0_2)),
                    orders: ::core::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::PartialEq> ::core::cmp::PartialEq for LinkedItem<T>
    where
        T: Trait,
        T::Price: ::core::cmp::PartialEq,
        T::Price: ::core::cmp::PartialEq,
        T::Price: ::core::cmp::PartialEq,
        T::Hash: ::core::cmp::PartialEq,
    {
        #[inline]
        fn eq(&self, other: &LinkedItem<T>) -> bool {
            match *other {
                LinkedItem {
                    price: ref __self_1_0,
                    next: ref __self_1_1,
                    prev: ref __self_1_2,
                    orders: ref __self_1_3,
                } => match *self {
                    LinkedItem {
                        price: ref __self_0_0,
                        next: ref __self_0_1,
                        prev: ref __self_0_2,
                        orders: ref __self_0_3,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &LinkedItem<T>) -> bool {
            match *other {
                LinkedItem {
                    price: ref __self_1_0,
                    next: ref __self_1_1,
                    prev: ref __self_1_2,
                    orders: ref __self_1_3,
                } => match *self {
                    LinkedItem {
                        price: ref __self_0_0,
                        next: ref __self_0_1,
                        prev: ref __self_0_2,
                        orders: ref __self_0_3,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::Eq> ::core::cmp::Eq for LinkedItem<T>
    where
        T: Trait,
        T::Price: ::core::cmp::Eq,
        T::Price: ::core::cmp::Eq,
        T::Price: ::core::cmp::Eq,
        T::Hash: ::core::cmp::Eq,
    {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<Option<T::Price>>;
                let _: ::core::cmp::AssertParamIsEq<Option<T::Price>>;
                let _: ::core::cmp::AssertParamIsEq<Option<T::Price>>;
                let _: ::core::cmp::AssertParamIsEq<Vec<T::Hash>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::fmt::Debug> ::core::fmt::Debug for LinkedItem<T>
    where
        T: Trait,
        T::Price: ::core::fmt::Debug,
        T::Price: ::core::fmt::Debug,
        T::Price: ::core::fmt::Debug,
        T::Hash: ::core::fmt::Debug,
    {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                LinkedItem {
                    price: ref __self_0_0,
                    next: ref __self_0_1,
                    prev: ref __self_0_2,
                    orders: ref __self_0_3,
                } => {
                    let mut debug_trait_builder = f.debug_struct("LinkedItem");
                    let _ = debug_trait_builder.field("price", &&(*__self_0_0));
                    let _ = debug_trait_builder.field("next", &&(*__self_0_1));
                    let _ = debug_trait_builder.field("prev", &&(*__self_0_2));
                    let _ = debug_trait_builder.field("orders", &&(*__self_0_3));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl<T: Trait> LinkedItemList<T> {
        pub fn read_head(key: T::Hash) -> LinkedItem<T> {
            Self::read(key, None)
        }
        pub fn read_bottom(key: T::Hash) -> LinkedItem<T> {
            Self::read(key, Some(T::Price::min_value()))
        }
        pub fn read_top(key: T::Hash) -> LinkedItem<T> {
            Self::read(key, Some(T::Price::max_value()))
        }
        pub fn read(key1: T::Hash, key2: Option<T::Price>) -> LinkedItem<T> {
            Self::get((key1, key2)).unwrap_or_else(|| {
                let bottom = LinkedItem {
                    prev: Some(T::Price::max_value()),
                    next: None,
                    price: Some(T::Price::min_value()),
                    orders: Vec::new(),
                };
                let top = LinkedItem {
                    prev: None,
                    next: Some(T::Price::min_value()),
                    price: Some(T::Price::max_value()),
                    orders: Vec::new(),
                };
                let item = LinkedItem {
                    prev: Some(T::Price::min_value()),
                    next: Some(T::Price::max_value()),
                    price: None,
                    orders: Vec::new(),
                };
                Self::write(key1, key2, item.clone());
                Self::write(key1, bottom.price, bottom);
                Self::write(key1, top.price, top);
                item
            })
        }
        pub fn write(key1: T::Hash, key2: Option<T::Price>, item: LinkedItem<T>) {
            Self::insert((key1, key2), item);
        }
        pub fn append(key1: T::Hash, key2: T::Price, order_hash: T::Hash, otype: OrderType) {
            let item = Self::get((key1, Some(key2)));
            match item {
                Some(mut item) => {
                    item.orders.push(order_hash);
                    Self::write(key1, Some(key2), item);
                    return;
                }
                None => {
                    let start_item;
                    let end_item;
                    match otype {
                        OrderType::Buy => {
                            start_item = Some(T::Price::min_value());
                            end_item = None;
                        }
                        OrderType::Sell => {
                            start_item = None;
                            end_item = Some(T::Price::max_value());
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
                    let new_prev = LinkedItem {
                        next: Some(key2),
                        ..item
                    };
                    Self::write(key1, new_prev.price, new_prev.clone());
                    let next = Self::read(key1, item.next);
                    let new_next = LinkedItem {
                        prev: Some(key2),
                        ..next
                    };
                    Self::write(key1, new_next.price, new_next.clone());
                    let mut v = Vec::new();
                    v.push(order_hash);
                    let item = LinkedItem {
                        prev: new_prev.price,
                        next: new_next.price,
                        price: Some(key2),
                        orders: v,
                    };
                    Self::write(key1, Some(key2), item);
                }
            }
        }
        pub fn remove_items(key1: T::Hash, otype: OrderType) -> Result {
            let end_item;
            if otype == OrderType::Buy {
                end_item = Some(T::Price::min_value());
            } else {
                end_item = Some(T::Price::max_value());
            }
            let mut head = Self::read_head(key1);
            loop {
                let key2;
                if otype == OrderType::Buy {
                    key2 = head.prev;
                } else {
                    key2 = head.next;
                }
                if key2 == end_item {
                    break;
                }
                Self::remove_item(key1, key2.unwrap())?;
                head = Self::read_head(key1);
            }
            Ok(())
        }
        pub fn remove_item(key1: T::Hash, key2: T::Price) -> Result {
            match Self::get((key1, Some(key2))) {
                Some(mut item) => {
                    while item.orders.len() > 0 {
                        let order_hash = item.orders.get(0).ok_or("can not get order hash")?;
                        let order = <Orders<T>>::get(order_hash).ok_or("can not get order")?;
                        {
                            if !order.is_finished() {
                                {
                                    return Err("try to remove not finished order");
                                };
                            }
                        };
                        item.orders.remove(0);
                        Self::write(key1, Some(key2), item.clone());
                    }
                    if item.orders.len() == 0 {
                        if let Some(item) = Self::take((key1, Some(key2))) {
                            Self::mutate((key1.clone(), item.prev), |x| {
                                if let Some(x) = x {
                                    x.next = item.next;
                                }
                            });
                            Self::mutate((key1.clone(), item.next), |x| {
                                if let Some(x) = x {
                                    x.prev = item.prev;
                                }
                            });
                        }
                    }
                }
                None => {}
            }
            Ok(())
        }
    }
    #[doc(hidden)]
    mod sr_api_hidden_includes_decl_storage {
        pub extern crate support as hidden_include;
    }
    struct TradePairsByHash<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>,
    );
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , TradePair < T :: Hash > > for TradePairsByHash < T > { type Query = Option < TradePair < T :: Hash > > ; type Hasher = self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 ; # [ doc = r" Get the prefix key in storage." ] fn prefix ( ) -> & 'static [ u8 ] { "trade TradePairsByHash" . as_bytes ( ) } # [ doc = r" Get the storage key used to fetch a value corresponding to a specific key." ] fn key_for ( x : & T :: Hash ) -> self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: vec :: Vec < u8 > { let mut key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , TradePair < T :: Hash > > > :: prefix ( ) . to_vec ( ) ; self :: sr_api_hidden_includes_decl_storage :: hidden_include :: codec :: Encode :: encode_to ( x , & mut key ) ; key } # [ doc = r" Load the value associated with the given key from the map." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: Hash , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , TradePair < T :: Hash > > > :: key_for ( key ) ; storage . get ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Take the value, reading and removing it." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: Hash , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , TradePair < T :: Hash > > > :: key_for ( key ) ; storage . take ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key" ] fn mutate < R , F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: Hash , f : F , storage : & S ) -> R { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , TradePair < T :: Hash > > > :: get ( key , storage ) ; let ret = f ( & mut val ) ; match val { Some ( ref val ) => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , TradePair < T :: Hash > > > :: insert ( key , & val , storage ) , None => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , TradePair < T :: Hash > > > :: remove ( key , storage ) , } ; ret } }
    struct TradePairHashByBaseQuote<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>,
    );
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , T :: Hash ) , T :: Hash > for TradePairHashByBaseQuote < T > { type Query = Option < T :: Hash > ; type Hasher = self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 ; # [ doc = r" Get the prefix key in storage." ] fn prefix ( ) -> & 'static [ u8 ] { "trade TradePairHashByBaseQuote" . as_bytes ( ) } # [ doc = r" Get the storage key used to fetch a value corresponding to a specific key." ] fn key_for ( x : & ( T :: Hash , T :: Hash ) ) -> self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: vec :: Vec < u8 > { let mut key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , T :: Hash ) , T :: Hash > > :: prefix ( ) . to_vec ( ) ; self :: sr_api_hidden_includes_decl_storage :: hidden_include :: codec :: Encode :: encode_to ( x , & mut key ) ; key } # [ doc = r" Load the value associated with the given key from the map." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: Hash , T :: Hash ) , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , T :: Hash ) , T :: Hash > > :: key_for ( key ) ; storage . get ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Take the value, reading and removing it." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: Hash , T :: Hash ) , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , T :: Hash ) , T :: Hash > > :: key_for ( key ) ; storage . take ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key" ] fn mutate < R , F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: Hash , T :: Hash ) , f : F , storage : & S ) -> R { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , T :: Hash ) , T :: Hash > > :: get ( key , storage ) ; let ret = f ( & mut val ) ; match val { Some ( ref val ) => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , T :: Hash ) , T :: Hash > > :: insert ( key , & val , storage ) , None => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , T :: Hash ) , T :: Hash > > :: remove ( key , storage ) , } ; ret } }
    struct Orders<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>,
    );
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , LimitOrder < T > > for Orders < T > { type Query = Option < LimitOrder < T > > ; type Hasher = self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 ; # [ doc = r" Get the prefix key in storage." ] fn prefix ( ) -> & 'static [ u8 ] { "trade Orders" . as_bytes ( ) } # [ doc = r" Get the storage key used to fetch a value corresponding to a specific key." ] fn key_for ( x : & T :: Hash ) -> self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: vec :: Vec < u8 > { let mut key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , LimitOrder < T > > > :: prefix ( ) . to_vec ( ) ; self :: sr_api_hidden_includes_decl_storage :: hidden_include :: codec :: Encode :: encode_to ( x , & mut key ) ; key } # [ doc = r" Load the value associated with the given key from the map." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: Hash , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , LimitOrder < T > > > :: key_for ( key ) ; storage . get ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Take the value, reading and removing it." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: Hash , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , LimitOrder < T > > > :: key_for ( key ) ; storage . take ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key" ] fn mutate < R , F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: Hash , f : F , storage : & S ) -> R { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , LimitOrder < T > > > :: get ( key , storage ) ; let ret = f ( & mut val ) ; match val { Some ( ref val ) => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , LimitOrder < T > > > :: insert ( key , & val , storage ) , None => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , LimitOrder < T > > > :: remove ( key , storage ) , } ; ret } }
    struct OwnedOrders<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>,
    );
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , u64 ) , T :: Hash > for OwnedOrders < T > { type Query = Option < T :: Hash > ; type Hasher = self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 ; # [ doc = r" Get the prefix key in storage." ] fn prefix ( ) -> & 'static [ u8 ] { "trade OwnedOrders" . as_bytes ( ) } # [ doc = r" Get the storage key used to fetch a value corresponding to a specific key." ] fn key_for ( x : & ( T :: AccountId , u64 ) ) -> self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: vec :: Vec < u8 > { let mut key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , u64 ) , T :: Hash > > :: prefix ( ) . to_vec ( ) ; self :: sr_api_hidden_includes_decl_storage :: hidden_include :: codec :: Encode :: encode_to ( x , & mut key ) ; key } # [ doc = r" Load the value associated with the given key from the map." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: AccountId , u64 ) , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , u64 ) , T :: Hash > > :: key_for ( key ) ; storage . get ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Take the value, reading and removing it." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: AccountId , u64 ) , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , u64 ) , T :: Hash > > :: key_for ( key ) ; storage . take ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key" ] fn mutate < R , F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: AccountId , u64 ) , f : F , storage : & S ) -> R { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , u64 ) , T :: Hash > > :: get ( key , storage ) ; let ret = f ( & mut val ) ; match val { Some ( ref val ) => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , u64 ) , T :: Hash > > :: insert ( key , & val , storage ) , None => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , u64 ) , T :: Hash > > :: remove ( key , storage ) , } ; ret } }
    struct OwnedOrdersIndex<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>,
    );
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: AccountId , u64 > for OwnedOrdersIndex < T > { type Query = u64 ; type Hasher = self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 ; # [ doc = r" Get the prefix key in storage." ] fn prefix ( ) -> & 'static [ u8 ] { "trade OwnedOrdersIndex" . as_bytes ( ) } # [ doc = r" Get the storage key used to fetch a value corresponding to a specific key." ] fn key_for ( x : & T :: AccountId ) -> self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: vec :: Vec < u8 > { let mut key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: AccountId , u64 > > :: prefix ( ) . to_vec ( ) ; self :: sr_api_hidden_includes_decl_storage :: hidden_include :: codec :: Encode :: encode_to ( x , & mut key ) ; key } # [ doc = r" Load the value associated with the given key from the map." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: AccountId , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: AccountId , u64 > > :: key_for ( key ) ; storage . get ( & key [ .. ] ) . unwrap_or_else ( | | Default :: default ( ) ) } # [ doc = r" Take the value, reading and removing it." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: AccountId , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: AccountId , u64 > > :: key_for ( key ) ; storage . take ( & key [ .. ] ) . unwrap_or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key" ] fn mutate < R , F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: AccountId , f : F , storage : & S ) -> R { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: AccountId , u64 > > :: get ( key , storage ) ; let ret = f ( & mut val ) ; < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: AccountId , u64 > > :: insert ( key , & val , storage ) ; ret } }
    struct TradePairOwnedOrders<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>,
    );
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , u64 ) , T :: Hash > for TradePairOwnedOrders < T > { type Query = Option < T :: Hash > ; type Hasher = self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 ; # [ doc = r" Get the prefix key in storage." ] fn prefix ( ) -> & 'static [ u8 ] { "trade TradePairOwnedOrders" . as_bytes ( ) } # [ doc = r" Get the storage key used to fetch a value corresponding to a specific key." ] fn key_for ( x : & ( T :: Hash , u64 ) ) -> self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: vec :: Vec < u8 > { let mut key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , u64 ) , T :: Hash > > :: prefix ( ) . to_vec ( ) ; self :: sr_api_hidden_includes_decl_storage :: hidden_include :: codec :: Encode :: encode_to ( x , & mut key ) ; key } # [ doc = r" Load the value associated with the given key from the map." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: Hash , u64 ) , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , u64 ) , T :: Hash > > :: key_for ( key ) ; storage . get ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Take the value, reading and removing it." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: Hash , u64 ) , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , u64 ) , T :: Hash > > :: key_for ( key ) ; storage . take ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key" ] fn mutate < R , F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: Hash , u64 ) , f : F , storage : & S ) -> R { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , u64 ) , T :: Hash > > :: get ( key , storage ) ; let ret = f ( & mut val ) ; match val { Some ( ref val ) => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , u64 ) , T :: Hash > > :: insert ( key , & val , storage ) , None => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , u64 ) , T :: Hash > > :: remove ( key , storage ) , } ; ret } }
    struct TradePairOwnedOrdersIndex<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>,
    );
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , u64 > for TradePairOwnedOrdersIndex < T > { type Query = u64 ; type Hasher = self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 ; # [ doc = r" Get the prefix key in storage." ] fn prefix ( ) -> & 'static [ u8 ] { "trade TradePairOwnedOrdersIndex" . as_bytes ( ) } # [ doc = r" Get the storage key used to fetch a value corresponding to a specific key." ] fn key_for ( x : & T :: Hash ) -> self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: vec :: Vec < u8 > { let mut key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , u64 > > :: prefix ( ) . to_vec ( ) ; self :: sr_api_hidden_includes_decl_storage :: hidden_include :: codec :: Encode :: encode_to ( x , & mut key ) ; key } # [ doc = r" Load the value associated with the given key from the map." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: Hash , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , u64 > > :: key_for ( key ) ; storage . get ( & key [ .. ] ) . unwrap_or_else ( | | Default :: default ( ) ) } # [ doc = r" Take the value, reading and removing it." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: Hash , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , u64 > > :: key_for ( key ) ; storage . take ( & key [ .. ] ) . unwrap_or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key" ] fn mutate < R , F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & T :: Hash , f : F , storage : & S ) -> R { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , u64 > > :: get ( key , storage ) ; let ret = f ( & mut val ) ; < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , u64 > > :: insert ( key , & val , storage ) ; ret } }
    struct LinkedItemList<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>,
    );
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , Option < T :: Price > ) , LinkedItem < T > > for LinkedItemList < T > { type Query = Option < LinkedItem < T > > ; type Hasher = self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 ; # [ doc = r" Get the prefix key in storage." ] fn prefix ( ) -> & 'static [ u8 ] { "trade LinkedItemList" . as_bytes ( ) } # [ doc = r" Get the storage key used to fetch a value corresponding to a specific key." ] fn key_for ( x : & ( T :: Hash , Option < T :: Price > ) ) -> self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: vec :: Vec < u8 > { let mut key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , Option < T :: Price > ) , LinkedItem < T > > > :: prefix ( ) . to_vec ( ) ; self :: sr_api_hidden_includes_decl_storage :: hidden_include :: codec :: Encode :: encode_to ( x , & mut key ) ; key } # [ doc = r" Load the value associated with the given key from the map." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: Hash , Option < T :: Price > ) , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , Option < T :: Price > ) , LinkedItem < T > > > :: key_for ( key ) ; storage . get ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Take the value, reading and removing it." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: Hash , Option < T :: Price > ) , storage : & S ) -> Self :: Query { let key = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , Option < T :: Price > ) , LinkedItem < T > > > :: key_for ( key ) ; storage . take ( & key [ .. ] ) . or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key" ] fn mutate < R , F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Blake2_256 > > ( key : & ( T :: Hash , Option < T :: Price > ) , f : F , storage : & S ) -> R { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , Option < T :: Price > ) , LinkedItem < T > > > :: get ( key , storage ) ; let ret = f ( & mut val ) ; match val { Some ( ref val ) => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , Option < T :: Price > ) , LinkedItem < T > > > :: insert ( key , & val , storage ) , None => < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , Option < T :: Price > ) , LinkedItem < T > > > :: remove ( key , storage ) , } ; ret } }
    struct Nonce<T: Trait>(
        self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<(T)>,
    );
    impl < T : Trait > self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < u64 > for Nonce < T > { type Query = u64 ; # [ doc = r" Get the storage key." ] fn key ( ) -> & 'static [ u8 ] { "trade Nonce" . as_bytes ( ) } # [ doc = r" Load the value from the provided storage instance." ] fn get < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Twox128 > > ( storage : & S ) -> Self :: Query { storage . get ( < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < u64 > > :: key ( ) ) . unwrap_or_else ( | | Default :: default ( ) ) } # [ doc = r" Take a value from storage, removing it afterwards." ] fn take < S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Twox128 > > ( storage : & S ) -> Self :: Query { storage . take ( < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < u64 > > :: key ( ) ) . unwrap_or_else ( | | Default :: default ( ) ) } # [ doc = r" Mutate the value under a key." ] fn mutate < R , F : FnOnce ( & mut Self :: Query ) -> R , S : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: HashedStorage < self :: sr_api_hidden_includes_decl_storage :: hidden_include :: Twox128 > > ( f : F , storage : & S ) -> R { let mut val = < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < u64 > > :: get ( storage ) ; let ret = f ( & mut val ) ; < Self as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageValue < u64 > > :: put ( & val , storage ) ; ret } }
    trait Store {
        type TradePairsByHash;
        type TradePairHashByBaseQuote;
        type Orders;
        type OwnedOrders;
        type OwnedOrdersIndex;
        type TradePairOwnedOrders;
        type TradePairOwnedOrdersIndex;
        type LinkedItemList;
        type Nonce;
    }
    #[doc(hidden)]
    pub struct __GetByteStructTradePairsByHash<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_TradePairsByHash:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructTradePairsByHash<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_TradePairsByHash
                .get_or_init(|| {
                    let def_val: Option<TradePair<T::Hash>> = Default::default();
                    <Option<TradePair<T::Hash>> as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructTradePairHashByBaseQuote<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_TradePairHashByBaseQuote:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructTradePairHashByBaseQuote<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_TradePairHashByBaseQuote
                .get_or_init(|| {
                    let def_val: Option<T::Hash> = Default::default();
                    <Option<T::Hash> as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructOrders<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_Orders:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructOrders<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_Orders
                .get_or_init(|| {
                    let def_val: Option<LimitOrder<T>> = Default::default();
                    <Option<LimitOrder<T>> as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructOwnedOrders<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_OwnedOrders:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructOwnedOrders<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_OwnedOrders
                .get_or_init(|| {
                    let def_val: Option<T::Hash> = Default::default();
                    <Option<T::Hash> as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructOwnedOrdersIndex<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_OwnedOrdersIndex:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructOwnedOrdersIndex<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_OwnedOrdersIndex
                .get_or_init(|| {
                    let def_val: u64 = Default::default();
                    <u64 as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructTradePairOwnedOrders<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_TradePairOwnedOrders:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructTradePairOwnedOrders<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_TradePairOwnedOrders
                .get_or_init(|| {
                    let def_val: Option<T::Hash> = Default::default();
                    <Option<T::Hash> as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructTradePairOwnedOrdersIndex<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_TradePairOwnedOrdersIndex:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructTradePairOwnedOrdersIndex<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_TradePairOwnedOrdersIndex
                .get_or_init(|| {
                    let def_val: u64 = Default::default();
                    <u64 as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructLinkedItemList<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_LinkedItemList:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructLinkedItemList<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_LinkedItemList
                .get_or_init(|| {
                    let def_val: Option<LinkedItem<T>> = Default::default();
                    <Option<LinkedItem<T>> as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    #[doc(hidden)]
    pub struct __GetByteStructNonce<T>(
        pub  self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::marker::PhantomData<
            (T),
        >,
    );
    #[cfg(feature = "std")]
    #[allow(non_upper_case_globals)]
    static __CACHE_GET_BYTE_STRUCT_Nonce:
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell<
            self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8>,
        > =
        self::sr_api_hidden_includes_decl_storage::hidden_include::once_cell::sync::OnceCell::INIT;
    #[cfg(feature = "std")]
    impl<T: Trait> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::DefaultByte
        for __GetByteStructNonce<T>
    {
        fn default_byte(
            &self,
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::vec::Vec<u8> {
            use self::sr_api_hidden_includes_decl_storage::hidden_include::codec::Encode;
            __CACHE_GET_BYTE_STRUCT_Nonce
                .get_or_init(|| {
                    let def_val: u64 = Default::default();
                    <u64 as Encode>::encode(&def_val)
                })
                .clone()
        }
    }
    impl<T: Trait> Store for Module<T> {
        type TradePairsByHash = TradePairsByHash<T>;
        type TradePairHashByBaseQuote = TradePairHashByBaseQuote<T>;
        type Orders = Orders<T>;
        type OwnedOrders = OwnedOrders<T>;
        type OwnedOrdersIndex = OwnedOrdersIndex<T>;
        type TradePairOwnedOrders = TradePairOwnedOrders<T>;
        type TradePairOwnedOrdersIndex = TradePairOwnedOrdersIndex<T>;
        type LinkedItemList = LinkedItemList<T>;
        type Nonce = Nonce<T>;
    }
    impl<T: 'static + Trait> Module<T> {
        pub fn trade_pair_by_hash<
            K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<
                T::Hash,
            >,
        >(
            key: K,
        ) -> Option<TradePair<T::Hash>> {
            < TradePairsByHash < T > as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , TradePair < T :: Hash > > > :: get ( key . borrow ( ) , & self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: RuntimeStorage )
        }
        pub fn get_trade_pair_hash_by_base_quote<
            K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<(
                T::Hash,
                T::Hash,
            )>,
        >(
            key: K,
        ) -> Option<T::Hash> {
            < TradePairHashByBaseQuote < T > as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , T :: Hash ) , T :: Hash > > :: get ( key . borrow ( ) , & self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: RuntimeStorage )
        }
        pub fn order<
            K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<
                T::Hash,
            >,
        >(
            key: K,
        ) -> Option<LimitOrder<T>> {
            < Orders < T > as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , LimitOrder < T > > > :: get ( key . borrow ( ) , & self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: RuntimeStorage )
        }
        pub fn owned_order<
            K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<(
                T::AccountId,
                u64,
            )>,
        >(
            key: K,
        ) -> Option<T::Hash> {
            < OwnedOrders < T > as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: AccountId , u64 ) , T :: Hash > > :: get ( key . borrow ( ) , & self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: RuntimeStorage )
        }
        pub fn owned_orders_index<
            K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<
                T::AccountId,
            >,
        >(
            key: K,
        ) -> u64 {
            < OwnedOrdersIndex < T > as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: AccountId , u64 > > :: get ( key . borrow ( ) , & self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: RuntimeStorage )
        }
        pub fn trade_pair_owned_order<
            K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<(
                T::Hash,
                u64,
            )>,
        >(
            key: K,
        ) -> Option<T::Hash> {
            < TradePairOwnedOrders < T > as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , u64 ) , T :: Hash > > :: get ( key . borrow ( ) , & self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: RuntimeStorage )
        }
        pub fn trade_pair_owned_orders_index<
            K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<
                T::Hash,
            >,
        >(
            key: K,
        ) -> u64 {
            < TradePairOwnedOrdersIndex < T > as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < T :: Hash , u64 > > :: get ( key . borrow ( ) , & self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: RuntimeStorage )
        }
        pub fn sell_order<
            K: self::sr_api_hidden_includes_decl_storage::hidden_include::rstd::borrow::Borrow<(
                T::Hash,
                Option<T::Price>,
            )>,
        >(
            key: K,
        ) -> Option<LinkedItem<T>> {
            < LinkedItemList < T > as self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: hashed :: generator :: StorageMap < ( T :: Hash , Option < T :: Price > ) , LinkedItem < T > > > :: get ( key . borrow ( ) , & self :: sr_api_hidden_includes_decl_storage :: hidden_include :: storage :: RuntimeStorage )
        }
        #[doc(hidden)]
        pub fn store_metadata(
        ) -> self::sr_api_hidden_includes_decl_storage::hidden_include::metadata::StorageMetadata
        {
            self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageMetadata { functions : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( { & [ self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "TradePairsByHash" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Hash" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "TradePair<T::Hash>" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructTradePairsByHash :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "TradePairHashByBaseQuote" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "(T::Hash, T::Hash)" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Hash" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructTradePairHashByBaseQuote :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Orders" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Hash" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "LimitOrder<T>" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructOrders :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "OwnedOrders" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "(T::AccountId, u64)" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Hash" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructOwnedOrders :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "OwnedOrdersIndex" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Default , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::AccountId" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "u64" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructOwnedOrdersIndex :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "TradePairOwnedOrders" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "(T::Hash, u64)" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Hash" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructTradePairOwnedOrders :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "TradePairOwnedOrdersIndex" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Default , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Hash" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "u64" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructTradePairOwnedOrdersIndex :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "LinkedItemList" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "(T::Hash, Option<T::Price>)" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "LinkedItem<T>" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructLinkedItemList :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Nonce" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Default , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Plain ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "u64" ) ) , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructNonce :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } ] } ) , }
        }
        #[doc(hidden)]pub fn store_metadata_functions ( ) -> & 'static [ self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata ]{
            {
                & [ self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "TradePairsByHash" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Hash" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "TradePair<T::Hash>" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructTradePairsByHash :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "TradePairHashByBaseQuote" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "(T::Hash, T::Hash)" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Hash" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructTradePairHashByBaseQuote :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Orders" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Hash" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "LimitOrder<T>" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructOrders :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "OwnedOrders" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "(T::AccountId, u64)" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Hash" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructOwnedOrders :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "OwnedOrdersIndex" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Default , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::AccountId" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "u64" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructOwnedOrdersIndex :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "TradePairOwnedOrders" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "(T::Hash, u64)" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Hash" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructTradePairOwnedOrders :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "TradePairOwnedOrdersIndex" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Default , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "T::Hash" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "u64" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructTradePairOwnedOrdersIndex :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "LinkedItemList" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Optional , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Map { hasher : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageHasher :: Blake2_256 , key : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "(T::Hash, Option<T::Price>)" ) , value : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "LinkedItem<T>" ) , is_linked : false , } , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructLinkedItemList :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } , self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionMetadata { name : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "Nonce" ) , modifier : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionModifier :: Default , ty : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: StorageFunctionType :: Plain ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( "u64" ) ) , default : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DefaultByteGetter ( & __GetByteStructNonce :: < T > ( self :: sr_api_hidden_includes_decl_storage :: hidden_include :: rstd :: marker :: PhantomData ) ) ) , documentation : self :: sr_api_hidden_includes_decl_storage :: hidden_include :: metadata :: DecodeDifferent :: Encode ( & [ ] ) , } ]
            }
        }
        #[doc(hidden)]
        pub fn store_metadata_name() -> &'static str {
            "trade"
        }
    }
    #[cfg(feature = "std")]
    pub struct Module<T: Trait>(::std::marker::PhantomData<(T)>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::clone::Clone + Trait> ::core::clone::Clone for Module<T> {
        #[inline]
        fn clone(&self) -> Module<T> {
            match *self {
                Module(ref __self_0_0) => Module(::core::clone::Clone::clone(&(*__self_0_0))),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::marker::Copy + Trait> ::core::marker::Copy for Module<T> {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::PartialEq + Trait> ::core::cmp::PartialEq for Module<T> {
        #[inline]
        fn eq(&self, other: &Module<T>) -> bool {
            match *other {
                Module(ref __self_1_0) => match *self {
                    Module(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Module<T>) -> bool {
            match *other {
                Module(ref __self_1_0) => match *self {
                    Module(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::cmp::Eq + Trait> ::core::cmp::Eq for Module<T> {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<::std::marker::PhantomData<(T)>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<T: ::core::fmt::Debug + Trait> ::core::fmt::Debug for Module<T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Module(ref __self_0_0) => {
                    let mut debug_trait_builder = f.debug_tuple("Module");
                    let _ = debug_trait_builder.field(&&(*__self_0_0));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl<T: Trait> ::srml_support::runtime_primitives::traits::OnInitialize<T::BlockNumber>
        for Module<T>
    {
    }
    impl<T: Trait> ::srml_support::runtime_primitives::traits::OnFinalize<T::BlockNumber>
        for Module<T>
    {
    }
    impl<T: Trait> ::srml_support::runtime_primitives::traits::OffchainWorker<T::BlockNumber>
        for Module<T>
    {
    }
    impl<T: Trait> Module<T> {
        fn deposit_event(event: Event<T>) {
            <system::Module<T>>::deposit_event(<T as Trait>::from(event).into());
        }
    }
    /// Can also be called using [`Call`].
    ///
    /// [`Call`]: enum.Call.html
    impl<T: Trait> Module<T> {
        pub fn create_trade_pair(origin: T::Origin, base: T::Hash, quote: T::Hash) -> Result {
            Self::do_create_trade_pair(origin, base, quote)
        }
        pub fn create_limit_order(
            origin: T::Origin,
            base: T::Hash,
            quote: T::Hash,
            otype: OrderType,
            price: T::Price,
            amount: T::Balance,
        ) -> Result {
            let sender = ensure_signed(origin)?;
            let tp = Self::ensure_trade_pair(base, quote)?;
            {
                if !(price > Zero::zero()) {
                    {
                        return Err("price must be positive");
                    };
                }
            };
            let op_token_hash;
            match otype {
                OrderType::Buy => op_token_hash = base,
                OrderType::Sell => op_token_hash = quote,
            };
            let order = LimitOrder::<T>::new(base, quote, sender.clone(), price, amount, otype);
            let hash = order.hash;
            <token::Module<T>>::ensure_free_balance(sender.clone(), op_token_hash, amount)?;
            <token::Module<T>>::do_freeze(sender.clone(), op_token_hash, amount)?;
            <Orders<T>>::insert(hash, order);
            let owned_index = Self::owned_orders_index(sender.clone());
            <OwnedOrders<T>>::insert((sender.clone(), owned_index), hash);
            <OwnedOrdersIndex<T>>::insert(sender.clone(), owned_index + 1);
            let tp_owned_index = Self::trade_pair_owned_orders_index(tp);
            <TradePairOwnedOrders<T>>::insert((tp, tp_owned_index), hash);
            <TradePairOwnedOrdersIndex<T>>::insert(tp, tp_owned_index + 1);
            <LinkedItemList<T>>::append(tp, price, hash, otype);
            Self::deposit_event(RawEvent::OrderCreated(
                sender, base, quote, hash, price, amount,
            ));
            Ok(())
        }
    }
    #[cfg(feature = "std")]
    pub enum Call<T: Trait> {
        #[doc(hidden)]
        __PhantomItem(
            ::std::marker::PhantomData<(T)>,
            ::srml_support::dispatch::Never,
        ),
        #[allow(non_camel_case_types)]
        create_trade_pair(T::Hash, T::Hash),
        #[allow(non_camel_case_types)]
        create_limit_order(T::Hash, T::Hash, OrderType, T::Price, T::Balance),
    }
    impl<T: Trait> ::srml_support::dispatch::Clone for Call<T> {
        fn clone(&self) -> Self {
            match *self {
                Call::create_trade_pair(ref base, ref quote) => {
                    Call::create_trade_pair((*base).clone(), (*quote).clone())
                }
                Call::create_limit_order(ref base, ref quote, ref otype, ref price, ref amount) => {
                    Call::create_limit_order(
                        (*base).clone(),
                        (*quote).clone(),
                        (*otype).clone(),
                        (*price).clone(),
                        (*amount).clone(),
                    )
                }
                _ => ::std::rt::begin_panic(
                    "internal error: entered unreachable code",
                    &("runtime/src/trade.rs", 296u32, 1u32),
                ),
            }
        }
    }
    impl<T: Trait> ::srml_support::dispatch::PartialEq for Call<T> {
        fn eq(&self, _other: &Self) -> bool {
            match *self {
                Call::create_trade_pair(ref base, ref quote) => {
                    let self_params = (base, quote);
                    if let Call::create_trade_pair(ref base, ref quote) = *_other {
                        self_params == (base, quote)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => ::std::rt::begin_panic(
                                "internal error: entered unreachable code",
                                &("runtime/src/trade.rs", 296u32, 1u32),
                            ),
                            _ => false,
                        }
                    }
                }
                Call::create_limit_order(ref base, ref quote, ref otype, ref price, ref amount) => {
                    let self_params = (base, quote, otype, price, amount);
                    if let Call::create_limit_order(
                        ref base,
                        ref quote,
                        ref otype,
                        ref price,
                        ref amount,
                    ) = *_other
                    {
                        self_params == (base, quote, otype, price, amount)
                    } else {
                        match *_other {
                            Call::__PhantomItem(_, _) => ::std::rt::begin_panic(
                                "internal error: entered unreachable code",
                                &("runtime/src/trade.rs", 296u32, 1u32),
                            ),
                            _ => false,
                        }
                    }
                }
                _ => ::std::rt::begin_panic(
                    "internal error: entered unreachable code",
                    &("runtime/src/trade.rs", 296u32, 1u32),
                ),
            }
        }
    }
    impl<T: Trait> ::srml_support::dispatch::Eq for Call<T> {}
    #[cfg(feature = "std")]
    impl<T: Trait> ::srml_support::dispatch::fmt::Debug for Call<T> {
        fn fmt(
            &self,
            _f: &mut ::srml_support::dispatch::fmt::Formatter,
        ) -> ::srml_support::dispatch::result::Result<(), ::srml_support::dispatch::fmt::Error>
        {
            match *self {
                Call::create_trade_pair(ref base, ref quote) => {
                    _f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["", ""],
                        &match (&"create_trade_pair", &(base.clone(), quote.clone())) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                            ],
                        },
                    ))
                }
                Call::create_limit_order(ref base, ref quote, ref otype, ref price, ref amount) => {
                    _f.write_fmt(::core::fmt::Arguments::new_v1(
                        &["", ""],
                        &match (
                            &"create_limit_order",
                            &(
                                base.clone(),
                                quote.clone(),
                                otype.clone(),
                                price.clone(),
                                amount.clone(),
                            ),
                        ) {
                            (arg0, arg1) => [
                                ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Debug::fmt),
                            ],
                        },
                    ))
                }
                _ => ::std::rt::begin_panic(
                    "internal error: entered unreachable code",
                    &("runtime/src/trade.rs", 296u32, 1u32),
                ),
            }
        }
    }
    impl<T: Trait> ::srml_support::dispatch::Decode for Call<T> {
        fn decode<Input: ::srml_support::dispatch::Input>(input: &mut Input) -> Option<Self> {
            let _input_id = input.read_byte()?;
            {
                if _input_id == (0) {
                    let base = ::srml_support::dispatch::Decode::decode(input)?;
                    let quote = ::srml_support::dispatch::Decode::decode(input)?;
                    return Some(Call::create_trade_pair(base, quote));
                }
                {
                    if _input_id == (0 + 1) {
                        let base = ::srml_support::dispatch::Decode::decode(input)?;
                        let quote = ::srml_support::dispatch::Decode::decode(input)?;
                        let otype = ::srml_support::dispatch::Decode::decode(input)?;
                        let price = ::srml_support::dispatch::Decode::decode(input)?;
                        let amount = ::srml_support::dispatch::Decode::decode(input)?;
                        return Some(Call::create_limit_order(base, quote, otype, price, amount));
                    }
                    None
                }
            }
        }
    }
    impl<T: Trait> ::srml_support::dispatch::Encode for Call<T> {
        fn encode_to<W: ::srml_support::dispatch::Output>(&self, _dest: &mut W) {
            {
                if let Call::create_trade_pair(ref base, ref quote) = *self {
                    _dest.push_byte((0) as u8);
                    base.encode_to(_dest);
                    quote.encode_to(_dest);
                }
                {
                    if let Call::create_limit_order(
                        ref base,
                        ref quote,
                        ref otype,
                        ref price,
                        ref amount,
                    ) = *self
                    {
                        _dest.push_byte((0 + 1) as u8);
                        base.encode_to(_dest);
                        quote.encode_to(_dest);
                        otype.encode_to(_dest);
                        price.encode_to(_dest);
                        amount.encode_to(_dest);
                    }
                    {}
                }
            };
        }
    }
    impl<T: Trait> ::srml_support::dispatch::Dispatchable for Call<T> {
        type Trait = T;
        type Origin = T::Origin;
        fn dispatch(self, _origin: Self::Origin) -> ::srml_support::dispatch::Result {
            match self {
                Call::create_trade_pair(base, quote) => {
                    <Module<T>>::create_trade_pair(_origin, base, quote)
                }
                Call::create_limit_order(base, quote, otype, price, amount) => {
                    <Module<T>>::create_limit_order(_origin, base, quote, otype, price, amount)
                }
                Call::__PhantomItem(_, _) => ::std::rt::begin_panic_fmt(
                    &::core::fmt::Arguments::new_v1(
                        &["internal error: entered unreachable code: "],
                        &match (&"__PhantomItem should never be used.",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ),
                    &("runtime/src/trade.rs", 296u32, 1u32),
                ),
            }
        }
    }
    impl<T: Trait> ::srml_support::dispatch::Callable for Module<T> {
        type Call = Call<T>;
    }
    impl<T: Trait> Module<T> {
        #[doc(hidden)]
        pub fn dispatch<D: ::srml_support::dispatch::Dispatchable<Trait = T>>(
            d: D,
            origin: D::Origin,
        ) -> ::srml_support::dispatch::Result {
            d.dispatch(origin)
        }
    }
    impl<T: Trait> Module<T> {
        #[doc(hidden)]
        pub fn call_functions() -> &'static [::srml_support::dispatch::FunctionMetadata] {
            &[
                ::srml_support::dispatch::FunctionMetadata {
                    name: ::srml_support::dispatch::DecodeDifferent::Encode("create_trade_pair"),
                    arguments: ::srml_support::dispatch::DecodeDifferent::Encode(&[
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("base"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("T::Hash"),
                        },
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("quote"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("T::Hash"),
                        },
                    ]),
                    documentation: ::srml_support::dispatch::DecodeDifferent::Encode(&[]),
                },
                ::srml_support::dispatch::FunctionMetadata {
                    name: ::srml_support::dispatch::DecodeDifferent::Encode("create_limit_order"),
                    arguments: ::srml_support::dispatch::DecodeDifferent::Encode(&[
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("base"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("T::Hash"),
                        },
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("quote"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("T::Hash"),
                        },
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("otype"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("OrderType"),
                        },
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("price"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("T::Price"),
                        },
                        ::srml_support::dispatch::FunctionArgumentMetadata {
                            name: ::srml_support::dispatch::DecodeDifferent::Encode("amount"),
                            ty: ::srml_support::dispatch::DecodeDifferent::Encode("T::Balance"),
                        },
                    ]),
                    documentation: ::srml_support::dispatch::DecodeDifferent::Encode(&[]),
                },
            ]
        }
    }
    impl<T: Trait> Module<T> {
        pub fn ensure_trade_pair(
            base: T::Hash,
            quote: T::Hash,
        ) -> result::Result<T::Hash, &'static str> {
            let tp = Self::get_trade_pair_hash_by_base_quote((base, quote));
            {
                if !tp.is_some() {
                    {
                        return Err("");
                    };
                }
            };
            match tp {
                Some(tp) => Ok(tp),
                None => Err(""),
            }
        }
        pub fn do_create_trade_pair(origin: T::Origin, base: T::Hash, quote: T::Hash) -> Result {
            let sender = ensure_signed(origin)?;
            {
                if !(base != quote) {
                    {
                        return Err("base can not equal to quote");
                    };
                }
            };
            let base_owner = <token::Module<T>>::owner(base);
            let quote_owner = <token::Module<T>>::owner(quote);
            {
                if !(base_owner.is_some() && quote_owner.is_some()) {
                    {
                        return Err("");
                    };
                }
            };
            let base_owner = base_owner.unwrap();
            let quote_owner = quote_owner.unwrap();
            {
                if !(sender == base_owner || sender == quote_owner) {
                    {
                        return Err("");
                    };
                }
            };
            let bq = Self::get_trade_pair_hash_by_base_quote((base, quote));
            let qb = Self::get_trade_pair_hash_by_base_quote((quote, base));
            {
                if !(!bq.is_some() && !qb.is_some()) {
                    {
                        return Err("");
                    };
                }
            };
            let nonce = <Nonce<T>>::get();
            let hash = (
                base,
                quote,
                nonce,
                sender.clone(),
                <system::Module<T>>::random_seed(),
            )
                .using_encoded(<T as system::Trait>::hash);
            let tp = TradePair { hash, base, quote };
            <Nonce<T>>::mutate(|n| *n += 1);
            <TradePairsByHash<T>>::insert(hash, tp.clone());
            <TradePairHashByBaseQuote<T>>::insert((base, quote), hash);
            Self::deposit_event(RawEvent::TradePairCreated(sender, hash, base, quote, tp));
            Ok(())
        }
    }
    /// [`RawEvent`] specialized for the configuration [`Trait`]
    ///
    /// [`RawEvent`]: enum.RawEvent.html
    /// [`Trait`]: trait.Trait.html
    pub type Event<T> = RawEvent<
        <T as system::Trait>::AccountId,
        <T as system::Trait>::Hash,
        <T as Trait>::Price,
        <T as balances::Trait>::Balance,
        TradePair<<T as system::Trait>::Hash>,
    >;
    /// Events for this module.
    ///
    pub enum RawEvent<AccountId, Hash, Price, Balance, TradePair> {
        TradePairCreated(AccountId, Hash, Hash, Hash, TradePair),
        OrderCreated(AccountId, Hash, Hash, Hash, Price, Balance),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<
            AccountId: ::core::clone::Clone,
            Hash: ::core::clone::Clone,
            Price: ::core::clone::Clone,
            Balance: ::core::clone::Clone,
            TradePair: ::core::clone::Clone,
        > ::core::clone::Clone for RawEvent<AccountId, Hash, Price, Balance, TradePair>
    {
        #[inline]
        fn clone(&self) -> RawEvent<AccountId, Hash, Price, Balance, TradePair> {
            match (&*self,) {
                (&RawEvent::TradePairCreated(
                    ref __self_0,
                    ref __self_1,
                    ref __self_2,
                    ref __self_3,
                    ref __self_4,
                ),) => RawEvent::TradePairCreated(
                    ::core::clone::Clone::clone(&(*__self_0)),
                    ::core::clone::Clone::clone(&(*__self_1)),
                    ::core::clone::Clone::clone(&(*__self_2)),
                    ::core::clone::Clone::clone(&(*__self_3)),
                    ::core::clone::Clone::clone(&(*__self_4)),
                ),
                (&RawEvent::OrderCreated(
                    ref __self_0,
                    ref __self_1,
                    ref __self_2,
                    ref __self_3,
                    ref __self_4,
                    ref __self_5,
                ),) => RawEvent::OrderCreated(
                    ::core::clone::Clone::clone(&(*__self_0)),
                    ::core::clone::Clone::clone(&(*__self_1)),
                    ::core::clone::Clone::clone(&(*__self_2)),
                    ::core::clone::Clone::clone(&(*__self_3)),
                    ::core::clone::Clone::clone(&(*__self_4)),
                    ::core::clone::Clone::clone(&(*__self_5)),
                ),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<
            AccountId: ::core::cmp::PartialEq,
            Hash: ::core::cmp::PartialEq,
            Price: ::core::cmp::PartialEq,
            Balance: ::core::cmp::PartialEq,
            TradePair: ::core::cmp::PartialEq,
        > ::core::cmp::PartialEq for RawEvent<AccountId, Hash, Price, Balance, TradePair>
    {
        #[inline]
        fn eq(&self, other: &RawEvent<AccountId, Hash, Price, Balance, TradePair>) -> bool {
            {
                let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi =
                    unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &RawEvent::TradePairCreated(
                                ref __self_0,
                                ref __self_1,
                                ref __self_2,
                                ref __self_3,
                                ref __self_4,
                            ),
                            &RawEvent::TradePairCreated(
                                ref __arg_1_0,
                                ref __arg_1_1,
                                ref __arg_1_2,
                                ref __arg_1_3,
                                ref __arg_1_4,
                            ),
                        ) => {
                            (*__self_0) == (*__arg_1_0)
                                && (*__self_1) == (*__arg_1_1)
                                && (*__self_2) == (*__arg_1_2)
                                && (*__self_3) == (*__arg_1_3)
                                && (*__self_4) == (*__arg_1_4)
                        }
                        (
                            &RawEvent::OrderCreated(
                                ref __self_0,
                                ref __self_1,
                                ref __self_2,
                                ref __self_3,
                                ref __self_4,
                                ref __self_5,
                            ),
                            &RawEvent::OrderCreated(
                                ref __arg_1_0,
                                ref __arg_1_1,
                                ref __arg_1_2,
                                ref __arg_1_3,
                                ref __arg_1_4,
                                ref __arg_1_5,
                            ),
                        ) => {
                            (*__self_0) == (*__arg_1_0)
                                && (*__self_1) == (*__arg_1_1)
                                && (*__self_2) == (*__arg_1_2)
                                && (*__self_3) == (*__arg_1_3)
                                && (*__self_4) == (*__arg_1_4)
                                && (*__self_5) == (*__arg_1_5)
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
                } else {
                    false
                }
            }
        }
        #[inline]
        fn ne(&self, other: &RawEvent<AccountId, Hash, Price, Balance, TradePair>) -> bool {
            {
                let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
                let __arg_1_vi =
                    unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
                if true && __self_vi == __arg_1_vi {
                    match (&*self, &*other) {
                        (
                            &RawEvent::TradePairCreated(
                                ref __self_0,
                                ref __self_1,
                                ref __self_2,
                                ref __self_3,
                                ref __self_4,
                            ),
                            &RawEvent::TradePairCreated(
                                ref __arg_1_0,
                                ref __arg_1_1,
                                ref __arg_1_2,
                                ref __arg_1_3,
                                ref __arg_1_4,
                            ),
                        ) => {
                            (*__self_0) != (*__arg_1_0)
                                || (*__self_1) != (*__arg_1_1)
                                || (*__self_2) != (*__arg_1_2)
                                || (*__self_3) != (*__arg_1_3)
                                || (*__self_4) != (*__arg_1_4)
                        }
                        (
                            &RawEvent::OrderCreated(
                                ref __self_0,
                                ref __self_1,
                                ref __self_2,
                                ref __self_3,
                                ref __self_4,
                                ref __self_5,
                            ),
                            &RawEvent::OrderCreated(
                                ref __arg_1_0,
                                ref __arg_1_1,
                                ref __arg_1_2,
                                ref __arg_1_3,
                                ref __arg_1_4,
                                ref __arg_1_5,
                            ),
                        ) => {
                            (*__self_0) != (*__arg_1_0)
                                || (*__self_1) != (*__arg_1_1)
                                || (*__self_2) != (*__arg_1_2)
                                || (*__self_3) != (*__arg_1_3)
                                || (*__self_4) != (*__arg_1_4)
                                || (*__self_5) != (*__arg_1_5)
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() },
                    }
                } else {
                    true
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<
            AccountId: ::core::cmp::Eq,
            Hash: ::core::cmp::Eq,
            Price: ::core::cmp::Eq,
            Balance: ::core::cmp::Eq,
            TradePair: ::core::cmp::Eq,
        > ::core::cmp::Eq for RawEvent<AccountId, Hash, Price, Balance, TradePair>
    {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<AccountId>;
                let _: ::core::cmp::AssertParamIsEq<Hash>;
                let _: ::core::cmp::AssertParamIsEq<Hash>;
                let _: ::core::cmp::AssertParamIsEq<Hash>;
                let _: ::core::cmp::AssertParamIsEq<TradePair>;
                let _: ::core::cmp::AssertParamIsEq<AccountId>;
                let _: ::core::cmp::AssertParamIsEq<Hash>;
                let _: ::core::cmp::AssertParamIsEq<Hash>;
                let _: ::core::cmp::AssertParamIsEq<Hash>;
                let _: ::core::cmp::AssertParamIsEq<Price>;
                let _: ::core::cmp::AssertParamIsEq<Balance>;
            }
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_RawEvent: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl<AccountId, Hash, Price, Balance, TradePair> _parity_codec::Encode
            for RawEvent<AccountId, Hash, Price, Balance, TradePair>
        where
            AccountId: _parity_codec::Encode,
            AccountId: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            TradePair: _parity_codec::Encode,
            TradePair: _parity_codec::Encode,
            AccountId: _parity_codec::Encode,
            AccountId: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Hash: _parity_codec::Encode,
            Price: _parity_codec::Encode,
            Price: _parity_codec::Encode,
            Balance: _parity_codec::Encode,
            Balance: _parity_codec::Encode,
        {
            fn encode_to<EncOut: _parity_codec::Output>(&self, dest: &mut EncOut) {
                match *self {
                    RawEvent::TradePairCreated(ref aa, ref ba, ref ca, ref da, ref ea) => {
                        dest.push_byte(0usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                        dest.push(ca);
                        dest.push(da);
                        dest.push(ea);
                    }
                    RawEvent::OrderCreated(ref aa, ref ba, ref ca, ref da, ref ea, ref fa) => {
                        dest.push_byte(1usize as u8);
                        dest.push(aa);
                        dest.push(ba);
                        dest.push(ca);
                        dest.push(da);
                        dest.push(ea);
                        dest.push(fa);
                    }
                    _ => (),
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_RawEvent: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl<AccountId, Hash, Price, Balance, TradePair> _parity_codec::Decode
            for RawEvent<AccountId, Hash, Price, Balance, TradePair>
        where
            AccountId: _parity_codec::Decode,
            AccountId: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            TradePair: _parity_codec::Decode,
            TradePair: _parity_codec::Decode,
            AccountId: _parity_codec::Decode,
            AccountId: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Hash: _parity_codec::Decode,
            Price: _parity_codec::Decode,
            Price: _parity_codec::Decode,
            Balance: _parity_codec::Decode,
            Balance: _parity_codec::Decode,
        {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn) -> Option<Self> {
                match input.read_byte()? {
                    x if x == 0usize as u8 => Some(RawEvent::TradePairCreated(
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                    )),
                    x if x == 1usize as u8 => Some(RawEvent::OrderCreated(
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                        _parity_codec::Decode::decode(input)?,
                    )),
                    _ => None,
                }
            }
        }
    };
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl<
            AccountId: ::core::fmt::Debug,
            Hash: ::core::fmt::Debug,
            Price: ::core::fmt::Debug,
            Balance: ::core::fmt::Debug,
            TradePair: ::core::fmt::Debug,
        > ::core::fmt::Debug for RawEvent<AccountId, Hash, Price, Balance, TradePair>
    {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&RawEvent::TradePairCreated(
                    ref __self_0,
                    ref __self_1,
                    ref __self_2,
                    ref __self_3,
                    ref __self_4,
                ),) => {
                    let mut debug_trait_builder = f.debug_tuple("TradePairCreated");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    let _ = debug_trait_builder.field(&&(*__self_2));
                    let _ = debug_trait_builder.field(&&(*__self_3));
                    let _ = debug_trait_builder.field(&&(*__self_4));
                    debug_trait_builder.finish()
                }
                (&RawEvent::OrderCreated(
                    ref __self_0,
                    ref __self_1,
                    ref __self_2,
                    ref __self_3,
                    ref __self_4,
                    ref __self_5,
                ),) => {
                    let mut debug_trait_builder = f.debug_tuple("OrderCreated");
                    let _ = debug_trait_builder.field(&&(*__self_0));
                    let _ = debug_trait_builder.field(&&(*__self_1));
                    let _ = debug_trait_builder.field(&&(*__self_2));
                    let _ = debug_trait_builder.field(&&(*__self_3));
                    let _ = debug_trait_builder.field(&&(*__self_4));
                    let _ = debug_trait_builder.field(&&(*__self_5));
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl<AccountId, Hash, Price, Balance, TradePair>
        From<RawEvent<AccountId, Hash, Price, Balance, TradePair>> for ()
    {
        fn from(_: RawEvent<AccountId, Hash, Price, Balance, TradePair>) -> () {
            ()
        }
    }
    impl<AccountId, Hash, Price, Balance, TradePair>
        RawEvent<AccountId, Hash, Price, Balance, TradePair>
    {
        #[allow(dead_code)]
        pub fn metadata() -> &'static [::srml_support::event::EventMetadata] {
            &[
                ::srml_support::event::EventMetadata {
                    name: ::srml_support::event::DecodeDifferent::Encode("TradePairCreated"),
                    arguments: ::srml_support::event::DecodeDifferent::Encode(&[
                        "AccountId",
                        "Hash",
                        "Hash",
                        "Hash",
                        "TradePair",
                    ]),
                    documentation: ::srml_support::event::DecodeDifferent::Encode(&[]),
                },
                ::srml_support::event::EventMetadata {
                    name: ::srml_support::event::DecodeDifferent::Encode("OrderCreated"),
                    arguments: ::srml_support::event::DecodeDifferent::Encode(&[
                        "AccountId",
                        "Hash",
                        "Hash",
                        "Hash",
                        "Price",
                        "Balance",
                    ]),
                    documentation: ::srml_support::event::DecodeDifferent::Encode(&[]),
                },
            ]
        }
    }
}
/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core datastructures.
pub mod opaque {
    use super::*;
    /// Opaque, encoded, unchecked extrinsic.
    pub struct UncheckedExtrinsic(#[serde(with = "bytes")] pub Vec<u8>);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for UncheckedExtrinsic {
        #[inline]
        fn eq(&self, other: &UncheckedExtrinsic) -> bool {
            match *other {
                UncheckedExtrinsic(ref __self_1_0) => match *self {
                    UncheckedExtrinsic(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &UncheckedExtrinsic) -> bool {
            match *other {
                UncheckedExtrinsic(ref __self_1_0) => match *self {
                    UncheckedExtrinsic(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for UncheckedExtrinsic {
        #[inline]
        #[doc(hidden)]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<Vec<u8>>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for UncheckedExtrinsic {
        #[inline]
        fn clone(&self) -> UncheckedExtrinsic {
            match *self {
                UncheckedExtrinsic(ref __self_0_0) => {
                    UncheckedExtrinsic(::core::clone::Clone::clone(&(*__self_0_0)))
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for UncheckedExtrinsic {
        #[inline]
        fn default() -> UncheckedExtrinsic {
            UncheckedExtrinsic(::core::default::Default::default())
        }
    }
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_ENCODE_FOR_UncheckedExtrinsic: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Encode for UncheckedExtrinsic {
            fn encode_to<EncOut: _parity_codec::Output>(&self, dest: &mut EncOut) {
                dest.push(&self.0);
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DECODE_FOR_UncheckedExtrinsic: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate parity_codec as _parity_codec;
        impl _parity_codec::Decode for UncheckedExtrinsic {
            fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn) -> Option<Self> {
                Some(UncheckedExtrinsic(_parity_codec::Decode::decode(input)?))
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_SERIALIZE_FOR_UncheckedExtrinsic: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for UncheckedExtrinsic {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::export::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                _serde::Serializer::serialize_newtype_struct(__serializer, "UncheckedExtrinsic", {
                    struct __SerializeWith<'__a> {
                        values: (&'__a Vec<u8>,),
                        phantom: _serde::export::PhantomData<UncheckedExtrinsic>,
                    }
                    impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                        fn serialize<__S>(
                            &self,
                            __s: __S,
                        ) -> _serde::export::Result<__S::Ok, __S::Error>
                        where
                            __S: _serde::Serializer,
                        {
                            bytes::serialize(self.values.0, __s)
                        }
                    }
                    &__SerializeWith {
                        values: (&self.0,),
                        phantom: _serde::export::PhantomData::<UncheckedExtrinsic>,
                    }
                })
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _IMPL_DESERIALIZE_FOR_UncheckedExtrinsic: () = {
        #[allow(unknown_lints)]
        #[allow(rust_2018_idioms)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for UncheckedExtrinsic {
            fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                struct __Visitor<'de> {
                    marker: _serde::export::PhantomData<UncheckedExtrinsic>,
                    lifetime: _serde::export::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UncheckedExtrinsic;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::export::Formatter,
                    ) -> _serde::export::fmt::Result {
                        _serde::export::Formatter::write_str(
                            __formatter,
                            "tuple struct UncheckedExtrinsic",
                        )
                    }
                    #[inline]
                    fn visit_newtype_struct<__E>(
                        self,
                        __e: __E,
                    ) -> _serde::export::Result<Self::Value, __E::Error>
                    where
                        __E: _serde::Deserializer<'de>,
                    {
                        let __field0: Vec<u8> = match bytes::deserialize(__e) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        };
                        _serde::export::Ok(UncheckedExtrinsic(__field0))
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::export::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match {
                            struct __DeserializeWith<'de> {
                                value: Vec<u8>,
                                phantom: _serde::export::PhantomData<UncheckedExtrinsic>,
                                lifetime: _serde::export::PhantomData<&'de ()>,
                            }
                            impl<'de> _serde::Deserialize<'de> for __DeserializeWith<'de> {
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::export::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::export::Ok(__DeserializeWith {
                                        value: match bytes::deserialize(__deserializer) {
                                            _serde::export::Ok(__val) => __val,
                                            _serde::export::Err(__err) => {
                                                return _serde::export::Err(__err);
                                            }
                                        },
                                        phantom: _serde::export::PhantomData,
                                        lifetime: _serde::export::PhantomData,
                                    })
                                }
                            }
                            _serde::export::Option::map(
                                match _serde::de::SeqAccess::next_element::<__DeserializeWith<'de>>(
                                    &mut __seq,
                                ) {
                                    _serde::export::Ok(__val) => __val,
                                    _serde::export::Err(__err) => {
                                        return _serde::export::Err(__err);
                                    }
                                },
                                |__wrap| __wrap.value,
                            )
                        } {
                            _serde::export::Some(__value) => __value,
                            _serde::export::None => {
                                return _serde::export::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"tuple struct UncheckedExtrinsic with 1 element",
                                ));
                            }
                        };
                        _serde::export::Ok(UncheckedExtrinsic(__field0))
                    }
                }
                _serde::Deserializer::deserialize_newtype_struct(
                    __deserializer,
                    "UncheckedExtrinsic",
                    __Visitor {
                        marker: _serde::export::PhantomData::<UncheckedExtrinsic>,
                        lifetime: _serde::export::PhantomData,
                    },
                )
            }
        }
    };
    #[cfg(feature = "std")]
    impl std::fmt::Debug for UncheckedExtrinsic {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            fmt.write_fmt(::core::fmt::Arguments::new_v1(
                &[""],
                &match (&primitives::hexdisplay::HexDisplay::from(&self.0),) {
                    (arg0,) => [::core::fmt::ArgumentV1::new(
                        arg0,
                        ::core::fmt::Display::fmt,
                    )],
                },
            ))
        }
    }
    impl traits::Extrinsic for UncheckedExtrinsic {
        fn is_signed(&self) -> Option<bool> {
            None
        }
    }
    /// Opaque block header type.
    pub type Header = generic::Header<
        BlockNumber,
        BlakeTwo256,
        generic::DigestItem<Hash, AuthorityId, AuthoritySignature>,
    >;
    /// Opaque block type.
    pub type Block = generic::Block<Header, UncheckedExtrinsic>;
    /// Opaque block identifier type.
    pub type BlockId = generic::BlockId<Block>;
    /// Opaque session key type.
    pub type SessionKey = AuthorityId;
}
/// This runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
    spec_name: { ::std::borrow::Cow::Borrowed("dex") },
    impl_name: { ::std::borrow::Cow::Borrowed("dex") },
    authoring_version: 3,
    spec_version: 4,
    impl_version: 4,
    apis: RUNTIME_API_VERSIONS,
};
/// The version infromation used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
    NativeVersion {
        runtime_version: VERSION,
        can_author_with: Default::default(),
    }
}
impl system::Trait for Runtime {
    /// The identifier used to distinguish between accounts.
    type AccountId = AccountId;
    /// The lookup mechanism to get account ID from whatever is passed in dispatchers.
    type Lookup = Indices;
    /// The index type for storing how many extrinsics an account has signed.
    type Index = Nonce;
    /// The index type for blocks.
    type BlockNumber = BlockNumber;
    /// The type for hashing blocks and tries.
    type Hash = Hash;
    /// The hashing algorithm used.
    type Hashing = BlakeTwo256;
    /// The header digest type.
    type Digest = generic::Digest<Log>;
    /// The header type.
    type Header = generic::Header<BlockNumber, BlakeTwo256, Log>;
    /// The ubiquitous event type.
    type Event = Event;
    /// The ubiquitous log type.
    type Log = Log;
    /// The ubiquitous origin type.
    type Origin = Origin;
}
impl aura::Trait for Runtime {
    type HandleReport = ();
}
impl consensus::Trait for Runtime {
    /// The identifier we use to refer to authorities.
    type SessionKey = AuthorityId;
    type InherentOfflineReport = ();
    /// The ubiquitous log type.
    type Log = Log;
}
impl indices::Trait for Runtime {
    /// The type for recording indexing into the account enumeration. If this ever overflows, there
    /// will be problems!
    type AccountIndex = u32;
    /// Use the standard means of resolving an index hint from an id.
    type ResolveHint = indices::SimpleResolveHint<Self::AccountId, Self::AccountIndex>;
    /// Determine whether an account is dead.
    type IsDeadAccount = Balances;
    /// The uniquitous event type.
    type Event = Event;
}
impl timestamp::Trait for Runtime {
    /// A timestamp: seconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Aura;
}
impl balances::Trait for Runtime {
    /// The type for recording an account's balance.
    type Balance = u128;
    /// What to do if an account's free balance gets zeroed.
    type OnFreeBalanceZero = ();
    /// What to do if a new account is created.
    type OnNewAccount = Indices;
    /// The uniquitous event type.
    type Event = Event;
    type TransactionPayment = ();
    type DustRemoval = ();
    type TransferPayment = ();
}
impl sudo::Trait for Runtime {
    /// The uniquitous event type.
    type Event = Event;
    type Proposal = Call;
}
/// Used for the module template in `./template.rs`
impl template::Trait for Runtime {
    type Event = Event;
}
/// Used for the module template in `./template.rs`
impl token::Trait for Runtime {
    type Event = Event;
}
impl trade::Trait for Runtime {
    type Event = Event;
    type Price = u64;
}
pub struct Runtime;
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Runtime {
    #[inline]
    fn clone(&self) -> Runtime {
        {
            *self
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::marker::Copy for Runtime {}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Runtime {
    #[inline]
    fn eq(&self, other: &Runtime) -> bool {
        match *other {
            Runtime => match *self {
                Runtime => true,
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for Runtime {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {}
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Runtime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Runtime => {
                let mut debug_trait_builder = f.debug_tuple("Runtime");
                debug_trait_builder.finish()
            }
        }
    }
}
impl ::srml_support::runtime_primitives::traits::GetNodeBlockType for Runtime {
    type NodeBlock = opaque::Block;
}
impl ::srml_support::runtime_primitives::traits::GetRuntimeBlockType for Runtime {
    type RuntimeBlock = Block;
}
#[allow(non_camel_case_types)]
pub enum Event {
    system(system::Event),
    indices(indices::Event<Runtime>),
    balances(balances::Event<Runtime>),
    sudo(sudo::Event<Runtime>),
    template(template::Event<Runtime>),
    token(token::Event<Runtime>),
    trade(trade::Event<Runtime>),
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::clone::Clone for Event {
    #[inline]
    fn clone(&self) -> Event {
        match (&*self,) {
            (&Event::system(ref __self_0),) => {
                Event::system(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::indices(ref __self_0),) => {
                Event::indices(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::balances(ref __self_0),) => {
                Event::balances(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::sudo(ref __self_0),) => Event::sudo(::core::clone::Clone::clone(&(*__self_0))),
            (&Event::template(ref __self_0),) => {
                Event::template(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::token(ref __self_0),) => {
                Event::token(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Event::trade(ref __self_0),) => {
                Event::trade(::core::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::PartialEq for Event {
    #[inline]
    fn eq(&self, other: &Event) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Event::system(ref __self_0), &Event::system(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::indices(ref __self_0), &Event::indices(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::balances(ref __self_0), &Event::balances(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::sudo(ref __self_0), &Event::sudo(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::template(ref __self_0), &Event::template(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::token(ref __self_0), &Event::token(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Event::trade(ref __self_0), &Event::trade(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &Event) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Event::system(ref __self_0), &Event::system(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::indices(ref __self_0), &Event::indices(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::balances(ref __self_0), &Event::balances(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::sudo(ref __self_0), &Event::sudo(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::template(ref __self_0), &Event::template(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::token(ref __self_0), &Event::token(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Event::trade(ref __self_0), &Event::trade(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::Eq for Event {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<system::Event>;
            let _: ::core::cmp::AssertParamIsEq<indices::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<balances::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<sudo::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<template::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<token::Event<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<trade::Event<Runtime>>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_Event: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate parity_codec as _parity_codec;
    impl _parity_codec::Encode for Event {
        fn encode_to<EncOut: _parity_codec::Output>(&self, dest: &mut EncOut) {
            match *self {
                Event::system(ref aa) => {
                    dest.push_byte(0usize as u8);
                    dest.push(aa);
                }
                Event::indices(ref aa) => {
                    dest.push_byte(1usize as u8);
                    dest.push(aa);
                }
                Event::balances(ref aa) => {
                    dest.push_byte(2usize as u8);
                    dest.push(aa);
                }
                Event::sudo(ref aa) => {
                    dest.push_byte(3usize as u8);
                    dest.push(aa);
                }
                Event::template(ref aa) => {
                    dest.push_byte(4usize as u8);
                    dest.push(aa);
                }
                Event::token(ref aa) => {
                    dest.push_byte(5usize as u8);
                    dest.push(aa);
                }
                Event::trade(ref aa) => {
                    dest.push_byte(6usize as u8);
                    dest.push(aa);
                }
                _ => (),
            }
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_Event: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate parity_codec as _parity_codec;
    impl _parity_codec::Decode for Event {
        fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn) -> Option<Self> {
            match input.read_byte()? {
                x if x == 0usize as u8 => {
                    Some(Event::system(_parity_codec::Decode::decode(input)?))
                }
                x if x == 1usize as u8 => {
                    Some(Event::indices(_parity_codec::Decode::decode(input)?))
                }
                x if x == 2usize as u8 => {
                    Some(Event::balances(_parity_codec::Decode::decode(input)?))
                }
                x if x == 3usize as u8 => Some(Event::sudo(_parity_codec::Decode::decode(input)?)),
                x if x == 4usize as u8 => {
                    Some(Event::template(_parity_codec::Decode::decode(input)?))
                }
                x if x == 5usize as u8 => Some(Event::token(_parity_codec::Decode::decode(input)?)),
                x if x == 6usize as u8 => Some(Event::trade(_parity_codec::Decode::decode(input)?)),
                _ => None,
            }
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::fmt::Debug for Event {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&Event::system(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("system");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::indices(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("indices");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::balances(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("balances");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::sudo(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("sudo");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::template(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("template");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::token(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("token");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Event::trade(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("trade");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
impl From<system::Event> for Event {
    fn from(x: system::Event) -> Self {
        Event::system(x)
    }
}
impl From<indices::Event<Runtime>> for Event {
    fn from(x: indices::Event<Runtime>) -> Self {
        Event::indices(x)
    }
}
impl From<balances::Event<Runtime>> for Event {
    fn from(x: balances::Event<Runtime>) -> Self {
        Event::balances(x)
    }
}
impl From<sudo::Event<Runtime>> for Event {
    fn from(x: sudo::Event<Runtime>) -> Self {
        Event::sudo(x)
    }
}
impl From<template::Event<Runtime>> for Event {
    fn from(x: template::Event<Runtime>) -> Self {
        Event::template(x)
    }
}
impl From<token::Event<Runtime>> for Event {
    fn from(x: token::Event<Runtime>) -> Self {
        Event::token(x)
    }
}
impl From<trade::Event<Runtime>> for Event {
    fn from(x: trade::Event<Runtime>) -> Self {
        Event::trade(x)
    }
}
impl Runtime {
    #[allow(dead_code)]
    pub fn outer_event_metadata() -> ::srml_support::event::OuterEventMetadata {
        ::srml_support::event::OuterEventMetadata {
            name: ::srml_support::event::DecodeDifferent::Encode("Event"),
            events: ::srml_support::event::DecodeDifferent::Encode(&[
                (
                    "system",
                    ::srml_support::event::FnEncode(system::Event::metadata),
                ),
                (
                    "indices",
                    ::srml_support::event::FnEncode(indices::Event::<Runtime>::metadata),
                ),
                (
                    "balances",
                    ::srml_support::event::FnEncode(balances::Event::<Runtime>::metadata),
                ),
                (
                    "sudo",
                    ::srml_support::event::FnEncode(sudo::Event::<Runtime>::metadata),
                ),
                (
                    "template",
                    ::srml_support::event::FnEncode(template::Event::<Runtime>::metadata),
                ),
                (
                    "token",
                    ::srml_support::event::FnEncode(token::Event::<Runtime>::metadata),
                ),
                (
                    "trade",
                    ::srml_support::event::FnEncode(trade::Event::<Runtime>::metadata),
                ),
            ]),
        }
    }
    #[allow(dead_code)]
    pub fn __module_events_system() -> &'static [::srml_support::event::EventMetadata] {
        system::Event::metadata()
    }
    pub fn __module_events_indices() -> &'static [::srml_support::event::EventMetadata] {
        indices::Event::<Runtime>::metadata()
    }
    pub fn __module_events_balances() -> &'static [::srml_support::event::EventMetadata] {
        balances::Event::<Runtime>::metadata()
    }
    pub fn __module_events_sudo() -> &'static [::srml_support::event::EventMetadata] {
        sudo::Event::<Runtime>::metadata()
    }
    pub fn __module_events_template() -> &'static [::srml_support::event::EventMetadata] {
        template::Event::<Runtime>::metadata()
    }
    pub fn __module_events_token() -> &'static [::srml_support::event::EventMetadata] {
        token::Event::<Runtime>::metadata()
    }
    pub fn __module_events_trade() -> &'static [::srml_support::event::EventMetadata] {
        trade::Event::<Runtime>::metadata()
    }
}
#[allow(non_camel_case_types)]
pub enum Origin {
    system(system::Origin<Runtime>),
    #[allow(dead_code)]
    Void(::srml_support::Void),
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::clone::Clone for Origin {
    #[inline]
    fn clone(&self) -> Origin {
        match (&*self,) {
            (&Origin::system(ref __self_0),) => {
                Origin::system(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Origin::Void(ref __self_0),) => {
                Origin::Void(::core::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::PartialEq for Origin {
    #[inline]
    fn eq(&self, other: &Origin) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Origin::system(ref __self_0), &Origin::system(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Origin::Void(ref __self_0), &Origin::Void(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &Origin) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Origin::system(ref __self_0), &Origin::system(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Origin::Void(ref __self_0), &Origin::Void(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::Eq for Origin {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<system::Origin<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<::srml_support::Void>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::fmt::Debug for Origin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&Origin::system(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("system");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Origin::Void(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Void");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(dead_code)]
impl Origin {
    pub const INHERENT: Self = Origin::system(system::RawOrigin::Inherent);
    pub const ROOT: Self = Origin::system(system::RawOrigin::Root);
    pub fn signed(by: <Runtime as system::Trait>::AccountId) -> Self {
        Origin::system(system::RawOrigin::Signed(by))
    }
}
impl From<system::Origin<Runtime>> for Origin {
    fn from(x: system::Origin<Runtime>) -> Self {
        Origin::system(x)
    }
}
impl Into<Option<system::Origin<Runtime>>> for Origin {
    fn into(self) -> Option<system::Origin<Runtime>> {
        if let Origin::system(l) = self {
            Some(l)
        } else {
            None
        }
    }
}
impl From<Option<<Runtime as system::Trait>::AccountId>> for Origin {
    fn from(x: Option<<Runtime as system::Trait>::AccountId>) -> Self {
        <system::Origin<Runtime>>::from(x).into()
    }
}
pub type System = system::Module<Runtime>;
pub type Timestamp = timestamp::Module<Runtime>;
pub type Consensus = consensus::Module<Runtime>;
pub type Aura = aura::Module<Runtime>;
pub type Indices = indices::Module<Runtime>;
pub type Balances = balances::Module<Runtime>;
pub type Sudo = sudo::Module<Runtime>;
pub type TemplateModule = template::Module<Runtime>;
pub type TokenModule = token::Module<Runtime>;
pub type TradeModule = trade::Module<Runtime>;
type AllModules = (
    Timestamp,
    Consensus,
    Aura,
    Indices,
    Balances,
    Sudo,
    TemplateModule,
    TokenModule,
    TradeModule,
);
pub enum Call {
    Timestamp(::srml_support::dispatch::CallableCallFor<Timestamp>),
    Consensus(::srml_support::dispatch::CallableCallFor<Consensus>),
    Indices(::srml_support::dispatch::CallableCallFor<Indices>),
    Balances(::srml_support::dispatch::CallableCallFor<Balances>),
    Sudo(::srml_support::dispatch::CallableCallFor<Sudo>),
    TemplateModule(::srml_support::dispatch::CallableCallFor<TemplateModule>),
    TokenModule(::srml_support::dispatch::CallableCallFor<TokenModule>),
    TradeModule(::srml_support::dispatch::CallableCallFor<TradeModule>),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::clone::Clone for Call {
    #[inline]
    fn clone(&self) -> Call {
        match (&*self,) {
            (&Call::Timestamp(ref __self_0),) => {
                Call::Timestamp(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Consensus(ref __self_0),) => {
                Call::Consensus(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Indices(ref __self_0),) => {
                Call::Indices(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Balances(ref __self_0),) => {
                Call::Balances(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::Sudo(ref __self_0),) => Call::Sudo(::core::clone::Clone::clone(&(*__self_0))),
            (&Call::TemplateModule(ref __self_0),) => {
                Call::TemplateModule(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::TokenModule(ref __self_0),) => {
                Call::TokenModule(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&Call::TradeModule(ref __self_0),) => {
                Call::TradeModule(::core::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::PartialEq for Call {
    #[inline]
    fn eq(&self, other: &Call) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Call::Timestamp(ref __self_0), &Call::Timestamp(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Consensus(ref __self_0), &Call::Consensus(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Indices(ref __self_0), &Call::Indices(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Balances(ref __self_0), &Call::Balances(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::Sudo(ref __self_0), &Call::Sudo(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::TemplateModule(ref __self_0), &Call::TemplateModule(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::TokenModule(ref __self_0), &Call::TokenModule(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (&Call::TradeModule(ref __self_0), &Call::TradeModule(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &Call) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&Call::Timestamp(ref __self_0), &Call::Timestamp(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Consensus(ref __self_0), &Call::Consensus(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Indices(ref __self_0), &Call::Indices(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Balances(ref __self_0), &Call::Balances(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::Sudo(ref __self_0), &Call::Sudo(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::TemplateModule(ref __self_0), &Call::TemplateModule(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::TokenModule(ref __self_0), &Call::TokenModule(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (&Call::TradeModule(ref __self_0), &Call::TradeModule(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::cmp::Eq for Call {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<
                ::srml_support::dispatch::CallableCallFor<Timestamp>,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                ::srml_support::dispatch::CallableCallFor<Consensus>,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                ::srml_support::dispatch::CallableCallFor<Indices>,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                ::srml_support::dispatch::CallableCallFor<Balances>,
            >;
            let _: ::core::cmp::AssertParamIsEq<::srml_support::dispatch::CallableCallFor<Sudo>>;
            let _: ::core::cmp::AssertParamIsEq<
                ::srml_support::dispatch::CallableCallFor<TemplateModule>,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                ::srml_support::dispatch::CallableCallFor<TokenModule>,
            >;
            let _: ::core::cmp::AssertParamIsEq<
                ::srml_support::dispatch::CallableCallFor<TradeModule>,
            >;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Call {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&Call::Timestamp(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Timestamp");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::Consensus(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Consensus");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::Indices(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Indices");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::Balances(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Balances");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::Sudo(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("Sudo");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::TemplateModule(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("TemplateModule");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::TokenModule(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("TokenModule");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&Call::TradeModule(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("TradeModule");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
impl ::srml_support::dispatch::Decode for Call {
    fn decode<I: ::srml_support::dispatch::Input>(input: &mut I) -> Option<Self> {
        let input_id = input.read_byte()?;
        {
            if input_id == (0) {
                let outer_dispatch_param = ::srml_support::dispatch::Decode::decode(input)?;
                return Some(Call::Timestamp(outer_dispatch_param));
            }
            {
                if input_id == (0 + 1) {
                    let outer_dispatch_param = ::srml_support::dispatch::Decode::decode(input)?;
                    return Some(Call::Consensus(outer_dispatch_param));
                }
                {
                    if input_id == (0 + 1 + 1) {
                        let outer_dispatch_param = ::srml_support::dispatch::Decode::decode(input)?;
                        return Some(Call::Indices(outer_dispatch_param));
                    }
                    {
                        if input_id == (0 + 1 + 1 + 1) {
                            let outer_dispatch_param =
                                ::srml_support::dispatch::Decode::decode(input)?;
                            return Some(Call::Balances(outer_dispatch_param));
                        }
                        {
                            if input_id == (0 + 1 + 1 + 1 + 1) {
                                let outer_dispatch_param =
                                    ::srml_support::dispatch::Decode::decode(input)?;
                                return Some(Call::Sudo(outer_dispatch_param));
                            }
                            {
                                if input_id == (0 + 1 + 1 + 1 + 1 + 1) {
                                    let outer_dispatch_param =
                                        ::srml_support::dispatch::Decode::decode(input)?;
                                    return Some(Call::TemplateModule(outer_dispatch_param));
                                }
                                {
                                    if input_id == (0 + 1 + 1 + 1 + 1 + 1 + 1) {
                                        let outer_dispatch_param =
                                            ::srml_support::dispatch::Decode::decode(input)?;
                                        return Some(Call::TokenModule(outer_dispatch_param));
                                    }
                                    {
                                        if input_id == (0 + 1 + 1 + 1 + 1 + 1 + 1 + 1) {
                                            let outer_dispatch_param =
                                                ::srml_support::dispatch::Decode::decode(input)?;
                                            return Some(Call::TradeModule(outer_dispatch_param));
                                        }
                                        None
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
impl ::srml_support::dispatch::Encode for Call {
    fn encode_to<W: ::srml_support::dispatch::Output>(&self, dest: &mut W) {
        {
            if let Call::Timestamp(ref outer_dispatch_param) = *self {
                dest.push_byte((0) as u8);
                outer_dispatch_param.encode_to(dest);
            }
            {
                if let Call::Consensus(ref outer_dispatch_param) = *self {
                    dest.push_byte((0 + 1) as u8);
                    outer_dispatch_param.encode_to(dest);
                }
                {
                    if let Call::Indices(ref outer_dispatch_param) = *self {
                        dest.push_byte((0 + 1 + 1) as u8);
                        outer_dispatch_param.encode_to(dest);
                    }
                    {
                        if let Call::Balances(ref outer_dispatch_param) = *self {
                            dest.push_byte((0 + 1 + 1 + 1) as u8);
                            outer_dispatch_param.encode_to(dest);
                        }
                        {
                            if let Call::Sudo(ref outer_dispatch_param) = *self {
                                dest.push_byte((0 + 1 + 1 + 1 + 1) as u8);
                                outer_dispatch_param.encode_to(dest);
                            }
                            {
                                if let Call::TemplateModule(ref outer_dispatch_param) = *self {
                                    dest.push_byte((0 + 1 + 1 + 1 + 1 + 1) as u8);
                                    outer_dispatch_param.encode_to(dest);
                                }
                                {
                                    if let Call::TokenModule(ref outer_dispatch_param) = *self {
                                        dest.push_byte((0 + 1 + 1 + 1 + 1 + 1 + 1) as u8);
                                        outer_dispatch_param.encode_to(dest);
                                    }
                                    {
                                        if let Call::TradeModule(ref outer_dispatch_param) = *self {
                                            dest.push_byte((0 + 1 + 1 + 1 + 1 + 1 + 1 + 1) as u8);
                                            outer_dispatch_param.encode_to(dest);
                                        }
                                        {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
impl ::srml_support::dispatch::Dispatchable for Call {
    type Origin = Origin;
    type Trait = Call;
    fn dispatch(self, origin: Origin) -> ::srml_support::dispatch::Result {
        match self {
            Call::Timestamp(call) => call.dispatch(origin),
            Call::Consensus(call) => call.dispatch(origin),
            Call::Indices(call) => call.dispatch(origin),
            Call::Balances(call) => call.dispatch(origin),
            Call::Sudo(call) => call.dispatch(origin),
            Call::TemplateModule(call) => call.dispatch(origin),
            Call::TokenModule(call) => call.dispatch(origin),
            Call::TradeModule(call) => call.dispatch(origin),
        }
    }
}
impl ::srml_support::dispatch::IsSubType<Timestamp> for Call {
    fn is_aux_sub_type(&self) -> Option<&<Timestamp as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Timestamp(ref r) = *self {
            Some(r)
        } else {
            None
        }
    }
}
impl ::srml_support::dispatch::IsSubType<Consensus> for Call {
    fn is_aux_sub_type(&self) -> Option<&<Consensus as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Consensus(ref r) = *self {
            Some(r)
        } else {
            None
        }
    }
}
impl ::srml_support::dispatch::IsSubType<Indices> for Call {
    fn is_aux_sub_type(&self) -> Option<&<Indices as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Indices(ref r) = *self {
            Some(r)
        } else {
            None
        }
    }
}
impl ::srml_support::dispatch::IsSubType<Balances> for Call {
    fn is_aux_sub_type(&self) -> Option<&<Balances as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Balances(ref r) = *self {
            Some(r)
        } else {
            None
        }
    }
}
impl ::srml_support::dispatch::IsSubType<Sudo> for Call {
    fn is_aux_sub_type(&self) -> Option<&<Sudo as ::srml_support::dispatch::Callable>::Call> {
        if let Call::Sudo(ref r) = *self {
            Some(r)
        } else {
            None
        }
    }
}
impl ::srml_support::dispatch::IsSubType<TemplateModule> for Call {
    fn is_aux_sub_type(
        &self,
    ) -> Option<&<TemplateModule as ::srml_support::dispatch::Callable>::Call> {
        if let Call::TemplateModule(ref r) = *self {
            Some(r)
        } else {
            None
        }
    }
}
impl ::srml_support::dispatch::IsSubType<TokenModule> for Call {
    fn is_aux_sub_type(
        &self,
    ) -> Option<&<TokenModule as ::srml_support::dispatch::Callable>::Call> {
        if let Call::TokenModule(ref r) = *self {
            Some(r)
        } else {
            None
        }
    }
}
impl ::srml_support::dispatch::IsSubType<TradeModule> for Call {
    fn is_aux_sub_type(
        &self,
    ) -> Option<&<TradeModule as ::srml_support::dispatch::Callable>::Call> {
        if let Call::TradeModule(ref r) = *self {
            Some(r)
        } else {
            None
        }
    }
}
impl Runtime {
    pub fn metadata() -> ::srml_support::metadata::RuntimeMetadataPrefixed {
        ::srml_support::metadata::RuntimeMetadata::V4(::srml_support::metadata::RuntimeMetadataV4 {
            modules: ::srml_support::metadata::DecodeDifferent::Encode(&[
                ::srml_support::metadata::ModuleMetadata {
                    name: ::srml_support::metadata::DecodeDifferent::Encode("system"),
                    prefix: ::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            system::Module::<Runtime>::store_metadata_name,
                        ),
                    ),
                    storage: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            system::Module::<Runtime>::store_metadata_functions,
                        ),
                    )),
                    calls: None,
                    event: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode({
                            enum ProcMacroHack {
                                Value = ("Runtime :: [< __module_events_ system >]", 0).1,
                            }
                            {
                                Runtime::__module_events_system
                            }
                        }),
                    )),
                },
                ::srml_support::metadata::ModuleMetadata {
                    name: ::srml_support::metadata::DecodeDifferent::Encode("timestamp"),
                    prefix: ::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            timestamp::Module::<Runtime>::store_metadata_name,
                        ),
                    ),
                    storage: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            timestamp::Module::<Runtime>::store_metadata_functions,
                        ),
                    )),
                    calls: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            timestamp::Module::<Runtime>::call_functions,
                        ),
                    )),
                    event: None,
                },
                ::srml_support::metadata::ModuleMetadata {
                    name: ::srml_support::metadata::DecodeDifferent::Encode("consensus"),
                    prefix: ::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            consensus::Module::<Runtime>::store_metadata_name,
                        ),
                    ),
                    storage: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            consensus::Module::<Runtime>::store_metadata_functions,
                        ),
                    )),
                    calls: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            consensus::Module::<Runtime>::call_functions,
                        ),
                    )),
                    event: None,
                },
                ::srml_support::metadata::ModuleMetadata {
                    name: ::srml_support::metadata::DecodeDifferent::Encode("aura"),
                    prefix: ::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(|| ""),
                    ),
                    storage: None,
                    calls: None,
                    event: None,
                },
                ::srml_support::metadata::ModuleMetadata {
                    name: ::srml_support::metadata::DecodeDifferent::Encode("indices"),
                    prefix: ::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            indices::Module::<Runtime>::store_metadata_name,
                        ),
                    ),
                    storage: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            indices::Module::<Runtime>::store_metadata_functions,
                        ),
                    )),
                    calls: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            indices::Module::<Runtime>::call_functions,
                        ),
                    )),
                    event: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode({
                            enum ProcMacroHack {
                                Value = ("Runtime :: [< __module_events_ indices >]", 0).1,
                            }
                            {
                                Runtime::__module_events_indices
                            }
                        }),
                    )),
                },
                ::srml_support::metadata::ModuleMetadata {
                    name: ::srml_support::metadata::DecodeDifferent::Encode("balances"),
                    prefix: ::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            balances::Module::<Runtime>::store_metadata_name,
                        ),
                    ),
                    storage: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            balances::Module::<Runtime>::store_metadata_functions,
                        ),
                    )),
                    calls: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            balances::Module::<Runtime>::call_functions,
                        ),
                    )),
                    event: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode({
                            enum ProcMacroHack {
                                Value = ("Runtime :: [< __module_events_ balances >]", 0).1,
                            }
                            {
                                Runtime::__module_events_balances
                            }
                        }),
                    )),
                },
                ::srml_support::metadata::ModuleMetadata {
                    name: ::srml_support::metadata::DecodeDifferent::Encode("sudo"),
                    prefix: ::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            sudo::Module::<Runtime>::store_metadata_name,
                        ),
                    ),
                    storage: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            sudo::Module::<Runtime>::store_metadata_functions,
                        ),
                    )),
                    calls: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(sudo::Module::<Runtime>::call_functions),
                    )),
                    event: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode({
                            enum ProcMacroHack {
                                Value = ("Runtime :: [< __module_events_ sudo >]", 0).1,
                            }
                            {
                                Runtime::__module_events_sudo
                            }
                        }),
                    )),
                },
                ::srml_support::metadata::ModuleMetadata {
                    name: ::srml_support::metadata::DecodeDifferent::Encode("template"),
                    prefix: ::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            template::Module::<Runtime>::store_metadata_name,
                        ),
                    ),
                    storage: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            template::Module::<Runtime>::store_metadata_functions,
                        ),
                    )),
                    calls: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            template::Module::<Runtime>::call_functions,
                        ),
                    )),
                    event: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode({
                            enum ProcMacroHack {
                                Value = ("Runtime :: [< __module_events_ template >]", 0).1,
                            }
                            {
                                Runtime::__module_events_template
                            }
                        }),
                    )),
                },
                ::srml_support::metadata::ModuleMetadata {
                    name: ::srml_support::metadata::DecodeDifferent::Encode("token"),
                    prefix: ::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            token::Module::<Runtime>::store_metadata_name,
                        ),
                    ),
                    storage: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            token::Module::<Runtime>::store_metadata_functions,
                        ),
                    )),
                    calls: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            token::Module::<Runtime>::call_functions,
                        ),
                    )),
                    event: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode({
                            enum ProcMacroHack {
                                Value = ("Runtime :: [< __module_events_ token >]", 0).1,
                            }
                            {
                                Runtime::__module_events_token
                            }
                        }),
                    )),
                },
                ::srml_support::metadata::ModuleMetadata {
                    name: ::srml_support::metadata::DecodeDifferent::Encode("trade"),
                    prefix: ::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            trade::Module::<Runtime>::store_metadata_name,
                        ),
                    ),
                    storage: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            trade::Module::<Runtime>::store_metadata_functions,
                        ),
                    )),
                    calls: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode(
                            trade::Module::<Runtime>::call_functions,
                        ),
                    )),
                    event: Some(::srml_support::metadata::DecodeDifferent::Encode(
                        ::srml_support::metadata::FnEncode({
                            enum ProcMacroHack {
                                Value = ("Runtime :: [< __module_events_ trade >]", 0).1,
                            }
                            {
                                Runtime::__module_events_trade
                            }
                        }),
                    )),
                },
            ]),
        })
        .into()
    }
}
/// Wrapper for all possible log entries for the `$trait` runtime. Provides binary-compatible
/// `Encode`/`Decode` implementations with the corresponding `generic::DigestItem`.
#[allow(non_camel_case_types)]
pub struct Log(InternalLog);
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::clone::Clone for Log {
    #[inline]
    fn clone(&self) -> Log {
        match *self {
            Log(ref __self_0_0) => Log(::core::clone::Clone::clone(&(*__self_0_0))),
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::PartialEq for Log {
    #[inline]
    fn eq(&self, other: &Log) -> bool {
        match *other {
            Log(ref __self_1_0) => match *self {
                Log(ref __self_0_0) => (*__self_0_0) == (*__self_1_0),
            },
        }
    }
    #[inline]
    fn ne(&self, other: &Log) -> bool {
        match *other {
            Log(ref __self_1_0) => match *self {
                Log(ref __self_0_0) => (*__self_0_0) != (*__self_1_0),
            },
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::Eq for Log {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<InternalLog>;
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::fmt::Debug for Log {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Log(ref __self_0_0) => {
                let mut debug_trait_builder = f.debug_tuple("Log");
                let _ = debug_trait_builder.field(&&(*__self_0_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_Log: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Log {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::export::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            _serde::Serializer::serialize_newtype_struct(__serializer, "Log", &self.0)
        }
    }
};
/// All possible log entries for the `$trait` runtime. `Encode`/`Decode` implementations
/// are auto-generated => it is not binary-compatible with `generic::DigestItem`.
#[allow(non_camel_case_types)]
pub enum InternalLog {
    system(system::Log<Runtime>),
    consensus(consensus::Log<Runtime>),
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::clone::Clone for InternalLog {
    #[inline]
    fn clone(&self) -> InternalLog {
        match (&*self,) {
            (&InternalLog::system(ref __self_0),) => {
                InternalLog::system(::core::clone::Clone::clone(&(*__self_0)))
            }
            (&InternalLog::consensus(ref __self_0),) => {
                InternalLog::consensus(::core::clone::Clone::clone(&(*__self_0)))
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::PartialEq for InternalLog {
    #[inline]
    fn eq(&self, other: &InternalLog) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&InternalLog::system(ref __self_0), &InternalLog::system(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    (
                        &InternalLog::consensus(ref __self_0),
                        &InternalLog::consensus(ref __arg_1_0),
                    ) => (*__self_0) == (*__arg_1_0),
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                false
            }
        }
    }
    #[inline]
    fn ne(&self, other: &InternalLog) -> bool {
        {
            let __self_vi = unsafe { ::core::intrinsics::discriminant_value(&*self) } as isize;
            let __arg_1_vi = unsafe { ::core::intrinsics::discriminant_value(&*other) } as isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&InternalLog::system(ref __self_0), &InternalLog::system(ref __arg_1_0)) => {
                        (*__self_0) != (*__arg_1_0)
                    }
                    (
                        &InternalLog::consensus(ref __self_0),
                        &InternalLog::consensus(ref __arg_1_0),
                    ) => (*__self_0) != (*__arg_1_0),
                    _ => unsafe { ::core::intrinsics::unreachable() },
                }
            } else {
                true
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::cmp::Eq for InternalLog {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () {
        {
            let _: ::core::cmp::AssertParamIsEq<system::Log<Runtime>>;
            let _: ::core::cmp::AssertParamIsEq<consensus::Log<Runtime>>;
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_ENCODE_FOR_InternalLog: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate parity_codec as _parity_codec;
    impl _parity_codec::Encode for InternalLog {
        fn encode_to<EncOut: _parity_codec::Output>(&self, dest: &mut EncOut) {
            match *self {
                InternalLog::system(ref aa) => {
                    dest.push_byte(0usize as u8);
                    dest.push(aa);
                }
                InternalLog::consensus(ref aa) => {
                    dest.push_byte(1usize as u8);
                    dest.push(aa);
                }
                _ => (),
            }
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DECODE_FOR_InternalLog: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate parity_codec as _parity_codec;
    impl _parity_codec::Decode for InternalLog {
        fn decode<DecIn: _parity_codec::Input>(input: &mut DecIn) -> Option<Self> {
            match input.read_byte()? {
                x if x == 0usize as u8 => {
                    Some(InternalLog::system(_parity_codec::Decode::decode(input)?))
                }
                x if x == 1usize as u8 => Some(InternalLog::consensus(
                    _parity_codec::Decode::decode(input)?,
                )),
                _ => None,
            }
        }
    }
};
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(non_camel_case_types)]
impl ::core::fmt::Debug for InternalLog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match (&*self,) {
            (&InternalLog::system(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("system");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
            (&InternalLog::consensus(ref __self_0),) => {
                let mut debug_trait_builder = f.debug_tuple("consensus");
                let _ = debug_trait_builder.field(&&(*__self_0));
                debug_trait_builder.finish()
            }
        }
    }
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_InternalLog: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for InternalLog {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::export::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            match *self {
                InternalLog::system(ref __field0) => _serde::Serializer::serialize_newtype_variant(
                    __serializer,
                    "InternalLog",
                    0u32,
                    "system",
                    __field0,
                ),
                InternalLog::consensus(ref __field0) => {
                    _serde::Serializer::serialize_newtype_variant(
                        __serializer,
                        "InternalLog",
                        1u32,
                        "consensus",
                        __field0,
                    )
                }
            }
        }
    }
};
impl Log {
    /// Try to convert `$name` into `generic::DigestItemRef`. Returns Some when
    /// `self` is a 'system' log && it has been marked as 'system' in macro call.
    /// Otherwise, None is returned.
    #[allow(unreachable_patterns)]
    fn dref<'a>(
        &'a self,
    ) -> Option<::sr_primitives::generic::DigestItemRef<'a, Hash, AuthorityId, AuthoritySignature>>
    {
        match self.0 {
            InternalLog::system(system::RawLog::ChangesTrieRoot(ref v)) => {
                Some(::sr_primitives::generic::DigestItemRef::ChangesTrieRoot(v))
            }
            InternalLog::consensus(consensus::RawLog::AuthoritiesChange(ref v)) => Some(
                ::sr_primitives::generic::DigestItemRef::AuthoritiesChange(v),
            ),
            _ => None,
        }
    }
}
impl ::sr_primitives::traits::DigestItem for Log {
    type Hash = < :: sr_primitives :: generic :: DigestItem < Hash , AuthorityId , AuthoritySignature > as :: sr_primitives :: traits :: DigestItem > :: Hash ;
    type AuthorityId = < :: sr_primitives :: generic :: DigestItem < Hash , AuthorityId , AuthoritySignature > as :: sr_primitives :: traits :: DigestItem > :: AuthorityId ;
    fn as_authorities_change(&self) -> Option<&[Self::AuthorityId]> {
        self.dref().and_then(|dref| dref.as_authorities_change())
    }
    fn as_changes_trie_root(&self) -> Option<&Self::Hash> {
        self.dref().and_then(|dref| dref.as_changes_trie_root())
    }
}
impl From<::sr_primitives::generic::DigestItem<Hash, AuthorityId, AuthoritySignature>> for Log {
    /// Converts `generic::DigestItem` into `$name`. If `generic::DigestItem` represents
    /// a system item which is supported by the runtime, it is returned.
    /// Otherwise we expect a `Other` log item. Trying to convert from anything other
    /// will lead to panic in runtime, since the runtime does not supports this 'system'
    /// log item.
    #[allow(unreachable_patterns)]
    fn from(
        gen: ::sr_primitives::generic::DigestItem<Hash, AuthorityId, AuthoritySignature>,
    ) -> Self {
        match gen {
            ::sr_primitives::generic::DigestItem::ChangesTrieRoot(value) => {
                Log(InternalLog::system(system::RawLog::ChangesTrieRoot(value)))
            }
            ::sr_primitives::generic::DigestItem::AuthoritiesChange(value) => Log(
                InternalLog::consensus(consensus::RawLog::AuthoritiesChange(value)),
            ),
            _ => gen
                .as_other()
                .and_then(|value| ::sr_primitives::codec::Decode::decode(&mut &value[..]))
                .map(Log)
                .expect("not allowed to fail in runtime"),
        }
    }
}
impl ::sr_primitives::codec::Decode for Log {
    /// `generic::DigestItem` binary compatible decode.
    fn decode<I: ::sr_primitives::codec::Input>(input: &mut I) -> Option<Self> {
        let gen: ::sr_primitives::generic::DigestItem<Hash, AuthorityId, AuthoritySignature> =
            ::sr_primitives::codec::Decode::decode(input)?;
        Some(Log::from(gen))
    }
}
impl ::sr_primitives::codec::Encode for Log {
    /// `generic::DigestItem` binary compatible encode.
    fn encode(&self) -> Vec<u8> {
        match self.dref() {
            Some(dref) => dref.encode(),
            None => {
                let gen: ::sr_primitives::generic::DigestItem<
                    Hash,
                    AuthorityId,
                    AuthoritySignature,
                > = ::sr_primitives::generic::DigestItem::Other(self.0.encode());
                gen.encode()
            }
        }
    }
}
impl From<system::Log<Runtime>> for Log {
    /// Converts single module log item into `$name`.
    fn from(x: system::Log<Runtime>) -> Self {
        Log(x.into())
    }
}
impl From<system::Log<Runtime>> for InternalLog {
    /// Converts single module log item into `$internal`.
    fn from(x: system::Log<Runtime>) -> Self {
        InternalLog::system(x)
    }
}
impl From<consensus::Log<Runtime>> for Log {
    /// Converts single module log item into `$name`.
    fn from(x: consensus::Log<Runtime>) -> Self {
        Log(x.into())
    }
}
impl From<consensus::Log<Runtime>> for InternalLog {
    /// Converts single module log item into `$internal`.
    fn from(x: consensus::Log<Runtime>) -> Self {
        InternalLog::consensus(x)
    }
}
#[cfg(any(feature = "std", test))]
pub type SystemConfig = system::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type TimestampConfig = timestamp::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type ConsensusConfig = consensus::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type IndicesConfig = indices::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type BalancesConfig = balances::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
pub type SudoConfig = sudo::GenesisConfig<Runtime>;
#[cfg(any(feature = "std", test))]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct GenesisConfig {
    pub system: Option<SystemConfig>,
    pub timestamp: Option<TimestampConfig>,
    pub consensus: Option<ConsensusConfig>,
    pub indices: Option<IndicesConfig>,
    pub balances: Option<BalancesConfig>,
    pub sudo: Option<SudoConfig>,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_SERIALIZE_FOR_GenesisConfig: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for GenesisConfig {
        fn serialize<__S>(&self, __serializer: __S) -> _serde::export::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "GenesisConfig",
                false as usize + 1 + 1 + 1 + 1 + 1 + 1,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "system",
                &self.system,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "timestamp",
                &self.timestamp,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "consensus",
                &self.consensus,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "indices",
                &self.indices,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "balances",
                &self.balances,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "sudo",
                &self.sudo,
            ) {
                _serde::export::Ok(__val) => __val,
                _serde::export::Err(__err) => {
                    return _serde::export::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_GenesisConfig: () = {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for GenesisConfig {
        fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __field5,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::export::Ok(__Field::__field0),
                        1u64 => _serde::export::Ok(__Field::__field1),
                        2u64 => _serde::export::Ok(__Field::__field2),
                        3u64 => _serde::export::Ok(__Field::__field3),
                        4u64 => _serde::export::Ok(__Field::__field4),
                        5u64 => _serde::export::Ok(__Field::__field5),
                        _ => _serde::export::Err(_serde::de::Error::invalid_value(
                            _serde::de::Unexpected::Unsigned(__value),
                            &"field index 0 <= i < 6",
                        )),
                    }
                }
                fn visit_str<__E>(self, __value: &str) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "system" => _serde::export::Ok(__Field::__field0),
                        "timestamp" => _serde::export::Ok(__Field::__field1),
                        "consensus" => _serde::export::Ok(__Field::__field2),
                        "indices" => _serde::export::Ok(__Field::__field3),
                        "balances" => _serde::export::Ok(__Field::__field4),
                        "sudo" => _serde::export::Ok(__Field::__field5),
                        _ => _serde::export::Err(_serde::de::Error::unknown_field(__value, FIELDS)),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::export::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"system" => _serde::export::Ok(__Field::__field0),
                        b"timestamp" => _serde::export::Ok(__Field::__field1),
                        b"consensus" => _serde::export::Ok(__Field::__field2),
                        b"indices" => _serde::export::Ok(__Field::__field3),
                        b"balances" => _serde::export::Ok(__Field::__field4),
                        b"sudo" => _serde::export::Ok(__Field::__field5),
                        _ => {
                            let __value = &_serde::export::from_utf8_lossy(__value);
                            _serde::export::Err(_serde::de::Error::unknown_field(__value, FIELDS))
                        }
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(__deserializer: __D) -> _serde::export::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::export::PhantomData<GenesisConfig>,
                lifetime: _serde::export::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = GenesisConfig;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::export::Formatter,
                ) -> _serde::export::fmt::Result {
                    _serde::export::Formatter::write_str(__formatter, "struct GenesisConfig")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match match _serde::de::SeqAccess::next_element::<
                        Option<SystemConfig>,
                    >(&mut __seq)
                    {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct GenesisConfig with 6 elements",
                            ));
                        }
                    };
                    let __field1 = match match _serde::de::SeqAccess::next_element::<
                        Option<TimestampConfig>,
                    >(&mut __seq)
                    {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                1usize,
                                &"struct GenesisConfig with 6 elements",
                            ));
                        }
                    };
                    let __field2 = match match _serde::de::SeqAccess::next_element::<
                        Option<ConsensusConfig>,
                    >(&mut __seq)
                    {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                2usize,
                                &"struct GenesisConfig with 6 elements",
                            ));
                        }
                    };
                    let __field3 = match match _serde::de::SeqAccess::next_element::<
                        Option<IndicesConfig>,
                    >(&mut __seq)
                    {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                3usize,
                                &"struct GenesisConfig with 6 elements",
                            ));
                        }
                    };
                    let __field4 = match match _serde::de::SeqAccess::next_element::<
                        Option<BalancesConfig>,
                    >(&mut __seq)
                    {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                4usize,
                                &"struct GenesisConfig with 6 elements",
                            ));
                        }
                    };
                    let __field5 = match match _serde::de::SeqAccess::next_element::<
                        Option<SudoConfig>,
                    >(&mut __seq)
                    {
                        _serde::export::Ok(__val) => __val,
                        _serde::export::Err(__err) => {
                            return _serde::export::Err(__err);
                        }
                    } {
                        _serde::export::Some(__value) => __value,
                        _serde::export::None => {
                            return _serde::export::Err(_serde::de::Error::invalid_length(
                                5usize,
                                &"struct GenesisConfig with 6 elements",
                            ));
                        }
                    };
                    _serde::export::Ok(GenesisConfig {
                        system: __field0,
                        timestamp: __field1,
                        consensus: __field2,
                        indices: __field3,
                        balances: __field4,
                        sudo: __field5,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::export::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::export::Option<Option<SystemConfig>> =
                        _serde::export::None;
                    let mut __field1: _serde::export::Option<Option<TimestampConfig>> =
                        _serde::export::None;
                    let mut __field2: _serde::export::Option<Option<ConsensusConfig>> =
                        _serde::export::None;
                    let mut __field3: _serde::export::Option<Option<IndicesConfig>> =
                        _serde::export::None;
                    let mut __field4: _serde::export::Option<Option<BalancesConfig>> =
                        _serde::export::None;
                    let mut __field5: _serde::export::Option<Option<SudoConfig>> =
                        _serde::export::None;
                    while let _serde::export::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::export::Option::is_some(&__field0) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "system",
                                        ),
                                    );
                                }
                                __field0 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<Option<SystemConfig>>(
                                        &mut __map,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::export::Option::is_some(&__field1) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "timestamp",
                                        ),
                                    );
                                }
                                __field1 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<Option<TimestampConfig>>(
                                        &mut __map,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field2 => {
                                if _serde::export::Option::is_some(&__field2) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "consensus",
                                        ),
                                    );
                                }
                                __field2 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<Option<ConsensusConfig>>(
                                        &mut __map,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field3 => {
                                if _serde::export::Option::is_some(&__field3) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "indices",
                                        ),
                                    );
                                }
                                __field3 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<Option<IndicesConfig>>(
                                        &mut __map,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field4 => {
                                if _serde::export::Option::is_some(&__field4) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "balances",
                                        ),
                                    );
                                }
                                __field4 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<Option<BalancesConfig>>(
                                        &mut __map,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field5 => {
                                if _serde::export::Option::is_some(&__field5) {
                                    return _serde::export::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("sudo"),
                                    );
                                }
                                __field5 = _serde::export::Some(
                                    match _serde::de::MapAccess::next_value::<Option<SudoConfig>>(
                                        &mut __map,
                                    ) {
                                        _serde::export::Ok(__val) => __val,
                                        _serde::export::Err(__err) => {
                                            return _serde::export::Err(__err);
                                        }
                                    },
                                );
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::export::Some(__field0) => __field0,
                        _serde::export::None => {
                            match _serde::private::de::missing_field("system") {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::export::Some(__field1) => __field1,
                        _serde::export::None => {
                            match _serde::private::de::missing_field("timestamp") {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::export::Some(__field2) => __field2,
                        _serde::export::None => {
                            match _serde::private::de::missing_field("consensus") {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::export::Some(__field3) => __field3,
                        _serde::export::None => match _serde::private::de::missing_field("indices")
                        {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        },
                    };
                    let __field4 = match __field4 {
                        _serde::export::Some(__field4) => __field4,
                        _serde::export::None => {
                            match _serde::private::de::missing_field("balances") {
                                _serde::export::Ok(__val) => __val,
                                _serde::export::Err(__err) => {
                                    return _serde::export::Err(__err);
                                }
                            }
                        }
                    };
                    let __field5 = match __field5 {
                        _serde::export::Some(__field5) => __field5,
                        _serde::export::None => match _serde::private::de::missing_field("sudo") {
                            _serde::export::Ok(__val) => __val,
                            _serde::export::Err(__err) => {
                                return _serde::export::Err(__err);
                            }
                        },
                    };
                    _serde::export::Ok(GenesisConfig {
                        system: __field0,
                        timestamp: __field1,
                        consensus: __field2,
                        indices: __field3,
                        balances: __field4,
                        sudo: __field5,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &[
                "system",
                "timestamp",
                "consensus",
                "indices",
                "balances",
                "sudo",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "GenesisConfig",
                FIELDS,
                __Visitor {
                    marker: _serde::export::PhantomData::<GenesisConfig>,
                    lifetime: _serde::export::PhantomData,
                },
            )
        }
    }
};
#[cfg(any(feature = "std", test))]
impl ::sr_primitives::BuildStorage for GenesisConfig {
    fn assimilate_storage(
        self,
        top: &mut ::sr_primitives::StorageOverlay,
        children: &mut ::sr_primitives::ChildrenStorageOverlay,
    ) -> ::std::result::Result<(), String> {
        if let Some(extra) = self.system {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.timestamp {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.consensus {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.indices {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.balances {
            extra.assimilate_storage(top, children)?;
        }
        if let Some(extra) = self.sudo {
            extra.assimilate_storage(top, children)?;
        }
        Ok(())
    }
}
trait InherentDataExt {
    fn create_extrinsics(
        &self,
    ) -> ::srml_support::inherent::Vec<<Block as ::srml_support::inherent::BlockT>::Extrinsic>;
    fn check_extrinsics(&self, block: &Block) -> ::srml_support::inherent::CheckInherentsResult;
}
impl InherentDataExt for ::srml_support::inherent::InherentData {
    fn create_extrinsics(
        &self,
    ) -> ::srml_support::inherent::Vec<<Block as ::srml_support::inherent::BlockT>::Extrinsic> {
        use srml_support::inherent::ProvideInherent;
        let mut inherents = Vec::new();
        if let Some(inherent) = Timestamp::create_inherent(self) {
            inherents.push(UncheckedExtrinsic::new_unsigned(Call::Timestamp(inherent)));
        }
        if let Some(inherent) = Consensus::create_inherent(self) {
            inherents.push(UncheckedExtrinsic::new_unsigned(Call::Consensus(inherent)));
        }
        inherents
    }
    fn check_extrinsics(&self, block: &Block) -> ::srml_support::inherent::CheckInherentsResult {
        use srml_support::inherent::{IsFatalError, ProvideInherent};
        let mut result = ::srml_support::inherent::CheckInherentsResult::new();
        for xt in block.extrinsics() {
            if ::srml_support::inherent::Extrinsic::is_signed(xt).unwrap_or(false) {
                break;
            }
            match xt.function {
                Call::Timestamp(ref call) => {
                    if let Err(e) = Timestamp::check_inherent(call, self) {
                        result
                            .put_error(Timestamp::INHERENT_IDENTIFIER, &e)
                            .expect("There is only one fatal error; qed");
                        if e.is_fatal_error() {
                            return result;
                        }
                    }
                }
                _ => {}
            }
            match xt.function {
                Call::Consensus(ref call) => {
                    if let Err(e) = Consensus::check_inherent(call, self) {
                        result
                            .put_error(Consensus::INHERENT_IDENTIFIER, &e)
                            .expect("There is only one fatal error; qed");
                        if e.is_fatal_error() {
                            return result;
                        }
                    }
                }
                _ => {}
            }
        }
        result
    }
}
/// The type used as a helper for interpreting the sender of transactions.
type Context = system::ChainContext<Runtime>;
/// The address format for describing accounts.
type Address = <Indices as StaticLookup>::Source;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256, Log>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
    generic::UncheckedMortalCompactExtrinsic<Address, Nonce, Call, AccountSignature>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Nonce, Call>;
/// Executive: handles dispatch to the various modules.
pub type Executive = executive::Executive<Runtime, Block, Context, Balances, AllModules>;
#[doc(hidden)]
mod sr_api_hidden_includes_IMPL_RUNTIME_APIS {
    pub extern crate client as sr_api_client;
}
pub struct RuntimeApi {}
/// Implements all runtime apis for the client side.
#[cfg(any(feature = "std", test))]
pub struct RuntimeApiImpl < C : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: CallRuntimeAt < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > + 'static > { call : & 'static C , commit_on_success : :: std :: cell :: RefCell < bool > , initialized_block : :: std :: cell :: RefCell < Option < self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: BlockId < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > > > , changes : :: std :: cell :: RefCell < self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: OverlayedChanges > , }
#[cfg(any(feature = "std", test))]
unsafe impl < C : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: CallRuntimeAt < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > > Send for RuntimeApiImpl < C > { }
#[cfg(any(feature = "std", test))]
unsafe impl < C : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: CallRuntimeAt < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > > Sync for RuntimeApiImpl < C > { }
#[cfg(any(feature = "std", test))]
impl < C : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: CallRuntimeAt < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > > self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: ApiExt < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > for RuntimeApiImpl < C > { fn map_api_result < F : FnOnce ( & Self ) -> :: std :: result :: Result < R , E > , R , E > ( & self , map_call : F ) -> :: std :: result :: Result < R , E > where Self : Sized { * self . commit_on_success . borrow_mut ( ) = false ; let res = map_call ( self ) ; * self . commit_on_success . borrow_mut ( ) = true ; self . commit_on_ok ( & res ) ; res } fn runtime_version_at ( & self , at : & self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: BlockId < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > ) -> self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: error :: Result < self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: RuntimeVersion > { self . call . runtime_version_at ( at ) } }
#[cfg(any(feature = "std", test))]
impl < C : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: CallRuntimeAt < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > + 'static > self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: ConstructRuntimeApi < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock , C > for RuntimeApi { type RuntimeApi = RuntimeApiImpl < C > ; fn construct_runtime_api < 'a > ( call : & 'a C ) -> self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: ApiRef < 'a , Self :: RuntimeApi > { RuntimeApiImpl { call : unsafe { :: std :: mem :: transmute ( call ) } , commit_on_success : true . into ( ) , initialized_block : None . into ( ) , changes : Default :: default ( ) , } . into ( ) } }
#[cfg(any(feature = "std", test))]
impl < C : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: CallRuntimeAt < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > > RuntimeApiImpl < C > { fn call_api_at < R : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Encode + self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Decode + PartialEq , F : FnOnce ( & C , & mut self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: OverlayedChanges , & mut Option < self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: BlockId < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > > ) -> self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: error :: Result < self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: NativeOrEncoded < R > > > ( & self , call_api_at : F ) -> self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: error :: Result < self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: NativeOrEncoded < R > > { let res = unsafe { call_api_at ( & self . call , & mut * self . changes . borrow_mut ( ) , & mut * self . initialized_block . borrow_mut ( ) ) } ; self . commit_on_ok ( & res ) ; res } fn commit_on_ok < R , E > ( & self , res : & :: std :: result :: Result < R , E > ) { if * self . commit_on_success . borrow ( ) { if res . is_err ( ) { self . changes . borrow_mut ( ) . discard_prospective ( ) ; } else { self . changes . borrow_mut ( ) . commit_prospective ( ) ; } } } }
impl runtime_api::runtime_decl_for_Core::Core<Block> for Runtime {
    fn version() -> RuntimeVersion {
        VERSION
    }
    fn execute_block(block: Block) {
        Executive::execute_block(block)
    }
    fn initialize_block(header: &<Block as BlockT>::Header) {
        Executive::initialize_block(header)
    }
    fn authorities() -> Vec<AuthorityId> {
        {
            ::std::rt::begin_panic(
                "Deprecated, please use `AuthoritiesApi`.",
                &("runtime/src/lib.rs", 259u32, 4u32),
            )
        }
    }
}
impl runtime_api::runtime_decl_for_Metadata::Metadata<Block> for Runtime {
    fn metadata() -> OpaqueMetadata {
        Runtime::metadata().into()
    }
}
impl block_builder_api::runtime_decl_for_BlockBuilder::BlockBuilder<Block> for Runtime {
    fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyResult {
        Executive::apply_extrinsic(extrinsic)
    }
    fn finalize_block() -> <Block as BlockT>::Header {
        Executive::finalize_block()
    }
    fn inherent_extrinsics(data: InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
        data.create_extrinsics()
    }
    fn check_inherents(block: Block, data: InherentData) -> CheckInherentsResult {
        data.check_extrinsics(&block)
    }
    fn random_seed() -> <Block as BlockT>::Hash {
        System::random_seed()
    }
}
impl runtime_api::runtime_decl_for_TaggedTransactionQueue::TaggedTransactionQueue<Block>
    for Runtime
{
    fn validate_transaction(tx: <Block as BlockT>::Extrinsic) -> TransactionValidity {
        Executive::validate_transaction(tx)
    }
}
impl consensus_aura::runtime_decl_for_AuraApi::AuraApi<Block> for Runtime {
    fn slot_duration() -> u64 {
        Aura::slot_duration()
    }
}
impl offchain_primitives::runtime_decl_for_OffchainWorkerApi::OffchainWorkerApi<Block> for Runtime {
    fn offchain_worker(n: NumberFor<Block>) {
        Executive::offchain_worker(n)
    }
}
impl consensus_authorities::runtime_decl_for_AuthoritiesApi::AuthoritiesApi<Block> for Runtime {
    fn authorities() -> Vec<AuthorityId> {
        Consensus::authorities()
    }
}
#[cfg(any(feature = "std", test))]
impl < RuntimeApiImplCall : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: CallRuntimeAt < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > + 'static > runtime_api :: Core < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > for RuntimeApiImpl < RuntimeApiImplCall > { fn Core_version_runtime_api_impl ( & self , at : & self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: BlockId < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > , context : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: ExecutionContext , params : Option < ( ) > , params_encoded : Vec < u8 > ) -> self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: error :: Result < self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: NativeOrEncoded < RuntimeVersion > > { self . call_api_at ( | call_runtime_at , changes , initialized_block | { runtime_api :: runtime_decl_for_Core :: version_call_api_at ( call_runtime_at , at , params_encoded , changes , initialized_block , params . map ( | p | { runtime_api :: runtime_decl_for_Core :: version_native_call_generator :: < Runtime , < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock , Block > ( ) } ) , context ) } ) } fn Core_execute_block_runtime_api_impl ( & self , at : & self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: BlockId < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > , context : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: ExecutionContext , params : Option < ( < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock ) > , params_encoded : Vec < u8 > ) -> self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: error :: Result < self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: NativeOrEncoded < ( ) > > { self . call_api_at ( | call_runtime_at , changes , initialized_block | { runtime_api :: runtime_decl_for_Core :: execute_block_call_api_at ( call_runtime_at , at , params_encoded , changes , initialized_block , params . map ( | p | { runtime_api :: runtime_decl_for_Core :: execute_block_native_call_generator :: < Runtime , < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock , Block > ( p ) } ) , context ) } ) } fn Core_initialize_block_runtime_api_impl ( & self , at : & self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: BlockId < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > , context : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: ExecutionContext , params : Option < ( & < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock as BlockT > :: Header ) > , params_encoded : Vec < u8 > ) -> self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: error :: Result < self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: NativeOrEncoded < ( ) > > { self . call_api_at ( | call_runtime_at , changes , initialized_block | { runtime_api :: runtime_decl_for_Core :: initialize_block_call_api_at ( call_runtime_at , at , params_encoded , changes , initialized_block , params . map ( | p | { runtime_api :: runtime_decl_for_Core :: initialize_block_native_call_generator :: < Runtime , < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock , Block > ( p ) } ) , context ) } ) } fn Core_authorities_runtime_api_impl ( & self , at : & self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: BlockId < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > , context : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: ExecutionContext , params : Option < ( ) > , params_encoded : Vec < u8 > ) -> self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: error :: Result < self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: NativeOrEncoded < Vec < AuthorityId > > > { self . call_api_at ( | call_runtime_at , changes , initialized_block | { runtime_api :: runtime_decl_for_Core :: authorities_call_api_at ( call_runtime_at , at , params_encoded , changes , initialized_block , params . map ( | p | { runtime_api :: runtime_decl_for_Core :: authorities_native_call_generator :: < Runtime , < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock , Block > ( ) } ) , context ) } ) } }
#[cfg(any(feature = "std", test))]
impl < RuntimeApiImplCall : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: CallRuntimeAt < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > + 'static > runtime_api :: Metadata < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > for RuntimeApiImpl < RuntimeApiImplCall > { fn Metadata_metadata_runtime_api_impl ( & self , at : & self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: BlockId < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > , context : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: ExecutionContext , params : Option < ( ) > , params_encoded : Vec < u8 > ) -> self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: error :: Result < self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: NativeOrEncoded < OpaqueMetadata > > { self . call_api_at ( | call_runtime_at , changes , initialized_block | { runtime_api :: runtime_decl_for_Metadata :: metadata_call_api_at ( call_runtime_at , at , params_encoded , changes , initialized_block , params . map ( | p | { runtime_api :: runtime_decl_for_Metadata :: metadata_native_call_generator :: < Runtime , < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock , Block > ( ) } ) , context ) } ) } }
#[cfg(any(feature = "std", test))]
impl < RuntimeApiImplCall : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: CallRuntimeAt < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > + 'static > block_builder_api :: BlockBuilder < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > for RuntimeApiImpl < RuntimeApiImplCall > { fn BlockBuilder_apply_extrinsic_runtime_api_impl ( & self , at : & self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: BlockId < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > , context : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: ExecutionContext , params : Option < ( < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock as BlockT > :: Extrinsic ) > , params_encoded : Vec < u8 > ) -> self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: error :: Result < self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: NativeOrEncoded < ApplyResult > > { self . call_api_at ( | call_runtime_at , changes , initialized_block | { block_builder_api :: runtime_decl_for_BlockBuilder :: apply_extrinsic_call_api_at ( call_runtime_at , at , params_encoded , changes , initialized_block , params . map ( | p | { block_builder_api :: runtime_decl_for_BlockBuilder :: apply_extrinsic_native_call_generator :: < Runtime , < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock , Block > ( p ) } ) , context ) } ) } fn BlockBuilder_finalize_block_runtime_api_impl ( & self , at : & self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: BlockId < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > , context : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: ExecutionContext , params : Option < ( ) > , params_encoded : Vec < u8 > ) -> self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: error :: Result < self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: NativeOrEncoded < < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock as BlockT > :: Header > > { self . call_api_at ( | call_runtime_at , changes , initialized_block | { block_builder_api :: runtime_decl_for_BlockBuilder :: finalize_block_call_api_at ( call_runtime_at , at , params_encoded , changes , initialized_block , params . map ( | p | { block_builder_api :: runtime_decl_for_BlockBuilder :: finalize_block_native_call_generator :: < Runtime , < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock , Block > ( ) } ) , context ) } ) } fn BlockBuilder_inherent_extrinsics_runtime_api_impl ( & self , at : & self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: BlockId < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > , context : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: ExecutionContext , params : Option < ( InherentData ) > , params_encoded : Vec < u8 > ) -> self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: error :: Result < self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: NativeOrEncoded < Vec < < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock as BlockT > :: Extrinsic > > > { self . call_api_at ( | call_runtime_at , changes , initialized_block | { block_builder_api :: runtime_decl_for_BlockBuilder :: inherent_extrinsics_call_api_at ( call_runtime_at , at , params_encoded , changes , initialized_block , params . map ( | p | { block_builder_api :: runtime_decl_for_BlockBuilder :: inherent_extrinsics_native_call_generator :: < Runtime , < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock , Block > ( p ) } ) , context ) } ) } fn BlockBuilder_check_inherents_runtime_api_impl ( & self , at : & self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: BlockId < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > , context : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: ExecutionContext , params : Option < ( < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock , InherentData ) > , params_encoded : Vec < u8 > ) -> self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: error :: Result < self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: NativeOrEncoded < CheckInherentsResult > > { self . call_api_at ( | call_runtime_at , changes , initialized_block | { block_builder_api :: runtime_decl_for_BlockBuilder :: check_inherents_call_api_at ( call_runtime_at , at , params_encoded , changes , initialized_block , params . map ( | p | { block_builder_api :: runtime_decl_for_BlockBuilder :: check_inherents_native_call_generator :: < Runtime , < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock , Block > ( p . 0 , p . 1 ) } ) , context ) } ) } fn BlockBuilder_random_seed_runtime_api_impl ( & self , at : & self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: BlockId < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > , context : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: ExecutionContext , params : Option < ( ) > , params_encoded : Vec < u8 > ) -> self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: error :: Result < self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: NativeOrEncoded < < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock as BlockT > :: Hash > > { self . call_api_at ( | call_runtime_at , changes , initialized_block | { block_builder_api :: runtime_decl_for_BlockBuilder :: random_seed_call_api_at ( call_runtime_at , at , params_encoded , changes , initialized_block , params . map ( | p | { block_builder_api :: runtime_decl_for_BlockBuilder :: random_seed_native_call_generator :: < Runtime , < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock , Block > ( ) } ) , context ) } ) } }
#[cfg(any(feature = "std", test))]
impl < RuntimeApiImplCall : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: CallRuntimeAt < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > + 'static > runtime_api :: TaggedTransactionQueue < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > for RuntimeApiImpl < RuntimeApiImplCall > { fn TaggedTransactionQueue_validate_transaction_runtime_api_impl ( & self , at : & self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: BlockId < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > , context : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: ExecutionContext , params : Option < ( < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock as BlockT > :: Extrinsic ) > , params_encoded : Vec < u8 > ) -> self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: error :: Result < self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: NativeOrEncoded < TransactionValidity > > { self . call_api_at ( | call_runtime_at , changes , initialized_block | { runtime_api :: runtime_decl_for_TaggedTransactionQueue :: validate_transaction_call_api_at ( call_runtime_at , at , params_encoded , changes , initialized_block , params . map ( | p | { runtime_api :: runtime_decl_for_TaggedTransactionQueue :: validate_transaction_native_call_generator :: < Runtime , < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock , Block > ( p ) } ) , context ) } ) } }
#[cfg(any(feature = "std", test))]
impl < RuntimeApiImplCall : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: CallRuntimeAt < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > + 'static > consensus_aura :: AuraApi < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > for RuntimeApiImpl < RuntimeApiImplCall > { fn AuraApi_slot_duration_runtime_api_impl ( & self , at : & self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: BlockId < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > , context : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: ExecutionContext , params : Option < ( ) > , params_encoded : Vec < u8 > ) -> self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: error :: Result < self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: NativeOrEncoded < u64 > > { self . call_api_at ( | call_runtime_at , changes , initialized_block | { consensus_aura :: runtime_decl_for_AuraApi :: slot_duration_call_api_at ( call_runtime_at , at , params_encoded , changes , initialized_block , params . map ( | p | { consensus_aura :: runtime_decl_for_AuraApi :: slot_duration_native_call_generator :: < Runtime , < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock , Block > ( ) } ) , context ) } ) } }
#[cfg(any(feature = "std", test))]
impl < RuntimeApiImplCall : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: CallRuntimeAt < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > + 'static > offchain_primitives :: OffchainWorkerApi < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > for RuntimeApiImpl < RuntimeApiImplCall > { fn OffchainWorkerApi_offchain_worker_runtime_api_impl ( & self , at : & self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: BlockId < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > , context : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: ExecutionContext , params : Option < ( NumberFor < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > ) > , params_encoded : Vec < u8 > ) -> self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: error :: Result < self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: NativeOrEncoded < ( ) > > { self . call_api_at ( | call_runtime_at , changes , initialized_block | { offchain_primitives :: runtime_decl_for_OffchainWorkerApi :: offchain_worker_call_api_at ( call_runtime_at , at , params_encoded , changes , initialized_block , params . map ( | p | { offchain_primitives :: runtime_decl_for_OffchainWorkerApi :: offchain_worker_native_call_generator :: < Runtime , < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock , Block > ( p ) } ) , context ) } ) } }
#[cfg(any(feature = "std", test))]
impl < RuntimeApiImplCall : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: CallRuntimeAt < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > + 'static > consensus_authorities :: AuthoritiesApi < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > for RuntimeApiImpl < RuntimeApiImplCall > { fn AuthoritiesApi_authorities_runtime_api_impl ( & self , at : & self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: BlockId < < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock > , context : self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: ExecutionContext , params : Option < ( ) > , params_encoded : Vec < u8 > ) -> self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: error :: Result < self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: NativeOrEncoded < Vec < AuthorityId > > > { self . call_api_at ( | call_runtime_at , changes , initialized_block | { consensus_authorities :: runtime_decl_for_AuthoritiesApi :: authorities_call_api_at ( call_runtime_at , at , params_encoded , changes , initialized_block , params . map ( | p | { consensus_authorities :: runtime_decl_for_AuthoritiesApi :: authorities_native_call_generator :: < Runtime , < Runtime as self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: GetNodeBlockType > :: NodeBlock , Block > ( ) } ) , context ) } ) } }
const RUNTIME_API_VERSIONS:
    self::sr_api_hidden_includes_IMPL_RUNTIME_APIS::sr_api_client::runtime_api::ApisVec =
    ::std::borrow::Cow::Borrowed(&[
        (
            runtime_api::runtime_decl_for_Core::ID,
            runtime_api::runtime_decl_for_Core::VERSION,
        ),
        (
            runtime_api::runtime_decl_for_Metadata::ID,
            runtime_api::runtime_decl_for_Metadata::VERSION,
        ),
        (
            block_builder_api::runtime_decl_for_BlockBuilder::ID,
            block_builder_api::runtime_decl_for_BlockBuilder::VERSION,
        ),
        (
            runtime_api::runtime_decl_for_TaggedTransactionQueue::ID,
            runtime_api::runtime_decl_for_TaggedTransactionQueue::VERSION,
        ),
        (
            consensus_aura::runtime_decl_for_AuraApi::ID,
            consensus_aura::runtime_decl_for_AuraApi::VERSION,
        ),
        (
            offchain_primitives::runtime_decl_for_OffchainWorkerApi::ID,
            offchain_primitives::runtime_decl_for_OffchainWorkerApi::VERSION,
        ),
        (
            consensus_authorities::runtime_decl_for_AuthoritiesApi::ID,
            consensus_authorities::runtime_decl_for_AuthoritiesApi::VERSION,
        ),
    ]);
pub mod api {
    use super::*;
    #[cfg(feature = "std")]
    pub fn dispatch(method: &str, mut data: &[u8]) -> Option<Vec<u8>> {
        match method {
            "Core_version" => Some({
                #[allow(deprecated)]
                let output =
                    <Runtime as runtime_api::runtime_decl_for_Core::Core<Block>>::version();
                self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Encode :: encode ( & output )
            }),
            "Core_execute_block" => Some({
                let block : Block = match self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Decode :: decode ( & mut data ) { Some ( input ) => input , None => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Bad input data provided to " ] , & match ( & "execute_block" , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "runtime/src/lib.rs" , 244u32 , 1u32 ) ) } } ;
                #[allow(deprecated)]
                let output =
                    <Runtime as runtime_api::runtime_decl_for_Core::Core<Block>>::execute_block(
                        block,
                    );
                self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Encode :: encode ( & output )
            }),
            "Core_initialize_block" => {
                Some({
                    let header : < Block as BlockT > :: Header = match self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Decode :: decode ( & mut data ) { Some ( input ) => input , None => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Bad input data provided to " ] , & match ( & "initialize_block" , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "runtime/src/lib.rs" , 244u32 , 1u32 ) ) } } ;
                    # [ allow ( deprecated ) ] let output = < Runtime as runtime_api :: runtime_decl_for_Core :: Core < Block > > :: initialize_block ( & header ) ;
                    self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Encode :: encode ( & output )
                })
            }
            "Core_authorities" => Some({
                #[allow(deprecated)]
                let output =
                    <Runtime as runtime_api::runtime_decl_for_Core::Core<Block>>::authorities();
                self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Encode :: encode ( & output )
            }),
            "Metadata_metadata" => Some({
                #[allow(deprecated)]
                let output =
                    <Runtime as runtime_api::runtime_decl_for_Metadata::Metadata<Block>>::metadata(
                    );
                self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Encode :: encode ( & output )
            }),
            "BlockBuilder_apply_extrinsic" => {
                Some({
                    let extrinsic : < Block as BlockT > :: Extrinsic = match self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Decode :: decode ( & mut data ) { Some ( input ) => input , None => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Bad input data provided to " ] , & match ( & "apply_extrinsic" , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "runtime/src/lib.rs" , 244u32 , 1u32 ) ) } } ;
                    # [ allow ( deprecated ) ] let output = < Runtime as block_builder_api :: runtime_decl_for_BlockBuilder :: BlockBuilder < Block > > :: apply_extrinsic ( extrinsic ) ;
                    self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Encode :: encode ( & output )
                })
            }
            "BlockBuilder_finalize_block" => {
                Some({
                    # [ allow ( deprecated ) ] let output = < Runtime as block_builder_api :: runtime_decl_for_BlockBuilder :: BlockBuilder < Block > > :: finalize_block ( ) ;
                    self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Encode :: encode ( & output )
                })
            }
            "BlockBuilder_inherent_extrinsics" => {
                Some({
                    let data : InherentData = match self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Decode :: decode ( & mut data ) { Some ( input ) => input , None => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Bad input data provided to " ] , & match ( & "inherent_extrinsics" , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "runtime/src/lib.rs" , 244u32 , 1u32 ) ) } } ;
                    # [ allow ( deprecated ) ] let output = < Runtime as block_builder_api :: runtime_decl_for_BlockBuilder :: BlockBuilder < Block > > :: inherent_extrinsics ( data ) ;
                    self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Encode :: encode ( & output )
                })
            }
            "BlockBuilder_check_inherents" => {
                Some({
                    let block : Block = match self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Decode :: decode ( & mut data ) { Some ( input ) => input , None => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Bad input data provided to " ] , & match ( & "check_inherents" , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "runtime/src/lib.rs" , 244u32 , 1u32 ) ) } } ;
                    let data : InherentData = match self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Decode :: decode ( & mut data ) { Some ( input ) => input , None => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Bad input data provided to " ] , & match ( & "check_inherents" , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "runtime/src/lib.rs" , 244u32 , 1u32 ) ) } } ;
                    # [ allow ( deprecated ) ] let output = < Runtime as block_builder_api :: runtime_decl_for_BlockBuilder :: BlockBuilder < Block > > :: check_inherents ( block , data ) ;
                    self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Encode :: encode ( & output )
                })
            }
            "BlockBuilder_random_seed" => {
                Some({
                    # [ allow ( deprecated ) ] let output = < Runtime as block_builder_api :: runtime_decl_for_BlockBuilder :: BlockBuilder < Block > > :: random_seed ( ) ;
                    self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Encode :: encode ( & output )
                })
            }
            "TaggedTransactionQueue_validate_transaction" => Some({
                let tx : < Block as BlockT > :: Extrinsic = match self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Decode :: decode ( & mut data ) { Some ( input ) => input , None => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Bad input data provided to " ] , & match ( & "validate_transaction" , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "runtime/src/lib.rs" , 244u32 , 1u32 ) ) } } ;
                # [ allow ( deprecated ) ] let output = < Runtime as runtime_api :: runtime_decl_for_TaggedTransactionQueue :: TaggedTransactionQueue < Block > > :: validate_transaction ( tx ) ;
                self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Encode :: encode ( & output )
            }),
            "AuraApi_slot_duration" => Some({
                #[allow(deprecated)]
                let output = <Runtime as consensus_aura::runtime_decl_for_AuraApi::AuraApi<
                    Block,
                >>::slot_duration();
                self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Encode :: encode ( & output )
            }),
            "OffchainWorkerApi_offchain_worker" => Some({
                let n : NumberFor < Block > = match self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Decode :: decode ( & mut data ) { Some ( input ) => input , None => { :: std :: rt :: begin_panic_fmt ( & :: core :: fmt :: Arguments :: new_v1 ( & [ "Bad input data provided to " ] , & match ( & "offchain_worker" , ) { ( arg0 , ) => [ :: core :: fmt :: ArgumentV1 :: new ( arg0 , :: core :: fmt :: Display :: fmt ) ] , } ) , & ( "runtime/src/lib.rs" , 244u32 , 1u32 ) ) } } ;
                # [ allow ( deprecated ) ] let output = < Runtime as offchain_primitives :: runtime_decl_for_OffchainWorkerApi :: OffchainWorkerApi < Block > > :: offchain_worker ( n ) ;
                self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Encode :: encode ( & output )
            }),
            "AuthoritiesApi_authorities" => Some({
                # [ allow ( deprecated ) ] let output = < Runtime as consensus_authorities :: runtime_decl_for_AuthoritiesApi :: AuthoritiesApi < Block > > :: authorities ( ) ;
                self :: sr_api_hidden_includes_IMPL_RUNTIME_APIS :: sr_api_client :: runtime_api :: Encode :: encode ( & output )
            }),
            _ => None,
        }
    }
}
