#![allow(unused_imports)]

use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::ready;
use std::task::Context;
use std::task::Poll;

use derive_more::derive::Debug;
use derive_more::Deref;
use derive_more::DerefMut;
use fork_stream::Forked;
use fork_stream::StreamExt as _;
use futures::channel::mpsc::unbounded;
use futures::channel::mpsc::UnboundedReceiver;
use futures::channel::mpsc::UnboundedSender;
use futures::future;
use futures::future::Ready;
use futures::lock::Mutex;
use futures::pin_mut;
use futures::stream::AndThen;
use futures::stream::MapErr;
use futures::stream::SplitSink;
use futures::stream::SplitStream;
use futures::FutureExt;
use futures::SinkExt;
use futures::Stream;
use futures::StreamExt;
use futures::TryStreamExt;
use futures_timer::Delay;
use gloo_net::websocket::futures::WebSocket;
use gloo_net::websocket::Message;
use gloo_net::websocket::WebSocketError;
use pin_project::pin_project;
use pin_project::pinned_drop;
use send_wrapper::SendWrapper;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use solana_sdk::account_info::AccountInfo;
use typed_builder::TypedBuilder;

use crate::ClientRequest;
use crate::ClientResult;
use crate::SolanaRpcClientError;
use crate::SubscriptionId;
use crate::SubscriptionResponse;
use crate::SubscriptionResult;
use crate::WebsocketMethod;
use crate::WebsocketNotification;

#[pin_project(PinnedDrop)]
#[derive(Clone, TypedBuilder)]
pub struct Subscription<T: DeserializeOwned + WebsocketNotification> {
	pub(crate) sender: Arc<Mutex<SendWrapper<SplitSink<WebSocket, Message>>>>,
	#[pin]
	pub(crate) receiver: Forked<UnboundedReceiver<Value>>,
	#[builder(default)]
	pub(crate) latest: Option<T>,
	pub(crate) id: SubscriptionId,
	#[builder(default)]
	pub(crate) count: u64,
}

impl<T: DeserializeOwned + WebsocketNotification> Subscription<T> {
	pub async fn unsubscribe(&self) -> ClientResult<()> {
		let request = ClientRequest::builder()
			.method(T::UNSUBSCRIBE)
			.params(serde_json::json!([self.id]))
			.build();
		let message = Message::Text(
			serde_json::to_string(&request)
				.map_err(|_| SolanaRpcClientError::new("Could not serialize request"))?,
		);

		self.sender
			.lock()
			.await
			.send(message)
			.await
			.map_err(|e| SolanaRpcClientError::new(format!("Could not unsubscribe: {e:?}")))?;

		Ok(())
	}
}

#[pinned_drop]
impl<T: DeserializeOwned + WebsocketNotification> PinnedDrop for Subscription<T> {
	fn drop(self: Pin<&mut Self>) {
		let _ = self.unsubscribe().now_or_never();
	}
}

impl<T: DeserializeOwned + WebsocketNotification> Stream for Subscription<T> {
	type Item = SubscriptionResponse<T>;

	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		let subscription_id = self.id;
		self.count += 1;
		let mut this = self.project();

		let Some(value) = ready!(this.receiver.as_mut().poll_next(cx)) else {
			return Poll::Ready(None);
		};

		let Some(json) = serde_json::from_value::<SubscriptionResponse<T>>(value).ok() else {
			return Poll::Pending;
		};

		if json.method != T::NOTIFICATION || json.params.subscription != subscription_id {
			return Poll::Pending;
		}

		Poll::Ready(Some(json))
	}
}

#[derive(Clone, Debug)]
pub struct WebSocketProvider {
	url: String,
	#[debug(skip)]
	pub(crate) sender: Arc<Mutex<SendWrapper<SplitSink<WebSocket, Message>>>>,
	#[debug(skip)]
	pub(crate) receiver: Forked<UnboundedReceiver<Value>>,
	/// The client ID which identifies current client ID.
	id: u32,
}

impl WebSocketProvider {
	pub fn try_new(url: impl Into<String>) -> ClientResult<Self> {
		let url = get_ws_url(url);
		let ws = WebSocket::open(&url)
			.map_err(|_| SolanaRpcClientError::new("Websocket connection failed"))?;
		let (sink, mut stream) = ws.split();
		let (tx, rx) = unbounded::<Value>();

		// TODO check that this is the correct way to do this
		spawn_local(async move {
			while let Some(result) = stream.next().await {
				match result {
					Ok(Message::Bytes(bytes)) => {
						tx.unbounded_send(serde_json::from_slice(&bytes).unwrap())
							.unwrap();
					}
					Ok(Message::Text(text)) => {
						tx.unbounded_send(serde_json::from_str(&text).unwrap())
							.unwrap();
					}
					Err(_) => {}
				}
			}
		});

		let receiver = rx.fork();
		let sender = Arc::new(Mutex::new(SendWrapper::new(sink)));

		Ok(Self {
			url,
			sender,
			receiver,
			id: 0,
		})
	}

	pub fn url(&self) -> &str {
		&self.url
	}

	/// Create a subscription and return the subscription id once a response
	/// is received.
	///
	/// ```rust
	/// ```
	pub async fn create_subscription<T: WebsocketMethod>(
		&mut self,
		params: T,
	) -> ClientResult<SubscriptionId> {
		let id = self.id;
		self.id += 1;
		let request = ClientRequest::builder()
			.method(T::SUBSCRIBE)
			.params(params)
			.id(id)
			.build();
		let json_string = serde_json::to_string(&request)
			.map_err(|_| SolanaRpcClientError::new("Could not serialize params"))?;

		self.sender
			.lock()
			.await
			.send(Message::Text(json_string))
			.await
			.map_err(|e| SolanaRpcClientError::new(format!("Could not subscribe: {e:?}")))?;

		let mut stream = self.receiver.clone().filter_map(|value| {
			future::ready(
				serde_json::from_value::<SubscriptionResult>(value)
					.ok()
					.filter(|value| value.id == id),
			)
		});

		let Some(response) = stream.next().await else {
			return Err(SolanaRpcClientError::new("Could not get subscription_id"));
		};

		Ok(response.result)
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

pub fn spawn_local<F>(fut: F)
where
	F: Future<Output = ()> + 'static,
{
	cfg_if::cfg_if! {
		if #[cfg(feature = "js")] {
			wasm_bindgen_futures::spawn_local(fut);
		} else if #[cfg(feature = "ssr")] {
			tokio::task::spawn_local(fut);
		}  else {
			futures::executor::block_on(fut);
		}
	}
}
