---
memory_wallet: fix
test_utils_solana: fix
test_utils_insta: fix
test_utils_keypairs: fix
wasm_client_solana: fix
solana-account-decoder-client-types-wasm: fix
solana-account-decoder-wasm: fix
solana-transaction-status-client-types-wasm: fix
solana-transaction-status-wasm: fix
---

# Publish the versioned crates that missed the v0.11.1 release

The v0.11.1 publish published the forks and `wasm_client_solana`, but `memory_wallet` failed because `cargo publish --locked` resolves dev-dependencies and `test_utils_solana ^0.11.1` was not on crates.io yet. The publish order now includes dev-dependencies, so this release publishes the remaining crates.
