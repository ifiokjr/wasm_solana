use serde::Deserialize;
use serde_tuple::Serialize_tuple;
use serde_with::skip_serializing_none;
use solana_sdk::clock::Slot;

use crate::impl_http_method;
use crate::rpc_config::RpcLeaderScheduleConfig;
use crate::rpc_response::RpcLeaderSchedule;

#[skip_serializing_none]
#[derive(Debug, Default, Serialize_tuple)]
pub struct GetLeaderScheduleRequest {
	#[serialize_always]
	pub slot: Option<Slot>,
	pub config: Option<RpcLeaderScheduleConfig>,
}

impl_http_method!(GetLeaderScheduleRequest, "getLeaderSchedule");

impl GetLeaderScheduleRequest {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn new_with_slot_and_config(slot: Slot, config: RpcLeaderScheduleConfig) -> Self {
		Self {
			slot: Some(slot),
			config: Some(config),
		}
	}

	pub fn new_with_config(config: RpcLeaderScheduleConfig) -> Self {
		Self {
			slot: None,
			config: Some(config),
		}
	}
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct GetLeaderScheduleResponse(Option<RpcLeaderSchedule>);

impl From<GetLeaderScheduleResponse> for Option<RpcLeaderSchedule> {
	fn from(value: GetLeaderScheduleResponse) -> Self {
		value.0
	}
}

#[cfg(test)]
mod tests {
	use std::collections::HashMap;

	use assert2::check;
	use solana_sdk::pubkey;
	use solana_sdk::pubkey::Pubkey;

	use super::*;
	use crate::methods::HttpMethod;
	use crate::ClientRequest;
	use crate::ClientResponse;

	#[test]
	fn request() {
		let request = ClientRequest::builder()
			.method(GetLeaderScheduleRequest::NAME)
			.id(1)
			.params(GetLeaderScheduleRequest::new_with_config(
				RpcLeaderScheduleConfig {
					identity: Some(pubkey!("4Qkev8aNZcqFNSRhQzwyLMFSsi94jHqE8WNVTJzTP99F")),
					..Default::default()
				},
			))
			.build();

		insta::assert_compact_json_snapshot!(request, @r###"
  {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getLeaderSchedule",
    "params": [
      null,
      {
        "identity": "4Qkev8aNZcqFNSRhQzwyLMFSsi94jHqE8WNVTJzTP99F"
      }
    ]
  }
  "###);
	}

	#[test]
	fn response() {
		let raw_json = r#"{"jsonrpc":"2.0","result":{"4Qkev8aNZcqFNSRhQzwyLMFSsi94jHqE8WNVTJzTP99F":[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,33,34,35,36,37,38,39,40,41,42,43,44,45,46,47,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63]},"id":1}"#;
		let schedule: HashMap<Pubkey, Vec<usize>> = HashMap::from([(
			pubkey!("4Qkev8aNZcqFNSRhQzwyLMFSsi94jHqE8WNVTJzTP99F"),
			vec![
				0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
				23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
				44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
			],
		)]);
		let response: ClientResponse<GetLeaderScheduleResponse> =
			serde_json::from_str(raw_json).unwrap();
		let expected = ClientResponse {
			jsonrpc: String::from("2.0"),
			result: GetLeaderScheduleResponse(Some(RpcLeaderSchedule(schedule))),
			id: 1,
		};

		check!(response == expected);
	}
}
