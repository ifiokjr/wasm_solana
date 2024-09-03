#![allow(unsafe_code)]

use std::future::Future;

use js_sys::Array;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;
use wallet_standard::ExperimentalEncryptOutput;
use wallet_standard::ExperimentalEncryptProps;
use wallet_standard::WalletError;
use wallet_standard::WalletExperimentalEncrypt;
use wallet_standard::WalletResult;
use wallet_standard::EXPERIMENTAL_ENCRYPT;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

use crate::BrowserWallet;
use crate::BrowserWalletAccountInfo;
use crate::FeatureFromJs;

impl ExperimentalEncryptOutput for BrowserExperimentalEncryptOutput {
	fn cipher_text(&self) -> Vec<u8> {
		self._cipher_text()
	}

	fn nonce(&self) -> Vec<u8> {
		self._nonce()
	}
}

#[wasm_bindgen]
extern "C" {
	#[derive(Clone, Debug)]
	pub type BrowserExperimentalEncryptOutput;
	/// `ciphertext` that was encrypted.
	#[wasm_bindgen(method, getter, js_name = "cipher_text")]
	pub fn _cipher_text(this: &BrowserExperimentalEncryptOutput) -> Vec<u8>;
	/// Nonce that was used for encryption.
	#[wasm_bindgen(method, getter, js_name = "nonce")]
	pub fn _nonce(this: &BrowserExperimentalEncryptOutput) -> Vec<u8>;
	#[derive(Clone, Debug)]
	pub type ExperimentalEncryptFeature;
	/// Version of the feature API.
	#[wasm_bindgen(method, getter)]
	pub fn version(this: &ExperimentalEncryptFeature) -> String;
	/// List of ciphers supported for encryption.
	#[wasm_bindgen(method, getter)]
	pub fn ciphers(this: &ExperimentalEncryptFeature) -> Vec<String>;
	/// Encrypt cleartexts using the account's secret key.
	///
	/// @param inputs Inputs for encryption.
	///
	/// @return Outputs of encryption.
	#[allow(unused_qualifications)]
	#[wasm_bindgen(method, variadic, catch)]
	pub async fn encrypt(
		this: &ExperimentalEncryptFeature,
		args: Array,
	) -> Result<JsValue, JsValue>;
}

impl FeatureFromJs for ExperimentalEncryptFeature {
	const NAME: &'static str = EXPERIMENTAL_ENCRYPT;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ExperimentalEncryptInput {
	/// Account to use.
	#[serde(with = "serde_wasm_bindgen::preserve")]
	pub account: BrowserWalletAccountInfo,
	#[serde(flatten)]
	pub props: ExperimentalEncryptProps,
}

impl WalletExperimentalEncrypt for BrowserWallet {
	type Output = BrowserExperimentalEncryptOutput;

	fn encrypt_many(
		&self,
		props: Vec<ExperimentalEncryptProps>,
	) -> impl Future<Output = WalletResult<Vec<Self::Output>>> {
		async move {
			let Some(ref wallet_account) = self.wallet_account else {
				return Err(WalletError::WalletAccount);
			};

			let input = props
				.into_iter()
				.map(|p| {
					ExperimentalEncryptInput::builder()
						.account(wallet_account.clone())
						.props(p)
						.build()
				})
				.collect::<Vec<_>>();

			let feature = self.wallet.get_feature::<ExperimentalEncryptFeature>()?;
			let inputs: Array = serde_wasm_bindgen::to_value(&input)?.unchecked_into();
			let result: Array = feature.encrypt(inputs).await?.unchecked_into();

			Ok(result
				.into_iter()
				.map(wasm_bindgen::JsCast::unchecked_into)
				.collect())
		}
	}

	fn encrypt(
		&self,
		props: ExperimentalEncryptProps,
	) -> impl Future<Output = WalletResult<Self::Output>> {
		async move {
			self.encrypt_many(vec![props])
				.await?
				.first()
				.cloned()
				.ok_or(WalletError::WalletEncrypt)
		}
	}
}
