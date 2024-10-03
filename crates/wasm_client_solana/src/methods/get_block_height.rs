use serde::Deserialize;
use serde_tuple::Serialize_tuple;
use serde_with::skip_serializing_none;
use solana_sdk::commitment_config::CommitmentConfig;

use crate::impl_http_method;

#[skip_serializing_none]
#[derive(Debug, Serialize_tuple, Default)]
pub struct GetBlockHeightRequest {
	pub config: Option<CommitmentConfig>,
}

impl_http_method!(GetBlockHeightRequest, "getBlockHeight");

impl GetBlockHeightRequest {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn new_with_config(config: CommitmentConfig) -> Self {
		Self {
			config: Some(config),
		}
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct GetBlockHeightResponse(u64);

impl From<GetBlockHeightResponse> for u64 {
	fn from(value: GetBlockHeightResponse) -> Self {
		value.0
	}
}

#[cfg(test)]
mod tests {
	use assert2::check;

	use super::*;
	use crate::ClientRequest;
	use crate::ClientResponse;
	use crate::methods::HttpMethod;

	#[test]
	fn request() {
		let request = ClientRequest::builder()
			.method(GetBlockHeightRequest::NAME)
			.id(1)
			.params(GetBlockHeightRequest::new())
			.build();

		insta::assert_compact_json_snapshot!(request, @r###"{"jsonrpc": "2.0", "id": 1, "method": "getBlockHeight"}"###);
	}

	#[test]
	fn response() {
		let raw_json = r#"{"jsonrpc":"2.0","result":1233,"id":1}"#;

		let response: ClientResponse<GetBlockHeightResponse> =
			serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");

		check!(response.result.0 == 1233);
	}
}
