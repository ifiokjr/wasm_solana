use std::future::Future;

use serde::Deserialize;
use serde::Serialize;
use solana_sdk::signature::Signature;
use typed_builder::TypedBuilder;

use crate::WalletAccountInfo;
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

pub trait SolanaSignMessageOutput: SolanaSignatureOutput {
	/// Message bytes that were signed.
	/// The wallet may prefix or otherwise modify the message before signing it.
	fn signed_message(&self) -> Vec<u8>;
	/// Optional type of the message signature produced.
	/// If not provided, the signature must be Ed25519.
	fn signature_type(&self) -> Option<String>;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SolanaSignMessageInput<Account: WalletAccountInfo> {
	/// Account to use.
	#[cfg_attr(feature = "browser", serde(with = "serde_wasm_bindgen::preserve"))]
	pub account: Account,
	/// Message to sign, as raw bytes.
	#[serde(with = "serde_bytes")]
	#[builder(setter(into))]
	pub message: Vec<u8>,
}

pub trait WalletSolanaSignMessage {
	type Output: SolanaSignMessageOutput;

	/// Sign a  message using the account's secret key.
	fn sign_message(
		&self,
		message: impl Into<Vec<u8>>,
	) -> impl Future<Output = WalletResult<Self::Output>>;

	/// Sign a list of messages using the account's secret key.
	fn sign_messages<M: Into<Vec<u8>>>(
		&self,
		messages: Vec<M>,
	) -> impl Future<Output = WalletResult<Vec<Self::Output>>>;
}
