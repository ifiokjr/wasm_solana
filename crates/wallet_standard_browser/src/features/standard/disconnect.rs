#![allow(unsafe_code)]

use async_trait::async_trait;
use wallet_standard::Wallet;
use wallet_standard::WalletError;
use wallet_standard::WalletResult;
use wallet_standard::WalletStandardDisconnect;
use wallet_standard::STANDARD_DISCONNECT;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::impl_feature_from_js;
use crate::BrowserWallet;

#[wasm_bindgen]
extern "C" {
	#[derive(Clone, Debug)]
	pub type StandardDisconnectFeature;
	#[wasm_bindgen(method, getter)]
	pub fn version(this: &StandardDisconnectFeature) -> String;
	#[allow(unused_qualifications)]
	#[wasm_bindgen(method, catch, js_name = disconnect)]
	pub async fn _disconnect(this: &StandardDisconnectFeature) -> Result<(), JsValue>;
}

impl_feature_from_js!(StandardDisconnectFeature, STANDARD_DISCONNECT);

impl StandardDisconnectFeature {
	pub async fn disconnect(&self) -> WalletResult<()> {
		self._disconnect().await?;
		Ok(())
	}
}

#[async_trait(?Send)]
impl WalletStandardDisconnect for BrowserWallet {
	async fn disconnect(&self) -> WalletResult<()> {
		self.wallet
			.get_feature::<StandardDisconnectFeature>()?
			.disconnect()
			.await
	}

	async fn disconnect_mut(&mut self) -> WalletResult<()> {
		if !self.connected() {
			return Err(WalletError::WalletDisconnected);
		}

		self.disconnect().await?;
		self.wallet_account = None;

		Ok(())
	}
}
