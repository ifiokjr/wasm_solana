use serde::Deserialize;
use serde::Serialize;
use solana_sdk::clock::Slot;

use crate::impl_http_method;

#[derive(Debug, Serialize)]
pub struct GetFirstAvailableBlockRequest;

impl_http_method!(GetFirstAvailableBlockRequest, "getFirstAvailableBlock");

#[derive(Debug, Deserialize)]
pub struct GetFirstAvailableBlockResponse(Slot);

impl From<GetFirstAvailableBlockResponse> for Slot {
	fn from(val: GetFirstAvailableBlockResponse) -> Self {
		val.0
	}
}

#[cfg(test)]
mod tests {
	use assert2::check;
	use serde_json::Value;

	use super::*;
	use crate::methods::HttpMethod;
	use crate::ClientRequest;
	use crate::ClientResponse;

	#[test]
	fn request() {
		let request = ClientRequest::new(GetFirstAvailableBlockRequest::NAME)
			.id(1)
			.params(GetFirstAvailableBlockRequest);

		let ser_value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1,"method":"getFirstAvailableBlock"}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(ser_value == raw_value);
	}

	#[test]
	fn response() {
		let raw_json = r#"{ "jsonrpc": "2.0", "result": 250000, "id": 1 }"#;

		let response: ClientResponse<GetFirstAvailableBlockResponse> =
			serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");
		check!(response.result.0 == 250_000);
	}
}
