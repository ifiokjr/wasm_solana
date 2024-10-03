#![allow(unsafe_code)]

use async_trait::async_trait;
use wallet_standard::STANDARD_DISCONNECT;
use wallet_standard::Wallet;
use wallet_standard::WalletError;
use wallet_standard::WalletResult;
use wallet_standard::WalletStandardDisconnect;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

use crate::BrowserWallet;
use crate::impl_feature_from_js;

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
	async fn disconnect(&mut self) -> WalletResult<()> {
		if !self.connected() {
			return Err(WalletError::WalletDisconnected);
		}

		self.disconnect().await?;
		self.wallet_account = None;

		Ok(())
	}
}
