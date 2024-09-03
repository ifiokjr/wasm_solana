#![allow(unsafe_code)]

use std::future::Future;

use wallet_standard::Wallet;
use wallet_standard::WalletError;
use wallet_standard::WalletResult;
use wallet_standard::WalletStandardDisconnect;
use wallet_standard::STANDARD_DISCONNECT;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::BrowserWallet;
use crate::FeatureFromJs;

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

impl FeatureFromJs for StandardDisconnectFeature {
	const NAME: &'static str = STANDARD_DISCONNECT;
}

impl StandardDisconnectFeature {
	pub async fn disconnect(&self) -> WalletResult<()> {
		self._disconnect().await?;
		Ok(())
	}
}

impl WalletStandardDisconnect for BrowserWallet {
	fn disconnect(&self) -> impl Future<Output = WalletResult<()>> {
		async move {
			self.wallet
				.get_feature::<StandardDisconnectFeature>()?
				.disconnect()
				.await
		}
	}

	fn disconnect_mut(&mut self) -> impl Future<Output = WalletResult<()>> {
		async move {
			if !self.connected() {
				return Err(WalletError::WalletDisconnected);
			}

			self.disconnect().await?;
			self.wallet_account = None;

			Ok(())
		}
	}
}
