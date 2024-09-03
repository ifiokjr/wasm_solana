use std::future::Future;

use crate::Wallet;
use crate::WalletResult;

pub const STANDARD_DISCONNECT: &str = "standard:disconnect";

pub trait WalletStandardDisconnect: Wallet {
	fn disconnect(&self) -> impl Future<Output = WalletResult<()>>;
	fn disconnect_mut(&mut self) -> impl Future<Output = WalletResult<()>>;
}
