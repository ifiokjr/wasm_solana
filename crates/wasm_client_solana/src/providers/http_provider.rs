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
	use std::pin::Pin;
	use std::task::Context;
	use std::task::Poll;

	use futures::Future;
	use gloo_net::http::Request;
	use gloo_net::http::Response;
	use gloo_net::Error;
	use pin_project::pin_project;
	use pin_project::pinned_drop;
	use wasm_bindgen::prelude::*;
	use web_sys::AbortController;

	use super::*;
	use crate::ClientError;

	#[pin_project(PinnedDrop)]
	struct WrappedSend<F: Future<Output = Result<Response, Error>>> {
		#[pin]
		fut: F,
		controller: AbortController,
	}

	impl<F: Future<Output = Result<Response, Error>>> WrappedSend<F> {
		fn new(fut: F, controller: AbortController) -> Self {
			Self { fut, controller }
		}
	}

	impl<F: Future<Output = Result<Response, Error>>> Future for WrappedSend<F> {
		type Output = Result<Response, Error>;

		fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
			self.project().fut.as_mut().poll(cx)
		}
	}

	#[pinned_drop]
	impl<F: Future<Output = Result<Response, Error>>> PinnedDrop for WrappedSend<F> {
		fn drop(self: Pin<&mut Self>) {
			self.controller.abort();
		}
	}

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
			let controller = AbortController::new().unwrap_throw();
			let client_request = ClientRequest::builder()
				.method(T::NAME)
				.id(0)
				.params(request)
				.build();
			let request = Request::post(&self.0)
				.abort_signal(Some(&controller.signal()))
				.json(&client_request)?;
			let response = WrappedSend::new(request.send(), controller).await?;
			let result: Value = response.json().await?;

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
