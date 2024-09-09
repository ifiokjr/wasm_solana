use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, thiserror::Error, Eq, PartialEq, Serialize, Deserialize)]
pub enum WalletError {
	#[error("the arguments provided are not valid")]
	InvalidArguments,
	#[error("icon is not valid")]
	InvalidIcon,
	#[error("The identifier could not be parsed: {0}")]
	InvalidIdentifier(String),
	#[error("The signature is not valid")]
	InvalidSignature,
	#[error("Signer: {0}")]
	Signer(String),
	#[error("{0}")]
	Js(String),
	#[error("Parsing string failed: {0}")]
	ParseString(String),
	#[error(transparent)]
	#[cfg(feature = "solana")]
	Program(#[from] solana_sdk::program_error::ProgramError),
	#[error("an error occured during deserialization: {0}")]
	Serde(String),
	#[cfg(feature = "solana")]
	#[error(transparent)]
	Transaction(#[from] solana_sdk::transaction::TransactionError),
	#[error("the requested feature: `{feature}` is not supported for this wallet: `{wallet}`")]
	UnsupportedFeature { feature: String, wallet: String },
	#[error("icon type is not supported")]
	UnsupportedIconType,
	#[error("The transaction version is not supported by this wallet")]
	UnsupportedTransactionVersion,
	#[error("Wallet account not connected")]
	WalletAccount,
	#[error("The wallet configuration is invalid")]
	WalletConfig,
	#[error("An error occurred while connecting to the wallet")]
	WalletConnection,
	#[error("Could not decrypt the provided data")]
	WalletDecrypt,
	#[error("Action can't be performed because the wallet is disconnected")]
	WalletDisconnected,
	#[error("Error while disconnecting wallet")]
	WalletDisconnection,
	#[error("Could not encrypt the provided data")]
	WalletEncrypt,
	#[error("Wallet keypair")]
	WalletKeypair,
	#[error("Error loading the wallet")]
	WalletLoad,
	#[error("Wallet not connected")]
	WalletNotConnected,
	#[error("The wallet is not yet ready")]
	WalletNotReady,
	#[error("Invalid wallet public key")]
	WalletPublicKey,
	#[error("Wallet send transaction")]
	WalletSendTransaction,
	#[error("Wallet sign in")]
	WalletSignIn,
	#[error("Wallet sign in fields: {0}")]
	WalletSignInFields(String),
	#[error("Wallet sign message")]
	WalletSignMessage,
	#[error("Wallet sign transaction")]
	WalletSignTransaction,
	#[error("Wallet timeout")]
	WalletTimeout,
	#[error("Wallet window blocked")]
	WalletWindowBlocked,
	#[error("Wallet window closed")]
	WalletWindowClosed,
	#[error("Other: {0}")]
	Other(String),
}

impl From<core::fmt::Error> for WalletError {
	fn from(value: core::fmt::Error) -> Self {
		WalletError::ParseString(value.to_string())
	}
}

#[cfg(feature = "browser")]
#[allow(unused_qualifications)]
impl From<wasm_bindgen::JsValue> for WalletError {
	#[allow(deprecated)]
	fn from(source: wasm_bindgen::JsValue) -> Self {
		WalletError::Js(
			source
				.as_string()
				.unwrap_or("An error occurred in the JavaScript.".to_string()),
		)
	}
}
#[cfg(feature = "solana")]
impl From<solana_sdk::signer::SignerError> for WalletError {
	fn from(error: solana_sdk::signer::SignerError) -> Self {
		WalletError::Signer(error.to_string())
	}
}

#[cfg(feature = "browser")]
impl From<serde_wasm_bindgen::Error> for WalletError {
	fn from(source: serde_wasm_bindgen::Error) -> Self {
		WalletError::Serde(source.to_string())
	}
}

pub type WalletResult<T> = Result<T, WalletError>;

pub trait IntoWalletError: Display {}

impl<E: IntoWalletError> From<E> for WalletError {
	fn from(value: E) -> Self {
		WalletError::Other(value.to_string())
	}
}
