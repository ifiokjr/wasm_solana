use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use solana_sdk::pubkey::Pubkey;

use crate::impl_http_method;

#[derive(Debug, Serialize)]
pub struct GetIdentityRequest;

impl_http_method!(GetIdentityRequest, "getIdentity");

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct GetIdentityResponse {
	#[serde_as(as = "DisplayFromStr")]
	pub identity: Pubkey,
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
			.method(GetIdentityRequest::NAME)
			.id(1)
			.params(GetIdentityRequest)
			.build();

		insta::assert_json_snapshot!(request, @"");

		let ser_value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1, "method":"getIdentity"}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(ser_value == raw_value);
	}

	#[test]
	fn response() {
		let raw_json = r#"{"jsonrpc":"2.0","result":{"identity":"2r1F4iWqVcb8M1DbAjQuFpebkQHY9hcVU4WuW2DJBppN"},"id":1}"#;

		let response: ClientResponse<GetIdentityResponse> = serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");
		check!(response.result.identity == pubkey!("2r1F4iWqVcb8M1DbAjQuFpebkQHY9hcVU4WuW2DJBppN"));
	}
}
