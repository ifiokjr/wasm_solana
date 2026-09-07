---
memory_wallet: breaking
test_utils_insta: breaking
test_utils_keypairs: breaking
test_utils_solana: breaking
wasm_client_solana: breaking
solana-account-decoder-client-types-wasm: breaking
solana-account-decoder-wasm: breaking
solana-transaction-status-client-types-wasm: breaking
solana-transaction-status-wasm: breaking
---

# Update Solana dependencies to Agave 4.x

Move every Solana crate to the latest stable Agave 4.2 family, regenerate the wasm forks from `solana-account-decoder` / `solana-transaction-status` 4.2 sources, depend on `wallet_standard` 0.6, raise the MSRV to 1.89.0 and the pinned toolchain to 1.98.1. The `agave-unstable-api` feature is enabled on the gated upstream crates.
