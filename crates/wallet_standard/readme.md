# `wallet_standard`

<br />

> Utilities and extensions for testing solana in wasm compatible environments.

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
