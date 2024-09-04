use serde::Deserialize;
use serde_tuple::Serialize_tuple;
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use solana_sdk::pubkey::Pubkey;

use crate::impl_http_method;
use crate::rpc_response::RpcPrioritizationFee;

#[serde_as]
#[derive(Debug, Serialize_tuple)]
pub struct GetRecentPrioritizationFeesRequest {
	#[serde_as(as = "Option<Vec<DisplayFromStr>>")]
	accounts: Option<Vec<Pubkey>>,
}

impl_http_method!(
	GetRecentPrioritizationFeesRequest,
	"getRecentPrioritizationFees"
);

impl Default for GetRecentPrioritizationFeesRequest {
	fn default() -> Self {
		Self::new()
	}
}

impl GetRecentPrioritizationFeesRequest {
	pub fn new() -> Self {
		GetRecentPrioritizationFeesRequest { accounts: None }
	}

	pub fn new_with_accounts(accounts: Vec<Pubkey>) -> Self {
		GetRecentPrioritizationFeesRequest {
			accounts: Some(accounts),
		}
	}
}

#[derive(Debug, Deserialize)]
pub struct GetRecentPrioritizationFeesResponse(Vec<RpcPrioritizationFee>);

impl From<GetRecentPrioritizationFeesResponse> for Vec<RpcPrioritizationFee> {
	fn from(value: GetRecentPrioritizationFeesResponse) -> Self {
		value.0
	}
}

#[cfg(test)]
mod tests {
	use assert2::check;
	use serde_json::Value;
	use solana_sdk::pubkey;

	use super::*;
	use crate::methods::HttpMethod;
	use crate::ClientRequest;
	use crate::ClientResponse;

	#[test]
	fn request() {
		let request = ClientRequest::builder()
			.method(GetRecentPrioritizationFeesRequest::NAME)
			.id(1)
			.params(GetRecentPrioritizationFeesRequest::new_with_accounts(vec![
				pubkey!("CxELquR1gPP8wHe33gZ4QxqGB3sZ9RSwsJ2KshVewkFY"),
			]))
			.build();

		insta::assert_json_snapshot!(request, @"");

		let value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1,"method":"getRecentPrioritizationFees","params":[["CxELquR1gPP8wHe33gZ4QxqGB3sZ9RSwsJ2KshVewkFY"]]}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(value == raw_value);
	}

	#[test]
	fn response() {
		let raw_json = r#"{"jsonrpc":"2.0","result":[{"slot":348125,"prioritizationFee":0},{"slot":348126,"prioritizationFee":1000},{"slot":348127,"prioritizationFee":500},{"slot":348128,"prioritizationFee":0},{"slot":348129,"prioritizationFee":1234}],"id":1}"#;

		let response: ClientResponse<GetRecentPrioritizationFeesResponse> =
			serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");
		check!(
			response.result.0
				== vec![
					RpcPrioritizationFee {
						slot: 348_125,
						prioritization_fee: 0,
					},
					RpcPrioritizationFee {
						slot: 348_126,
						prioritization_fee: 1000
					},
					RpcPrioritizationFee {
						slot: 348_127,
						prioritization_fee: 500
					},
					RpcPrioritizationFee {
						slot: 348_128,
						prioritization_fee: 0
					},
					RpcPrioritizationFee {
						slot: 348_129,
						prioritization_fee: 1234
					}
				]
		);
	}
}
