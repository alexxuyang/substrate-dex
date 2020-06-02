use crate::Trait;
use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
use sp_core::H256;
// The testing primitives are very useful for avoiding having to work with signatures
// or public keys. `u64` is used as the `AccountId` and no `Signature`s are required.
use sp_runtime::{
	Perbill,
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

use system as frame_system;
use balances as pallet_balances;

impl_outer_origin! {
	pub enum Origin for Test  where system = frame_system {}
}

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}

impl frame_system::Trait for Test {
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Call = ();
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
    type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ();
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type ModuleToIndex = ();
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
	pub const PriceFactor: u128 = 100_000_000;
	pub const BlocksPerDay: u32 = 10;
	pub const OpenedOrdersArrayCap: u8 = 20;
	pub const ClosedOrdersArrayCap: u8 = 100;
}

impl pallet_balances::Trait for Test {
	type Balance = u128;
	type DustRemoval = ();
	type Event = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
}

impl Trait for Test {
	type Event = ();
	type Price = u128;
	type PriceFactor = PriceFactor;
	type BlocksPerDay = BlocksPerDay;
	type OpenedOrdersArrayCap = OpenedOrdersArrayCap;
	type ClosedOrdersArrayCap = ClosedOrdersArrayCap;
}

impl token::Trait for Test {
	type Event = ();
}

type System = frame_system::Module<Test>;
