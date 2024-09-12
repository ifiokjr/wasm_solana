use std::collections::HashMap;
use std::collections::HashSet;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;

use anchor_lang::AnchorSerialize;
use anchor_lang::Discriminator;
use anyhow::Context;
use anyhow::Result;
use crossbeam_channel::unbounded;
use futures::future::Shared;
use futures::Future;
use futures::FutureExt;
use lazy_static::lazy_static;
use port_check::is_local_ipv4_port_free;
use rand::Rng;
use send_wrapper::SendWrapper;
use solana_faucet::faucet::run_local_faucet_with_port;
use solana_rpc::rpc::JsonRpcConfig;
use solana_sdk::account::AccountSharedData;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::native_token::sol_to_lamports;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_program;
use solana_test_validator::TestValidator;
use solana_test_validator::TestValidatorGenesis;
use solana_test_validator::UpgradeableProgramInfo;
use typed_builder::TypedBuilder;
use wasm_client_solana::SolanaRpcClient;

use crate::FromAnchorData;

#[derive(Default, Clone, TypedBuilder)]
pub struct TestValidatorRunnerProps {
	/// The programs to add to the validator.
	#[builder(default)]
	pub programs: Vec<TestProgramInfo>,
	/// The pubkeys to fund with an amount of sol each.
	#[builder(default)]
	pub pubkeys: Vec<Pubkey>,
	/// The initial lamports to add to each pubkey account.
	#[builder(default = sol_to_lamports(50.0))]
	pub initial_lamports: u64,
	/// The default commitment level to use for the validator client rpc.
	#[builder(default, setter(into))]
	pub commitment: CommitmentLevel,
	/// Custom accounts to add during genesis. These accounts can include custom
	/// data and state.
	#[builder(default)]
	pub accounts: HashMap<Pubkey, AccountSharedData>,
	/// The namespace to use for the validator client rpc. This is used to share
	/// runners. Leave blank to always create a new runner.
	#[builder(default, setter(into, strip_option(fallback = namespace_opt)))]
	pub namespace: Option<&'static str>,
}

#[derive(Clone, TypedBuilder)]
pub struct TestProgramInfo {
	pub program_id: Pubkey,
	#[builder(setter(into))]
	pub program_path: PathBuf,
	#[builder(default = Pubkey::default())]
	pub upgrade_authority: Pubkey,
	#[builder(default = solana_sdk::bpf_loader_upgradeable::ID)]
	pub loader: Pubkey,
}

impl From<TestProgramInfo> for UpgradeableProgramInfo {
	fn from(
		TestProgramInfo {
			program_id,
			program_path,
			upgrade_authority,
			loader,
		}: TestProgramInfo,
	) -> Self {
		Self {
			program_id,
			loader,
			upgrade_authority,
			program_path,
		}
	}
}

/// A local test validator runner which can be used for the test validator.
#[derive(Clone)]
pub struct TestValidatorRunner {
	genesis: Arc<TestValidatorGenesis>,
	/// The ports used for the validator.
	/// The first port is the `rpc_port`, the second is the `pubsub_port`, and
	/// the third is the `faucet_port` to allow for airdrops.
	ports: (u16, u16, u16),
	/// The original wrapped test validator
	validator: Arc<TestValidator>,
	/// This is the keypair for the mint account and is funded with 500 SOL.
	mint_keypair: Arc<Keypair>,
	/// The rpc client for the validator.
	rpc: SolanaRpcClient,
}

impl TestValidatorRunner {
	async fn run_internal(props: TestValidatorRunnerProps) -> Result<Self> {
		let mut genesis = TestValidatorGenesis::default();
		let faucet_keypair = Keypair::new();
		let faucet_pubkey = faucet_keypair.pubkey();
		let programs = props
			.programs
			.into_iter()
			.map(Into::into)
			.collect::<Vec<_>>();
		let (rpc_port, pubsub_port, faucet_port) =
			find_ports().context("could not find a port for the solana localnet")?;

		mark_port_used(rpc_port);
		mark_port_used(pubsub_port);
		mark_port_used(faucet_port);

		let (sender, receiver) = unbounded();
		let faucet_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), faucet_port);
		// run the faucet in a seperate thread
		run_local_faucet_with_port(faucet_keypair, sender, None, None, None, faucet_port);

		let _ = receiver
			.recv()
			.expect("run solana faucet")
			.expect("there was an error running the solana faucet");

		let funded_accounts = props.pubkeys.iter().map(|pubkey| {
			(
				*pubkey,
				AccountSharedData::new(sol_to_lamports(100.0), 0, &Pubkey::default()),
			)
		});

		genesis
			.rpc_port(rpc_port)
			.rpc_config(JsonRpcConfig {
				faucet_addr: Some(faucet_addr),
				enable_rpc_transaction_history: true,
				..JsonRpcConfig::default_for_test()
			})
			// Needed to prevent all account transactions from failing with this error:
			// `Attempt to debit an account but found no record of a prior credit.`
			.warp_slot(100)
			.add_upgradeable_programs_with_path(&programs)
			.add_account(
				faucet_pubkey,
				AccountSharedData::new(sol_to_lamports(1_000_000.0), 0, &system_program::ID),
			)
			.add_accounts(funded_accounts)
			.add_accounts(props.accounts);

		let wrapped_future = SendWrapper::new(genesis.start_async());
		let (validator, mint_keypair) = wrapped_future.await;

		let rpc = SolanaRpcClient::new_with_ws_and_commitment(
			&validator.rpc_url(),
			&validator.rpc_pubsub_url(),
			CommitmentConfig {
				commitment: props.commitment,
			},
		);

		// waiting for fees to stablize doesn't seem to work, so here waiting for this
		// random airdrop to succeed seems to work. An alternative is a 15 second daily.
		// The validator to be warmed up.
		// futures_timer::Delay::new(std::time::Duration::from_secs(15)).await;
		rpc.request_airdrop(&mint_keypair.pubkey(), sol_to_lamports(500.0))
			.await?;

		let runner = Self {
			genesis: Arc::new(genesis),
			ports: (rpc_port, pubsub_port, faucet_port),
			validator: Arc::new(validator),
			mint_keypair: Arc::new(mint_keypair),
			rpc,
		};

		Ok(runner)
	}

	/// Create a new test validator runner.
	///
	/// This method is `Send` safe and can be called with a namespace that is
	/// used to reuse runners.
	///
	/// ```rust
	/// use std::sync::Arc;
	///
	/// use test_utils_solana::TestValidatorRunner;
	///
	/// async fn run() -> Arc<TestValidatorRunner> {
	/// 	TestValidatorRunner::run(
	/// 		TestValidatorRunnerProps::builder()
	/// 			.namespace("tests")
	/// 			.build(),
	/// 	)
	/// 	.await
	/// }
	/// ```
	pub async fn run(props: TestValidatorRunnerProps) -> Self {
		let namespace = props.namespace;

		if let Some(wrapped_future) = namespace.and_then(get_runner_future) {
			return wrapped_future.await;
		}

		let future = async { Self::run_internal(props).await.unwrap() };
		let wrapped_future = SendWrapper::new(future.boxed().shared());

		if let Some(name) = namespace {
			set_runner_future(name, wrapped_future.clone());
		}

		wrapped_future.await
	}

	pub fn rpc_url(&self) -> String {
		self.validator.rpc_url()
	}

	pub fn pubsub_url(&self) -> String {
		self.validator.rpc_pubsub_url()
	}

	pub fn rpc(&self) -> &SolanaRpcClient {
		&self.rpc
	}

	pub fn validator(&self) -> &TestValidator {
		&self.validator
	}

	pub fn genesis(&self) -> &TestValidatorGenesis {
		&self.genesis
	}

	pub fn ports(&self) -> (u16, u16, u16) {
		self.ports
	}

	pub fn mint_keypair(&self) -> &Keypair {
		&self.mint_keypair
	}
}

impl Drop for TestValidatorRunner {
	fn drop(&mut self) {
		free_port(self.ports.0);
		free_port(self.ports.1);
		free_port(self.ports.2);
	}
}

lazy_static! {
	static ref USED_PORTS: Arc<Mutex<HashSet<u16>>> = Arc::new(Mutex::new(HashSet::new()));
	static ref RUNNERS: Arc<Mutex<HashMap<&'static str, RunnerFuture>>> =
		Arc::new(Mutex::new(HashMap::new()));
}

pub type RunnerFuture =
	SendWrapper<Shared<Pin<Box<dyn Future<Output = TestValidatorRunner> + Send>>>>;

fn set_runner_future(name: &'static str, runner: RunnerFuture) {
	let mut runners = RUNNERS.lock().unwrap();
	runners.insert(name, runner);
}

fn get_runner_future(name: &'static str) -> Option<RunnerFuture> {
	let runners = RUNNERS.lock().unwrap();
	runners.get(name).cloned()
}

fn is_port_available(port: u16) -> bool {
	let used_ports = USED_PORTS.lock().unwrap();

	is_local_ipv4_port_free(port) && !used_ports.contains(&port)
}

fn mark_port_used(port: u16) {
	let mut used_ports = USED_PORTS.lock().unwrap();
	used_ports.insert(port);
}

fn free_port(port: u16) {
	let mut used_ports = USED_PORTS.lock().unwrap();
	used_ports.remove(&port);
}

fn find_ports() -> Option<(u16, u16, u16)> {
	let mut rng = rand::thread_rng();
	let max = u16::MAX - 2;
	let mut attempts = 100;

	loop {
		attempts -= 1;
		let port: u16 = rng.gen_range(1000..max);
		let ports = (port, port + 1, port + 2);

		if is_port_available(ports.0) && is_port_available(ports.1) && is_port_available(ports.2) {
			return Some(ports);
		}

		if attempts <= 0 {
			return None;
		}
	}
}

pub trait TestValidatorGenesisExtensions {
	fn add_account_with_anchor<T: AnchorSerialize + Discriminator>(
		&mut self,
		pubkey: Pubkey,
		owner: Pubkey,
		data: T,
	);
}

impl TestValidatorGenesisExtensions for TestValidatorGenesis {
	fn add_account_with_anchor<T: AnchorSerialize + Discriminator>(
		&mut self,
		address: Pubkey,
		owner: Pubkey,
		data: T,
	) {
		self.add_account(address, AccountSharedData::from_anchor_data(data, owner));
	}
}
