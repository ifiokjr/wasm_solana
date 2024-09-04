use serde::Deserialize;
use serde_tuple::Serialize_tuple;
use serde_with::skip_serializing_none;
use solana_sdk::clock::Slot;
use solana_sdk::commitment_config::CommitmentConfig;

use crate::impl_http_method;

#[skip_serializing_none]
#[derive(Debug, Default, Serialize_tuple)]
pub struct GetSlotRequest {
	config: Option<CommitmentConfig>,
}

impl_http_method!(GetSlotRequest, "getSlot");

impl GetSlotRequest {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn new_with_config(config: CommitmentConfig) -> Self {
		Self {
			config: Some(config),
		}
	}
}

#[derive(Debug, Deserialize)]
pub struct GetSlotResponse(Slot);

impl From<GetSlotResponse> for Slot {
	fn from(val: GetSlotResponse) -> Self {
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
		let request = ClientRequest::new(GetSlotRequest::NAME)
			.id(1)
			.params(GetSlotRequest::new());

		let ser_value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1, "method":"getSlot"}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(ser_value == raw_value);
		insta::assert_json_snapshot!(ser_value, @"");
	}

	#[test]
	fn response() {
		let raw_json = r#"{ "jsonrpc": "2.0", "result": 1234, "id": 1 }"#;

		let response: ClientResponse<GetSlotResponse> = serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");
		check!(response.result.0 == 1234);
	}
}
