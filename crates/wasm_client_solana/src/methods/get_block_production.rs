use serde::Deserialize;
use serde_tuple::Serialize_tuple;
use serde_with::skip_serializing_none;

use super::Context;
use crate::impl_http_method;
use crate::rpc_config::RpcBlockProductionConfig;
use crate::rpc_response::RpcBlockProduction;

#[skip_serializing_none]
#[derive(Debug, Serialize_tuple, Default)]
pub struct GetBlockProductionRequest {
	pub config: Option<RpcBlockProductionConfig>,
}

impl_http_method!(GetBlockProductionRequest, "getBlockProduction");

impl GetBlockProductionRequest {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn new_with_config(config: RpcBlockProductionConfig) -> Self {
		Self {
			config: Some(config),
		}
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct GetBlockProductionResponse {
	pub context: Context,
	pub value: RpcBlockProduction,
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;

	use assert2::check;

	use super::*;
	use crate::methods::HttpMethod;
	use crate::rpc_response::RpcBlockProductionRange;
	use crate::ClientRequest;
	use crate::ClientResponse;

	#[test]
	fn request() {
		let request = ClientRequest::builder()
			.method(GetBlockProductionRequest::NAME)
			.id(1)
			.params(GetBlockProductionRequest::new())
			.build();

		insta::assert_compact_json_snapshot!(request, @r###"{"jsonrpc": "2.0", "id": 1, "method": "getBlockProduction"}"###);
	}

	#[test]
	fn response() {
		let raw_json = r#"{"jsonrpc":"2.0","result":{"context":{"slot":9887},"value":{"byIdentity":{"85iYT5RuzRTDgjyRa3cP8SYhM2j21fj7NhfJ3peu1DPr":[9888,9886]},"range":{"firstSlot":0,"lastSlot":9887}}},"id":1}"#;

		let response: ClientResponse<GetBlockProductionResponse> =
			serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");
		check!(response.result.context.slot == 9887);

		let value = response.result.value;
		check!(
			value.by_identity
				== HashMap::from_iter([(
					"85iYT5RuzRTDgjyRa3cP8SYhM2j21fj7NhfJ3peu1DPr".to_string(),
					(9888, 9886)
				)])
		);
		check!(
			value.range
				== RpcBlockProductionRange {
					first_slot: 0,
					last_slot: 9887
				}
		);
	}
}
