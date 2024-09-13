use std::cmp::Ordering;

use async_trait::async_trait;
use solana_sdk::address_lookup_table::instruction::create_lookup_table;
use solana_sdk::address_lookup_table::instruction::extend_lookup_table;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::hash::Hash;
use solana_sdk::instruction::Instruction;
use solana_sdk::message::v0;
use solana_sdk::message::CompileError;
use solana_sdk::message::VersionedMessage;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::signer::Signer;
use solana_sdk::signer::SignerError;
use solana_sdk::signers::Signers;
use solana_sdk::transaction::VersionedTransaction;
use typed_builder::TypedBuilder;
use wallet_standard::AsyncSigner;
use wallet_standard::AsyncSigners;

use crate::ClientResult;
use crate::SolanaRpcClient;

/// Add extensions which make it possible to partially sign a versioned
/// transaction.
pub trait VersionedTransactionExtension {
	/// Create a new unsigned transaction from from the provided
	/// [`VersionedMessage`].
	fn new<T: Signers + ?Sized>(message: VersionedMessage, keypairs: &T) -> Self;
	/// Create a new unsigned transction from the payer and instructions with a
	/// recent blockhash. Under the hood this creates the message which needs to
	/// be signed.
	fn new_unsigned_v0(
		payer: &Pubkey,
		instructions: &[Instruction],
		recent_blockhash: Hash,
	) -> Result<VersionedTransaction, CompileError>;
	fn new_unsigned(message: VersionedMessage) -> VersionedTransaction;
	/// Attempt to sign this transaction with provided signers.
	fn try_sign<T: Signers + ?Sized>(
		&mut self,
		signers: &T,
		recent_blockhash: Option<Hash>,
	) -> Result<&mut Self, SignerError>;
	/// Sign the transaction with a subset of required keys, returning any
	/// errors.
	///
	/// This places each of the signatures created from `keypairs` in the
	/// corresponding position, as specified in the `positions` vector, in the
	/// transactions [`signatures`] field. It does not verify that the signature
	/// positions are correct.
	///
	/// [`signatures`]: VersionedTransaction::signatures
	///
	/// # Errors
	///
	/// Returns an error if signing fails.
	fn try_sign_unchecked<T: Signers + ?Sized>(
		&mut self,
		signers: &T,
		positions: Vec<usize>,
		recent_blockhash: Option<Hash>,
	) -> Result<(), SignerError>;
	fn get_signing_keypair_positions(
		&self,
		pubkeys: &[Pubkey],
	) -> Result<Vec<Option<usize>>, SignerError>;
	/// Sign the transaction with a subset of required keys, panicking when an
	/// error is met.
	fn sign<T: Signers + ?Sized>(
		&mut self,
		signers: &T,
		recent_blockhash: Option<Hash>,
	) -> &mut Self {
		self.try_sign(signers, recent_blockhash).unwrap()
	}
	/// Check whether the transaction is fully signed with valid signatures.
	fn is_signed(&self) -> bool;
}

impl VersionedTransactionExtension for VersionedTransaction {
	fn new<T: Signers + ?Sized>(message: VersionedMessage, keypairs: &T) -> Self {
		Self::try_new(message, keypairs).unwrap()
	}

	fn new_unsigned_v0(
		payer: &Pubkey,
		instructions: &[Instruction],
		recent_blockhash: Hash,
	) -> Result<Self, CompileError> {
		let message = v0::Message::try_compile(payer, instructions, &[], recent_blockhash)?;
		let versioned_message = VersionedMessage::V0(message);

		Ok(Self::new_unsigned(versioned_message))
	}

	/// Create an unsigned transction from a [`VersionedMessage`].
	fn new_unsigned(message: VersionedMessage) -> Self {
		let signatures =
			vec![Signature::default(); message.header().num_required_signatures as usize];

		Self {
			signatures,
			message,
		}
	}

	/// Sign the transaction with a subset of required keys, returning any
	/// errors.
	///
	/// Unlike [`VersionedTransaction::try_new`], this method does not require
	/// all keypairs to be provided, allowing a transaction to be signed in
	/// multiple steps.
	///
	/// It is permitted to sign a transaction with the same keypair multiple
	/// times.
	///
	/// If `recent_blockhash` is different than recorded in the transaction
	/// message's [`VersionedMessage::recent_blockhash()`] method, then the
	/// message's `recent_blockhash` will be updated to the provided
	/// `recent_blockhash`, and any prior signatures will be cleared.
	fn try_sign<T: Signers + ?Sized>(
		&mut self,
		keypairs: &T,
		recent_blockhash: Option<Hash>,
	) -> Result<&mut Self, SignerError> {
		let positions = self
			.get_signing_keypair_positions(&keypairs.pubkeys())?
			.iter()
			.map(|pos| pos.ok_or(SignerError::KeypairPubkeyMismatch))
			.collect::<Result<Vec<_>, _>>()?;
		self.try_sign_unchecked(keypairs, positions, recent_blockhash)?;

		Ok(self)
	}

	fn try_sign_unchecked<T: Signers + ?Sized>(
		&mut self,
		keypairs: &T,
		positions: Vec<usize>,
		recent_blockhash: Option<Hash>,
	) -> Result<(), SignerError> {
		let message_blockhash = *self.message.recent_blockhash();
		let recent_blockhash = recent_blockhash.unwrap_or(message_blockhash);

		if recent_blockhash != message_blockhash {
			self.message.set_recent_blockhash(recent_blockhash);

			// reset signatures if blockhash has changed
			self.signatures
				.iter_mut()
				.for_each(|signature| *signature = Signature::default());
		}

		let signatures = keypairs.try_sign_message(&self.message.serialize())?;

		for ii in 0..positions.len() {
			self.signatures[positions[ii]] = signatures[ii];
		}

		Ok(())
	}

	/// Get the positions of the pubkeys in
	/// [`VersionedMessage::static_account_keys`] associated with
	/// signing keypairs.
	fn get_signing_keypair_positions(
		&self,
		pubkeys: &[Pubkey],
	) -> Result<Vec<Option<usize>>, SignerError> {
		let static_account_keys = self.message.static_account_keys();

		if static_account_keys.len() < self.message.header().num_required_signatures as usize {
			return Err(SignerError::InvalidInput("invalid message".to_string()));
		}

		let signed_keys =
			&static_account_keys[0..self.message.header().num_required_signatures as usize];

		Ok(pubkeys
			.iter()
			.map(|pubkey| signed_keys.iter().position(|x| x == pubkey))
			.collect())
	}

	fn is_signed(&self) -> bool {
		self.signatures.len() == self.message.header().num_required_signatures as usize
			&& self
				.signatures
				.iter()
				.all(|signature| *signature != Signature::default())
	}
}

pub trait VersionedMessageExtension {
	fn into_versioned_transaction(self) -> VersionedTransaction;
}

impl VersionedMessageExtension for VersionedMessage {
	fn into_versioned_transaction(self) -> VersionedTransaction {
		VersionedTransaction::new_unsigned(self)
	}
}

#[async_trait(?Send)]
pub trait AsyncVersionedTransactionExtension {
	async fn try_new_async<S: Signers + ?Sized, A: AsyncSigners + ?Sized>(
		message: VersionedMessage,
		sync_signers: &S,
		async_signers: &A,
	) -> Result<VersionedTransaction, SignerError>;
}

#[async_trait(?Send)]
impl AsyncVersionedTransactionExtension for VersionedTransaction {
	async fn try_new_async<S: Signers + ?Sized, A: AsyncSigners + ?Sized>(
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
}

const MAX_LOOKUP_ADDRESSES_PER_TRANSACTION: usize = 30;

/// Initialize a lookup table that can be used with versioned transactions.
pub async fn initialize_lookup_table(
	async_signer: &impl AsyncSigner,
	rpc: SolanaRpcClient,
	addresses: &[Pubkey],
) -> ClientResult<Pubkey> {
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
		VersionedTransaction::try_new_async(versioned_message, &signers, &async_signers).await?;
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
			VersionedTransaction::try_new_async(versioned_message, &signers, &async_signers)
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
		return (0, MAX_LOOKUP_ADDRESSES_PER_TRANSACTION.min(length));
	};

	(
		previous_end,
		length.min(previous_end + MAX_LOOKUP_ADDRESSES_PER_TRANSACTION),
	)
}

#[derive(Debug, TypedBuilder)]
pub struct CreateVersionedMessage<'a> {
	pub rpc: &'a SolanaRpcClient,
	pub payer: &'a Pubkey,
	pub instructions: &'a [Instruction],
	#[builder(default, setter(into, strip_option))]
	pub lookup_accounts: Option<&'a [AddressLookupTableAccount]>,
}

impl<'a> CreateVersionedMessage<'a> {
	pub async fn run(&self) -> ClientResult<VersionedMessage> {
		let hash = self.rpc.get_latest_blockhash().await?;
		let lookup_accounts = self.lookup_accounts.unwrap_or(&[]);
		let message =
			v0::Message::try_compile(self.payer, self.instructions, lookup_accounts, hash)?;

		Ok(VersionedMessage::V0(message))
	}
}
