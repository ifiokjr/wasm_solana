#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/readme.md"))]

pub use error::*;
pub use experimental::*;
#[cfg(feature = "solana")]
pub use solana::*;
pub use standard::*;
pub use types::*;

mod error;
mod experimental;
#[cfg(feature = "solana")]
mod solana;
mod standard;
mod types;

pub mod prelude {
	#[cfg(feature = "solana")]
	pub use super::solana::prelude::*;
	pub use super::ExperimentalDecryptOutput;
	pub use super::ExperimentalEncryptOutput;
	pub use super::IntoWalletError;
	pub use super::StandardConnectOutput;
	pub use super::Wallet;
	pub use super::WalletAccountInfo;
	pub use super::WalletError;
	pub use super::WalletExperimentalDecrypt;
	pub use super::WalletExperimentalEncrypt;
	pub use super::WalletInfo;
	pub use super::WalletResult;
	pub use super::WalletStandard;
	pub use super::WalletStandardConnect;
	pub use super::WalletStandardDisconnect;
}
