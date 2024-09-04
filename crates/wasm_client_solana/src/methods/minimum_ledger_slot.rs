use serde::Deserialize;
use serde::Serialize;
use solana_sdk::clock::Slot;

use crate::impl_http_method;

#[derive(Debug, Serialize)]
pub struct MinimumLedgerSlotRequest;

impl_http_method!(MinimumLedgerSlotRequest, "minimumLedgerSlot");

#[derive(Debug, Deserialize)]
pub struct MinimumLedgerSlotResponse(Slot);

impl From<MinimumLedgerSlotResponse> for Slot {
	fn from(val: MinimumLedgerSlotResponse) -> Self {
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
			.method(MinimumLedgerSlotRequest::NAME)
			.id(1)
			.params(MinimumLedgerSlotRequest)
			.build();

		let value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1, "method":"minimumLedgerSlot"}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(value == raw_value);
		insta::assert_json_snapshot!(value, @"");
	}

	#[test]
	fn response() {
		let raw_json = r#"{ "jsonrpc": "2.0", "result": 1234, "id": 1 }"#;

		let response: ClientResponse<MinimumLedgerSlotResponse> =
			serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");
		check!(response.result.0 == 1234);
	}
}
