# Pubsub and Streams

## Subscriptions

The client exposes typed subscriptions for every pubsub method: `account_notifications`, `program_notifications`, `logs_notifications`, `slot_notifications`, `signature_notifications`, `block_notifications`, `vote_notifications` and friends.

```rust,ignore
use wasm_client_solana::SolanaRpcClient;

let client = SolanaRpcClient::new("wss://api.devnet.solana.com");
let mut logs = client.logs_notifications(LogsFilter::All).await?;

while let Some(notification) = logs.next().await {
    // typed notification values
}
```

Each call returns a `Subscription<T>`: a fused stream that owns the websocket connection, resolves the first `subscription` id from the server, and yields typed notifications. Dropping the subscription unsubscribes.

## The provider split

`WebSocketProvider` has two implementations selected by feature flags:

- **`js`** — the browser `WebSocket` API through web-sys; notifications arrive as `web_sys::MessageEvent` and are converted with the `ToWebSocketValue` trait.
- **`ssr`** — `reqwest-websocket` over tokio, for native binaries that still want pubsub.

Both speak the same JSON-RPC notification protocol, so app code is transport agnostic.

## Streams utilities

`test_utils_solana` provides higher-level stream helpers for integration tests against a live validator:

```rust,ignore
use test_utils_solana::account_stream_subscription;
use test_utils_solana::log_stream_subscription;
```

- `log_stream_subscription` — subscribe to program logs and collect them until a condition matches (used to assert on-chain events).
- `account_stream_subscription` — watch account updates until a predicate fires.

These helpers make pubsub deterministic in tests: await the event instead of sleeping.

## Timeouts

Wasm browser sockets have no tokio to police them; `WASM_BINDGEN_TEST_TIMEOUT` governs test runs and the subscription streams surface connection failures as `ClientWebSocketError`. For native targets the provider relies on reqwest's own timeouts.
