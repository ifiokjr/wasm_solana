use serde::Deserialize;
use serde::Serialize;
use solana_sdk::clock::Slot;

use crate::impl_http_method;

#[derive(Debug, Serialize)]
pub struct MinimumLedgerSlotRequest;

impl_http_method!(MinimumLedgerSlotRequest, "minimumLedgerSlot");

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct MinimumLedgerSlotResponse(Slot);

impl From<MinimumLedgerSlotResponse> for Slot {
	fn from(val: MinimumLedgerSlotResponse) -> Self {
		val.0
	}
}

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
			.method(MinimumLedgerSlotRequest::NAME)
			.id(1)
			.params(MinimumLedgerSlotRequest)
			.build();
		insta::assert_compact_json_snapshot!(request, @r###"{"jsonrpc": "2.0", "id": 1, "method": "minimumLedgerSlot"}"###);
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
