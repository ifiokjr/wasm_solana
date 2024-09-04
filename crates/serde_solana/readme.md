# `serde_solana`

<br />

> Utility for serializing and deserializing solana types

<br />

[![Crate][crate-image]][crate-link] [![Docs][docs-image]][docs-link] [![Status][ci-status-image]][ci-status-link] [![Unlicense][unlicense-image]][unlicense-link] [![codecov][codecov-image]][codecov-link]

## Installation

To install you can used the following command:

```bash
cargo add serde_solana
```

Or directly add the following to your `Cargo.toml`:

```toml
[dependencies]
serde_solana = "0.1" # replace with the latest version
```

## Usage

The following example demonstrates how to serialize and deserialize solana types using the `serde_solana` crate.

```rust
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

#[derive(serde::Serialize, serde::Deserialize)]
struct TestStruct {
	#[serde(with = "serde_solana::hash")]
	hash: Hash,
	#[serde(with = "serde_solana::pubkey")]
	pubkey: Pubkey,
	#[serde(with = "serde_solana::signature")]
	signature: Pubkey,
}
```

[crate-image]: https://img.shields.io/crates/v/serde_solana.svg
[crate-link]: https://crates.io/crates/serde_solana
[docs-image]: https://docs.rs/serde_solana/badge.svg
[docs-link]: https://docs.rs/serde_solana/
[ci-status-image]: https://github.com/ifiokjr/wasm_solana/workflows/ci/badge.svg
[ci-status-link]: https://github.com/ifiokjr/wasm_solana/actions?query=workflow:ci
[unlicense-image]: https://img.shields.io/badge/license-Unlicence-blue.svg
[unlicense-link]: https://opensource.org/license/unlicense
[codecov-image]: https://codecov.io/github/ifiokjr/wasm_solana/graph/badge.svg?token=87K799Q78I
[codecov-link]: https://codecov.io/github/ifiokjr/wasm_solana
