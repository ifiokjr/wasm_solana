use serde::Deserialize;
use serde_tuple::Serialize_tuple;
use serde_with::serde_as;
use serde_with::skip_serializing_none;
use serde_with::DisplayFromStr;
use solana_sdk::hash::Hash;

use super::Context;
use crate::impl_http_method;
use crate::rpc_config::RpcContextConfig;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize_tuple, Default)]
pub struct IsBlockhashValidRequest {
	#[serde_as(as = "DisplayFromStr")]
	blockhash: Hash,
	config: Option<RpcContextConfig>,
}

impl_http_method!(IsBlockhashValidRequest, "isBlockhashValid");

impl IsBlockhashValidRequest {
	pub fn new(blockhash: Hash) -> Self {
		Self {
			blockhash,
			config: None,
		}
	}

	pub fn new_with_config(blockhash: Hash, config: RpcContextConfig) -> Self {
		Self {
			blockhash,
			config: Some(config),
		}
	}
}

#[derive(Debug, Deserialize)]
pub struct IsBlockhashValidResponse {
	pub context: Context,
	pub value: bool,
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use assert2::check;
	use serde_json::Value;
	use solana_sdk::commitment_config::CommitmentConfig;

	use super::*;
	use crate::methods::HttpMethod;
	use crate::ClientRequest;
	use crate::ClientResponse;

	#[test]
	fn request() {
		let request = ClientRequest::new(IsBlockhashValidRequest::NAME)
			.id(45)
			.params(IsBlockhashValidRequest::new_with_config(
				Hash::from_str("J7rBdM6AecPDEZp8aPq5iPSNKVkU5Q76F3oAV4eW5wsW").unwrap(),
				RpcContextConfig {
					commitment: Some(CommitmentConfig::processed()),
					min_context_slot: None,
				},
			));

		let ser_value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"id":45,"jsonrpc":"2.0","method":"isBlockhashValid","params":["J7rBdM6AecPDEZp8aPq5iPSNKVkU5Q76F3oAV4eW5wsW",{"commitment":"processed"}]}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(ser_value == raw_value);
	}

	#[test]
	fn response() {
		let raw_json =
			r#"{"jsonrpc":"2.0","result":{"context":{"slot":2483},"value":false},"id":1}"#;

		let response: ClientResponse<IsBlockhashValidResponse> =
			serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");
		check!(!response.result.value);
	}
}
