use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::VersionedTransaction;

use crate::rpc_config::serialize_and_encode;
use crate::rpc_config::RpcSendTransactionConfig;
use crate::solana_transaction_status::UiTransactionEncoding;
use crate::ClientRequest;
use crate::ClientResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionRequest {
	transaction: VersionedTransaction,
	#[serde(skip_serializing_if = "Option::is_none")]
	config: Option<RpcSendTransactionConfig>,
}

impl SendTransactionRequest {
	pub fn new(transaction: VersionedTransaction) -> Self {
		Self {
			transaction,
			config: None,
		}
	}

	pub fn new_with_config(
		transaction: VersionedTransaction,
		config: RpcSendTransactionConfig,
	) -> Self {
		Self {
			transaction,
			config: Some(config),
		}
	}
}

impl From<SendTransactionRequest> for serde_json::Value {
	fn from(value: SendTransactionRequest) -> Self {
		let encoding = match value.config {
			Some(ref c) => c.encoding.unwrap_or(UiTransactionEncoding::Base64),
			None => UiTransactionEncoding::Base64,
		};

		let serialized_encoded =
			serialize_and_encode::<VersionedTransaction>(&value.transaction, encoding).unwrap();

		match value.config {
			Some(config) => serde_json::json!([serialized_encoded, config]),
			None => serde_json::json!([serialized_encoded]),
		}
	}
}

impl From<SendTransactionRequest> for ClientRequest {
	fn from(val: SendTransactionRequest) -> Self {
		let mut request = ClientRequest::new("sendTransaction");
		let params = val.into();

		request.params(params).clone()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionResponse(Signature);

impl From<SendTransactionResponse> for Signature {
	fn from(val: SendTransactionResponse) -> Self {
		val.0
	}
}

impl From<ClientResponse> for SendTransactionResponse {
	fn from(response: ClientResponse) -> Self {
		let signature = response.result.as_str().expect("invalid response");
		let signature = Signature::from_str(signature).expect("invalid signature");

		SendTransactionResponse(signature)
	}
}
