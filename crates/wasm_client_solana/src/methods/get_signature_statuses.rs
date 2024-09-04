use serde::Deserialize;
use serde_tuple::Serialize_tuple;
use serde_with::serde_as;
use serde_with::skip_serializing_none;
use serde_with::DisplayFromStr;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::TransactionError;

use super::Context;
use crate::impl_http_method;
use crate::rpc_config::RpcSignatureStatusConfig;
use crate::solana_transaction_status::TransactionConfirmationStatus;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize_tuple)]
pub struct GetSignatureStatusesRequest {
	#[serde_as(as = "Vec<DisplayFromStr>")]
	pub signatures: Vec<Signature>,
	pub config: Option<RpcSignatureStatusConfig>,
}

impl_http_method!(GetSignatureStatusesRequest, "getSignatureStatuses");

impl GetSignatureStatusesRequest {
	pub fn new(signatures: Vec<Signature>) -> Self {
		Self {
			signatures,
			config: None,
		}
	}

	pub fn new_with_config(signatures: Vec<Signature>, config: RpcSignatureStatusConfig) -> Self {
		Self {
			signatures,
			config: Some(config),
		}
	}
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SignatureStatusesValue {
	pub slot: u64,
	pub confirmations: Option<u64>,
	pub err: Option<TransactionError>,
	pub confirmation_status: Option<TransactionConfirmationStatus>,
}

#[derive(Debug, Deserialize)]
pub struct GetSignatureStatusesResponse {
	pub context: Context,
	pub value: Vec<Option<SignatureStatusesValue>>,
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use assert2::check;
	use serde_json::Value;

	use super::*;
	use crate::methods::HttpMethod;
	use crate::ClientRequest;
	use crate::ClientResponse;

	#[test]
	fn request() {
		let request = ClientRequest::new(GetSignatureStatusesRequest::NAME)
            .id(1)
            .params(GetSignatureStatusesRequest::new_with_config(vec![Signature::from_str("5VERv8NMvzbJMEkV8xnrLkEaWRtSz9CosKDYjCJjBRnbJLgp8uirBgmQpjKhoR4tjF3ZpRzrFmBV6UjKdiSZkQUW").unwrap()], RpcSignatureStatusConfig {search_transaction_history: true}));

		let value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1,"method":"getSignatureStatuses","params":[["5VERv8NMvzbJMEkV8xnrLkEaWRtSz9CosKDYjCJjBRnbJLgp8uirBgmQpjKhoR4tjF3ZpRzrFmBV6UjKdiSZkQUW"],{"searchTransactionHistory":true}]}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(value == raw_value);
		insta::assert_json_snapshot!(value, @"");
	}

	#[test]
	fn response() {
		let raw_json = r#"{"jsonrpc":"2.0","result":{"context":{"slot":82},"value":[{"slot":48,"confirmations":null,"err":null,"status":{"Ok":null},"confirmationStatus":"finalized"},null]},"id":1}"#;

		let response: ClientResponse<GetSignatureStatusesResponse> =
			serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");
		check!(response.result.context.slot == 82);
		check!(
			response.result.value
				== vec![
					Some(SignatureStatusesValue {
						slot: 48,
						err: None,
						confirmation_status: Some(TransactionConfirmationStatus::Finalized),
						confirmations: None
					}),
					None
				]
		);
	}
}
