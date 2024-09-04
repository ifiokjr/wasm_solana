use std::future::Future;

use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::Wallet;
use crate::WalletAccountInfo;
use crate::WalletResult;

pub const STANDARD_CONNECT: &str = "standard:connect";

pub trait StandardConnectOutput {
	type Account: WalletAccountInfo;

	fn accounts(&self) -> Vec<Self::Account>;
}

#[derive(Default, Debug, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "browser", wasm_bindgen::prelude::wasm_bindgen)]
pub struct StandardConnectInput {
	/// By default, using the {@link `StandardConnectFeature`} should prompt the
	/// user to request authorization to accounts. Set the `silent` flag to
	/// `true` to request accounts that have already been authorized without
	/// prompting.
	///
	/// This flag may or may not be used by the Wallet and the app should not
	/// depend on it being used. If this flag is used by the Wallet, the Wallet
	/// should not prompt the user, and should return only the accounts that the
	/// app is authorized to use.
	#[builder(default, setter(into, strip_option))]
	silent: Option<bool>,
}

#[async_trait(?Send)]
pub trait WalletStandardConnect: Wallet {
	async fn connect(&self) -> WalletResult<Vec<Self::Account>>;
	async fn connect_with_options(
		&self,
		options: StandardConnectInput,
	) -> WalletResult<Vec<Self::Account>>;
	async fn connect_mut(&mut self) -> WalletResult<Vec<Self::Account>>;
	async fn connect_with_options_mut(
		&mut self,
		options: StandardConnectInput,
	) -> WalletResult<Vec<Self::Account>>;
}
