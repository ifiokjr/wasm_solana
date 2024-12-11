#![allow(unsafe_code)]

use js_sys::Function;
use js_sys::Object;
use wallet_standard::ConnectedWalletStandardEvents;
use wallet_standard::StandardEventProperties;
use wallet_standard::WalletResult;
use wallet_standard::STANDARD_EVENTS;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;

use crate::impl_feature_from_js;
use crate::BrowserWallet;
use crate::BrowserWalletAccountInfo;

#[wasm_bindgen]
extern "C" {
	#[derive(Clone, Debug)]
	pub type BrowserStandardEventsProperties;
	/// {@link "@wallet-standard/base".Wallet.chains | Chains} supported by the
	/// Wallet.
	///
	/// The Wallet should only define this field if the value of the property
	/// has changed.
	///
	/// The value must be the **new** value of the property.
	#[wasm_bindgen(method, getter, js_name = chains)]
	pub fn _chains(this: &BrowserStandardEventsProperties) -> Option<Vec<String>>;
	/// {@link "@wallet-standard/base".Wallet.features | Features} supported by
	/// the Wallet.
	///
	/// The Wallet should only define this field if the value of the property
	/// has changed.
	///
	/// The value must be the **new** value of the property.
	#[wasm_bindgen(method, getter, js_name = features)]
	pub fn _features(this: &BrowserStandardEventsProperties) -> Option<Object>;
	/// {@link "@wallet-standard/base".Wallet.accounts | Accounts} that the app
	/// is authorized to use.
	///
	/// The Wallet should only define this field if the value of the property
	/// has changed.
	///
	/// The value must be the **new** value of the property.
	#[wasm_bindgen(method, getter, js_name = accounts)]
	pub fn _accounts(
		this: &BrowserStandardEventsProperties,
	) -> Option<Vec<BrowserWalletAccountInfo>>;
	#[derive(Clone, Debug)]
	pub type StandardEventsFeature;
	#[wasm_bindgen(method, getter)]
	pub fn version(this: &StandardEventsFeature) -> String;
	#[wasm_bindgen(method, js_name = on)]
	pub fn on(
		this: &StandardEventsFeature,
		event: &str,
		callback: &Closure<dyn Fn(BrowserStandardEventsProperties)>,
	) -> Function;
}

impl StandardEventProperties for BrowserStandardEventsProperties {
	type Features = Object;
	type WalletAccount = BrowserWalletAccountInfo;

	fn chains(&self) -> Option<Vec<String>> {
		self._chains()
	}

	fn features(&self) -> Option<Self::Features> {
		self._features()
	}

	fn accounts(&self) -> Option<Vec<Self::WalletAccount>> {
		self._accounts()
	}
}

impl_feature_from_js!(StandardEventsFeature, STANDARD_EVENTS);

impl ConnectedWalletStandardEvents for BrowserWallet {
	type Callback = Closure<dyn Fn(BrowserStandardEventsProperties)>;

	fn on(&self, event: impl AsRef<str>, callback: &Self::Callback) -> WalletResult<Box<dyn Fn()>> {
		let feature = self.wallet.get_feature::<StandardEventsFeature>()?;
		let dispose = feature.on(event.as_ref(), callback);

		Ok(Box::new(move || {
			let _ = dispose.call0(&JsValue::NULL);
		}))
	}
}
