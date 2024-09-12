use serde::de::DeserializeOwned;
use serde_json::Value;
#[cfg(feature = "ssr")]
pub use ssr_http_provider::HttpProvider;
#[cfg(not(feature = "ssr"))]
pub use wasm_http_provider::HttpProvider;

use crate::ClientRequest;
use crate::ClientResult;
use crate::HttpMethod;
use crate::RpcError;
use crate::RpcErrorDetails;
use crate::DEFAULT_ERROR_CODE;

#[cfg(feature = "ssr")]
mod ssr_http_provider {
	use reqwest::header::HeaderMap;
	use reqwest::header::CONTENT_TYPE;
	use reqwest::Client;

	use super::*;
	use crate::ClientError;

	#[derive(Debug, Clone)]
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

		pub async fn send<T: HttpMethod, R: DeserializeOwned>(
			&self,
			request: &T,
		) -> ClientResult<R> {
			let client_request = ClientRequest::builder()
				.method(T::NAME)
				.id(1)
				.params(request)
				.build();
			let result: Value = self
				.client
				.post(&self.url)
				.headers(self.headers.clone())
				.json(&client_request)
				.send()
				.await?
				.json()
				.await?;

			if let Ok(response) = serde_json::from_value::<R>(result.clone()) {
				Ok(response)
			} else {
				match serde_json::from_value::<RpcError>(result) {
					Ok(error) => Err(error.into()),
					Err(error) => Err(ClientError::Other(error.to_string())),
				}
			}
		}
	}

	impl From<reqwest::Error> for RpcError {
		fn from(error: reqwest::Error) -> Self {
			let message = error.to_string();
			let code = i32::from(error.status().map_or(DEFAULT_ERROR_CODE, |s| s.as_u16()));
			let error = RpcErrorDetails { code, message };

			RpcError {
				error,
				..Default::default()
			}
		}
	}

	impl From<reqwest::Error> for ClientError {
		fn from(value: reqwest::Error) -> Self {
			ClientError::Rpc(value.into())
		}
	}
}

#[cfg(not(feature = "ssr"))]
mod wasm_http_provider {
	use gloo_net::http::Request;

	use super::*;
	use crate::ClientError;

	#[derive(Debug, Clone)]
	pub struct HttpProvider(String);

	impl HttpProvider {
		pub fn new(url: impl Into<String>) -> Self {
			Self(url.into())
		}

		pub fn url(&self) -> &str {
			&self.0
		}

		pub async fn send<T: HttpMethod, R: DeserializeOwned>(
			&self,
			request: &T,
		) -> ClientResult<R> {
			let client_request = ClientRequest::builder()
				.method(T::NAME)
				.id(0)
				.params(request)
				.build();
			let result: Value = Request::post(&self.0)
				.json(&client_request)?
				.send()
				.await?
				.json()
				.await?;

			if let Ok(response) = serde_json::from_value::<R>(result.clone()) {
				Ok(response)
			} else {
				match serde_json::from_value::<RpcError>(result) {
					Ok(error) => Err(error.into()),
					Err(error) => Err(ClientError::Other(error.to_string())),
				}
			}
		}
	}

	impl From<gloo_net::Error> for RpcError {
		fn from(error: gloo_net::Error) -> Self {
			let message = error.to_string();
			let code = i32::from(DEFAULT_ERROR_CODE);
			let error = RpcErrorDetails { code, message };

			RpcError {
				error,
				..Default::default()
			}
		}
	}

	impl From<gloo_net::Error> for ClientError {
		fn from(value: gloo_net::Error) -> Self {
			Self::Rpc(value.into())
		}
	}
}

pub const DEVNET: &str = "https://api.devnet.solana.com";
pub const TESTNET: &str = "https://api.testnet.solana.com";
pub const MAINNET: &str = "https://api.mainnet-beta.solana.com";
pub const LOCALNET: &str = "http://127.0.0.1:8899";
pub const DEBUG: &str = "http://34.90.18.145:8899";
