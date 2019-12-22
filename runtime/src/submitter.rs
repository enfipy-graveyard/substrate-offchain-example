use sp_runtime::app_crypto::{AppPublic, RuntimeAppPublic};
use sp_runtime::traits::{Extrinsic as ExtrinsicT, IdentifyAccount};
use sp_std::prelude::Vec;
use system::offchain::*;
use system::*;

pub type PublicOf<T, Call, X> = <<X as SubmitSignedTransaction<T, Call>>::CreateTransaction as CreateTransaction<
    T,
    <X as SubmitSignedTransaction<T, Call>>::Extrinsic,
>>::Public;

/// A utility trait to easily create signed transactions
/// from accounts in node's local keystore.
///
/// NOTE: Most likely you should not implement this trait yourself.
/// There is an implementation for `TransactionSubmitter` type, which
/// you should use.
pub trait SubmitAndSignTransaction<T: Trait, Call> {
    /// A `SubmitSignedTransaction` implementation.
    type SignAndSubmit: SubmitSignedTransaction<T, Call>;

    /// Find local keys that match given list of accounts.
    ///
    /// Technically it finds an intersection between given list of `AccountId`s
    /// and accounts that are represented by public keys in local keystore.
    /// If `None` is passed it returns all accounts in the keystore.
    ///
    /// Returns both public keys and `AccountId`s of accounts that are available.
    /// Such accounts can later be used to sign a payload or send signed transactions.
    fn get_local_keys() -> Vec<(T::AccountId, PublicOf<T, Call, Self::SignAndSubmit>)>;

    fn sign_and_submit(call: impl Into<Call>, public: PublicOf<T, Call, Self::SignAndSubmit>) -> Result<(), ()> {
        Self::SignAndSubmit::sign_and_submit(call, public)
    }
}

/// A default type used to submit transactions to the pool.
pub struct TransactionSubmitter<S, C, E> {
    _signer: sp_std::marker::PhantomData<(S, C, E)>,
}

impl<S, C, E> Default for TransactionSubmitter<S, C, E> {
    fn default() -> Self {
        Self {
            _signer: Default::default(),
        }
    }
}

/// A blanket implementation to simplify creation of transaction signer & submitter in the runtime.
impl<T, E, S, C, Call> SubmitSignedTransaction<T, Call> for TransactionSubmitter<S, C, E>
where
    T: Trait,
    C: CreateTransaction<T, E>,
    S: Signer<<C as CreateTransaction<T, E>>::Public, <C as CreateTransaction<T, E>>::Signature>,
    E: ExtrinsicT<Call = Call> + codec::Encode,
{
    type Extrinsic = E;
    type CreateTransaction = C;
    type Signer = S;
}

/// A blanket impl to use the same submitter for usigned transactions as well.
impl<T, E, S, C, Call> SubmitUnsignedTransaction<T, Call> for TransactionSubmitter<S, C, E>
where
    T: Trait,
    E: ExtrinsicT<Call = Call> + codec::Encode,
{
    type Extrinsic = E;
}

/// A blanket implementation to support local keystore of application-crypto types.
impl<T, C, E, S, Call> SubmitAndSignTransaction<T, Call> for TransactionSubmitter<S, C, E>
where
    T: Trait,
    C: CreateTransaction<T, E>,
    E: ExtrinsicT<Call = Call> + codec::Encode,
    S: Signer<<C as CreateTransaction<T, E>>::Public, <C as CreateTransaction<T, E>>::Signature>,
    // Make sure we can unwrap the app crypto key.
    S: RuntimeAppPublic + AppPublic + Into<<S as AppPublic>::Generic>,
    // Make sure we can convert from wrapped crypto to public key (e.g. `MultiSigner`)
    S::Generic: Into<PublicOf<T, Call, Self>>,
    // For simplicity we require the same trait to implement `SubmitSignedTransaction` too.
    Self: SubmitSignedTransaction<T, Call, Signer = S, Extrinsic = E, CreateTransaction = C>,
{
    type SignAndSubmit = Self;

    fn get_local_keys() -> Vec<(T::AccountId, PublicOf<T, Call, Self::SignAndSubmit>)> {
        // Convert app-specific keys into generic ones.
        S::all()
            .into_iter()
            .map(|app_key| {
                // unwrap app-crypto
                let generic_pub_key: <S as AppPublic>::Generic = app_key.into();
                // convert to expected public key type (might be MultiSigner)
                let signer_pub_key: PublicOf<T, Call, Self::SignAndSubmit> = generic_pub_key.into();
                // lookup accountid for that pubkey
                let account = signer_pub_key.clone().into_account();
                (account, signer_pub_key)
            })
            .collect::<Vec<_>>()
    }
}
