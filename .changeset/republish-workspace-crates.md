---
memory_wallet: fix
test_utils_insta: fix
test_utils_keypairs: fix
test_utils_solana: fix
wasm_client_solana: fix
solana-account-decoder-client-types-wasm: fix
solana-account-decoder-wasm: fix
solana-transaction-status-client-types-wasm: fix
solana-transaction-status-wasm: fix
---

# Mark the workspace crates as publishable

The v0.11.0 release record had no package publications because every crate manifest still carried `publish = false` from the knope era; crates.io never received the versioned crates. With `publish = true` restored this release publishes the already-versioned crates.
