use async_trait::async_trait;
use futures::future::try_join_all;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;

use crate::WalletResult;

pub const SOLANA_SIGN_MESSAGE: &str = "solana:signMessage";

pub trait SolanaSignatureOutput {
	/// Message signature produced.
	/// If the signature type is provided, the signature must be Ed25519.
	fn try_signature(&self) -> WalletResult<Signature>;
	/// Message signature produced.
	/// If the signature type is provided, the signature must be Ed25519.
	fn signature(&self) -> Signature;
}

impl SolanaSignatureOutput for Signature {
	fn try_signature(&self) -> WalletResult<Signature> {
		Ok(*self)
	}

	fn signature(&self) -> Signature {
		*self
	}
}

pub trait SolanaSignMessageOutput: SolanaSignatureOutput {
	/// Message bytes that were signed.
	/// The wallet may prefix or otherwise modify the message before signing it.
	fn signed_message(&self) -> Vec<u8>;
	/// Optional type of the message signature produced.
	/// If not provided, the signature must be Ed25519.
	fn signature_type(&self) -> Option<String>;
}

impl SolanaSignatureOutput for (Signature, Vec<u8>, Option<String>) {
	fn try_signature(&self) -> WalletResult<Signature> {
		self.0.try_signature()
	}

	fn signature(&self) -> Signature {
		self.0.signature()
	}
}

impl SolanaSignMessageOutput for (Signature, Vec<u8>, Option<String>) {
	fn signed_message(&self) -> Vec<u8> {
		self.1.clone()
	}

	fn signature_type(&self) -> Option<String> {
		self.2.clone()
	}
}

#[async_trait(?Send)]
pub trait WalletSolanaSignMessage {
	type Output: SolanaSignMessageOutput;

	/// Sign a  message using the account's secret key. This is prefixed with
	/// `solana` to prevent clashes with the commonly used
	/// [`solana_sdk::signer::Signer`] trait.
	async fn sign_message_async(&self, message: impl Into<Vec<u8>>) -> WalletResult<Self::Output>;

	/// Sign a list of messages using the account's secret key.
	async fn sign_messages<M: Into<Vec<u8>>>(
		&self,
		messages: Vec<M>,
	) -> WalletResult<Vec<Self::Output>>;
}

#[async_trait(?Send)]
impl WalletSolanaSignMessage for Keypair {
	type Output = (Signature, Vec<u8>, Option<String>);

	async fn sign_message_async(&self, message: impl Into<Vec<u8>>) -> WalletResult<Self::Output> {
		let message: Vec<u8> = message.into();
		let signature = Signer::try_sign_message(self, &message)?;

		Ok((signature, message, None))
	}

	async fn sign_messages<M: Into<Vec<u8>>>(
		&self,
		messages: Vec<M>,
	) -> WalletResult<Vec<Self::Output>> {
		let futures = messages
			.into_iter()
			.map(|message| WalletSolanaSignMessage::sign_message_async(self, message));
		let result = try_join_all(futures).await?;

		Ok(result)
	}
}
