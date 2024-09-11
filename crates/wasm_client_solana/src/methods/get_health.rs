use serde::Deserialize;
use serde::Serialize;

use crate::impl_http_method;

#[derive(Debug, Serialize)]
pub struct GetHealthRequest;

impl_http_method!(GetHealthRequest, "getHealth");

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct ErrorValue {
	pub code: i32,
	pub message: String,
	pub data: serde_json::Value,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct GetHealthResponse(pub String);

#[cfg(test)]
mod tests {
	use assert2::check;

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

		insta::assert_compact_json_snapshot!(request, @r###"{"jsonrpc": "2.0", "id": 1, "method": "getHealth"}"###);
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
