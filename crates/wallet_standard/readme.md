# `wallet_standard`

<br />

> An implementation of the solana wallet standard in rust.

<br />

[![Crate][crate-image]][crate-link] [![Docs][docs-image]][docs-link] [![Status][ci-status-image]][ci-status-link] [![Unlicense][unlicense-image]][unlicense-link] [![codecov][codecov-image]][codecov-link]

## Installation

To install you can used the following command:

```bash
cargo add wallet_standard
```

Or directly add the following to your `Cargo.toml`:

```toml
[dependencies]
wallet_standard = "0.1" # replace with the latest version
```

### Features

| Feature   | Description                                                            |
| --------- | ---------------------------------------------------------------------- |
| `browser` | Enables the `browser` feature for the `wallet_standard_browser` crate. |
| `solana`  | Enables the `solana` feature for the `wallet_standard_solana` crate.   |

## Usage

The [Wallet Standard](https://github.com/wallet-standard/wallet-standard) is a set of traits and conventions designed to improve the user experience and developer experience of wallets and applications for any blockchain.

This crate provides a Rust implementation of the Solana Wallet Standard, which aims to create a consistent interface for wallets and dApps to interact across different blockchain ecosystems. Here's a brief overview of how to use this crate:

1. Basic Setup:

All the traits can be imported using the prelude.

```rust
use wallet_standard::prelude::*;
```

A full example of how to use this crate can be found in the [wallet_standard_browser](https://github.com/ifiokjr/wasm_solana/tree/main/crates/wallet_standard_browser) crate and in the [wallet_standard_wallets](https://github.com/ifiokjr/wasm_solana/tree/main/crates/wallet_standard_wallets) crate.

[crate-image]: https://img.shields.io/crates/v/wallet_standard.svg
[crate-link]: https://crates.io/crates/wallet_standard
[docs-image]: https://docs.rs/wallet_standard/badge.svg
[docs-link]: https://docs.rs/wallet_standard/
[ci-status-image]: https://github.com/ifiokjr/wasm_solana/workflows/ci/badge.svg
[ci-status-link]: https://github.com/ifiokjr/wasm_solana/actions?query=workflow:ci
[unlicense-image]: https://img.shields.io/badge/license-Unlicence-blue.svg
[unlicense-link]: https://opensource.org/license/unlicense
[codecov-image]: https://codecov.io/github/ifiokjr/wasm_solana/graph/badge.svg?token=87K799Q78I
[codecov-link]: https://codecov.io/github/ifiokjr/wasm_solana
