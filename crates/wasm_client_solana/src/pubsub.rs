#![allow(unused_imports)]

use futures::FutureExt;
use futures::SinkExt;
use futures::StreamExt;
use futures::TryStreamExt;
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use serde_json::Value;

use crate::ClientRequest;
use crate::ClientResult;
use crate::SolanaRpcClientError;

pub struct WebSocketProvider {
	pub url: String,
	pub ws: WebSocket,
}

impl WebSocketProvider {
	pub fn try_new(url: impl Into<String>) -> ClientResult<Self> {
		let url = get_ws_url(url);
		let ws = WebSocket::open(&url)
			.map_err(|_| SolanaRpcClientError::new("Websocket connection failed"))?;

		Ok(Self { url, ws })
	}
}

fn get_ws_url(url: impl Into<String>) -> String {
	let url: String = url.into();

	if url.starts_with("http") {
		// Replace to wss
		let first_index = url.find(':').expect("Invalid URL");
		let mut url = url.to_string();
		url.replace_range(
			..first_index,
			if url.starts_with("https") {
				"wss"
			} else {
				"ws"
			},
		);

		// Increase the port number by 1 if the port is specified
		let last_index = url.rfind(':').unwrap();
		if last_index != first_index {
			if let Some(Ok(mut port)) = url.get(last_index + 1..).map(str::parse::<u16>) {
				port += 1;
				url.replace_range(last_index + 1.., &port.to_string());
			}
		}
	}

	url
}
