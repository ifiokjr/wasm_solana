# The RPC Client

## Constructor

```rust,ignore
use wasm_client_solana::SolanaRpcClient;

let client = SolanaRpcClient::new("https://api.devnet.solana.com");
```

- On **native** (`ssr`) targets the URL is an `http(s)://` endpoint.
- On **wasm** (`js`) targets pass an `http(s)://` URL for RPC; pubsub takes the `wss://` variant.

The client is cheap to clone and methods take `&self`.

## Typed methods

Every RPC method lives in its own module under `methods/` with a `Get<...>Request` struct and a matching response type. The client methods are thin wrappers:

```rust,ignore
// request + response are public types, so you can also send them manually
let request = GetBalanceRequest::new_with_config(pubkey, CommitmentConfig::confirmed());
let response: ClientResponse<GetBalanceResponse> = client.send(request).await?;
```

`ClientResponse<T>` carries `id`, `jsonrpc` and `result` — the raw JSON-RPC envelope — so nothing is hidden from you.

Coverage spans the full read surface of the JSON-RPC spec: accounts, blocks, epochs, inflation, stakes, token accounts, transactions, plus write methods (`send_transaction`, `request_airdrop`, `simulate_transaction`) and helper checks (`is_blockhash_valid`, `minimum_ledger_slot`).

## Commitments

Every method has a `*_with_config` variant accepting `CommitmentConfig`, and config types (`RpcBlockConfig`, ...) expose the standard options (`encoding`, `data_slice`, filters). Encodings map onto `UiAccountEncoding` from the forked decoder crates, including `base64+zstd` on native targets.

## Errors

RPC failures surface as `ClientError` with structured contents (`RpcError` with code + message), and Solana transaction errors are typed through `solana_transaction_error` — see `errors.rs` for the full hierarchy. The forked client-types also preserve `OptionSerializer<T>` for fields the RPC may omit, null out, or skip for backwards compatibility.

## Re-exports

The client re-exports the wasm decoder crates under their familiar names:

```rust,ignore
pub use solana_account_decoder_client_types_wasm as solana_account_decoder_client_types;
pub use solana_account_decoder_wasm as solana_account_decoder;
pub use solana_transaction_status_client_types_wasm as solana_transaction_status_client_types;
pub use solana_transaction_status_wasm as solana_transaction_status;
```

This gives consumers a single import path for parsed accounts, transaction statuses and their types, regardless of which forked crate actually hosts them.

## Versioning

The crate versions track the Agave family it targets. The workspace dependency table pins each `solana-*` crate to the range the stable Agave train publishes (see the workspace `Cargo.toml`), so a wasm client compiled against Agave 4.2.x talks to validators of the same family without type drift.
