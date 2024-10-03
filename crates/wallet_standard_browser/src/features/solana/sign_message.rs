#![allow(unsafe_code)]

use async_trait::async_trait;
use js_sys::Array;
use serde::Deserialize;
use serde::Serialize;
use solana_sdk::signature::Signature;
use typed_builder::TypedBuilder;
use wallet_standard::SOLANA_SIGN_MESSAGE;
use wallet_standard::SolanaSignMessageOutput;
use wallet_standard::SolanaSignatureOutput;
use wallet_standard::WalletError;
use wallet_standard::WalletResult;
use wallet_standard::WalletSolanaSignMessage;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

use crate::BrowserWallet;
use crate::BrowserWalletAccountInfo;
use crate::impl_feature_from_js;

#[wasm_bindgen]
extern "C" {
	#[derive(Clone, Debug)]
	pub type BrowserSolanaSignMessageOutput;
	/// Message bytes that were signed.
	/// The wallet may prefix or otherwise modify the message before signing it.
	#[wasm_bindgen(method, getter, js_name = signedMessage)]
	pub fn _signed_message(this: &BrowserSolanaSignMessageOutput) -> Vec<u8>;
	/// Message signature produced.
	/// If the signature type is provided, the signature must be Ed25519.
	#[wasm_bindgen(method, getter, js_name = signature)]
	pub fn _signature(this: &BrowserSolanaSignMessageOutput) -> Vec<u8>;
	/// Optional type of the message signature produced.
	/// If not provided, the signature must be Ed25519.
	#[wasm_bindgen(method, getter, js_name = signatureType)]
	pub fn _signature_type(this: &BrowserSolanaSignMessageOutput) -> Option<String>;
	#[derive(Clone, Debug)]
	pub type SolanaSignMessageFeature;
	/// Version of the feature API.
	#[wasm_bindgen(method, getter)]
	pub fn version(this: &SolanaSignMessageFeature) -> String;
	/// Sign messages (arbitrary bytes) using the account's secret key.
	#[allow(unused_qualifications)]
	#[wasm_bindgen(method, catch, variadic, js_name = signMessage)]
	pub async fn _sign_message(
		this: &SolanaSignMessageFeature,
		args: Array,
	) -> Result<JsValue, JsValue>;
}

impl SolanaSignatureOutput for BrowserSolanaSignMessageOutput {
	fn try_signature(&self) -> WalletResult<Signature> {
		self._signature()
			.try_into()
			.map_err(|_| WalletError::InvalidSignature)
	}

	fn signature(&self) -> Signature {
		self.try_signature().unwrap_throw()
	}
}

impl SolanaSignMessageOutput for BrowserSolanaSignMessageOutput {
	fn signed_message(&self) -> Vec<u8> {
		self._signed_message()
	}

	fn signature_type(&self) -> Option<String> {
		self._signature_type()
	}
}

impl TryFrom<BrowserSolanaSignMessageOutput> for Signature {
	type Error = WalletError;

	fn try_from(value: BrowserSolanaSignMessageOutput) -> Result<Self, Self::Error> {
		Ok(value.signature())
	}
}

impl_feature_from_js!(SolanaSignMessageFeature, SOLANA_SIGN_MESSAGE);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SolanaSignMessageInput {
	/// Account to use.
	#[serde(with = "serde_wasm_bindgen::preserve")]
	pub account: BrowserWalletAccountInfo,
	/// Message to sign, as raw bytes.
	#[serde(with = "serde_bytes")]
	#[builder(setter(into))]
	pub message: Vec<u8>,
}

impl SolanaSignMessageFeature {
	/// Sign a  message using the account's secret key.
	pub async fn sign_message(
		&self,
		account: BrowserWalletAccountInfo,
		message: impl Into<Vec<u8>>,
	) -> WalletResult<BrowserSolanaSignMessageOutput> {
		let input = SolanaSignMessageInput::builder()
			.account(account)
			.message(message)
			.build();
		self.sign_messages(vec![input])
			.await?
			.first()
			.cloned()
			.ok_or(WalletError::WalletSignMessage)
	}

	/// Sign a list of messages using the account's secret key.
	pub async fn sign_messages(
		&self,
		inputs: Vec<SolanaSignMessageInput>,
	) -> WalletResult<Vec<BrowserSolanaSignMessageOutput>> {
		let array: Array = serde_wasm_bindgen::to_value(&inputs)?.unchecked_into();
		let results: Array = self._sign_message(array).await?.dyn_into()?;

		Ok(results.into_iter().map(JsCast::unchecked_into).collect())
	}
}

#[async_trait(?Send)]
impl WalletSolanaSignMessage for BrowserWallet {
	type Output = BrowserSolanaSignMessageOutput;

	async fn sign_message(&self, message: impl Into<Vec<u8>>) -> WalletResult<Self::Output> {
		let Some(ref wallet_account) = self.wallet_account else {
			return Err(WalletError::WalletAccount);
		};

		self.wallet
			.get_feature::<SolanaSignMessageFeature>()?
			.sign_message(wallet_account.clone(), message)
			.await
	}

	async fn sign_messages<M: Into<Vec<u8>>>(
		&self,
		messages: Vec<M>,
	) -> WalletResult<Vec<Self::Output>> {
		let Some(ref wallet_account) = self.wallet_account else {
			return Err(WalletError::WalletAccount);
		};
		let inputs = messages
			.into_iter()
			.map(|message| {
				SolanaSignMessageInput::builder()
					.account(wallet_account.clone())
					.message(message)
					.build()
			})
			.collect();

		self.wallet
			.get_feature::<SolanaSignMessageFeature>()?
			.sign_messages(inputs)
			.await
	}
}
