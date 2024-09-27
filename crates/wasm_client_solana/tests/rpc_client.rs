#![cfg(feature = "js")]

use anyhow::Result;
use assert2::check;
use solana_sdk::account::ReadableAccount;
use solana_sdk::native_token::sol_to_lamports;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use wasm_bindgen_test::console_log;
use wasm_bindgen_test::wasm_bindgen_test;
use wasm_bindgen_test::wasm_bindgen_test_configure;
use wasm_client_solana::prelude::*;
use wasm_client_solana::rpc_config::LogsSubscribeRequest;
use wasm_client_solana::rpc_config::RpcTransactionLogsFilter;
use wasm_client_solana::SolanaRpcClient;
use wasm_client_solana::LOCALNET;

#[wasm_bindgen_test]
async fn request_airdrop() -> Result<()> {
	let rpc = SolanaRpcClient::new(LOCALNET);
	let pubkey = Pubkey::new_unique();
	let initial_account = rpc.get_account(&pubkey).await.ok();
	let initial_lamports = initial_account.map_or(0, |account| account.lamports());
	let lamports = sol_to_lamports(1.0);
	let signature = rpc.request_airdrop(&pubkey, lamports).await?;
	rpc.confirm_transaction(&signature).await?;

	let account = rpc.get_account(&pubkey).await?;

	console_log!("{account:#?}");
	check!(account.lamports() - initial_lamports == lamports);

	Ok(())
}

#[wasm_bindgen_test]
async fn logs_subscription() -> Result<()> {
	let rpc = SolanaRpcClient::new(LOCALNET);
	let subscription = rpc
		.logs_subscribe(
			LogsSubscribeRequest::builder()
				.filter(RpcTransactionLogsFilter::AllWithVotes)
				.build(),
		)
		.await?;

	check!(subscription.subscription_id() == 0);

	let mut stream5 = subscription.clone().take(5);

	while let Some(log_notification_request) = stream5.next().await {
		check!(log_notification_request.method == "logsNotification");
	}

	subscription.unsubscribe().await?;

	Ok(())
}
