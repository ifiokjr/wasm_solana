# The Wasm Forks

Four crates under `forks/` are maintained forks of the official Solana decoder crates. They exist because the upstream versions cannot compile on `wasm32-unknown-unknown`.

| Fork                                          | Upstream                                 |
| --------------------------------------------- | ---------------------------------------- |
| `solana-account-decoder-client-types-wasm`    | `solana-account-decoder-client-types`    |
| `solana-account-decoder-wasm`                 | `solana-account-decoder`                 |
| `solana-transaction-status-client-types-wasm` | `solana-transaction-status-client-types` |
| `solana-transaction-status-wasm`              | `solana-transaction-status`              |

## What the forks change

1. **Dependency pruning.** Native-only transitive dependencies (tokio-era crates, native C `zstd` bindings in optional positions) are removed or feature-gated so the crates compile for wasm.
2. **Optional zstd.** Upstream made `zstd` a hard dependency of `solana-account-decoder`; the fork restores it as an optional feature with a plain-base64 fallback, because `zstd-sys` cannot build for wasm32-unknown-unknown. Enable the `zstd` feature on native targets.
3. **Independent versioning.** The forks carry their own semver train (currently 3.0.10, moving to 4.0.0 with the Agave 4 family) rather than mirroring upstream patch versions. Their docs still link to the upstream crate documentation.
4. **Ergonomic enhancements.** `UiAccount` gains `serde_as`/`skip_serializing_none` attributes, a `TypedBuilder`, and a `Pubkey`-typed `owner` (still serialized as its base58 string). `UiTokenAmount` implements `Eq` so downstream response types can derive `Eq`.
5. **Upstream parity helpers.** `UiAccount::to_account` / `to_account_shared_data` follow the current upstream API.

Everything else — wire formats, field names, semantics — is byte-for-byte upstream.

## Re-syncing on an Agave upgrade

When the workspace bumps to a new Agave family, the forks are regenerated from the matching upstream sources:

1. Copy the upstream `src/` over each fork.
2. Re-apply the deltas above (dependency renames to the `-wasm` client types, the builder/`Eq`/zstd gating).
3. Run `dprint fmt` and compile for both native and wasm targets.
4. The RPC response snapshot tests in `wasm_client_solana` pin the wire format — they fail loudly if the fork drifted from the real RPC.

The `agave-unstable-api` feature is enabled on the upstream crates that gate themselves behind it, and the client-types forks compile unconditionally (they are published wasm crates).
