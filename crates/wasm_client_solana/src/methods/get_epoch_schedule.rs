use serde::Deserialize;
use serde::Serialize;
use solana_sdk::epoch_schedule::EpochSchedule;

use crate::impl_http_method;

#[derive(Debug, Serialize)]
pub struct GetEpochScheduleRequest;

impl_http_method!(GetEpochScheduleRequest, "getEpochSchedule");

#[derive(Debug, Deserialize)]
pub struct GetEpochScheduleResponse(EpochSchedule);

impl From<GetEpochScheduleResponse> for EpochSchedule {
	fn from(value: GetEpochScheduleResponse) -> Self {
		value.0
	}
}

#[cfg(test)]
mod tests {
	use assert2::check;
	use serde_json::Value;

	use super::*;
	use crate::methods::HttpMethod;
	use crate::ClientRequest;
	use crate::ClientResponse;

	#[test]
	fn request() {
		let request = ClientRequest::builder()
			.method(GetEpochScheduleRequest::NAME)
			.id(1)
			.params(GetEpochScheduleRequest)
			.build();

		insta::assert_json_snapshot!(request, @"");

		let ser_value = serde_json::to_value(request).unwrap();
		let raw_json = r#"{"jsonrpc":"2.0","id":1,"method":"getEpochSchedule"}"#;
		let raw_value: Value = serde_json::from_str(raw_json).unwrap();

		check!(ser_value == raw_value);
	}

	#[test]
	fn response() {
		let raw_json = r#"{"jsonrpc":"2.0","result":{"firstNormalEpoch":8,"firstNormalSlot":8160,"leaderScheduleSlotOffset":8192,"slotsPerEpoch":8192,"warmup":true},"id":1}"#;

		let response: ClientResponse<GetEpochScheduleResponse> =
			serde_json::from_str(raw_json).unwrap();

		check!(response.id == 1);
		check!(response.jsonrpc == "2.0");

		let value = response.result.0;
		check!(value.first_normal_epoch == 8);
		check!(value.first_normal_slot == 8160);
		check!(value.leader_schedule_slot_offset == 8192);
		check!(value.slots_per_epoch == 8192);
		check!(value.warmup);
	}
}
