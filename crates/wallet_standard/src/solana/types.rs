use std::ops::Deref;
use std::sync::Arc;

use async_trait::async_trait;
use futures::future::join_all;
use futures::future::try_join_all;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;

pub trait SolanaWalletPubkey {
	fn try_pubkey(&self) -> WalletResult<Pubkey>;
	fn pubkey(&self) -> Pubkey;
}

impl<T> SolanaWalletPubkey for T
where
	T: WalletAccountInfo,
{
	fn try_pubkey(&self) -> WalletResult<Pubkey> {
		Pubkey::try_from(self.public_key()).map_err(|_| WalletError::WalletPublicKey)
	}

	fn pubkey(&self) -> Pubkey {
		self.try_pubkey().unwrap_or_default()
	}
}

use crate::WalletAccountInfo;
use crate::WalletError;
use crate::WalletResult;
use crate::WalletSolanaSignAndSendTransaction;
use crate::WalletSolanaSignIn;
use crate::WalletSolanaSignMessage;
use crate::WalletSolanaSignTransaction;
use crate::WalletStandard;

pub trait WalletSolana:
	WalletSolanaSignMessage
	+ WalletSolanaSignTransaction
	+ WalletSolanaSignAndSendTransaction
	+ WalletSolanaSignIn
	+ WalletStandard
{
}

impl<T> WalletSolana for T where
	T: WalletSolanaSignMessage
		+ WalletSolanaSignTransaction
		+ WalletSolanaSignAndSendTransaction
		+ WalletSolanaSignIn
		+ WalletStandard
{
}

/// An async version of the keypair.
#[derive(Debug, Clone, derive_more::From)]
pub struct AsyncKeypair(Arc<Keypair>);

impl From<Keypair> for AsyncKeypair {
	fn from(value: Keypair) -> Self {
		Self(Arc::new(value))
	}
}

impl From<&Keypair> for AsyncKeypair {
	fn from(value: &Keypair) -> Self {
		Self(Arc::new(Keypair::from_bytes(&value.to_bytes()).unwrap()))
	}
}

impl AsyncKeypair {
	pub fn new() -> Self {
		Self(Arc::new(Keypair::new()))
	}

	/// Get the original keypair from the `AsyncKeypair`
	pub fn keypair(&self) -> Keypair {
		Keypair::from_bytes(&self.0.to_bytes()).unwrap()
	}
}

impl Default for AsyncKeypair {
	fn default() -> Self {
		Self::new()
	}
}

#[async_trait(?Send)]
impl AsyncSigner for AsyncKeypair {
	fn try_pubkey(&self) -> WalletResult<Pubkey> {
		Signer::try_pubkey(&self.0).map_err(|_| WalletError::WalletPublicKey)
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Signature> {
		let result = Signer::try_sign_message(&self.0, message)?;

		Ok(result)
	}
}

/// An alternative to the [`solana_sdk::signer::Signer`] but with an async
/// `sign_message` and `sign_messages` method
#[async_trait(?Send)]
pub trait AsyncSigner {
	fn try_pubkey(&self) -> WalletResult<Pubkey>;
	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Signature>;

	/// Infallibly gets the implementor's public key. Returns the all-zeros
	/// `Pubkey` if the implementor has none.
	fn pubkey(&self) -> Pubkey {
		self.try_pubkey().unwrap_or_default()
	}

	/// Infallibly produces an Ed25519 signature over the provided `message`
	/// bytes. Returns the all-zeros `Signature` if signing is not possible.
	async fn sign_message(&self, message: &[u8]) -> Signature {
		self.try_sign_message(message).await.unwrap_or_default()
	}
}

/// Convenience trait for working with mixed collections of `AsyncSigner`s
#[async_trait(?Send)]
pub trait AsyncSigners {
	fn pubkeys(&self) -> Vec<Pubkey>;
	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>>;
	async fn sign_message(&self, message: &[u8]) -> Vec<Signature>;
	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>>;
}

// #[async_trait(?Send)]
// impl<T> AsyncSigner for T
// where
// 	T: Signer,
// {
// 	fn try_pubkey(&self) -> WalletResult<Pubkey> {
// 		let pubkey = Signer::try_pubkey(self).map_err(|_|
// WalletError::WalletPublicKey)?; 		Ok(pubkey)
// 	}

// 	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Signature>
// { 		Signer::try_sign_message(self, message).map_err(|_|
// WalletError::InvalidSignature) 	}
// }

impl<T> From<T> for Box<dyn AsyncSigner>
where
	T: AsyncSigner + 'static,
{
	fn from(signer: T) -> Self {
		Box::new(signer)
	}
}

/// This impl allows using AsyncSigner with types like Box/Rc/Arc.
#[async_trait(?Send)]
impl<Container: Deref<Target = impl AsyncSigner>> AsyncSigner for Container {
	fn try_pubkey(&self) -> WalletResult<Pubkey> {
		self.deref().try_pubkey()
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Signature> {
		self.deref().try_sign_message(message).await
	}
}

impl PartialEq for dyn AsyncSigner {
	fn eq(&self, other: &dyn AsyncSigner) -> bool {
		self.pubkey() == other.pubkey()
	}
}

impl Eq for dyn AsyncSigner {}

impl std::fmt::Debug for dyn AsyncSigner {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(fmt, "AsyncSigner: {:?}", self.pubkey())
	}
}

#[async_trait(?Send)]
impl<T: AsyncSigner> AsyncSigners for [&T] {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(AsyncSigner::pubkey).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl AsyncSigners for [Box<dyn AsyncSigner>] {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(|keypair| keypair.pubkey()).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl AsyncSigners for Vec<Box<dyn AsyncSigner>> {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(|keypair| keypair.pubkey()).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl AsyncSigners for [Arc<dyn AsyncSigner>] {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(|keypair| keypair.pubkey()).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl AsyncSigners for [Arc<dyn AsyncSigner>; 0] {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(|keypair| keypair.pubkey()).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl AsyncSigners for [Arc<dyn AsyncSigner>; 1] {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(|keypair| keypair.pubkey()).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl AsyncSigners for [Arc<dyn AsyncSigner>; 2] {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(|keypair| keypair.pubkey()).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl AsyncSigners for [Arc<dyn AsyncSigner>; 3] {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(|keypair| keypair.pubkey()).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl AsyncSigners for [Arc<dyn AsyncSigner>; 4] {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(|keypair| keypair.pubkey()).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl AsyncSigners for Vec<Arc<dyn AsyncSigner>> {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(|keypair| keypair.pubkey()).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl AsyncSigners for Vec<&dyn AsyncSigner> {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(|keypair| keypair.pubkey()).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl AsyncSigners for [&dyn AsyncSigner] {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(|keypair| keypair.pubkey()).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl AsyncSigners for [&dyn AsyncSigner; 0] {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(|keypair| keypair.pubkey()).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl AsyncSigners for [&dyn AsyncSigner; 1] {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(|keypair| keypair.pubkey()).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl AsyncSigners for [&dyn AsyncSigner; 2] {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(|keypair| keypair.pubkey()).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl AsyncSigners for [&dyn AsyncSigner; 3] {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(|keypair| keypair.pubkey()).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl AsyncSigners for [&dyn AsyncSigner; 4] {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(|keypair| keypair.pubkey()).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl<T: AsyncSigner> AsyncSigners for [&T; 0] {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(AsyncSigner::pubkey).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl<T: AsyncSigner> AsyncSigners for [&T; 1] {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(AsyncSigner::pubkey).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl<T: AsyncSigner> AsyncSigners for [&T; 2] {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(AsyncSigner::pubkey).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl<T: AsyncSigner> AsyncSigners for [&T; 3] {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(AsyncSigner::pubkey).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl<T: AsyncSigner> AsyncSigners for [&T; 4] {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(AsyncSigner::pubkey).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}

#[async_trait(?Send)]
impl<T: AsyncSigner> AsyncSigners for Vec<&T> {
	fn pubkeys(&self) -> Vec<Pubkey> {
		self.iter().map(AsyncSigner::pubkey).collect()
	}

	fn try_pubkeys(&self) -> WalletResult<Vec<Pubkey>> {
		let mut pubkeys = Vec::new();
		for keypair in self {
			pubkeys.push(keypair.try_pubkey()?);
		}

		Ok(pubkeys)
	}

	async fn sign_message(&self, message: &[u8]) -> Vec<Signature> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.sign_message(message));
		}

		join_all(futures).await
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Vec<Signature>> {
		let mut futures = Vec::new();

		for async_signer in self {
			futures.push(async_signer.try_sign_message(message));
		}

		try_join_all(futures).await
	}
}
