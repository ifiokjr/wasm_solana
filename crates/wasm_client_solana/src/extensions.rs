use solana_sdk::hash::Hash;
use solana_sdk::message::VersionedMessage;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::signer::SignerError;
use solana_sdk::signers::Signers;
use solana_sdk::transaction::VersionedTransaction;

/// Add extensions which make it possible to partially sign a versioned
/// transaction.
pub trait VersionedTransactionExtension {
	fn new<T: Signers + ?Sized>(message: VersionedMessage, keypairs: &T) -> Self;
	fn new_unsigned(message: VersionedMessage) -> Self;
	fn try_sign<T: Signers + ?Sized>(
		&mut self,
		keypairs: &T,
		recent_blockhash: Hash,
	) -> Result<(), SignerError>;
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
		keypairs: &T,
		positions: Vec<usize>,
		recent_blockhash: Hash,
	) -> Result<(), SignerError>;
	fn get_signing_keypair_positions(
		&self,
		pubkeys: &[Pubkey],
	) -> Result<Vec<Option<usize>>, SignerError>;
	fn sign<T: Signers + ?Sized>(&mut self, keypairs: &T, recent_blockhash: Hash) {
		self.try_sign(keypairs, recent_blockhash).unwrap();
	}
}

impl VersionedTransactionExtension for VersionedTransaction {
	fn new<T: Signers + ?Sized>(message: VersionedMessage, keypairs: &T) -> Self {
		Self::try_new(message, keypairs).unwrap()
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
		recent_blockhash: Hash,
	) -> Result<(), SignerError> {
		let positions = self
			.get_signing_keypair_positions(&keypairs.pubkeys())?
			.iter()
			.map(|pos| pos.ok_or(SignerError::KeypairPubkeyMismatch))
			.collect::<Result<Vec<_>, _>>()?;

		self.try_sign_unchecked(keypairs, positions, recent_blockhash)
	}

	fn try_sign_unchecked<T: Signers + ?Sized>(
		&mut self,
		keypairs: &T,
		positions: Vec<usize>,
		recent_blockhash: Hash,
	) -> Result<(), SignerError> {
		if &recent_blockhash != self.message.recent_blockhash() {
			self.message.set_recent_blockhash(recent_blockhash);
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
}
