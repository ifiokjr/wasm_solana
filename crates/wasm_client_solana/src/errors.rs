use std::fmt;

use serde::Deserialize;
use serde::Serialize;

pub const DEFAULT_ERROR_CODE: u16 = 500u16;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ErrorDetails {
	pub(crate) code: i32,
	pub(crate) message: String,
}

impl Default for ErrorDetails {
	fn default() -> Self {
		let message = "Internal Server Error".into();
		let code = DEFAULT_ERROR_CODE.into();

		Self { code, message }
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolanaRpcClientError {
	pub(crate) id: u16,
	pub(crate) jsonrpc: String,
	pub(crate) error: ErrorDetails,
}

impl std::error::Error for SolanaRpcClientError {}

impl Default for SolanaRpcClientError {
	fn default() -> Self {
		Self {
			id: 0,
			jsonrpc: String::from("2.0"),
			error: ErrorDetails::default(),
		}
	}
}

impl SolanaRpcClientError {
	pub fn new(message: impl Into<String>) -> Self {
		let message = message.into();
		let code = 303;
		let error = ErrorDetails { code, message };

		SolanaRpcClientError {
			error,
			..Default::default()
		}
	}
}

impl fmt::Display for SolanaRpcClientError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(format!("Client error: {}", self.error.message).as_str())
	}
}

pub type ClientResult<T> = Result<T, SolanaRpcClientError>;
