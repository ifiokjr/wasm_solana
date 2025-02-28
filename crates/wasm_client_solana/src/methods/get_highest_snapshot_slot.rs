use serde::Deserialize;
use serde::Serialize;

use crate::impl_http_method;

#[derive(Debug, Serialize)]
pub struct GetHighestSnapshotSlotRequest;

impl_http_method!(GetHighestSnapshotSlotRequest, "getHighestSnapshotSlot");

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct GetHighestSnapshotSlotResponse {
	pub full: u64,
	pub incremental: Option<u64>,
}

#[cfg(test)]
mod tests {

	use super::*;
	use crate::methods::HttpMethod;
	use crate::ClientRequest;
	use crate::ClientResponse;

	#[test]
	fn request() {
		let request = ClientRequest::builder()
			.method(GetHighestSnapshotSlotRequest::NAME)
			.id(1)
			.params(GetHighestSnapshotSlotRequest)
			.build();

		insta::assert_compact_json_snapshot!(request, @r###"{"jsonrpc": "2.0", "id": 1, "method": "getHighestSnapshotSlot"}"###);
	}

	#[test]
	fn response() {
		let raw_json = r#"{"jsonrpc":"2.0","result":{"full":100,"incremental":110},"id":1}"#;

		let response: ClientResponse<GetHighestSnapshotSlotResponse> =
			serde_json::from_str(raw_json).unwrap();

		assert_eq!(response.id, 1);
		assert_eq!(response.jsonrpc, "2.0");
		assert_eq!(response.result.full, 100);
		assert_eq!(response.result.incremental, Some(110));
	}
}
