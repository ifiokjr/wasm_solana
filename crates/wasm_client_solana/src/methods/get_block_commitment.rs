use serde::Deserialize;
use serde_tuple::Serialize_tuple;
use solana_sdk::vote::state::MAX_LOCKOUT_HISTORY;

use crate::impl_http_method;

#[derive(Debug, Serialize_tuple)]
pub struct GetBlockCommitmentRequest {
	pub slot: u64,
}

impl_http_method!(GetBlockCommitmentRequest, "getBlockCommitment");

impl GetBlockCommitmentRequest {
	pub fn new(slot: u64) -> Self {
		Self { slot }
	}
}

type BlockCommitmentArray = [u64; MAX_LOCKOUT_HISTORY + 1];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockCommitmentResponse {
	pub commitment: Option<BlockCommitmentArray>,
	pub total_stake: u64,
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
		let request = ClientRequest::new(GetBlockCommitmentRequest::NAME)
			.id(1)
			.params(GetBlockCommitmentRequest::new(5));

		let ser_value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1,"method":"getBlockCommitment","params":[5]}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(ser_value == raw_value);
	}

	#[test]
	fn response() {
		let raw_json = r#"{"jsonrpc":"2.0","result":{"commitment":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,10,32],"totalStake":42},"id":1}"#;

		let response: ClientResponse<GetBlockCommitmentResponse> =
			serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");

		check!(response.result.total_stake == 42);
		check!(
			response.result.commitment
				== Some([
					0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
					0, 0, 0, 0, 10, 32
				])
		);
	}
}
