use derive_more::derive::From;
use derive_more::derive::Into;
use serde::Deserialize;
use serde::Serialize;

use crate::impl_http_method;

#[derive(Debug, Serialize)]
pub struct GetMaxRetransmitSlotRequest;

impl_http_method!(GetMaxRetransmitSlotRequest, "getMaxRetransmitSlot");

#[derive(Debug, Deserialize, From, Into)]
pub struct GetMaxRetransmitSlotResponse(u64);

#[cfg(test)]
mod tests {
	use serde_json::Value;

	use super::*;
	use crate::methods::HttpMethod;
	use crate::ClientRequest;
	use crate::ClientResponse;

	#[test]
	fn request() {
		let request = ClientRequest::builder()
			.method(GetMaxRetransmitSlotRequest::NAME)
			.id(1)
			.params(GetMaxRetransmitSlotRequest)
			.build();

		insta::assert_json_snapshot!(request, @"");

		let ser_value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1, "method":"getMaxRetransmitSlot"}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		assert_eq!(ser_value, raw_value);
	}

	#[test]
	fn response() {
		let raw_json = r#"{ "jsonrpc": "2.0", "result": 1234, "id": 1 }"#;

		let response: ClientResponse<GetMaxRetransmitSlotResponse> =
			serde_json::from_str(raw_json).unwrap();

		assert_eq!(response.id, 1);
		assert_eq!(response.jsonrpc, "2.0");
		assert_eq!(response.result.0, 1234);
	}
}
