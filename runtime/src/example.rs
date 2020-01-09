use crate::submitter::{PublicOf, SubmitAndSignTransaction};

use frame_support::{
	debug, decl_event, decl_module, decl_storage, dispatch::DispatchResult, StorageValue,
};
use sp_runtime::offchain::http;
use sp_std::prelude::*;
use system::ensure_signed;

pub mod crypto {
	use sp_core::crypto::KeyTypeId;
	pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"reqs");

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
		Request(AccountId),
		Offchain(AccountId),
	}
);

decl_storage! {
	trait Store for Module<T: Trait> as Example {
		pub Authorities get(fn authorities) config(): Vec<T::AccountId> = vec![];

		pub DataRequest get(fn data_request): Option<u64> = None;
		pub Results get(fn results): Vec<u64> = vec![];
	}
}

decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		fn offchain_worker(_now: T::BlockNumber) {
			debug::RuntimeLogger::init();
			Self::offchain();
		}

		pub fn request(origin, value: u64) -> DispatchResult {
			let who = ensure_signed(origin)?;
			DataRequest::mutate(|x| *x = Some(value));
			Self::deposit_event(RawEvent::Request(who));
			Ok(())
		}

		pub fn store_result(origin, value: u64) -> DispatchResult {
			let who = ensure_signed(origin)?;
			DataRequest::mutate(|x| *x = None);
			Results::mutate(|x| x.push(value));
			Self::deposit_event(RawEvent::Offchain(who));
			Ok(())
		}
	}
}

impl<T: Trait> Module<T> {
	pub fn offchain() {
		let key = Self::authority_id();
		let data_request = Self::data_request();

		if key.is_none() || data_request.is_none() {
			debug::warn!("No authorized account or request");
			return;
		}

		let value = data_request.unwrap();
		debug::warn!("Start logic for #{:?}", value);

		// if let Ok(json) = Self::fetch_with_delay(
		// 	"http://www.mocky.io/v2/5e0006ca2f0000780013b267?mocky-delay=10ms",
		// ) {
		// 	debug::warn!("Fetched data: {}", core::str::from_utf8(&json).unwrap());
		// } else {
		// 	debug::warn!("Error fetching with delay.");
		// 	return;
		// }
		let call = Call::store_result(value);
		let res = T::SubmitTransaction::sign_and_submit(call, key.unwrap());
		debug::warn!("Finished: {:?}", res);
	}

	pub fn fetch_with_delay(url: &str) -> Result<Vec<u8>, http::Error> {
		let pending = http::Request::get(url)
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
				debug::warn!(
					"Unexpected (non-utf8 or too short) response received: {:?}",
					body
				);
				return Err(http::Error::Unknown);
			}
		};
		Ok(json.as_bytes().to_vec())
	}

	pub fn authority_id() -> Option<
		PublicOf<
			T,
			<T as Trait>::Call,
			<<T as Trait>::SubmitTransaction as SubmitAndSignTransaction<T, <T as Trait>::Call>>::SignAndSubmit,
		>,
	> {
		let accounts = Self::authorities();
		T::SubmitTransaction::get_local_keys().iter().find_map(|i| {
			if accounts.contains(&i.0) {
				Some(i.1.clone())
			} else {
				None
			}
		})
	}
}

#[cfg(test)]
mod tests {
	use crate::testing::*;
	use frame_support::assert_ok;

	#[test]
	fn it_works() {
		let (mut ext, state) = exec_with_offchain();
		ext.execute_with(|| {
			assert_eq!(Example::data_request(), None);
			assert_eq!(Example::results(), vec![]);

			let origin = Origin::signed(42.into());
			assert_ok!(Example::request(origin, 100));

			assert_eq!(Example::data_request(), Some(100));

			seal_block(state.clone());

			assert_eq!(Example::results(), vec![100]);
		});
	}
}
