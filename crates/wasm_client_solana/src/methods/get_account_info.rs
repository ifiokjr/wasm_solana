use serde::Deserialize;
use serde_tuple::Serialize_tuple;
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use solana_sdk::pubkey::Pubkey;

use super::Context;
use crate::impl_http_method;
use crate::impl_websocket_method;
use crate::impl_websocket_notification;
use crate::rpc_config::RpcAccountInfoConfig;
use crate::solana_account_decoder::UiAccount;
use crate::solana_account_decoder::UiAccountEncoding;

#[serde_as]
#[derive(Debug, Serialize_tuple)]
pub struct GetAccountInfoRequest {
	#[serde_as(as = "DisplayFromStr")]
	pub pubkey: Pubkey,
	pub config: RpcAccountInfoConfig,
}

impl_http_method!(GetAccountInfoRequest, "getAccountInfo");
impl_websocket_method!(GetAccountInfoRequest, "account");

impl GetAccountInfoRequest {
	pub fn new(pubkey: Pubkey) -> Self {
		Self {
			pubkey,
			config: RpcAccountInfoConfig {
				encoding: Some(UiAccountEncoding::Base64),
				data_slice: None,
				commitment: None,
				min_context_slot: None,
			},
		}
	}

	pub fn new_with_config(pubkey: Pubkey, config: RpcAccountInfoConfig) -> Self {
		Self { pubkey, config }
	}
}

#[derive(Debug, Deserialize)]
pub struct GetAccountInfoResponse {
	pub context: Context,
	pub value: Option<UiAccount>,
}

impl_websocket_notification!(GetAccountInfoResponse, "account");

#[cfg(test)]
mod tests {
	use assert2::check;
	use serde_json::Value;
	use solana_sdk::pubkey;

	use super::*;
	use crate::methods::HttpMethod;
	use crate::solana_account_decoder::UiAccountData;
	use crate::ClientRequest;
	use crate::ClientResponse;

	#[test]
	fn request() {
		let pubkey = pubkey!("vines1vzrYbzLMRdu58ou5XTby4qAqVRLmqo36NKPTg");
		let request = ClientRequest::new(GetAccountInfoRequest::NAME)
			.id(1)
			.params(GetAccountInfoRequest::new(pubkey));

		let ser_value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1,"method":"getAccountInfo","params":["vines1vzrYbzLMRdu58ou5XTby4qAqVRLmqo36NKPTg",{"encoding":"base58"}]}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(ser_value == raw_value);
	}

	#[test]
	fn response() {
		let raw_json = r#"{"jsonrpc":"2.0","result":{"context":{"slot":1},"value":{"data":["11116bv5nS2h3y12kD1yUKeMZvGcKLSjQgX6BeV7u1FrjeJcKfsHRTPuR3oZ1EioKtYGiYxpxMG5vpbZLsbcBYBEmZZcMKaSoGx9JZeAuWf","base58"],"executable":false,"lamports":1000000000,"owner":"11111111111111111111111111111111","rentEpoch":2,"space":80}},"id":1}"#;

		let response: ClientResponse<GetAccountInfoResponse> =
			serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");
		check!(response.result.context.slot == 1);

		let value = response.result.value.unwrap();
		assert!(!value.executable);
		check!(value.lamports == 1_000_000_000);
		check!(value.owner == "11111111111111111111111111111111");
		check!(value.rent_epoch == 2);
		check!(value.space == Some(80));
		check!(value.data == UiAccountData::Binary("11116bv5nS2h3y12kD1yUKeMZvGcKLSjQgX6BeV7u1FrjeJcKfsHRTPuR3oZ1EioKtYGiYxpxMG5vpbZLsbcBYBEmZZcMKaSoGx9JZeAuWf".to_string(), UiAccountEncoding::Base58));
	}
}
