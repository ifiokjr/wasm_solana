use serde::Deserialize;
use serde_tuple::Serialize_tuple;
use serde_with::skip_serializing_none;
use solana_sdk::clock::Slot;
use solana_sdk::commitment_config::CommitmentConfig;

use crate::impl_http_method;

#[skip_serializing_none]
#[derive(Debug, Serialize_tuple)]
#[serde(rename_all = "camelCase")]
pub struct GetBlocksRequest {
	pub start_slot: Slot,
	pub end_slot: Option<Slot>,
	pub config: Option<CommitmentConfig>,
}

impl_http_method!(GetBlocksRequest, "getBlocks");

impl GetBlocksRequest {
	pub fn new(start_slot: Slot, end_slot: Option<Slot>) -> Self {
		Self {
			start_slot,
			end_slot,
			config: None,
		}
	}

	pub fn new_with_config(
		start_slot: Slot,
		end_slot: Option<Slot>,
		config: CommitmentConfig,
	) -> Self {
		Self {
			start_slot,
			end_slot,
			config: Some(config),
		}
	}
}

#[derive(Debug, Deserialize)]
pub struct GetBlocksResponse(Vec<Slot>);

impl From<GetBlocksResponse> for Vec<Slot> {
	fn from(value: GetBlocksResponse) -> Self {
		value.0
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
		let request = ClientRequest::new(GetBlocksRequest::NAME)
			.id(1)
			.params(GetBlocksRequest::new(5, Some(10)));

		let ser_value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1,"method":"getBlocks","params":[5,10]}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(ser_value == raw_value);
	}

	#[test]
	fn response() {
		let raw_json = r#"{"jsonrpc":"2.0","result":[5,6,7,8,9,10],"id":1}"#;

		let response: ClientResponse<GetBlocksResponse> = serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");
		check!(response.result.0 == vec![5, 6, 7, 8, 9, 10]);
	}
}
