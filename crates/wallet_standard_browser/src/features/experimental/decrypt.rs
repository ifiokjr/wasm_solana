#![allow(unsafe_code)]

use async_trait::async_trait;
use js_sys::Array;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;
use wallet_standard::ExperimentalDecryptOutput;
use wallet_standard::ExperimentalDecryptProps;
use wallet_standard::WalletError;
use wallet_standard::WalletExperimentalDecrypt;
use wallet_standard::WalletResult;
use wallet_standard::EXPERIMENTAL_DECRYPT;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

use crate::impl_feature_from_js;
use crate::BrowserWallet;
use crate::BrowserWalletAccountInfo;

#[wasm_bindgen]
extern "C" {
	#[derive(Clone, Debug)]
	pub type BrowserExperimentalDecryptOutput;
	/// `cleartext` that was decrypted.
	#[wasm_bindgen(method, getter, js_name = "cleartext")]
	pub fn _cleartext(this: &BrowserExperimentalDecryptOutput) -> Vec<u8>;
	#[derive(Clone, Debug)]
	pub type ExperimentalDecryptFeature;
	/// Version of the feature API.
	#[wasm_bindgen(method, getter)]
	pub fn version(this: &ExperimentalDecryptFeature) -> String;
	/// List of ciphers supported for decryption.
	#[wasm_bindgen(method, getter)]
	pub fn ciphers(this: &ExperimentalDecryptFeature) -> Vec<String>;
	/// Decrypt cleartexts using the account's secret key.
	///
	/// @param inputs Inputs for decryption.
	///
	/// @return Outputs of decryption.
	#[allow(unused_qualifications)]
	#[wasm_bindgen(method, variadic, catch)]
	pub async fn decrypt(
		this: &ExperimentalDecryptFeature,
		args: Array,
	) -> Result<JsValue, JsValue>;
}

impl ExperimentalDecryptOutput for BrowserExperimentalDecryptOutput {
	fn cleartext(&self) -> Vec<u8> {
		self._cleartext()
	}
}

impl_feature_from_js!(ExperimentalDecryptFeature, EXPERIMENTAL_DECRYPT);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
pub struct ExperimentalDecryptInput {
	/// Account to use.
	#[serde(with = "serde_wasm_bindgen::preserve")]
	pub account: BrowserWalletAccountInfo,

	#[serde(flatten)]
	pub props: ExperimentalDecryptProps,
}

#[async_trait(?Send)]
impl WalletExperimentalDecrypt for BrowserWallet {
	type Output = BrowserExperimentalDecryptOutput;

	async fn decrypt_many(
		&self,
		props: Vec<ExperimentalDecryptProps>,
	) -> WalletResult<Vec<Self::Output>> {
		let Some(ref wallet_account) = self.wallet_account else {
			return Err(WalletError::WalletAccount);
		};

		let input = props
			.into_iter()
			.map(|props| {
				ExperimentalDecryptInput::builder()
					.account(wallet_account.clone())
					.props(props)
					.build()
			})
			.collect::<Vec<_>>();

		let feature = self.wallet.get_feature::<ExperimentalDecryptFeature>()?;
		let inputs: Array = serde_wasm_bindgen::to_value(&input)?.unchecked_into();
		let result: Array = feature.decrypt(inputs).await?.unchecked_into();

		Ok(result.into_iter().map(JsCast::unchecked_into).collect())
	}

	async fn decrypt(&self, props: ExperimentalDecryptProps) -> WalletResult<Self::Output> {
		self.decrypt_many(vec![props])
			.await?
			.first()
			.cloned()
			.ok_or(WalletError::WalletDecrypt)
	}
}
