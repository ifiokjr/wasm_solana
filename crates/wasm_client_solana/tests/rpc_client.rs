#![cfg(feature = "js")]

use std::time::Duration;

use anyhow::Result;
use assert2::check;
use futures_timer::Delay;
use solana_sdk::native_token::sol_to_lamports;
use solana_sdk::signature::Keypair;
use test_utils_keypairs::get_wallet_keypair;
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
	let pubkey = Keypair::new().pubkey();
	let lamports = sol_to_lamports(1.0);
	let signature = rpc.request_airdrop(&pubkey, lamports).await?;
	rpc.confirm_transaction(&signature).await?;

	let account = rpc.get_account(&pubkey).await?;

	check!(account.lamports == lamports);

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
	let unsubscription = subscription.get_unsubscription();
	let mut stream2 = subscription.take(2);

	while let Some(log_notification_request) = stream2.next().await {
		console_log!("log: {log_notification_request:#?}");
		check!(log_notification_request.method == "logsNotification");
	}

	unsubscription.run().await?;

	Ok(())
}

// TODO this test doesn't actually work. Spent too long trying to get it to
// fail for the correct reason. It seems like there is a lock somewhere that is
// only released on drop. So when the subscription is dropped all the stream
// updates are processed, but nothing happens in the subscription since it has
// already been dropped.
#[wasm_bindgen_test]
async fn account_subscription() -> Result<()> {
	let date = js_sys::Date::new_0();
	console_log!("initial: {}", date.to_iso_string());
	let rpc = SolanaRpcClient::new(LOCALNET);
	let new_account = Keypair::new();
	let mut subscription = rpc.account_subscribe(new_account.pubkey()).await?;
	let unsubscription = subscription.get_unsubscription();
	let space: u64 = 100; // Size of the account data
	let lamports = rpc
		.get_minimum_balance_for_rent_exemption(space as usize)
		.await?;
	let elapsed = js_sys::Date::now() - date.get_time();
	console_log!("elapsed: {elapsed}");

	wasm_bindgen_futures::spawn_local(async move {
		console_log!("inside spawn_local");
		while let Some(account_notification) = subscription.next().await {
			console_log!("account: {account_notification:#?}");
			let value = account_notification.params.result.value.unwrap();
			check!(value.space == Some(space));
			check!(value.lamports == lamports);
		}
	});

	let payer = get_wallet_keypair();
	let instruction = solana_sdk::system_instruction::create_account(
		&payer.pubkey(),
		&new_account.pubkey(),
		lamports,
		space,
		&solana_sdk::system_program::id(),
	);
	let signature = rpc
		.request_airdrop(&payer.pubkey(), sol_to_lamports(1.0))
		.await?;
	rpc.confirm_transaction(&signature).await?;

	let elapsed = js_sys::Date::now() - date.get_time();
	console_log!("elapsed: {elapsed}");

	let recent_blockhash = rpc.get_latest_blockhash().await.unwrap();
	let transaction = solana_sdk::transaction::Transaction::new_signed_with_payer(
		&[instruction],
		Some(&payer.pubkey()),
		&[&payer, &new_account],
		recent_blockhash,
	);

	let signature = rpc
		.send_and_confirm_transaction(&transaction.into())
		.await?;
	rpc.confirm_transaction(&signature).await?;
	let elapsed = js_sys::Date::now() - date.get_time();
	console_log!("elapsed: {elapsed}");

	let signature = rpc
		.request_airdrop(&new_account.pubkey(), sol_to_lamports(1.0))
		.await?;
	rpc.confirm_transaction(&signature).await?;

	let elapsed = js_sys::Date::now() - date.get_time();
	console_log!("elapsed: {elapsed}");

	let signature = rpc
		.request_airdrop(&new_account.pubkey(), sol_to_lamports(1.0))
		.await?;
	rpc.confirm_transaction(&signature).await?;
	let elapsed = js_sys::Date::now() - date.get_time();
	console_log!("elapsed: {elapsed}");

	Delay::new(Duration::from_secs(5)).await;

	unsubscription.run().await?;

	Ok(())
}
