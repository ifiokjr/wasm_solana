use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde::Deserialize;
use serde::Serialize;
use solana_sdk::transaction::TransactionError;
use solana_sdk::transaction::VersionedTransaction;

use crate::rpc_config::RpcSimulateTransactionConfig;
use crate::solana_account_decoder::UiAccount;
use crate::solana_transaction_status::UiTransactionEncoding;
use crate::ClientRequest;
use crate::ClientResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulateTransactionRequest {
	pub transaction: VersionedTransaction,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub config: Option<RpcSimulateTransactionConfig>,
}

impl SimulateTransactionRequest {
	pub fn new(transaction: VersionedTransaction) -> Self {
		Self {
			transaction,
			config: Some(RpcSimulateTransactionConfig {
				encoding: Some(UiTransactionEncoding::Base64),
				replace_recent_blockhash: true,
				..Default::default()
			}),
		}
	}

	pub fn new_with_config(
		transaction: VersionedTransaction,
		config: RpcSimulateTransactionConfig,
	) -> Self {
		Self {
			transaction,
			config: Some(config),
		}
	}
}

impl From<SimulateTransactionRequest> for serde_json::Value {
	fn from(value: SimulateTransactionRequest) -> Self {
		let transaction_data = bincode::serialize(&value.transaction).unwrap();
		let encoded_transaction = STANDARD.encode(transaction_data);

		match value.config {
			Some(config) => serde_json::json!([encoded_transaction, config]),
			None => serde_json::json!([encoded_transaction]),
		}
	}
}

impl From<SimulateTransactionRequest> for ClientRequest {
	fn from(val: SimulateTransactionRequest) -> Self {
		let mut request = ClientRequest::new("simulateTransaction");
		let params = val.into();

		request.params(params).clone()
	}
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SimulateTransactionResponse {
	pub err: Option<TransactionError>,
	pub logs: Option<Vec<String>>,
	pub accounts: Option<Vec<Option<UiAccount>>>,
	pub units_consumed: Option<u64>,
	pub return_data: Option<UiTransactionReturnData>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiTransactionReturnData {
	pub program_id: String,
	pub data: (String, UiReturnDataEncoding),
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum UiReturnDataEncoding {
	Base64,
}

impl From<ClientResponse> for SimulateTransactionResponse {
	fn from(response: ClientResponse) -> Self {
		let value = response.result["value"].clone();
		serde_json::from_value(value).unwrap()
	}
}
