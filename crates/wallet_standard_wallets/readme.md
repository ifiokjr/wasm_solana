# `wallet_standard_wallets`

<br />

> A collection of solana wallet implementations primarily used for testing.

<br />

[![Crate][crate-image]][crate-link] [![Docs][docs-image]][docs-link] [![Status][ci-status-image]][ci-status-link] [![Unlicense][unlicense-image]][unlicense-link] [![codecov][codecov-image]][codecov-link]

## Installation

To install you can used the following command:

```bash
cargo add wallet_standard_wallets
```

Or directly add the following to your `Cargo.toml`:

```toml
[dependencies]
wallet_standard_wallets = "0.1" # replace with the latest version
```

### Features

- `ssr` Enables the `ssr` feature for the `wallet_standard` crate.
- `js` Enables the `js` feature to unlock wasm support for the `wallet_standard` crate.

## Usage

The memory wallet is a simple wallet that stores all accounts in memory and conforms to the `WalletStandard` trait.

```rust
use anyhow::Result;
use solana_sdk::native_token::sol_to_lamports;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;
use solana_sdk::system_instruction;
use solana_sdk::transaction::VersionedTransaction;
use wallet_standard::SolanaSignTransactionProps;
use wallet_standard_wallets::prelude::*;
use wallet_standard_wallets::MemoryWallet;
use wasm_client_solana::SolanaRpcClient;
use wasm_client_solana::DEVNET;

async fn run() -> Result<()> {
	let keypair = Keypair::new();
	let pubkey = keypair.pubkey();
	let target_pubkey = Pubkey::new_unique();
	let instruction = system_instruction::transfer(&pubkey, &target_pubkey, sol_to_lamports(0.5));
	let rpc = SolanaRpcClient::new(DEVNET);
	let blockhash = rpc.get_latest_blockhash().await?;
	let transaction = VersionedTransaction::new_unsigned_v0(&pubkey, &[instruction], blockhash)?;
	let mut memory_wallet = MemoryWallet::new(rpc, &[keypair]);

	// connect the first account in the memory wallet accounts list
	memory_wallet.connect().await?;

	let props = SolanaSignTransactionProps::builder()
		.transaction(transaction)
		.build();
	let signed_transaction: VersionedTransaction = memory_wallet.sign_transaction(props).await?;

	Ok(())
}
```

[crate-image]: https://img.shields.io/crates/v/wallet_standard_wallets.svg
[crate-link]: https://crates.io/crates/wallet_standard_wallets
[docs-image]: https://docs.rs/wallet_standard_wallets/badge.svg
[docs-link]: https://docs.rs/wallet_standard_wallets/
[ci-status-image]: https://github.com/ifiokjr/wasm_solana/workflows/ci/badge.svg
[ci-status-link]: https://github.com/ifiokjr/wasm_solana/actions?query=workflow:ci
[unlicense-image]: https://img.shields.io/badge/license-Unlicence-blue.svg
[unlicense-link]: https://opensource.org/license/unlicense
[codecov-image]: https://codecov.io/github/ifiokjr/wasm_solana/graph/badge.svg?token=87K799Q78I
[codecov-link]: https://codecov.io/github/ifiokjr/wasm_solana
