use serde::Deserialize;
use serde_tuple::Serialize_tuple;
use serde_with::serde_as;
use serde_with::skip_serializing_none;
use serde_with::DisplayFromStr;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;

use super::Context;
use crate::impl_http_method;
use crate::solana_account_decoder::parse_token::UiTokenAmount;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize_tuple)]
pub struct GetTokenAccountBalanceRequest {
	#[serde_as(as = "DisplayFromStr")]
	pub account: Pubkey,
	pub config: Option<CommitmentConfig>,
}

impl_http_method!(GetTokenAccountBalanceRequest, "getTokenAccountBalance");

impl GetTokenAccountBalanceRequest {
	pub fn new(account: Pubkey) -> Self {
		Self {
			account,
			config: None,
		}
	}

	pub fn new_with_config(account: Pubkey, config: CommitmentConfig) -> Self {
		Self {
			account,
			config: Some(config),
		}
	}
}

#[derive(Debug, Deserialize)]
pub struct GetTokenAccountBalanceResponse {
	pub context: Context,
	pub value: UiTokenAmount,
}

#[cfg(test)]
mod tests {
	use assert2::check;
	use serde_json::Value;
	use solana_sdk::pubkey;

	use super::*;
	use crate::methods::HttpMethod;
	use crate::ClientRequest;
	use crate::ClientResponse;

	#[test]
	fn request() {
		let request = ClientRequest::builder()
			.method(GetTokenAccountBalanceRequest::NAME)
			.id(1)
			.params(GetTokenAccountBalanceRequest::new(pubkey!(
				"7fUAJdStEuGbc3sM84cKRL6yYaaSstyLSU4ve5oovLS7"
			)))
			.build();

		let ser_value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1,"method":"getTokenAccountBalance","params":["7fUAJdStEuGbc3sM84cKRL6yYaaSstyLSU4ve5oovLS7"]}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(ser_value == raw_value);
		insta::assert_json_snapshot!(ser_value, @"");
	}

	#[test]
	fn response() {
		let raw_json = r#"{"jsonrpc":"2.0","result":{"context":{"slot":1114},"value":{"amount":"9864","decimals":2,"uiAmount":98.64,"uiAmountString":"98.64"}},"id":1}"#;

		let response: ClientResponse<GetTokenAccountBalanceResponse> =
			serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");
		check!(response.result.context.slot == 1114);
		check!(
			response.result.value
				== UiTokenAmount {
					amount: "9864".to_string(),
					decimals: 2,
					ui_amount: Some(98.64),
					ui_amount_string: "98.64".to_string()
				}
		);
	}
}
