# test_utils_anchor

<br />

> Utilities and extensions for testing anchor programs in wasm compatible environments.

<br />

[![Crate][crate-image]][crate-link] [![Docs][docs-image]][docs-link] [![Status][ci-status-image]][ci-status-link] [![Unlicense][unlicense-image]][unlicense-link] [![codecov][codecov-image]][codecov-link]

## Installation

To install you can used the following command:

```bash
cargo add --dev test_utils_anchor
```

Or directly add the following to your `Cargo.toml`:

```toml
[dev-dependencies]
test_utils_anchor = "0.1" # replace with the latest version
```

### Features

| Feature          | Description                                                                 |
| ---------------- | --------------------------------------------------------------------------- |
| `test_validator` | Enables the `test_validator` feature for the `solana_test_validator` crate. |

[crate-image]: https://img.shields.io/crates/v/test_utils_anchor.svg
[crate-link]: https://crates.io/crates/test_utils_anchor
[docs-image]: https://docs.rs/test_utils_anchor/badge.svg
[docs-link]: https://docs.rs/test_utils_anchor/
[ci-status-image]: https://github.com/ifiokjr/wasm_solana/workflows/ci/badge.svg
[ci-status-link]: https://github.com/ifiokjr/wasm_solana/actions?query=workflow:ci
[unlicense-image]: https://img.shields.io/badge/license-Unlicence-blue.svg
[unlicense-link]: https://opensource.org/license/unlicense
[codecov-image]: https://codecov.io/github/ifiokjr/wasm_solana/graph/badge.svg?token=87K799Q78I
[codecov-link]: https://codecov.io/github/ifiokjr/wasm_solana
