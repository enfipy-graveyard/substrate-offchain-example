use crate::submitter::TransactionSubmitter;
use crate::testing::{authority::UintAuthorityId, types::*};
use crate::example;

use sp_core::H256;
use sp_runtime::{
    testing::{Header, TestXt},
    traits::{IdentityLookup, BlakeTwo256, Verify},
};
use frame_support::{impl_outer_origin, impl_outer_dispatch};

pub type Extrinsic = TestXt<Call, ()>;
pub type SubmitTransaction = TransactionSubmitter<UintAuthorityId, Call, Extrinsic>;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TestRuntime;

impl system::offchain::CreateTransaction<TestRuntime, Extrinsic> for Call {
	type Public = <UintAuthorityId as Verify>::Signer;
    type Signature = UintAuthorityId;

    fn create_transaction<F: system::offchain::Signer<Self::Public, Self::Signature>>(
        call: Call,
		_public: Self::Public,
        account: Self::Public,
        _index: AccountIndex,
    ) -> Option<(Call, <Extrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload)> {
        let extra = ();
        Some((call, (account.into(), extra)))
    }
}

impl system::Trait for TestRuntime {
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = UintAuthorityId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
	type ModuleToIndex = ();
}

impl example::Trait for TestRuntime {
    type Event = ();
    type Call = Call;
    type SubmitTransaction = SubmitTransaction;
}

impl_outer_origin!{
	pub enum Origin for TestRuntime {}
}

impl_outer_dispatch! {
	pub enum Call for TestRuntime where origin: Origin {
		example::Example,
	}
}

pub type System = system::Module<TestRuntime>;
pub type Example = example::Module<TestRuntime>;
