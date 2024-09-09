use serde::Deserialize;
use serde_tuple::Serialize_tuple;
use serde_with::skip_serializing_none;
use solana_sdk::commitment_config::CommitmentConfig;

use super::Context;
use crate::impl_http_method;

#[skip_serializing_none]
#[derive(Debug, Serialize_tuple)]
pub struct GetStakeMinimumDelegationRequest {
	config: Option<CommitmentConfig>,
}

impl_http_method!(
	GetStakeMinimumDelegationRequest,
	"getStakeMinimumDelegation"
);

impl Default for GetStakeMinimumDelegationRequest {
	fn default() -> Self {
		Self::new()
	}
}

impl GetStakeMinimumDelegationRequest {
	pub fn new() -> Self {
		Self { config: None }
	}

	pub fn new_with_config(config: CommitmentConfig) -> Self {
		Self {
			config: Some(config),
		}
	}
}

#[derive(Debug, Deserialize)]
pub struct GetStakeMinimumDelegationResponse {
	pub context: Context,
	pub value: u64,
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
			.method(GetStakeMinimumDelegationRequest::NAME)
			.id(1)
			.params(GetStakeMinimumDelegationRequest::new())
			.build();

		let ser_value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1,"method":"getStakeMinimumDelegation"}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(ser_value == raw_value);
	}

	#[test]
	fn response() {
		let raw_json =
			r#"{"jsonrpc":"2.0","result":{"context":{"slot":501},"value":1000000000},"id":1}"#;

		let response: ClientResponse<GetStakeMinimumDelegationResponse> =
			serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");
		check!(response.result.context.slot == 501);
		check!(response.result.value == 1_000_000_000);
	}
}
