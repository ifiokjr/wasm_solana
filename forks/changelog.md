# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## forks [4.0.0](https://github.com/pina-rs/wasm_solana/releases/tag/forks/v4.0.0) (2026-09-08)

Grouped release for `forks`.

### Breaking Changes

#### Update Solana dependencies to Agave 4.x

_Packages:_ _solana-account-decoder-client-types-wasm_, _solana-account-decoder-wasm_, _solana-transaction-status-client-types-wasm_, _solana-transaction-status-wasm_

Move every Solana crate to the latest stable Agave 4.2 family, regenerate the wasm forks from `solana-account-decoder` / `solana-transaction-status` 4.2 sources, depend on `wallet_standard` 0.6, raise the MSRV to 1.89.0 and the pinned toolchain to 1.98.1. The forks re-version to 4.2.0 to mirror the upstream Agave family, and the workspace crates unify on a single release version.

_Owner:_ [@ifiokjr](https://github.com/ifiokjr) · _Review:_ [PR #128](https://github.com/pina-rs/wasm_solana/pull/128) · _Related issues:_ [#126](https://github.com/pina-rs/wasm_solana/issues/126)

## forks [4.0.1](https://github.com/pina-rs/wasm_solana/releases/tag/forks/v4.0.1) (2026-09-08)

Grouped release for `forks`.

### Fixes

#### Mark the workspace crates as publishable

_Packages:_ _solana-account-decoder-client-types-wasm_, _solana-account-decoder-wasm_, _solana-transaction-status-client-types-wasm_, _solana-transaction-status-wasm_

The v0.11.0 release record had no package publications because every crate manifest still carried `publish = false` from the knope era; crates.io never received the versioned crates. With `publish = true` restored this release publishes the already-versioned crates.

_Owner:_ [@ifiokjr](https://github.com/ifiokjr) · _Review:_ [PR #139](https://github.com/pina-rs/wasm_solana/pull/139)

## forks [4.0.2](https://github.com/pina-rs/wasm_solana/releases/tag/forks/v4.0.2) (2026-09-09)

Grouped release for `forks`.

### Fixes

#### Publish the versioned crates that missed the v0.11.1 release

_Packages:_ _solana-account-decoder-client-types-wasm_, _solana-account-decoder-wasm_, _solana-transaction-status-client-types-wasm_, _solana-transaction-status-wasm_

The v0.11.1 publish published the forks and `wasm_client_solana`, but `memory_wallet` failed because `cargo publish --locked` resolves dev-dependencies and `test_utils_solana ^0.11.1` was not on crates.io yet. The publish order now includes dev-dependencies, so this release publishes the remaining crates.

_Owner:_ [@ifiokjr](https://github.com/ifiokjr) · _Review:_ [PR #142](https://github.com/pina-rs/wasm_solana/pull/142)
