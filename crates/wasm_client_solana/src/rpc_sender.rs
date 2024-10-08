use async_trait::async_trait;
use serde::de::DeserializeOwned;

use crate::ClientResult;
use crate::HttpMethod;

#[async_trait]
pub trait SolanaRpcSender {
	/// Send the request.
	async fn send<T: HttpMethod, R: DeserializeOwned>(&self, request: T) -> ClientResult<R>;
	/// Get the URL represented by this sender.
	fn url(&self) -> String;
}
