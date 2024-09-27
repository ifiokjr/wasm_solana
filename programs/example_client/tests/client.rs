use anyhow::Result;
use assert2::check;
use example_client::ExampleProgramClient;
use solana_sdk::account::Account;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::native_token::sol_to_lamports;
use solana_sdk::signature::Keypair;
use test_utils::SECRET_KEY_WALLET;
use test_utils_solana::anchor_processor;
use test_utils_solana::prelude::*;
use test_utils_solana::ProgramTest;
use test_utils_solana::ProgramTestContext;
use wallet_standard_wallets::MemoryWallet;
use wasm_client_solana::SolanaRpcClient;
use wasm_client_solana::LOCALNET;

#[test_log::test(tokio::test)]
async fn initialize() -> Result<()> {
	let keypair = get_wallet_keypair();
	let pubkey = keypair.pubkey();
	let (mut ctx, rpc) = create_program_test().await;
	let mut wallet = MemoryWallet::new(rpc.clone(), &[keypair]);

	wallet.connect().await?;

	let program: ExampleProgramClient<MemoryWallet> = ExampleProgramClient::builder()
		.wallet(wallet.clone())
		.rpc(rpc.clone())
		.build()
		.into();

	let simulation = program
		.initialize()
		.accounts(example_program::accounts::Initialize { unchecked: pubkey })
		.build()
		.sign_and_simulate_banks_client_transaction(&mut ctx.banks_client)
		.await?;

	check!(simulation.result.unwrap().is_ok());

	Ok(())
}

async fn create_program_test() -> (ProgramTestContext, SolanaRpcClient) {
	let pubkey = get_wallet_keypair().pubkey();
	let mut program_test = ProgramTest::new(
		"example_program",
		example_program::ID_CONST,
		anchor_processor!(example_program),
	);
	let rpc = SolanaRpcClient::new_with_commitment(LOCALNET, CommitmentConfig::finalized());

	program_test.add_account(
		pubkey,
		Account {
			lamports: sol_to_lamports(1.0),
			..Account::default()
		},
	);

	let ctx = program_test.start_with_context().await;

	(ctx, rpc)
}

pub fn get_wallet_keypair() -> Keypair {
	Keypair::from_bytes(&SECRET_KEY_WALLET).unwrap()
}
