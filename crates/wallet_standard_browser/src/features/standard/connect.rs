#![allow(unsafe_code)]
use std::future::Future;

use wallet_standard::StandardConnectInput;
use wallet_standard::StandardConnectOutput;
use wallet_standard::WalletError;
use wallet_standard::WalletResult;
use wallet_standard::WalletStandardConnect;
use wallet_standard::STANDARD_CONNECT;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

use crate::BrowserWallet;
use crate::BrowserWalletAccountInfo;
use crate::FeatureFromJs;

#[wasm_bindgen]
extern "C" {
	#[derive(Clone, Debug)]
	pub type BrowserStandardConnectOutput;
	/// List of accounts in the `crate::StandardWallet` that the
	/// app has been authorized to use.
	#[wasm_bindgen(method, getter, js_name = accounts)]
	pub fn _accounts(this: &BrowserStandardConnectOutput) -> Vec<BrowserWalletAccountInfo>;
	#[derive(Clone, Debug)]
	pub type StandardConnectFeature;
	#[wasm_bindgen(method, getter)]
	pub fn version(this: &StandardConnectFeature) -> String;
	#[allow(unused_qualifications)]
	#[wasm_bindgen(method, catch, js_name = connect)]
	pub async fn _connect(
		this: &StandardConnectFeature,
		input: StandardConnectInput,
	) -> Result<JsValue, JsValue>;
}

impl StandardConnectOutput for BrowserStandardConnectOutput {
	type Account = BrowserWalletAccountInfo;

	fn accounts(&self) -> Vec<Self::Account> {
		self._accounts()
	}
}

impl FeatureFromJs for StandardConnectFeature {
	const NAME: &'static str = STANDARD_CONNECT;
}

impl StandardConnectFeature {
	pub async fn connect(&self) -> WalletResult<Vec<BrowserWalletAccountInfo>> {
		self.connect_with_options(StandardConnectInput::default())
			.await
	}

	pub async fn connect_with_options(
		&self,
		options: StandardConnectInput,
	) -> WalletResult<Vec<BrowserWalletAccountInfo>> {
		let result: BrowserStandardConnectOutput = self._connect(options).await?.unchecked_into();

		Ok(result.accounts())
	}
}

impl WalletStandardConnect for BrowserWallet {
	/// Connect the account and automatically update the attached account.
	fn connect_mut(&mut self) -> impl Future<Output = WalletResult<Vec<Self::Account>>> {
		self.connect_with_options_mut(StandardConnectInput::default())
	}

	/// Connect the account and automatically update the attached account.
	#[allow(clippy::manual_async_fn)]
	fn connect_with_options_mut(
		&mut self,
		options: StandardConnectInput,
	) -> impl Future<Output = WalletResult<Vec<Self::Account>>> {
		async move {
			let accounts = self.connect_with_options(options).await?;
			let account = accounts
				.first()
				.cloned()
				.ok_or(WalletError::WalletConnection)?;
			self.wallet_account = Some(account);

			Ok(accounts)
		}
	}

	fn connect(&self) -> impl Future<Output = WalletResult<Vec<Self::Account>>> {
		self.connect_with_options(StandardConnectInput::default())
	}

	fn connect_with_options(
		&self,
		options: StandardConnectInput,
	) -> impl Future<Output = WalletResult<Vec<Self::Account>>> {
		async move {
			self.wallet
				.get_feature::<StandardConnectFeature>()?
				.connect_with_options(options)
				.await
		}
	}
}
