use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use typed_builder::TypedBuilder;

use crate::ClientWebSocketError;

#[derive(Debug, Clone, Serialize, TypedBuilder)]
pub struct ClientRequest {
	#[builder(default = "2.0")]
	pub jsonrpc: &'static str,
	#[builder(default)]
	pub id: u32,
	#[builder(setter(into))]
	pub method: String,
	#[serde(skip_serializing_if = "is_null")]
	#[builder(default = Value::Null, setter(transform = |value: impl Serialize| serde_json::to_value(value).unwrap_or_default()))]
	pub params: Value,
}

impl ClientRequest {
	pub fn try_to_value(&self) -> Result<Value, ClientWebSocketError> {
		serde_json::to_value(self).map_err(|_| ClientWebSocketError::InvalidMessage)
	}
}

pub type SubscriptionId = u64;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionResponse<T> {
	pub jsonrpc: String,
	pub method: String,
	pub params: SubscriptionParams<T>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubscriptionParams<T> {
	pub result: T,
	pub subscription: SubscriptionId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClientResponse<T> {
	pub jsonrpc: String,
	pub result: T,
	pub id: u32,
}

pub type SubscriptionResult = ClientResponse<SubscriptionId>;
pub type UnsubscriptionResult = ClientResponse<bool>;

pub const MAX_RETRIES: usize = 25;
pub const SLEEP_MS: u64 = 400; // solana block time

fn is_null(v: &Value) -> bool {
	match v {
		Value::Null => true,
		Value::Array(array) => array.iter().all(Value::is_null),
		_ => false,
	}
}
