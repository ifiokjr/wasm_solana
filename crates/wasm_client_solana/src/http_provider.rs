use serde_json::Value;
#[cfg(feature = "ssr")]
pub use ssr_http_provider::HttpProvider;
#[cfg(not(feature = "ssr"))]
pub use wasm_http_provider::HttpProvider;

use crate::ClientRequest;
use crate::ClientResponse;
use crate::ClientResult;
use crate::ErrorDetails;
use crate::SolanaRpcClientError;
use crate::DEFAULT_ERROR_CODE;

#[cfg(feature = "ssr")]
mod ssr_http_provider {
	use reqwest::header::HeaderMap;
	use reqwest::header::CONTENT_TYPE;
	use reqwest::Client;

	use super::*;
	#[derive(Clone)]
	pub struct HttpProvider {
		client: Client,
		headers: HeaderMap,
		url: String,
	}

	impl HttpProvider {
		pub fn new(url: impl Into<String>) -> Self {
			let client = Client::new();
			let url = url.into();
			let mut headers = HeaderMap::new();
			headers.append(CONTENT_TYPE, "application/json".parse().unwrap());

			Self {
				client,
				headers,
				url,
			}
		}

		pub fn url(&self) -> &str {
			&self.url
		}

		pub async fn send(&self, request: &ClientRequest) -> ClientResult<ClientResponse> {
			let request_result: Value = self
				.client
				.post(&self.url)
				.headers(self.headers.clone())
				.json(&request)
				.send()
				.await?
				.json()
				.await?;

			match serde_json::from_value::<ClientResponse>(request_result.clone()) {
				Ok(response) => Ok(response),
				Err(_) => {
					Err(serde_json::from_value::<SolanaRpcClientError>(request_result).unwrap())
				}
			}
		}
	}

	impl From<reqwest::Error> for SolanaRpcClientError {
		fn from(error: reqwest::Error) -> Self {
			let message = error.to_string();
			let code = i32::from(error.status().map_or(DEFAULT_ERROR_CODE, |s| s.as_u16()));
			let error = ErrorDetails { code, message };

			SolanaRpcClientError {
				error,
				..Default::default()
			}
		}
	}
}

mod wasm_http_provider {
	use gloo_net::http::Request;

	use super::*;

	#[derive(Clone)]
	pub struct HttpProvider(String);

	impl HttpProvider {
		pub fn new(url: impl Into<String>) -> Self {
			Self(url.into())
		}

		pub fn url(&self) -> &str {
			&self.0
		}

		pub async fn send(&self, request: &ClientRequest) -> ClientResult<ClientResponse> {
			let result: Value = Request::post(&self.0)
				.json(request)?
				.send()
				.await?
				.json()
				.await?;

			let Ok(response) = serde_json::from_value::<ClientResponse>(result.clone()) else {
				let error: SolanaRpcClientError =
					serde_json::from_value(result).unwrap_or_default();

				return Err(error);
			};

			Ok(response)
		}
	}

	impl From<gloo_net::Error> for SolanaRpcClientError {
		fn from(error: gloo_net::Error) -> Self {
			let message = error.to_string();
			let code = i32::from(DEFAULT_ERROR_CODE);
			let error = ErrorDetails { code, message };

			SolanaRpcClientError {
				error,
				..Default::default()
			}
		}
	}
}

pub const DEVNET: &str = "https://api.devnet.solana.com";
pub const TESTNET: &str = "https://api.testnet.solana.com";
pub const MAINNET: &str = "https://api.mainnet-beta.solana.com";
pub const LOCALNET: &str = "http://127.0.0.1:8899";
pub const DEBUG: &str = "http://34.90.18.145:8899";
