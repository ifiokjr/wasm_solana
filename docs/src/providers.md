# Providers

The client never performs I/O itself — it delegates to a **provider**. This is the seam that makes one codebase work on native and wasm.

## `RpcProvider`

```rust,ignore
#[async_trait(?Send)]
pub trait RpcProvider {
	async fn send(&self, request: &str) -> Result<String, ClientError>;
}
```

`HttpProvider` implements it twice, behind feature flags:

- **`js`**: gloo-net's `fetch` — runs on browser promises, no runtime required.
- **`ssr`**: `reqwest` with JSON bodies; supports the `zstd` feature for compressed validator payloads.

## `WebSocketProvider`

The pubsub counterpart. It manages connection lifecycle, subscription bookkeeping (`Subscription<T>`, `Unsubscription`), and reconnection semantics. The `js` implementation drives web-sys `WebSocket` events; the `ssr` implementation drives `reqwest-websocket`.

## Choosing at compile time

You never instantiate providers directly. `SolanaRpcClient::new(url)` builds the right one from your feature flags:

| Features        | Transport                        |
| --------------- | -------------------------------- |
| none (`wasm32`) | js (browser)                     |
| `js`            | js (explicit)                    |
| `ssr`           | reqwest                          |
| `ssr` + `js`    | ssr wins on native; js on wasm32 |

## Custom providers

Implement `RpcProvider` (and the websocket traits) to route RPC through your own transport — a proxy, a mock server, or an in-memory recorder for tests. The client and all typed methods work unchanged against any provider, which is exactly how `memory_wallet` plugs a test client into wallet-standard signing.
