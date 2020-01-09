use serde::{Serialize, Deserialize};
use std::{fmt::{self, Debug}, cell::RefCell};
use sp_runtime::codec::{Encode, Decode};
use sp_runtime::traits::{OpaqueKeys, IdentifyAccount, Verify, Lazy};
use sp_runtime::KeyTypeId;
use sp_core::{crypto::*, U256};
use sp_application_crypto::Derive;
use std::convert::TryInto;

/// Authority Id
#[derive(Default, PartialEq, Eq, Clone, Encode, Decode, Debug, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct UintAuthorityId(pub u64);

impl fmt::Display for UintAuthorityId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.0) }
}

impl From<u64> for UintAuthorityId {
	fn from(id: u64) -> Self {
		UintAuthorityId(id)
	}
}

impl From<UintAuthorityId> for u64 {
	fn from(id: UintAuthorityId) -> u64 {
		id.0
	}
}

impl UintAuthorityId {
	/// Convert this authority id into a public key.
	pub fn to_public_key<T: Public>(&self) -> T {
		let bytes: [u8; 32] = U256::from(self.0).into();
		T::from_slice(&bytes)
	}
}

impl CryptoType for UintAuthorityId {
	type Pair = Dummy;
}

impl AsRef<UintAuthorityId> for UintAuthorityId {
	fn as_ref(&self) -> &Self { self }
}

impl AsRef<[u8]> for UintAuthorityId {
	fn as_ref(&self) -> &[u8] {
		// Unsafe, i know, but it's test code and it's just there because it's really convenient to
		// keep `UintAuthorityId` as a u64 under the hood.
		unsafe {
			std::slice::from_raw_parts(&self.0 as *const u64 as *const _, std::mem::size_of::<u64>())
		}
	}
}

impl AsMut<UintAuthorityId> for UintAuthorityId {
	fn as_mut(&mut self) -> &mut Self { self }
}

impl AsMut<[u8]> for UintAuthorityId {
	fn as_mut(&mut self) -> &mut [u8] {
		unsafe {
			std::slice::from_raw_parts_mut(&mut self.0 as *mut u64 as *mut _, std::mem::size_of::<u64>())
		}
    }
}

thread_local! {
	/// A list of all UintAuthorityId keys returned to the runtime.
	static ALL_KEYS: RefCell<Vec<UintAuthorityId>> = RefCell::new(vec![]);
}

impl UintAuthorityId {
	/// Set the list of keys returned by the runtime call for all keys of that type.
	pub fn set_all_keys<T: Into<UintAuthorityId>>(keys: impl IntoIterator<Item=T>) {
		ALL_KEYS.with(|l| *l.borrow_mut() = keys.into_iter().map(Into::into).collect())
	}
}

impl sp_application_crypto::RuntimeAppPublic for UintAuthorityId {
	const ID: KeyTypeId = key_types::DUMMY;

	type Signature = UintAuthorityId;

	fn all() -> Vec<Self> {
		ALL_KEYS.with(|l| l.borrow().clone())
	}

	fn generate_pair(_: Option<Vec<u8>>) -> Self {
		use rand::RngCore;
		UintAuthorityId(rand::thread_rng().next_u64())
	}

	fn sign<M: AsRef<[u8]>>(&self, msg: &M) -> Option<Self::Signature> {
		let mut signature = [0u8; 8];
		msg.as_ref().iter()
			.chain(std::iter::repeat(&42u8))
			.take(8)
			.enumerate()
			.for_each(|(i, v)| { signature[i] = *v; });

		Some(Self(u64::from_le_bytes(signature)))
	}

	fn verify<M: AsRef<[u8]>>(&self, msg: &M, signature: &Self::Signature) -> bool {
		let mut msg_signature = [0u8; 8];
		msg.as_ref().iter()
			.chain(std::iter::repeat(&42))
			.take(8)
			.enumerate()
			.for_each(|(i, v)| { msg_signature[i] = *v; });

		Self(u64::from_le_bytes(msg_signature)) == *signature
	}
}

impl OpaqueKeys for UintAuthorityId {
	type KeyTypeIdProviders = ();

	fn key_ids() -> &'static [KeyTypeId] {
		&[key_types::DUMMY]
	}

	fn get_raw(&self, _: KeyTypeId) -> &[u8] {
		self.as_ref()
	}

	fn get<T: Decode>(&self, _: KeyTypeId) -> Option<T> {
		self.using_encoded(|mut x| T::decode(&mut x)).ok()
	}
}

impl sp_runtime::BoundToRuntimeAppPublic for UintAuthorityId {
	type Public = Self;
}

impl Derive for UintAuthorityId {}

impl sp_application_crypto::AppSignature for UintAuthorityId {
    type Generic = Self;
}

impl sp_application_crypto::Pair for UintAuthorityId {
    type Public = Self;
    type Seed = Dummy;
    type Signature = Dummy;
    type DeriveError = ();
    #[cfg(feature = "std")]
    fn generate_with_phrase(_: Option<&str>) -> (Self, String, Self::Seed) { Default::default() }
    #[cfg(feature = "std")]
    fn from_phrase(_: &str, _: Option<&str>)
        -> Result<(Self, Self::Seed), SecretStringError>
    {
        Ok(Default::default())
    }
    fn derive<
        Iter: Iterator<Item=DeriveJunction>,
    >(&self, _: Iter, _: Option<Dummy>) -> Result<(Self, Option<Dummy>), Self::DeriveError> { Ok((Self(self.0), None)) }
    fn from_seed(_: &Self::Seed) -> Self { Self(0u64) }
    fn from_seed_slice(_: &[u8]) -> Result<Self, SecretStringError> { Ok(Self(0u64)) }
    fn sign(&self, _: &[u8]) -> Self::Signature { Dummy }
    fn verify<M: AsRef<[u8]>>(_: &Self::Signature, _: M, _: &Self::Public) -> bool { true }
    fn verify_weak<P: AsRef<[u8]>, M: AsRef<[u8]>>(_: &[u8], _: M, _: P) -> bool { true }
    fn public(&self) -> Self::Public { Self(self.0) }
    fn to_raw_vec(&self) -> Vec<u8> { vec![] }
}

impl sp_application_crypto::AppPair for UintAuthorityId {
    type Generic = Self;
}

impl sp_application_crypto::AppKey for UintAuthorityId {
    type UntypedGeneric = Self;
    type Public = Self;
    type Pair = Self;
    type Signature = Self;
    const ID: sp_application_crypto::KeyTypeId = key_types::DUMMY;
}

impl sp_application_crypto::Public for UintAuthorityId {
    fn from_slice(x: &[u8]) -> Self {
		let (int_bytes, _) = x.split_at(std::mem::size_of::<u64>());
		Self(u64::from_be_bytes(int_bytes.try_into().unwrap()))
	}
}

impl sp_application_crypto::AppPublic for UintAuthorityId {
    type Generic = Self;
}

impl IdentifyAccount for UintAuthorityId {
	type AccountId = Self;
	fn into_account(self) -> Self { self }
}

impl Verify for UintAuthorityId {
	type Signer = UintAuthorityId;
	fn verify<L: Lazy<[u8]>>(&self, mut _msg: L, _signer: &UintAuthorityId) -> bool {
		true
	}
}
