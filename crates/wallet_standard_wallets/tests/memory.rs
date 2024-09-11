#![cfg(feature = "ssr")]

use std::sync::Arc;

use anyhow::Result;
use assert2::check;
use solana_sdk::native_token::sol_to_lamports;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signature::Signature;
use solana_sdk::system_instruction::transfer;
use solana_sdk::transaction::VersionedTransaction;
use test_utils::SECRET_KEY_WALLET;
use test_utils_solana::TestValidatorRunner;
use test_utils_solana::TestValidatorRunnerProps;
use wallet_standard::SolanaSignAndSendTransactionProps;
use wallet_standard::SolanaSignTransactionProps;
use wallet_standard_wallets::prelude::*;
use wallet_standard_wallets::MemoryWallet;

#[tokio::test]
async fn sign_transaction() -> Result<()> {
	let runner = run().await;
	let keypair = get_wallet_keypair();
	let pubkey = keypair.pubkey();
	let target_pubkey = Pubkey::new_unique();
	let instruction = transfer(&pubkey, &target_pubkey, sol_to_lamports(0.5));
	let blockhash = runner.rpc.get_latest_blockhash().await?;
	let transaction = VersionedTransaction::new_unsigned_v0(&pubkey, &[instruction], blockhash)?;
	let mut memory_wallet = MemoryWallet::new(runner.rpc.clone(), &[keypair]);

	memory_wallet.connect().await?;

	let props = SolanaSignTransactionProps::builder()
		.transaction(transaction)
		.build();
	let signed_transaction = memory_wallet.sign_transaction(props).await?;

	check!(signed_transaction.is_signed());

	Ok(())
}

#[tokio::test]
async fn sign_and_send_transaction() -> Result<()> {
	let runner = run().await;
	let keypair = get_wallet_keypair();
	let pubkey = keypair.pubkey();
	let target_pubkey = Pubkey::new_unique();
	let instruction = transfer(&pubkey, &target_pubkey, sol_to_lamports(0.5));
	let blockhash = runner.rpc.get_latest_blockhash().await?;
	let transaction = VersionedTransaction::new_unsigned_v0(&pubkey, &[instruction], blockhash)?;
	let mut memory_wallet = MemoryWallet::new(runner.rpc.clone(), &[keypair]);

	memory_wallet.connect().await?;

	let props = SolanaSignAndSendTransactionProps::builder()
		.transaction(transaction)
		.build();
	let signature = memory_wallet.sign_and_send_transaction(props).await?;

	check!(signature != Signature::default());

	Ok(())
}

async fn run() -> Arc<TestValidatorRunner> {
	let pubkey = get_wallet_keypair().pubkey();
	TestValidatorRunner::run_shared(
		Some("tests"),
		TestValidatorRunnerProps::builder()
			.pubkeys(vec![pubkey])
			.build(),
	)
	.await
}

pub fn get_wallet_keypair() -> Keypair {
	Keypair::from_bytes(&SECRET_KEY_WALLET).unwrap()
}
