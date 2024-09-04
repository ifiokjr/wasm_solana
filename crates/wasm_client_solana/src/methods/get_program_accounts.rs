use serde::Deserialize;
use serde_tuple::Serialize_tuple;
use serde_with::serde_as;
use serde_with::skip_serializing_none;
use serde_with::DisplayFromStr;
use solana_sdk::pubkey::Pubkey;

use crate::impl_http_method;
use crate::rpc_config::RpcKeyedAccount;
use crate::rpc_config::RpcProgramAccountsConfig;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize_tuple)]
pub struct GetProgramAccountsRequest {
	#[serde_as(as = "DisplayFromStr")]
	pub pubkey: Pubkey,
	pub config: Option<RpcProgramAccountsConfig>,
}

impl_http_method!(GetProgramAccountsRequest, "getProgramAccounts");

impl GetProgramAccountsRequest {
	pub fn new(pubkey: Pubkey) -> Self {
		Self {
			pubkey,
			config: None,
		}
	}

	pub fn new_with_config(pubkey: Pubkey, config: RpcProgramAccountsConfig) -> Self {
		Self {
			pubkey,
			config: Some(config),
		}
	}
}

#[derive(Debug, Deserialize)]
pub struct GetProgramAccountsResponse(Option<Vec<RpcKeyedAccount>>);

impl GetProgramAccountsResponse {
	pub fn keyed_accounts(&self) -> Option<&Vec<RpcKeyedAccount>> {
		self.0.as_ref()
	}
}

#[cfg(test)]
mod tests {
	use assert2::check;
	use serde_json::Value;
	use solana_sdk::pubkey;

	use super::*;
	use crate::methods::HttpMethod;
	use crate::rpc_config::RpcAccountInfoConfig;
	use crate::rpc_filter::Memcmp;
	use crate::rpc_filter::MemcmpEncodedBytes;
	use crate::rpc_filter::RpcFilterType;
	use crate::solana_account_decoder::UiAccount;
	use crate::solana_account_decoder::UiAccountData;
	use crate::ClientRequest;
	use crate::ClientResponse;

	#[test]
	fn request() {
		let request = ClientRequest::new(GetProgramAccountsRequest::NAME)
			.id(1)
			.params(GetProgramAccountsRequest::new_with_config(
				pubkey!("4Nd1mBQtrMJVYVfKf2PJy9NZUZdTAsp7D4xWLs4gDB4T"),
				RpcProgramAccountsConfig {
					filters: Some(vec![
						RpcFilterType::DataSize(17),
						RpcFilterType::Memcmp(Memcmp::new(
							4,
							MemcmpEncodedBytes::Base64("3Mc6vR".to_string()),
						)),
					]),
					account_config: RpcAccountInfoConfig::default(),
					with_context: None,
				},
			));

		let ser_value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1,"method":"getProgramAccounts","params":["4Nd1mBQtrMJVYVfKf2PJy9NZUZdTAsp7D4xWLs4gDB4T",{"filters":[{"dataSize":17},{"memcmp":{"offset":4,"bytes":"3Mc6vR"}}]}]}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(ser_value == raw_value);
	}

	#[test]
	fn response() {
		let raw_json = r#"{"jsonrpc":"2.0","result":[{"account":{"data":"2R9jLfiAQ9bgdcw6h8s44439","executable":false,"lamports":15298080,"owner":"4Nd1mBQtrMJVYVfKf2PJy9NZUZdTAsp7D4xWLs4gDB4T","rentEpoch":28,"space":42},"pubkey":"CxELquR1gPP8wHe33gZ4QxqGB3sZ9RSwsJ2KshVewkFY"}],"id":1}"#;

		let response: ClientResponse<GetProgramAccountsResponse> =
			serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");
		let value = response.result.0.unwrap();
		check!(
			value
				== vec![RpcKeyedAccount {
					account: UiAccount {
						executable: false,
						data: UiAccountData::LegacyBinary("2R9jLfiAQ9bgdcw6h8s44439".to_string()),
						lamports: 15_298_080,
						owner: "4Nd1mBQtrMJVYVfKf2PJy9NZUZdTAsp7D4xWLs4gDB4T".to_string(),
						rent_epoch: 28,
						space: Some(42)
					},
					pubkey: "CxELquR1gPP8wHe33gZ4QxqGB3sZ9RSwsJ2KshVewkFY".to_string()
				}]
		);
	}
}
