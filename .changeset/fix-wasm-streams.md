---
wasm_client_solana: fix
---

# Remove unused runtime dependencies from the wasm graph

Drop `solana-compute-budget` and `solana-system-program` from `wasm_client_solana`: they were unused and pulled `solana-program-runtime`, which cannot compile on wasm32-unknown-unknown.
