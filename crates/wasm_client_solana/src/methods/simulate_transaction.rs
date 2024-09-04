use serde::Deserialize;
use serde::Serialize;
use serde_tuple::Serialize_tuple;
use solana_sdk::transaction::TransactionError;
use solana_sdk::transaction::VersionedTransaction;

use super::Context;
use crate::impl_http_method;
use crate::rpc_config::RpcSimulateTransactionConfig;
use crate::solana_account_decoder::UiAccount;
use crate::solana_transaction_status::UiTransactionEncoding;

#[derive(Debug, Serialize_tuple)]
pub struct SimulateTransactionRequest {
	pub transaction: VersionedTransaction,
	pub config: Option<RpcSimulateTransactionConfig>,
}

impl_http_method!(SimulateTransactionRequest, "simulateTransaction");

impl SimulateTransactionRequest {
	pub fn new(transaction: VersionedTransaction) -> Self {
		Self {
			transaction,
			config: Some(RpcSimulateTransactionConfig {
				encoding: Some(UiTransactionEncoding::Base64),
				replace_recent_blockhash: Some(true),
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

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SimulateTransactionResponseValue {
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

#[derive(Debug, Deserialize)]
pub struct SimulateTransactionResponse {
	pub context: Context,
	pub value: SimulateTransactionResponseValue,
}

#[cfg(test)]
mod tests {
	use assert2::check;
	use base64::prelude::BASE64_STANDARD;
	use base64::Engine;
	use serde_json::Value;

	use super::*;
	use crate::methods::HttpMethod;
	use crate::ClientRequest;
	use crate::ClientResponse;

	#[test]
	fn request() {
		let tx = bincode::deserialize(&BASE64_STANDARD.decode("AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAEDArczbMia1tLmq7zz4DinMNN0pJ1JtLdqIJPUw3YrGCzYAMHBsgN27lcgB6H2WQvFgyZuJYHa46puOQo9yQ8CVQbd9uHXZaGT2cvhRs7reawctIXtX1s3kTqM9YV+/wCp20C7Wj2aiuk5TReAXo+VTVg8QTHjs0UjNMMKCvpzZ+ABAgEBARU=").unwrap()).unwrap();
		let request = ClientRequest::builder()
			.method(SimulateTransactionRequest::NAME)
			.id(1)
			.params(SimulateTransactionRequest::new_with_config(
				tx,
				RpcSimulateTransactionConfig {
					encoding: Some(UiTransactionEncoding::Base64),
					..Default::default()
				},
			))
			.build();

		let value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1,"method":"simulateTransaction","params":["AQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAEDArczbMia1tLmq7zz4DinMNN0pJ1JtLdqIJPUw3YrGCzYAMHBsgN27lcgB6H2WQvFgyZuJYHa46puOQo9yQ8CVQbd9uHXZaGT2cvhRs7reawctIXtX1s3kTqM9YV+/wCp20C7Wj2aiuk5TReAXo+VTVg8QTHjs0UjNMMKCvpzZ+ABAgEBARU=",{"encoding":"base64"}]}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(value == raw_value);
		insta::assert_json_snapshot!(value, @"");
	}

	#[test]
	fn response() {
		let raw_json = r#"{"jsonrpc":"2.0","result":{"context":{"slot":218},"value":{"err":null,"accounts":null,"logs":["Program 83astBRguLMdt2h5U1Tpdq5tjFoJ6noeGwaY3mDLVcri invoke [1]","Program 83astBRguLMdt2h5U1Tpdq5tjFoJ6noeGwaY3mDLVcri consumed 2366 of 1400000 compute units","Program return: 83astBRguLMdt2h5U1Tpdq5tjFoJ6noeGwaY3mDLVcri KgAAAAAAAAA=","Program 83astBRguLMdt2h5U1Tpdq5tjFoJ6noeGwaY3mDLVcri success"],"returnData":{"data":["Kg==","base64"],"programId":"83astBRguLMdt2h5U1Tpdq5tjFoJ6noeGwaY3mDLVcri"},"unitsConsumed":2366}},"id":1}"#;

		let response: ClientResponse<SimulateTransactionResponse> =
			serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");
		check!(response.result.context.slot == 218);
		check!(
			response.result.value
				== SimulateTransactionResponseValue {
					accounts: None,
					err: None,
					logs: Some(vec![
						"Program 83astBRguLMdt2h5U1Tpdq5tjFoJ6noeGwaY3mDLVcri invoke [1]"
							.to_string(),
						"Program 83astBRguLMdt2h5U1Tpdq5tjFoJ6noeGwaY3mDLVcri consumed 2366 of \
						 1400000 compute units"
							.to_string(),
						"Program return: 83astBRguLMdt2h5U1Tpdq5tjFoJ6noeGwaY3mDLVcri KgAAAAAAAAA="
							.to_string(),
						"Program 83astBRguLMdt2h5U1Tpdq5tjFoJ6noeGwaY3mDLVcri success".to_string()
					]),
					return_data: Some(UiTransactionReturnData {
						program_id: "83astBRguLMdt2h5U1Tpdq5tjFoJ6noeGwaY3mDLVcri".to_string(),
						data: ("Kg==".to_string(), UiReturnDataEncoding::Base64)
					}),
					units_consumed: Some(2366)
				}
		);
	}
}
