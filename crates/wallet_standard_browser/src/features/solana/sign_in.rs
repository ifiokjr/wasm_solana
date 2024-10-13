#![allow(unsafe_code)]

use async_trait::async_trait;
use js_sys::Array;
use solana_sdk::signature::Signature;
use wallet_standard::SOLANA_SIGN_IN;
use wallet_standard::SolanaSignInInput;
use wallet_standard::SolanaSignInOutput;
use wallet_standard::SolanaSignMessageOutput;
use wallet_standard::SolanaSignatureOutput;
use wallet_standard::WalletError;
use wallet_standard::WalletResult;
use wallet_standard::WalletSolanaPubkey;
use wallet_standard::WalletSolanaSignIn;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

use crate::BrowserWallet;
use crate::BrowserWalletAccountInfo;
use crate::impl_feature_from_js;

#[wasm_bindgen]
extern "C" {
	#[derive(Clone, Debug)]
	pub type BrowserSolanaSignInOutput;
	/// Account that was signed in.
	/// The address of the account may be different from the provided input
	/// Address.
	#[wasm_bindgen(method, getter)]
	pub fn _account(this: &BrowserSolanaSignInOutput) -> BrowserWalletAccountInfo;
	/// Message bytes that were signed.
	/// The wallet may prefix or otherwise modify the message before signing it.
	#[wasm_bindgen(method, getter)]
	pub fn _signed_message(this: &BrowserSolanaSignInOutput) -> Vec<u8>;
	/// Message signature produced.
	/// If the signature type is provided, the signature must be Ed25519.
	#[wasm_bindgen(method, getter)]
	pub fn _signature(this: &BrowserSolanaSignInOutput) -> Vec<u8>;
	/// Optional type of the message signature produced.
	/// If not provided, the signature must be Ed25519.
	#[wasm_bindgen(method, getter)]
	pub fn _signature_type(this: &BrowserSolanaSignInOutput) -> Option<String>;
	#[derive(Clone, Debug)]
	pub type SolanaSignInFeature;
	/// Version of the feature API.
	#[wasm_bindgen(method, getter)]
	pub fn version(this: &SolanaSignInFeature) -> String;
	/// Sign In With Solana (based on <https://eips.ethereum.org/EIPS/eip-4361> and <https://github.com/ChainAgnostic/CAIPs/blob/master/CAIPs/caip-122.md>).
	#[allow(unused_qualifications)]
	#[wasm_bindgen(method, catch, variadic, js_name = signIn)]
	pub async fn _sign_in(this: &SolanaSignInFeature, args: Array) -> Result<JsValue, JsValue>;
}

impl SolanaSignatureOutput for BrowserSolanaSignInOutput {
	fn try_signature(&self) -> WalletResult<Signature> {
		self._signature()
			.try_into()
			.map_err(|_| WalletError::InvalidSignature)
	}

	fn signature(&self) -> Signature {
		self.try_signature().unwrap_throw()
	}
}

impl SolanaSignMessageOutput for BrowserSolanaSignInOutput {
	fn signed_message(&self) -> Vec<u8> {
		self._signed_message()
	}

	fn signature_type(&self) -> Option<String> {
		self._signature_type()
	}
}

impl SolanaSignInOutput for BrowserSolanaSignInOutput {
	type Account = BrowserWalletAccountInfo;

	fn account(&self) -> Self::Account {
		self._account()
	}
}

impl_feature_from_js!(SolanaSignInFeature, SOLANA_SIGN_IN);

impl SolanaSignInFeature {
	pub async fn sign_in(
		&self,
		inputs: Vec<SolanaSignInInput>,
	) -> WalletResult<Vec<BrowserSolanaSignInOutput>> {
		if inputs.is_empty() {
			return Err(WalletError::InvalidArguments);
		}

		let args: Array = serde_wasm_bindgen::to_value(&inputs)?.dyn_into()?;
		let results: Array = self._sign_in(args).await?.dyn_into()?;

		Ok(results
			.into_iter()
			.map(wasm_bindgen::JsCast::unchecked_into)
			.collect())
	}
}

#[async_trait(?Send)]
impl WalletSolanaSignIn for BrowserWallet {
	type Output = BrowserSolanaSignInOutput;

	async fn sign_in(&self, input: SolanaSignInInput) -> WalletResult<Self::Output> {
		self.sign_in_many(vec![input])
			.await?
			.first()
			.cloned()
			.ok_or(WalletError::WalletSignIn)
	}

	async fn sign_in_many(
		&self,
		inputs: Vec<SolanaSignInInput>,
	) -> WalletResult<Vec<Self::Output>> {
		let Ok(address) = self.try_solana_pubkey().map(|pubkey| pubkey.to_string()) else {
			return Err(WalletError::WalletAccount);
		};

		let inputs = inputs
			.into_iter()
			.map(|input| {
				SolanaSignInInput {
					address: Some(address.clone()),
					..input
				}
			})
			.collect();

		self.wallet
			.get_feature::<SolanaSignInFeature>()?
			.sign_in(inputs)
			.await
	}
}
