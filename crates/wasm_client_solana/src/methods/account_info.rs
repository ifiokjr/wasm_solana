use serde::Deserialize;
use serde::Serialize;
use solana_sdk::pubkey::Pubkey;

use super::Context;
use crate::rpc_config::RpcAccountInfoConfig;
use crate::solana_account_decoder::UiAccount;
use crate::solana_account_decoder::UiAccountEncoding;
use crate::ClientRequest;
use crate::ClientResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountInfoRequest {
	pub config: RpcAccountInfoConfig,
	pub pubkey: Pubkey,
}

impl GetAccountInfoRequest {
	pub fn new(pubkey: Pubkey) -> Self {
		Self {
			pubkey,
			config: RpcAccountInfoConfig {
				encoding: Some(UiAccountEncoding::Base58),
				data_slice: None,
				commitment: None,
				min_context_slot: None,
			},
		}
	}

	pub fn new_with_config(pubkey: Pubkey, config: RpcAccountInfoConfig) -> Self {
		Self { config, pubkey }
	}
}

impl From<GetAccountInfoRequest> for serde_json::Value {
	fn from(value: GetAccountInfoRequest) -> Self {
		serde_json::json!([value.pubkey.to_string(), value.config])
	}
}

impl From<GetAccountInfoRequest> for ClientRequest {
	fn from(value: GetAccountInfoRequest) -> Self {
		let mut request = ClientRequest::new("getAccountInfo");
		let params = value.into();

		request.params(params).clone()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountInfoResponse {
	pub context: Context,
	pub value: Option<UiAccount>,
}

impl From<ClientResponse> for GetAccountInfoResponse {
	fn from(response: ClientResponse) -> Self {
		serde_json::from_value(response.result).unwrap()
	}
}
