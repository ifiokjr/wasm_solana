use serde::Deserialize;
use serde_tuple::Serialize_tuple;
use solana_sdk::clock::UnixTimestamp;

use crate::impl_http_method;

#[derive(Debug, Serialize_tuple)]
pub struct GetBlockTimeRequest {
	pub slot: u64,
}

impl_http_method!(GetBlockTimeRequest, "getBlockTime");

impl GetBlockTimeRequest {
	pub fn new(slot: u64) -> Self {
		Self { slot }
	}
}

#[derive(Debug, Deserialize)]
pub struct GetBlockTimeResponse(Option<UnixTimestamp>);

impl From<GetBlockTimeResponse> for Option<UnixTimestamp> {
	fn from(val: GetBlockTimeResponse) -> Self {
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
		let request = ClientRequest::new(GetBlockTimeRequest::NAME)
			.id(1)
			.params(GetBlockTimeRequest::new(5));

		let ser_value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1,"method":"getBlockTime","params":[5]}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(ser_value == raw_value);
	}

	#[test]
	fn response() {
		let raw_json = r#"{"jsonrpc":"2.0","result":1574721591,"id":1}"#;

		let response: ClientResponse<GetBlockTimeResponse> =
			serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");
		check!(response.result.0.unwrap() == 1_574_721_591);
	}
}
