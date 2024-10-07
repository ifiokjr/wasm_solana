use anchor_lang::AccountDeserialize;
use anchor_lang::Event;
use anchor_lang::Key;
use async_trait::async_trait;
use serde::Serialize;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::compute_budget::ComputeBudgetInstruction;
use solana_sdk::hash::Hash;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;
use solana_sdk::message::CompileError;
use solana_sdk::message::VersionedMessage;
use solana_sdk::message::v0;
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::ParsePubkeyError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;
use solana_sdk::signer::SignerError;
use solana_sdk::transaction::VersionedTransaction;
use typed_builder::TypedBuilder;
use wallet_standard::SolanaSignAndSendTransactionOptions;
use wallet_standard::SolanaSignTransactionProps;
use wallet_standard::WalletError;
use wallet_standard::prelude::*;
use wasm_client_solana::ClientError;
use wasm_client_solana::ClientWebSocketError;
use wasm_client_solana::RpcError;
use wasm_client_solana::SimulateTransactionResponse;
use wasm_client_solana::SolanaRpcClient;
use wasm_client_solana::prelude::*;
use wasm_client_solana::rpc_config::LogsSubscribeRequest;
use wasm_client_solana::rpc_config::RpcSimulateTransactionConfig;
use wasm_client_solana::rpc_config::RpcTransactionLogsFilter;

use crate::EventSubscription;

pub trait WalletAnchor: WalletSolana + std::fmt::Debug + Clone {}
impl<T> WalletAnchor for T where T: WalletSolana + std::fmt::Debug + Clone {}

/// Use this struct to interact with anchor programs.
#[derive(Clone, Debug, TypedBuilder)]
pub struct AnchorProgram<W: WalletAnchor> {
	program_id: Pubkey,
	wallet: W,
	#[builder(setter(into))]
	rpc: SolanaRpcClient,
}

impl<W: WalletAnchor> AnchorProgram<W> {
	pub fn new(wallet: W, rpc: SolanaRpcClient, program_id: Pubkey) -> Self {
		Self {
			program_id,
			wallet,
			rpc,
		}
	}

	/// Generate a custom anchor request for instruction that you want to
	/// declare yourself.
	pub fn request(&self) -> AnchorRequestBuilderPartial<'_, W> {
		AnchorRequest::builder()
			.rpc(self.rpc())
			.program_id(self.program_id)
			.wallet(&self.wallet)
	}

	/// Sometimes you don't want to interact with the program directly, but just
	/// need to send a transaction using the wallet.
	pub fn empty_request(&self) -> EmptyAnchorRequestBuilderPartial<'_, W> {
		EmptyAnchorRequest::builder()
			.rpc(self.rpc())
			.program_id(self.program_id)
			.wallet(&self.wallet)
	}

	pub fn payer(&self) -> Pubkey {
		self.wallet().pubkey()
	}

	pub fn wallet(&self) -> &W {
		&self.wallet
	}

	pub fn id(&self) -> Pubkey {
		self.program_id.key()
	}

	pub fn rpc(&self) -> &SolanaRpcClient {
		&self.rpc
	}

	/// Get the data stared by an anchor account.
	pub async fn account<T: AccountDeserialize>(&self, address: &Pubkey) -> AnchorClientResult<T> {
		get_anchor_account(&self.rpc, address).await
	}

	/// Get an anchor event subscription.
	pub async fn subscribe<T: Event>(&self) -> AnchorClientResult<EventSubscription<T>> {
		get_anchor_subscription(self.rpc(), &self.program_id).await
	}
}

/// Create a partially typed `AnchorProgramBuilder` with the `program_id`
/// defined.
pub type AnchorProgramPartialBuilder<W> = AnchorProgramBuilder<W, ((Pubkey,), (), ())>;

/// Create a partially typed `AnchorRequestBuilder` with the `rpc`, `program_id`
/// and `wallet` defined.
pub type AnchorRequestBuilderPartial<'a, W> = AnchorRequestBuilder<
	'a,
	W,
	(
		(&'a SolanaRpcClient,),
		(Pubkey,),
		(&'a W,),
		(),
		(),
		(),
		(),
		(),
		(),
	),
>;

/// A custom anchor request with the async signer as the payer.
#[derive(Clone, TypedBuilder)]
pub struct AnchorRequest<'a, W: WalletAnchor + 'a> {
	pub rpc: &'a SolanaRpcClient,
	pub program_id: Pubkey,
	pub wallet: &'a W,
	pub data: Vec<u8>,
	pub accounts: Vec<AccountMeta>,
	#[builder(default)]
	pub signers: Vec<&'a dyn Signer>,
	#[builder(default)]
	pub instructions: Vec<Instruction>,
	#[builder(default)]
	pub extra_instructions: Vec<Instruction>,
	#[builder(default)]
	pub options: SolanaSignAndSendTransactionOptions,
}

#[async_trait(?Send)]
impl<'a, W: WalletAnchor + 'a> AnchorRequestMethods<'a, W> for AnchorRequest<'a, W> {
	fn options(&self) -> SolanaSignAndSendTransactionOptions {
		self.options.clone()
	}

	fn wallet(&self) -> &'a W {
		self.wallet
	}

	fn rpc(&self) -> &'a SolanaRpcClient {
		self.rpc
	}

	fn signers(&self) -> Vec<&'a dyn Signer> {
		self.signers.clone()
	}

	fn instructions(&self) -> Vec<Instruction> {
		let mut instructions = self.instructions.clone();

		instructions.push(Instruction {
			program_id: self.program_id,
			accounts: self.accounts.clone(),
			data: self.data.clone(),
		});

		instructions.append(&mut self.extra_instructions.clone());

		instructions
	}
}

pub type EmptyAnchorRequestBuilderPartial<'a, W> =
	EmptyAnchorRequestBuilder<'a, W, ((&'a SolanaRpcClient,), (Pubkey,), (&'a W,), (), (), ())>;

/// A custom anchor request with the async signer as the payer.
#[derive(Clone, TypedBuilder)]
pub struct EmptyAnchorRequest<'a, W: WalletAnchor + 'a> {
	pub rpc: &'a SolanaRpcClient,
	pub program_id: Pubkey,
	pub wallet: &'a W,
	#[builder(default)]
	pub sync_signers: Vec<&'a dyn Signer>,
	#[builder(default)]
	pub instructions: Vec<Instruction>,
	#[builder(default)]
	pub options: SolanaSignAndSendTransactionOptions,
}

#[async_trait(?Send)]
impl<'a, W: WalletAnchor + 'a> AnchorRequestMethods<'a, W> for EmptyAnchorRequest<'a, W> {
	fn options(&self) -> SolanaSignAndSendTransactionOptions {
		self.options.clone()
	}

	fn wallet(&self) -> &'a W {
		self.wallet
	}

	fn rpc(&self) -> &'a SolanaRpcClient {
		self.rpc
	}

	fn signers(&self) -> Vec<&'a dyn Signer> {
		self.sync_signers.clone()
	}

	fn instructions(&self) -> Vec<Instruction> {
		self.instructions.clone()
	}
}

#[async_trait(?Send)]
pub trait AnchorRequestMethods<'a, W: WalletAnchor + 'a> {
	/// The additional options for signing and sending transactions.
	fn options(&self) -> SolanaSignAndSendTransactionOptions;
	/// The solana wallet that will pay for this transaction.
	fn wallet(&self) -> &'a W;
	/// The solana client that is used to send rpc methods.
	fn rpc(&self) -> &'a SolanaRpcClient;
	/// The sync signers
	fn signers(&self) -> Vec<&'a dyn Signer>;
	/// Get the custom instructions with the program instruction appended to the
	/// end.
	fn instructions(&self) -> Vec<Instruction>;
	/// Get the unsigned message with all the instructions and the current hash.
	fn message(&self, hash: Hash) -> AnchorClientResult<VersionedMessage> {
		let payer = self.wallet().pubkey();
		let instructions = self.instructions();
		let message = v0::Message::try_compile(&payer, &instructions, &[], hash)?;

		Ok(VersionedMessage::V0(message))
	}

	/// Get the unsigned [`VersionedTransaction`].
	async fn transaction(&self) -> AnchorClientResult<VersionedTransaction> {
		let hash = self.rpc().get_latest_blockhash().await?;
		let transaction = self.message(hash)?.into_versioned_transaction();

		Ok(transaction)
	}

	/// Sign the transaction with the provided signers using the provided
	/// [`AnchorRequestMethods::wallet`].
	async fn sign_transaction(&self) -> AnchorClientResult<VersionedTransaction> {
		let signers = self.signers();
		let mut transaction = self.transaction().await?;

		// sign the transaction with local signers.
		transaction.try_sign(&signers, None)?;

		// sign the transaction in the wallet.
		let props = SolanaSignTransactionProps::builder()
			.transaction(transaction)
			.options(self.options())
			.build();

		let signed_transaction = self
			.wallet()
			.sign_transaction(props)
			.await?
			.signed_transaction()?;

		Ok(signed_transaction)
	}

	/// Sign the transaction and send it direcly to the provided rpc.
	async fn sign_and_send_transaction(&self) -> AnchorClientResult<Signature> {
		let transaction = self.sign_transaction().await?;
		let signature = self
			.rpc()
			.send_and_confirm_transaction(&transaction)
			.await?;

		Ok(signature)
	}

	/// Sign the transaction and send it direcly to the provided rpc.
	async fn sign_and_send_transaction_with_confirmation(&self) -> AnchorClientResult<Signature> {
		let signature = self.sign_and_send_transaction().await?;
		self.rpc().confirm_transaction(&signature).await?;

		Ok(signature)
	}

	/// Sign and simulate the transaction on the provided rpc endpoint.
	#[deprecated(
		since = "0.3.0",
		note = "Use [`AnchorRequestMethods::simulate_transaction`]"
	)]
	async fn sign_and_simulate_transaction(
		&self,
	) -> AnchorClientResult<SimulateTransactionResponse> {
		let transaction = self.sign_transaction().await?;
		let result = self.rpc().simulate_transaction(&transaction).await;

		Ok(result?)
	}

	/// Simulate the transaction without signing.
	async fn simulate_transaction(&self) -> AnchorClientResult<SimulateTransactionResponse> {
		let compute_limit_instruction = ComputeBudgetInstruction::set_compute_unit_limit(1_400_000);
		let payer = self.wallet().pubkey();
		let mut instructions = self.instructions();
		instructions.insert(0, compute_limit_instruction);

		let transaction = VersionedMessage::V0(v0::Message::try_compile(
			&payer,
			&instructions,
			&[],
			Hash::default(),
		)?)
		.into_versioned_transaction();
		let result = self.rpc().simulate_transaction(&transaction).await;

		Ok(result?)
	}

	/// Sign and simulate the transaction on the provided rpc endpoint with a
	/// custom configuration.
	async fn sign_and_simulate_transaction_with_config(
		&self,
		config: RpcSimulateTransactionConfig,
	) -> AnchorClientResult<SimulateTransactionResponse> {
		let transaction = self.sign_transaction().await?;
		let result = self
			.rpc()
			.simulate_transaction_with_config(&transaction, config)
			.await?;

		Ok(result)
	}
}

#[derive(Debug, Serialize, thiserror::Error)]
pub enum AnchorClientError {
	#[error("Account not found: {0}")]
	AccountNotFound(Pubkey),
	#[error("{0}")]
	Anchor(String),
	#[error("{0}")]
	Program(#[from] ProgramError),
	#[error("{0}")]
	Signer(String),
	#[error("{0}")]
	Compile(String),
	#[error("{0}")]
	Custom(String),
	#[error("{0}")]
	Rpc(#[from] RpcError),
	#[error("{0}")]
	Client(#[from] ClientError),
	#[error("{0}")]
	ClientWebsocket(#[from] ClientWebSocketError),
	#[error("Unable to parse log: {0}")]
	LogParse(String),
	#[error(transparent)]
	Wallet(#[from] WalletError),
	#[error(transparent)]
	Pubkey(#[from] ParsePubkeyError),
}

impl From<CompileError> for AnchorClientError {
	fn from(value: CompileError) -> Self {
		AnchorClientError::Compile(value.to_string())
	}
}

impl From<SignerError> for AnchorClientError {
	fn from(value: SignerError) -> Self {
		AnchorClientError::Signer(value.to_string())
	}
}

impl From<anchor_lang::error::Error> for AnchorClientError {
	fn from(value: anchor_lang::error::Error) -> Self {
		AnchorClientError::Anchor(value.to_string())
	}
}

pub type AnchorClientResult<T> = Result<T, AnchorClientError>;

/// Get an anchor account from the Solana blockchain.
pub async fn get_anchor_account<T: AccountDeserialize>(
	rpc: &SolanaRpcClient,
	address: &Pubkey,
) -> AnchorClientResult<T> {
	let account = rpc
		.get_account_with_commitment(address, CommitmentConfig::processed())
		.await?
		.ok_or(AnchorClientError::AccountNotFound(*address))?;
	let mut data: &[u8] = &account.data;
	let result = T::try_deserialize(&mut data)?;

	Ok(result)
}

/// Get an anchor events subscription.
pub async fn get_anchor_subscription<T: Event>(
	rpc: &SolanaRpcClient,
	program_id: &Pubkey,
) -> AnchorClientResult<EventSubscription<T>> {
	let request = LogsSubscribeRequest::builder()
		.filter(RpcTransactionLogsFilter::Mentions(vec![
			program_id.to_string(),
		]))
		.build();
	let subscription = rpc.logs_subscribe(request).await?;
	let event_subscription = EventSubscription::builder()
		.subscription(subscription)
		.program_id(*program_id)
		.build();

	Ok(event_subscription)
}
