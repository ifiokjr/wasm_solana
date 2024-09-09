use crate::WalletStandardConnect;
use crate::WalletStandardDisconnect;

pub trait WalletInfo {
	type Account: WalletAccountInfo;

	/// {@link `WalletVersion` | Version} of the Wallet Standard implemented by
	/// the Wallet.
	///
	/// Must be read-only, static, and canonically defined by the Wallet
	/// Standard.
	fn version(&self) -> String;
	/// Name of the Wallet. This may be displayed by the app.
	///
	/// Must be read-only, static, descriptive, unique, and canonically defined
	/// by the wallet extension or application.
	fn name(&self) -> String;
	/// {@link `WalletIcon` | Icon} of the Wallet. This may be displayed by the
	/// app.
	///
	/// Must be read-only, static, and canonically defined by the wallet
	/// extension or application.
	fn icon(&self) -> String;
	/// Chains supported by the Wallet.
	///
	/// A **chain** is an {@link `IdentifierString`} which identifies a
	/// blockchain in a canonical, human-readable format. [CAIP-2](https://github.com/ChainAgnostic/CAIPs/blob/master/CAIPs/caip-2.md) chain IDs are compatible with this,
	/// but are not required to be used.
	///
	/// Each blockchain should define its own **chains** by extension of the
	/// Wallet Standard, using its own namespace. The `standard` and
	/// `experimental` namespaces are reserved by the Wallet Standard.
	///
	/// The {@link "@wallet-standard/features".EventsFeature | `standard:events`
	/// feature} should be used to notify the app if the value changes.
	fn chains(&self) -> Vec<String>;
	/// Features supported by the Wallet.
	///
	/// A **feature name** is an {@link `IdentifierString`} which identifies a
	/// **feature** in a canonical, human-readable format.
	///
	/// Each blockchain should define its own features by extension of the
	/// Wallet Standard.
	///
	/// The `standard` and `experimental` namespaces are reserved by the Wallet
	/// Standard.
	///
	/// A **feature** may have any type. It may be a single method or value, or
	/// a collection of them.
	///
	/// A **conventional feature** has the following structure:
	///
	/// ```ts
	///  export type ExperimentalEncryptFeature = {
	///      // Name of the feature.
	///      'experimental:encrypt': {
	///          // Version of the feature.
	///          version: '1.0.0';
	///          // Properties of the feature.
	///          ciphers: readonly 'x25519-xsalsa20-poly1305'[];
	///          // Methods of the feature.
	///          encrypt (data: Uint8Array): Promise<Uint8Array>;
	///      };
	///  };
	/// ```
	///
	/// The {@link "@wallet-standard/features".EventsFeature | `standard:events`
	/// feature} should be used to notify the app if the value changes.
	fn features(&self) -> Vec<String>;
	/// {@link `WalletAccount` | Accounts} that the app is authorized to use.
	///
	/// This can be set by the Wallet so the app can use authorized accounts on
	/// the initial page load.
	///
	/// The {@link "@wallet-standard/features".ConnectFeature |
	/// `standard:connect` feature} should be used to obtain authorization to
	/// the accounts.
	///
	/// The {@link "@wallet-standard/features".EventsFeature | `standard:events`
	/// feature} should be used to notify the app if the value changes.
	fn accounts(&self) -> Vec<Self::Account>;
}

/// Interface of a **`WalletAccount`**, also referred to as an **Account**.
///
/// An account is a _read-only data object_ that is provided from the Wallet to
/// the app, authorizing the app to use it.
///
/// The app can use an account to display and query information from a chain.
///
/// The app can also act using an account by passing it to {@link
/// Wallet.features | features} of the Wallet.
///
/// Wallets may use or extend {@link
/// "@wallet-standard/wallet".ReadonlyWalletAccount} which implements this
/// interface.
pub trait WalletAccountInfo {
	/// Address of the account, corresponding with a public key.
	fn address(&self) -> String;
	/// Public key of the account, corresponding with a secret key to use.
	fn public_key(&self) -> Vec<u8>;
	/// Chains supported by the account.
	///
	/// This must be a subset of the {@link Wallet.chains | chains} of the
	/// Wallet.
	fn chains(&self) -> Vec<String>;
	/// Feature names supported by the account.
	///
	/// This must be a subset of the names of {@link Wallet.features | features}
	/// of the Wallet.
	fn features(&self) -> Vec<String>;
	/// Optional user-friendly descriptive label or name for the account. This
	/// may be displayed by the app.
	fn label(&self) -> Option<String>;
	/// Optional user-friendly icon for the account. This may be displayed by
	/// the app.
	fn icon(&self) -> Option<String>;
}

pub trait Wallet {
	type Wallet: WalletInfo;
	type Account: WalletAccountInfo;

	fn wallet(&self) -> Self::Wallet;
	fn wallet_account(&self) -> Option<Self::Account>;

	fn name(&self) -> String {
		self.wallet().name()
	}

	fn icon(&self) -> String {
		self.wallet().icon()
	}

	fn connected(&self) -> bool {
		self.wallet_account().is_some()
	}

	fn try_public_key(&self) -> Option<Vec<u8>> {
		self.wallet_account().map(|account| account.public_key())
	}

	fn public_key(&self) -> Vec<u8> {
		self.try_public_key().unwrap()
	}
}

pub trait WalletStandard: WalletStandardConnect + WalletStandardDisconnect + Wallet {}

impl<T> WalletStandard for T where T: WalletStandardConnect + WalletStandardDisconnect + Wallet {}
