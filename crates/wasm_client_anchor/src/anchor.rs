use std::cmp::min;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::Map;
use std::vec::IntoIter;

use anchor_lang::AccountDeserialize;
use anchor_lang::Discriminator;
use anchor_lang::Key;
use async_trait::async_trait;
use serde::Serialize;
use solana_sdk::account::Account;
use solana_sdk::address_lookup_table::instruction::create_lookup_table;
use solana_sdk::address_lookup_table::instruction::extend_lookup_table;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::hash::Hash;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;
use solana_sdk::message::v0;
use solana_sdk::message::CompileError;
use solana_sdk::message::VersionedMessage;
use solana_sdk::program_error::ProgramError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;
use solana_sdk::signer::SignerError;
use solana_sdk::signers::Signers;
use solana_sdk::transaction::VersionedTransaction;
use typed_builder::TypedBuilder;
use wallet_standard::AsyncSigner;
use wallet_standard::AsyncSigners;
use wallet_standard::WalletError;
use wasm_client_solana::rpc_config::RpcAccountInfoConfig;
use wasm_client_solana::rpc_config::RpcProgramAccountsConfig;
use wasm_client_solana::rpc_config::RpcSimulateTransactionConfig;
use wasm_client_solana::rpc_filter::Memcmp;
use wasm_client_solana::rpc_filter::RpcFilterType;
use wasm_client_solana::solana_account_decoder::UiAccountEncoding;
use wasm_client_solana::SimulateTransactionResponse;
use wasm_client_solana::SolanaClient;
use wasm_client_solana::SolanaRpcClientError;

pub trait AnchorAsyncSigner: AsyncSigner + std::fmt::Debug + Clone {}
impl<T> AnchorAsyncSigner for T where T: AsyncSigner + std::fmt::Debug + Clone {}

#[derive(Clone, Debug, TypedBuilder)]
pub struct AnchorProgram<W: AnchorAsyncSigner> {
	program_id: Pubkey,
	wallet: W,
	#[builder(setter(into))]
	rpc: SolanaClient,
}

impl<W: AnchorAsyncSigner> AnchorProgram<W> {
	pub fn new(wallet: W, rpc: SolanaClient, program_id: Pubkey) -> Self {
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

	/// Generate a custom empty request which uses the provide async message
	/// signer as the payer.
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

	pub fn rpc(&self) -> &SolanaClient {
		&self.rpc
	}

	/// Get the data stared by an anchor account.
	pub async fn account<T: AccountDeserialize>(&self, address: &Pubkey) -> AnchorClientResult<T> {
		get_anchor_account(&self.rpc, address).await
	}

	pub async fn accounts_lazy<T: AccountDeserialize + Discriminator>(
		&self,
		filters: Vec<RpcFilterType>,
	) -> AnchorClientResult<ProgramAccountsIterator<T>> {
		let account_type_filter =
			RpcFilterType::Memcmp(Memcmp::new_base58_encoded(0, &T::discriminator()));
		let config = RpcProgramAccountsConfig {
			filters: Some([vec![account_type_filter], filters].concat()),
			account_config: RpcAccountInfoConfig {
				encoding: Some(UiAccountEncoding::Base64),
				..RpcAccountInfoConfig::default()
			},
			..RpcProgramAccountsConfig::default()
		};
		Ok(ProgramAccountsIterator {
			inner: self
				.rpc
				.get_program_accounts_with_config(&self.id(), config)
				.await?
				.into_iter()
				.map(|(key, account)| {
					Ok((key, T::try_deserialize(&mut (&account.data as &[u8]))?))
				}),
		})
	}

	pub async fn accounts<T: AccountDeserialize + Discriminator>(
		&self,
		filters: Vec<RpcFilterType>,
	) -> AnchorClientResult<HashMap<Pubkey, T>> {
		self.accounts_lazy(filters).await?.collect()
	}
}

pub type AnchorRequestBuilderPartial<'a, W> = AnchorRequestBuilder<
	'a,
	W,
	(
		(&'a SolanaClient,),
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
pub struct AnchorRequest<'a, W: AnchorAsyncSigner + 'a> {
	pub rpc: &'a SolanaClient,
	pub program_id: Pubkey,
	pub wallet: &'a W,
	pub args_data: Vec<u8>,
	pub accounts: Vec<AccountMeta>,
	#[builder(default)]
	pub sync_signers: Vec<&'a dyn Signer>,
	#[builder(default)]
	pub async_signers: Vec<&'a dyn AsyncSigner>,
	#[builder(default)]
	pub instructions: Vec<Instruction>,
	#[builder(default)]
	pub extra_instructions: Vec<Instruction>,
}

#[async_trait(?Send)]
impl<'a, W: AnchorAsyncSigner + 'a> AnchorRequestMethods<'a, W> for AnchorRequest<'a, W> {
	fn wallet(&self) -> &'a W {
		self.wallet
	}

	fn rpc(&self) -> &'a SolanaClient {
		self.rpc
	}

	fn sync_signers(&self) -> Vec<&'a dyn Signer> {
		self.sync_signers.clone()
	}

	fn async_signers(&self) -> Vec<&'a dyn AsyncSigner> {
		let mut signers = self.async_signers.clone();
		signers.append(&mut vec![self.wallet()]);

		signers
	}

	fn instructions(&self) -> Vec<Instruction> {
		let mut instructions = self.instructions.clone();

		instructions.push(Instruction {
			program_id: self.program_id,
			accounts: self.accounts.clone(),
			data: self.args_data.clone(),
		});

		instructions.append(&mut self.extra_instructions.clone());

		instructions
	}
}

pub type EmptyAnchorRequestBuilderPartial<'a, W> =
	EmptyAnchorRequestBuilder<'a, W, ((&'a SolanaClient,), (Pubkey,), (&'a W,), (), (), ())>;

/// A custom anchor request with the async signer as the payer.
#[derive(Clone, TypedBuilder)]
pub struct EmptyAnchorRequest<'a, W: AnchorAsyncSigner + 'a> {
	pub rpc: &'a SolanaClient,
	pub program_id: Pubkey,
	pub wallet: &'a W,
	#[builder(default)]
	pub sync_signers: Vec<&'a dyn Signer>,
	#[builder(default)]
	pub async_signers: Vec<&'a dyn AsyncSigner>,
	#[builder(default)]
	pub instructions: Vec<Instruction>,
}

#[async_trait(?Send)]
impl<'a, W: AnchorAsyncSigner + 'a> AnchorRequestMethods<'a, W> for EmptyAnchorRequest<'a, W> {
	fn wallet(&self) -> &'a W {
		self.wallet
	}

	fn rpc(&self) -> &'a SolanaClient {
		self.rpc
	}

	fn sync_signers(&self) -> Vec<&'a dyn Signer> {
		self.sync_signers.clone()
	}

	fn async_signers(&self) -> Vec<&'a dyn AsyncSigner> {
		let mut signers = self.async_signers.clone();
		signers.append(&mut vec![self.wallet()]);

		signers
	}

	fn instructions(&self) -> Vec<Instruction> {
		self.instructions.clone()
	}
}

#[async_trait(?Send)]
pub trait AnchorRequestMethods<'a, W: AnchorAsyncSigner + 'a> {
	/// The wallet that will pay for this transaction.
	fn wallet(&self) -> &'a W;
	/// The solana client that is used to send rpc methods.
	fn rpc(&self) -> &'a SolanaClient;
	/// The sync signers
	fn sync_signers(&self) -> Vec<&'a dyn Signer>;
	/// The async signers
	/// TODO verifiy whether there is ever a need for a custom async signer
	/// Perhaps the wallet is all that is needed.
	fn async_signers(&self) -> Vec<&'a dyn AsyncSigner>;
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

	/// Sign the transaction with the provided signers.
	async fn sign_transaction(&self) -> AnchorClientResult<VersionedTransaction> {
		let hash = self.rpc().get_latest_blockhash().await?;
		let sync_signers = self.sync_signers();
		let async_signers = self.async_signers();
		let transaction = self
			.message(hash)?
			.to_versioned_transaction(&sync_signers, &async_signers)
			.await?;

		Ok(transaction)
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
	async fn sign_and_simulate_transaction(
		&self,
	) -> AnchorClientResult<SimulateTransactionResponse> {
		let transaction = self.sign_transaction().await?;
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
	Rpc(#[from] SolanaRpcClientError),
	#[error("Unable to parse log: {0}")]
	LogParse(String),
	#[error(transparent)]
	Wallet(#[from] WalletError),
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

/// Iterator with items of type (Pubkey, T). Used to lazily deserialize account
/// structs. Wrapper type hides the inner type from usages so the implementation
/// can be changed.
pub struct ProgramAccountsIterator<T> {
	inner: Map<IntoIter<(Pubkey, Account)>, AccountConverterFunction<T>>,
}

/// Function type that accepts solana accounts and returns deserialized anchor
/// accounts
type AccountConverterFunction<T> = fn((Pubkey, Account)) -> Result<(Pubkey, T), AnchorClientError>;

impl<T> Iterator for ProgramAccountsIterator<T> {
	type Item = Result<(Pubkey, T), AnchorClientError>;

	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next()
	}
}

#[async_trait(?Send)]
pub trait AsyncVersionedMessage {
	async fn to_versioned_transaction<S: Signers + ?Sized, A: AsyncSigners + ?Sized>(
		self,
		sync_signers: &S,
		async_signers: &A,
	) -> Result<VersionedTransaction, SignerError>;
}

#[async_trait(?Send)]
impl AsyncVersionedMessage for VersionedMessage {
	async fn to_versioned_transaction<S: Signers + ?Sized, A: AsyncSigners + ?Sized>(
		self,
		sync_signers: &S,
		async_signers: &A,
	) -> Result<VersionedTransaction, SignerError> {
		try_new_async_versioned_transaction(self, sync_signers, async_signers).await
	}
}

/// Signs a versioned message and if successful, returns a signed
/// transaction.
pub async fn try_new_async_versioned_transaction<S: Signers + ?Sized, A: AsyncSigners + ?Sized>(
	message: VersionedMessage,
	sync_signers: &S,
	async_signers: &A,
) -> Result<VersionedTransaction, SignerError> {
	let static_account_keys = message.static_account_keys();

	if static_account_keys.len() < message.header().num_required_signatures as usize {
		return Err(SignerError::InvalidInput("invalid message".to_string()));
	}

	let sync_signer_keys = sync_signers.try_pubkeys()?;
	let async_signer_keys = async_signers
		.try_pubkeys()
		.map_err(|e| SignerError::Custom(e.to_string()))?;
	let signer_keys = sync_signer_keys
		.into_iter()
		.chain(async_signer_keys.into_iter())
		.collect::<Vec<_>>();
	let expected_signer_keys =
		&static_account_keys[0..message.header().num_required_signatures as usize];

	match signer_keys.len().cmp(&expected_signer_keys.len()) {
		Ordering::Greater => Err(SignerError::TooManySigners),
		Ordering::Less => Err(SignerError::NotEnoughSigners),
		Ordering::Equal => Ok(()),
	}?;

	let message_data = message.serialize();
	let signature_indexes: Vec<usize> = expected_signer_keys
		.iter()
		.map(|signer_key| {
			signer_keys
				.iter()
				.position(|key| key == signer_key)
				.ok_or(SignerError::KeypairPubkeyMismatch)
		})
		.collect::<Result<_, SignerError>>()?;

	let sync_signatures = sync_signers.try_sign_message(&message_data)?;
	let async_signatures = async_signers
		.try_sign_message(&message_data)
		.await
		.map_err(|e| SignerError::Custom(e.to_string()))?;
	let unordered_signatures = sync_signatures
		.into_iter()
		.chain(async_signatures.into_iter())
		.collect::<Vec<_>>();
	let signatures: Vec<Signature> = signature_indexes
		.into_iter()
		.map(|index| {
			unordered_signatures
				.get(index)
				.copied()
				.ok_or(SignerError::InvalidInput("invalid keypairs".to_string()))
		})
		.collect::<Result<_, SignerError>>()?;

	Ok(VersionedTransaction {
		signatures,
		message,
	})
}

const MAX_LOOKUP_ADDRESSES_PER_TRANSACTION: usize = 30;

/// Initialize a lookup table that can be used with versioned transactions.
/// TODO move to a new file
/// This is taken from the brilliant tutorial and converted to rust.
pub async fn initialize_lookup_table(
	async_signer: &impl AsyncSigner,
	rpc: SolanaClient,
	addresses: &[Pubkey],
) -> AnchorClientResult<Pubkey> {
	let slot = rpc.get_slot().await?;
	let payer = async_signer.try_pubkey()?;
	let total_addresses = addresses.len();
	let (mut start, mut end) = get_lookup_address_start_and_end(None, total_addresses);
	let (lookup_table_instruction, lookup_table_address) = create_lookup_table(payer, payer, slot);
	let extend_instruction = extend_lookup_table(
		lookup_table_address,
		payer,
		Some(payer),
		addresses[start..end].into(),
	);
	let instructions = &[lookup_table_instruction, extend_instruction];
	let versioned_message = CreateVersionedMessage::builder()
		.payer(&payer)
		.instructions(instructions)
		.rpc(&rpc)
		.build()
		.run()
		.await?;
	let signers = vec![] as Vec<&dyn Signer>;
	let async_signers = vec![async_signer as &dyn AsyncSigner];
	let versioned_transaction =
		try_new_async_versioned_transaction(versioned_message, &signers, &async_signers).await?;
	rpc.send_transaction(&versioned_transaction).await?;

	while start <= total_addresses {
		(start, end) = get_lookup_address_start_and_end(Some((start, end)), total_addresses);
		let extend_instruction = extend_lookup_table(
			lookup_table_address,
			payer,
			Some(payer),
			addresses[start..end].into(),
		);
		let versioned_message = CreateVersionedMessage::builder()
			.payer(&payer)
			.instructions(&[extend_instruction])
			.rpc(&rpc)
			.build()
			.run()
			.await?;
		let versioned_transaction =
			try_new_async_versioned_transaction(versioned_message, &signers, &async_signers)
				.await?;
		rpc.send_transaction(&versioned_transaction).await?;
	}

	Ok(lookup_table_address)
}

fn get_lookup_address_start_and_end(
	previous: Option<(usize, usize)>,
	length: usize,
) -> (usize, usize) {
	let Some((_, previous_end)) = previous else {
		return (0, min(MAX_LOOKUP_ADDRESSES_PER_TRANSACTION, length));
	};

	(
		previous_end,
		min(length, previous_end + MAX_LOOKUP_ADDRESSES_PER_TRANSACTION),
	)
}

#[derive(Debug, TypedBuilder)]
pub struct CreateVersionedMessage<'a> {
	pub rpc: &'a SolanaClient,
	pub payer: &'a Pubkey,
	pub instructions: &'a [Instruction],
	#[builder(default, setter(into, strip_option))]
	pub lookup_accounts: Option<&'a [AddressLookupTableAccount]>,
}

impl<'a> CreateVersionedMessage<'a> {
	pub async fn run(&self) -> AnchorClientResult<VersionedMessage> {
		let hash = self.rpc.get_latest_blockhash().await?;
		let lookup_accounts = self.lookup_accounts.unwrap_or(&[]);
		let message =
			v0::Message::try_compile(self.payer, self.instructions, lookup_accounts, hash)?;

		Ok(VersionedMessage::V0(message))
	}
}

pub async fn get_anchor_account<T: AccountDeserialize>(
	client: &SolanaClient,
	address: &Pubkey,
) -> AnchorClientResult<T> {
	let account = client
		.get_account_with_commitment(address, CommitmentConfig::processed())
		.await?
		.ok_or(AnchorClientError::AccountNotFound(*address))?;
	let mut data: &[u8] = &account.data;
	let result = T::try_deserialize(&mut data)?;

	Ok(result)
}
