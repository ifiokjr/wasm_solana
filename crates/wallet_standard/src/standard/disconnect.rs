use std::future::Future;

use async_trait::async_trait;

use crate::Wallet;
use crate::WalletResult;

pub const STANDARD_DISCONNECT: &str = "standard:disconnect";

#[async_trait(?Send)]
pub trait WalletStandardDisconnect: Wallet {
	async fn disconnect(&self) -> WalletResult<()>;
	async fn disconnect_mut(&mut self) -> WalletResult<()>;
}
