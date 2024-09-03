use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientRequest {
	id: u32,
	jsonrpc: String,
	method: String,
	params: Option<Value>,
}

impl ClientRequest {
	pub fn new(method: &str) -> Self {
		Self {
			id: 1,
			jsonrpc: "2.0".into(),
			method: method.into(),
			params: None,
		}
	}

	pub fn id(&mut self, id: u32) -> &mut ClientRequest {
		self.id = id;
		self
	}

	pub fn jsonrpc(&mut self, jsonrpc: &str) -> &mut ClientRequest {
		self.jsonrpc = jsonrpc.into();
		self
	}

	pub fn params(&mut self, params: Value) -> &mut ClientRequest {
		self.params = Some(params);
		self
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientResponse {
	pub id: u32,
	pub jsonrpc: String,
	pub result: Value,
}

pub const MAX_RETRIES: usize = 40;
pub const SLEEP_MS: u64 = 250;
