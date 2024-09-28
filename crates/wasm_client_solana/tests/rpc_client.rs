#![cfg(feature = "js")]

use anyhow::Result;
use assert2::check;
use solana_sdk::native_token::sol_to_lamports;
use solana_sdk::pubkey::Pubkey;
use wasm_bindgen_test::*;
use wasm_client_solana::prelude::*;
use wasm_client_solana::rpc_config::LogsSubscribeRequest;
use wasm_client_solana::rpc_config::RpcTransactionLogsFilter;
use wasm_client_solana::SolanaRpcClient;
use wasm_client_solana::LOCALNET;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn request_airdrop() -> Result<()> {
	let rpc = SolanaRpcClient::new(LOCALNET);
	let pubkey = Pubkey::new_unique();
	let initial_account = rpc.get_account(&pubkey).await.ok();
	let initial_lamports = initial_account.map_or(0, |account| account.lamports);
	let lamports = sol_to_lamports(1.0);
	let signature = rpc.request_airdrop(&pubkey, lamports).await?;
	rpc.confirm_transaction(&signature).await?;

	let account = rpc.get_account(&pubkey).await?;

	check!(account.lamports - initial_lamports == lamports);

	Ok(())
}

#[wasm_bindgen_test]
async fn log_subscription() -> Result<()> {
	let rpc = SolanaRpcClient::new(LOCALNET);
	let subscription = rpc
		.logs_subscribe(
			LogsSubscribeRequest::builder()
				.filter(RpcTransactionLogsFilter::AllWithVotes)
				.build(),
		)
		.await?;

	// this doesn't work work for the node runner
	let mut stream2 = subscription.clone().take(2);

	while let Some(log_notification_request) = stream2.next().await {
		check!(log_notification_request.method == "logsNotification");
	}

	subscription.unsubscribe().await?;

	Ok(())
}
