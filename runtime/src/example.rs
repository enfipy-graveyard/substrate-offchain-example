use crate::submitter::SubmitAndSignTransaction;

use frame_support::{debug, decl_event, decl_module, decl_storage, dispatch::DispatchResult};
use sp_std::prelude::*;
use system::ensure_signed;

pub trait Trait: timestamp::Trait {
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
	type Call: From<Call<Self>>;

	type SubmitTransaction: SubmitAndSignTransaction<Self, <Self as Trait>::Call>;
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
		pub AuthorizedAccounts get(fn authorized_accounts) config(): Vec<T::AccountId> = vec![];
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
		let accounts = Self::authorized_accounts();
		let key = T::SubmitTransaction::get_local_keys().iter().find_map(|i| {
			if accounts.contains(&i.0) {
				Some(i.1.clone())
			} else {
				None
			}
		});
		if key.is_none() {
			debug::warn!("No authority account to submit mint");
			return;
		}
		debug::warn!("Start mint submission logic");

		let call = Call::submit_from_offchain(Some(12_u64));
		let res = T::SubmitTransaction::sign_and_submit(call, key.unwrap());
		debug::warn!("Finished mint: {:?}", res);
	}
}
