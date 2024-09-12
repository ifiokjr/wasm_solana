# test_utils_solana

<br />

> Utilities and extensions for testing solana in wasm compatible environments.

<br />

[![Crate][crate-image]][crate-link] [![Docs][docs-image]][docs-link] [![Status][ci-status-image]][ci-status-link] [![Unlicense][unlicense-image]][unlicense-link] [![codecov][codecov-image]][codecov-link]

## Installation

To install you can used the following command:

```bash
cargo add --dev test_utils_solana
```

Or directly add the following to your `Cargo.toml`:

```toml
[dev-dependencies]
test_utils_solana = "0.1" # replace with the latest version
```

### Features

| Feature          | Description                                                                 |
| ---------------- | --------------------------------------------------------------------------- |
| `test_validator` | Enables the `test_validator` feature for the `solana_test_validator` crate. |
| `ssr`            | Enables the `ssr` feature for the `test_utils` crate.                       |
| `js`             | Enables the `js` feature for the `test_utils` crate.                        |

## Usage

The following requires the `test_validator` feature to be enabled.

```rust
use solana_sdk::pubkey;
use test_utils_solana::TestValidatorRunner;
use test_utils_solana::TestValidatorRunnerProps;

async fn run() -> TestValidatorRunner {
	let pubkey = pubkey!("99P8ZgtJYe1buSK8JXkvpLh8xPsCFuLYhz9hQFNw93WJ");
	let props = TestValidatorRunnerProps::builder()
		.pubkeys(vec![pubkey]) // pubkeys to fund with an amount of sol each
		.initial_lamports(1_000_000_000) // initial lamports to add to each pubkey account
		.namespace("tests") // namespace to use for the validator client rpc
		.build();

	TestValidatorRunner::run(props).await
}
```

[crate-image]: https://img.shields.io/crates/v/test_utils_solana.svg
[crate-link]: https://crates.io/crates/test_utils_solana
[docs-image]: https://docs.rs/test_utils_solana/badge.svg
[docs-link]: https://docs.rs/test_utils_solana/
[ci-status-image]: https://github.com/ifiokjr/wasm_solana/workflows/ci/badge.svg
[ci-status-link]: https://github.com/ifiokjr/wasm_solana/actions?query=workflow:ci
[unlicense-image]: https://img.shields.io/badge/license-Unlicence-blue.svg
[unlicense-link]: https://opensource.org/license/unlicense
[codecov-image]: https://codecov.io/github/ifiokjr/wasm_solana/graph/badge.svg?token=87K799Q78I
[codecov-link]: https://codecov.io/github/ifiokjr/wasm_solana
