use async_trait::async_trait;
use solana_sdk::signature::Signature;

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

#[async_trait(?Send)]
pub trait WalletSolanaSignMessage {
	type Output: SolanaSignMessageOutput;

	/// Sign a  message using the account's secret key.
	async fn sign_message(&self, message: impl Into<Vec<u8>>) -> WalletResult<Self::Output>;

	/// Sign a list of messages using the account's secret key.
	async fn sign_messages<M: Into<Vec<u8>>>(
		&self,
		messages: Vec<M>,
	) -> WalletResult<Vec<Self::Output>>;
}
