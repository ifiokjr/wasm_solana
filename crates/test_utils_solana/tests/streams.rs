#![cfg(all(feature = "test_validator", feature = "ssr"))]

//! Tests for the `solana_client_wasm` crate are placed here since this depends
//! on `wasm_client_solana`.

use assert2::check;
use solana_sdk::signature::Keypair;
use test_log::test;
use test_utils::SECRET_KEY_WALLET;
use test_utils_solana::prelude::*;
use test_utils_solana::TestValidatorRunner;
use test_utils_solana::TestValidatorRunnerProps;
use wasm_client_solana::rpc_config::LogsSubscribeRequest;
use wasm_client_solana::rpc_config::RpcTransactionLogsFilter;

#[test(tokio::test)]
async fn can_create_stream() -> anyhow::Result<()> {
	let runner = create_runner().await;
	let mut rpc = runner.rpc().clone();
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
	check!(false);

	Ok(())
}

async fn create_runner() -> TestValidatorRunner {
	let pubkey = get_wallet_keypair().pubkey();
	TestValidatorRunnerProps::builder()
		.pubkeys(vec![pubkey])
		.build()
		.run()
		.await
}
pub fn get_wallet_keypair() -> Keypair {
	Keypair::from_bytes(&SECRET_KEY_WALLET).unwrap()
}
