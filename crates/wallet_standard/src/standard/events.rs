use crate::WalletResult;

pub const STANDARD_EVENTS: &str = "standard:events";

pub trait StandardEventProperties {
	type Features;
	type WalletAccount;

	/// {@link "@wallet-standard/base".Wallet.chains | Chains} supported by the
	/// Wallet.
	///
	/// The Wallet should only define this field if the value of the property
	/// has changed.
	///
	/// The value must be the **new** value of the property.
	fn chains(&self) -> Option<Vec<String>>;
	/// {@link "@wallet-standard/base".Wallet.features | Features} supported by
	/// the Wallet.
	///
	/// The Wallet should only define this field if the value of the property
	/// has changed.
	///
	/// The value must be the **new** value of the property.
	fn features(&self) -> Option<Self::Features>;
	/// {@link "@wallet-standard/base".Wallet.accounts | Accounts} that the app
	/// is authorized to use.
	///
	/// The Wallet should only define this field if the value of the property
	/// has changed.
	///
	/// The value must be the **new** value of the property.
	fn accounts(&self) -> Option<Vec<Self::WalletAccount>>;
}

pub trait ConnectedWalletStandardEvents {
	type Callback;
	type Properties: StandardEventProperties;

	/// Listen for changes to the Wallet's properties.
	fn on(&self, event: impl AsRef<str>, callback: &Self::Callback) -> WalletResult<Box<dyn Fn()>>;
}
