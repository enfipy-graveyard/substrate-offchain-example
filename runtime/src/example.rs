use crate::submitter::SubmitAndSignTransaction;

use frame_support::{debug, decl_event, decl_module, decl_storage, dispatch::DispatchResult};
use sp_runtime::offchain::http;
use sp_std::prelude::*;
use system::ensure_signed;

pub mod crypto {
	use sp_core::crypto::KeyTypeId;
	pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"mint");

	use sp_runtime::app_crypto::{app_crypto, sr25519};
	app_crypto!(sr25519, KEY_TYPE);
}

pub trait Trait: system::Trait {
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
			debug::warn!("No authorized account");
			return;
		}
		debug::warn!("Start logic");

		if let Ok(json) = Self::fetch_with_delay() {
			debug::warn!("Fetched data: {}", core::str::from_utf8(&json).unwrap());
		} else {
			debug::warn!("Error fetching with delay.");
			return
		}
		let call = Call::submit_from_offchain(Some(12_u64));
		let res = T::SubmitTransaction::sign_and_submit(call, key.unwrap());
		debug::warn!("Finished: {:?}", res);
	}

	pub fn fetch_with_delay() -> Result<Vec<u8>, http::Error> {
		let pending = http::Request::get(
			"http://www.mocky.io/v2/5e0006ca2f0000780013b267?mocky-delay=3000ms",
		)
		.send()
		.map_err(|_| http::Error::IoError)?;
		let response = pending.wait()?;
		if response.code != 200 {
			debug::warn!("Unexpected status code: {}", response.code);
			return Err(http::Error::Unknown);
		}
		let body = response.body().collect::<Vec<u8>>();
		let json = match core::str::from_utf8(&body) {
			Ok(json) => json,
			_ => {
				debug::warn!("Unexpected (non-utf8 or too short) response received: {:?}", body);
				return Err(http::Error::Unknown);
			}
		};
		Ok(json.as_bytes().to_vec())
	}
}

#[cfg(test)]
mod tests {
	// use super::*;
	use crate::testing::*;

	#[test]
	fn it_works_for_default_value() {
		let (mut ext, _state) = exec_with_offchain();
		ext.execute_with(|| {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			assert_eq!(Example::something(), vec![]);
		});
	}
}
