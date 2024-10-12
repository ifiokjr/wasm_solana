#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/readme.md"))]

pub use crate::client::*;
pub use crate::constants::*;
pub use crate::errors::*;
pub use crate::extensions::*;
pub use crate::methods::*;
pub use crate::providers::*;
pub use crate::rpc_config::*;
pub use crate::solana_client::*;
pub use crate::utils::spawn_local;

mod client;
mod constants;
mod errors;
mod extensions;
mod methods;
pub mod nonce_utils;
mod providers;
pub mod rpc_config;
pub mod rpc_filter;
pub mod rpc_response;
mod rpc_sender;
pub mod runtime;
pub mod solana_account_decoder;
mod solana_client;
pub mod solana_config_program;
pub mod solana_rpc_client_api;
pub mod solana_transaction_status;
pub mod utils;

pub mod prelude {
	pub use futures::FutureExt;
	pub use futures::SinkExt;
	pub use futures::StreamExt;
	pub use futures::TryFutureExt;
	pub use futures::TryStreamExt;
	pub use wallet_standard::prelude::*;

	pub use crate::RpcProvider;
	pub use crate::extensions::VersionedMessageExtension;
	pub use crate::extensions::VersionedTransactionExtension;
	pub use crate::solana_account_decoder::ToUiAccount;
}
