use serde::Deserialize;
use serde::Serialize;

use crate::impl_http_method;

#[derive(Debug, Serialize)]
pub struct GetHealthRequest;

impl_http_method!(GetHealthRequest, "getHealth");

#[derive(Debug, Deserialize)]
pub struct ErrorValue {
	pub code: i32,
	pub message: String,
	pub data: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct GetHealthResponse(pub String);

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
		let request = ClientRequest::builder()
			.method(GetHealthRequest::NAME)
			.id(1)
			.params(GetHealthRequest)
			.build();

		let ser_value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1, "method":"getHealth"}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(ser_value == raw_value);
	}

	#[test]
	fn response() {
		let raw_json = r#"{ "jsonrpc": "2.0", "result": "ok", "id": 1 }"#;

		let response: ClientResponse<GetHealthResponse> = serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");
		check!(response.result.0 == "ok");
	}
}
