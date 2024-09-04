use serde::Deserialize;
use serde::Serialize;

use crate::impl_http_method;
use crate::rpc_response::RpcVersionInfo;

#[derive(Debug, Serialize)]
pub struct GetVersionRequest;

impl_http_method!(GetVersionRequest, "getVersion");

#[derive(Debug, Deserialize)]
pub struct GetVersionResponse(RpcVersionInfo);

impl From<GetVersionResponse> for RpcVersionInfo {
	fn from(value: GetVersionResponse) -> Self {
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
		let request = ClientRequest::new(GetVersionRequest::NAME)
			.id(1)
			.params(GetVersionRequest);

		let ser_value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1, "method":"getVersion"}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(ser_value == raw_value);
	}

	#[test]
	fn response() {
		let raw_json = r#"{"jsonrpc":"2.0","result":{"feature-set":2891131721,"solana-core":"1.16.7"},"id":1}"#;

		let response: ClientResponse<GetVersionResponse> = serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");
		check!(
			response.result.0
				== RpcVersionInfo {
					feature_set: Some(2_891_131_721),
					solana_core: "1.16.7".to_string()
				}
		);
	}
}
