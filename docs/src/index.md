# Wasm Solana

`wasm_solana` is the **WebAssembly-compatible alternative to the official Solana client**. The default `solana-client` crate depends on tokio and native networking and cannot run in a browser; this workspace rebuilds the parts of the Solana SDK that apps actually need so the same Rust code can run:

- **in the browser** as WASM (`wasm32-unknown-unknown`),
- **on the server** with the same code (the `ssr` feature),
- **against a test validator** with first-class testing utilities.

## What's inside

| Crate                                                                 | Description                                                                 |
| --------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| [`wasm_client_solana`](https://crates.io/crates/wasm_client_solana)   | A wasm compatible Solana RPC and pubsub client with 50+ typed methods.      |
| [`memory_wallet`](https://crates.io/crates/memory_wallet)             | A memory based Wallet Standard implementation, primarily for testing.       |
| [`test_utils_solana`](https://crates.io/crates/test_utils_solana)     | Test validator and `ProgramTest` utilities.                                 |
| [`test_utils_keypairs`](https://crates.io/crates/test_utils_keypairs) | Pre-defined keypairs for reproducible tests.                                |
| [`test_utils_insta`](https://crates.io/crates/test_utils_insta)       | Snapshot-test redactions for dynamic values like signatures.                |
| `solana-*-wasm` (forks)                                               | Wasm-compatible forks of the account decoder and transaction status crates. |

## What you get in this book

- The reasoning behind a wasm-native client and how it compares with `solana-client` ([Why Wasm Solana](./why-wasm-solana.md)).
- Setup and quickstarts for browsers and SSR ([Getting Started](./getting-started.md)).
- The RPC client, its typed requests/responses and the provider model.
- Pubsub subscriptions and the streams utilities.
- Wallet integration through the Wallet Standard.
- How the `solana-*-wasm` forks work and when they need re-syncing.
- Testing, security, development workflow, and the release process.

## Quick taste

```rust,ignore
use wasm_client_solana::SolanaRpcClient;
use solana_pubkey::Pubkey;

let client = SolanaRpcClient::new("https://api.devnet.solana.com");
let pubkey = "So11111111111111111111111111111111111111112".parse()?;

let balance = client.get_balance(&pubkey).await?;
println!("{balance} lamports");
```

The same code compiles for `wasm32-unknown-unknown` — no tokio, no native sockets.
