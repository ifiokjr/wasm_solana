use serde::Deserialize;
use serde::Serialize;

use crate::impl_http_method;

#[derive(Debug, Serialize)]
pub struct GetGenesisHashRequest;

impl_http_method!(GetGenesisHashRequest, "getGenesisHash");

#[derive(Debug, Deserialize)]
pub struct GetGenesisHashResponse(String);

impl From<GetGenesisHashResponse> for String {
	fn from(val: GetGenesisHashResponse) -> Self {
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
		let request = ClientRequest::builder()
			.method(GetGenesisHashRequest::NAME)
			.id(1)
			.params(GetGenesisHashRequest)
			.build();

		insta::assert_json_snapshot!(request, @"");

		let ser_value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1, "method":"getGenesisHash"}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(ser_value == raw_value);
	}

	#[test]
	fn response() {
		let raw_json =
			r#"{"jsonrpc":"2.0","result":"GH7ome3EiwEr7tu9JuTh2dpYWBJK3z69Xm1ZE3MEE6JC","id":1}"#;

		let response: ClientResponse<GetGenesisHashResponse> =
			serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");

		check!(response.result.0 == "GH7ome3EiwEr7tu9JuTh2dpYWBJK3z69Xm1ZE3MEE6JC");
	}
}
