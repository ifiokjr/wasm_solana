use serde::Deserialize;
use serde_tuple::Serialize_tuple;
use serde_with::skip_serializing_none;

use crate::impl_http_method;
use crate::rpc_config::RpcContextConfig;

#[skip_serializing_none]
#[derive(Debug, Default, Serialize_tuple)]

pub struct GetTransactionCountRequest {
	pub config: Option<RpcContextConfig>,
}

impl_http_method!(GetTransactionCountRequest, "getTransactionCount");

impl GetTransactionCountRequest {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn new_with_config(config: RpcContextConfig) -> Self {
		Self {
			config: Some(config),
		}
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct GetTransactionCountResponse(u64);

impl From<GetTransactionCountResponse> for u64 {
	fn from(val: GetTransactionCountResponse) -> Self {
		val.0
	}
}

#[cfg(test)]
mod tests {
	use assert2::check;

	use super::*;
	use crate::methods::HttpMethod;
	use crate::ClientRequest;
	use crate::ClientResponse;

	#[test]
	fn request() {
		let request = ClientRequest::builder()
			.method(GetTransactionCountRequest::NAME)
			.id(1)
			.params(GetTransactionCountRequest::new())
			.build();

		insta::assert_compact_json_snapshot!(request, @r###"{"jsonrpc": "2.0", "id": 1, "method": "getTransactionCount"}"###);
	}

	#[test]
	fn response() {
		let raw_json = r#"{ "jsonrpc": "2.0", "result": 268, "id": 1 }"#;

		let response: ClientResponse<GetTransactionCountResponse> =
			serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");
		check!(response.result.0 == 268);
	}
}
