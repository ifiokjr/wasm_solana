---
memory_wallet: patch
test_utils_insta: patch
test_utils_keypairs: patch
test_utils_solana: patch
wasm_client_solana: patch
solana-account-decoder-client-types-wasm: patch
solana-account-decoder-wasm: patch
solana-transaction-status-client-types-wasm: patch
solana-transaction-status-wasm: patch
---

# Migrate from knope to monochange

Replace knope with monochange for release planning and publishing. This removes `knope.toml`, adds `monochange.toml`, and replaces the knope-based CI workflows with monochange equivalents.
