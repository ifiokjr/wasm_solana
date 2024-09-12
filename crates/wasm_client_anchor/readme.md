# wasm_client_anchor

<br />

> Utilities and extensions for testing solana in wasm compatible environments.

<br />

[![Crate][crate-image]][crate-link] [![Docs][docs-image]][docs-link] [![Status][ci-status-image]][ci-status-link] [![Unlicense][unlicense-image]][unlicense-link] [![codecov][codecov-image]][codecov-link]

## Installation

To install you can used the following command:

```bash
cargo add wasm_client_anchor
```

Or directly add the following to your `Cargo.toml`:

```toml
[dependencies]
wasm_client_anchor = "0.1" # replace with the latest version
```

### Features

This crate provides the following features:

- `js`: Enables the use of the `wasm-bindgen` crate for the `js` target. This is useful for using the crate in a browser environment.
- `ssr`: Enables the use of the `reqwest` and `tokio` crates for using in a server or non-browser environment.

## Usage

Use `AnchorProgram` to interact directly with anchor programs.

```rust
use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::VersionedTransaction;
use wallet_standard_wallets::AnchorProgram;
use wallet_standard_wallets::AnchorRequestMethods;
use wallet_standard_wallets::WalletAnchor;
use wasm_client_anchor::AnchorClientResult;

async fn run() -> AnchorClientResult<()> {
	let program_id = pubkey!("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin");
	let anchor_program = AnchorProgram::builder()
		.rpc(rpc)
		.wallet(wallet)
		.program_id(program_id)
		.build();

	// sample instruction data
	let data = vec![0u8; 10];
	// sample accounts needed for the anchor program instruction
	let accounts = vec![AccountMeta::new(
		pubkey!("SysvarC1ock11111111111111111111111111111111"),
		false,
		true,
	)];

	// create an anchor request using the builder pattern
	let anchor_request = client.request().data(data).accounts(accounts).build();

	// get the instructions to be sent
	let instructions: Vec<Instruction> = anchor_request.instructions();

	// sign the transaction with the wallet as payer
	let versioned_transaction: VersionedTransaction =
		anchor_program.sign_transaction(anchor_request).await?;

	// send and send the transaction at the same time
	let signature: Signature = anchor_program
		.sign_and_send_transaction(anchor_request)
		.await?;

	Ok(())
}
```

[crate-image]: https://img.shields.io/crates/v/wasm_client_anchor.svg
[crate-link]: https://crates.io/crates/wasm_client_anchor
[docs-image]: https://docs.rs/wasm_client_anchor/badge.svg
[docs-link]: https://docs.rs/wasm_client_anchor/
[ci-status-image]: https://github.com/ifiokjr/wasm_solana/workflows/ci/badge.svg
[ci-status-link]: https://github.com/ifiokjr/wasm_solana/actions?query=workflow:ci
[unlicense-image]: https://img.shields.io/badge/license-Unlicence-blue.svg
[unlicense-link]: https://opensource.org/license/unlicense
[codecov-image]: https://codecov.io/github/ifiokjr/wasm_solana/graph/badge.svg?token=87K799Q78I
[codecov-link]: https://codecov.io/github/ifiokjr/wasm_solana
