use serde::Deserialize;
use serde::Serialize;

use crate::impl_http_method;

#[derive(Debug, Serialize)]
pub struct GetClusterNodesRequest;

impl_http_method!(GetClusterNodesRequest, "getClusterNodes");

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcContactInfoWasm {
	pub pubkey: String,
	pub gossip: Option<String>,
	pub tpu: Option<String>,
	pub rpc: Option<String>,
	pub version: Option<String>,
	pub feature_set: Option<u32>,
	pub shred_version: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct GetClusterNodesResponse(Vec<RpcContactInfoWasm>);

impl From<GetClusterNodesResponse> for Vec<RpcContactInfoWasm> {
	fn from(value: GetClusterNodesResponse) -> Self {
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
		let request = ClientRequest::builder()
			.method(GetClusterNodesRequest::NAME)
			.id(1)
			.params(GetClusterNodesRequest)
			.build();

		insta::assert_json_snapshot!(request, @"");
		let ser_value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1,"method":"getClusterNodes"}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(ser_value == raw_value);
	}

	#[test]
	fn response() {
		let raw_json = r#"{"jsonrpc":"2.0","result":[{"gossip":"10.239.6.48:8001","pubkey":"9QzsJf7LPLj8GkXbYT3LFDKqsj2hHG7TA3xinJHu8epQ","rpc":"10.239.6.48:8899","tpu":"10.239.6.48:8856","version":"1.0.0 c375ce1f"}],"id":1}"#;

		let response: ClientResponse<GetClusterNodesResponse> =
			serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");

		let value = &response.result.0[0];

		check!(value.gossip.as_ref().unwrap() == "10.239.6.48:8001");
		check!(value.pubkey == "9QzsJf7LPLj8GkXbYT3LFDKqsj2hHG7TA3xinJHu8epQ");
		check!(value.rpc.as_ref().unwrap() == "10.239.6.48:8899");
		check!(value.tpu.as_ref().unwrap() == "10.239.6.48:8856");
		check!(value.version.as_ref().unwrap() == "1.0.0 c375ce1f");
		check!(value.feature_set.is_none());
		check!(value.shred_version.is_none());
	}
}
