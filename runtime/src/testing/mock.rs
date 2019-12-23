use crate::testing::types::*;
use crate::example;

use sp_core::H256;
use sp_runtime::{
    testing::{Header, TestXt, UintAuthorityId},
    traits::{IdentityLookup, BlakeTwo256},
};
use frame_support::{impl_outer_origin, impl_outer_dispatch};
use crate::submitter::TransactionSubmitter;

pub type Extrinsic = TestXt<Call, ()>;
pub type SubmitTransaction = TransactionSubmitter<UintAuthorityId, Call, Extrinsic>;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TestRuntime;

impl system::offchain::CreateTransaction<TestRuntime, Extrinsic> for Call {
    type Signature = UintAuthorityId;

    fn create_transaction<F: system::offchain::Signer<AccountId, Self::Signature>>(
        call: Call,
        account: AccountId,
        _index: AccountId,
    ) -> Option<(Call, <Extrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload)> {
        let extra = ();
        Some((call, (account, extra)))
    }
}

impl system::Trait for TestRuntime {
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
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
