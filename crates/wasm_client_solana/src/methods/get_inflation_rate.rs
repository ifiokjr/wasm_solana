use derive_more::derive::Into;
use serde::Deserialize;
use serde::Serialize;

use crate::impl_http_method;
use crate::rpc_response::RpcInflationRate;

#[derive(Debug, Serialize)]
pub struct GetInflationRateRequest;

impl_http_method!(GetInflationRateRequest, "getInflationRate");

#[derive(Debug, Deserialize, Into)]
pub struct GetInflationRateResponse(RpcInflationRate);

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
			.method(GetInflationRateRequest::NAME)
			.id(1)
			.params(GetInflationRateRequest)
			.build();

		insta::assert_json_snapshot!(request, @"");

		let ser_value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1, "method":"getInflationRate"}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(ser_value == raw_value);
	}

	#[test]
	fn response() {
		let raw_json = r#"{"jsonrpc":"2.0","result":{"epoch":100,"foundation":0.001,"total":0.149,"validator":0.148},"id":1}"#;

		let response: ClientResponse<GetInflationRateResponse> =
			serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");
		let value = response.result.0;
		check!(value.foundation == 0.001);
		check!(value.epoch == 100);
		check!(value.total == 0.149);
		check!(value.validator == 0.148);
	}
}
