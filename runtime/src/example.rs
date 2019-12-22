use crate::submitter::SubmitAndSignTransaction;

use sp_std::prelude::*;
use frame_support::{decl_event, decl_module, decl_storage, dispatch::DispatchResult, debug};
use system::{ensure_signed, IsDeadAccount};
use sp_runtime::app_crypto::{RuntimeAppPublic, AppPublic};

pub trait Trait: timestamp::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type Call: From<Call<Self>>;

	type SubmitTransaction: SubmitAndSignTransaction<Self, <Self as Trait>::Call>;
	type KeyType: RuntimeAppPublic + AppPublic + From<Self::AccountId> + Into<Self::AccountId> + Clone; // IdentifyAccount<AccountId = Self::AccountId>
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as system::Trait>::AccountId,
	{
		Mint(AccountId),
		Offchain(AccountId),
	}
);

decl_storage! {
	trait Store for Module<T: Trait> as Example {
		pub Something get(fn something) config(): Vec<()> = vec![];
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		fn offchain_worker(_now: T::BlockNumber) {
			debug::RuntimeLogger::init();
			Self::offchain();
		}

		pub fn mint(origin) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::deposit_event(RawEvent::Mint(who));
			Ok(())
		}

		pub fn submit_from_offchain(origin, _data: Option<u64>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::deposit_event(RawEvent::Offchain(who));
			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
	pub fn offchain() {
		// If uncomment this - thread will panic with stack overflow
		let _key = Self::authority_id();
		debug::warn!("Start mint submit");

		let call = Call::submit_from_offchain(Some(12_u64));
		let res = T::SubmitTransaction::submit_signed(call); // , key.unwrap().into()
		debug::warn!("Finished mint: {:?}", res);
	}

	pub fn authority_id() -> Option<T::AccountId> {
		let _local_keys = T::KeyType::all().iter().map(
			|i| (*i).clone().into()
		).collect::<Vec<T::AccountId>>();

		None
	}
}

impl<T: Trait> IsDeadAccount<T::AccountId> for Module<T> {
	fn is_dead_account(_who: &T::AccountId) -> bool {
		false
	}
}
