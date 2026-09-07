# Why Wasm Solana

## The gap

Rust compiled to WASM is a natural fit for Solana apps: strong types, no GC pauses, tiny binaries, and one language across client and program. But the official [`solana-client`](https://crates.io/crates/solana-client) cannot follow you into the browser:

- It is built on **tokio** and native TCP/WebSocket stacks that don't exist on `wasm32-unknown-unknown`.
- The RPC **account decoding** crates (`solana-account-decoder`, `solana-transaction-status`) pull native-only transitive dependencies.
- There is no maintained `wasm` feature flag anywhere in the stack.

So every Rust/WASM Solana app had to hand-roll JSON-RPC calls, re-implement typed responses, and maintain its own pubsub glue.

## The approach

`wasm_solana` fills the gap with three layers:

1. **Forked decoding crates** — `solana-account-decoder-wasm` and `solana-transaction-status-wasm` are maintained forks of the official decoders with native-only dependencies removed (tokio, zstd C bindings, rocksdb-era deps) and wasm-friendly enhancements (typed builders, `Eq` derives, `Pubkey`-typed fields). They stay close to upstream so RPC data formats remain byte-for-byte compatible.
2. **A client crate** — `wasm_client_solana` implements the full JSON-RPC surface with typed requests and responses, plus pubsub subscriptions, behind a **provider** abstraction: `HttpProvider` (reqwest, native/SSR) and the browser WebSocket stack (`js` feature, web-sys/gloo).
3. **Wallet + testing crates** — `memory_wallet` implements the Wallet Standard for signing in tests; `test_utils_*` bootstrap validators, keypairs and snapshot redactions.

## Design principles

- **No tokio in the browser.** The `js` feature path uses browser-native promises and websockets; `ssr` unlocks reqwest + tokio for native targets.
- **Types mirror the RPC.** Requests and responses are one-to-one with the Solana JSON-RPC specification (via the forked client-types crates), so nothing is silently lossy.
- **The wallet bridge is trait-based.** Signing goes through the Wallet Standard traits, so the same code works with an injected browser wallet or an in-memory test wallet.
- **Forks stay close to upstream.** The `solana-*-wasm` crates carry the minimum diff needed for wasm: dependency pruning and ergonomic additions. Every Agave upgrade re-syncs them.

## When not to use it

If your app only runs natively (servers, CLIs, desktop), the official `solana-client` is mature and battle-tested. Use `wasm_solana` when the same Rust code must also run in a browser, or when you want the wallet-standard-native signing model.
