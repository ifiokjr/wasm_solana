//! Durable transaction nonce helpers.

use solana_sdk::account::Account;
use solana_sdk::account::ReadableAccount;
use solana_sdk::account_utils::StateMut;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::hash::Hash;
use solana_sdk::nonce::state::Data;
use solana_sdk::nonce::state::Versions;
use solana_sdk::nonce::State;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::system_program;

use crate::rpc_config::RpcAccountInfoConfig;
use crate::SolanaClient;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum NonceError {
	#[error("invalid account owner")]
	InvalidAccountOwner,
	#[error("invalid account data")]
	InvalidAccountData,
	#[error("unexpected account data size")]
	UnexpectedDataSize,
	#[error("provided hash ({provided}) does not match nonce hash ({expected})")]
	InvalidHash { provided: Hash, expected: Hash },
	#[error("provided authority ({provided}) does not match nonce authority ({expected})")]
	InvalidAuthority { provided: Pubkey, expected: Pubkey },
	#[error("invalid state for requested operation")]
	InvalidStateForOperation,
	#[error("client error: {0}")]
	Client(String),
}

/// Get a nonce account from the network.
///
/// This is like [`SolanaRpcClient::get_account`] except:
///
/// - it returns this module's [`NonceError`] type,
/// - it returns an error if any of the checks from [`account_identity_ok`]
///   fail.
pub async fn get_account(
	rpc_client: &SolanaClient,
	nonce_pubkey: &Pubkey,
) -> Result<Account, NonceError> {
	get_account_with_commitment(rpc_client, nonce_pubkey, CommitmentConfig::default()).await
}

/// Get a nonce account from the network.
///
/// This is like [`SolanaRpcClient::get_account_with_commitment`] except:
///
/// - it returns this module's [`NonceError`] type,
/// - it returns an error if the account does not exist,
/// - it returns an error if any of the checks from [`account_identity_ok`]
///   fail.
pub async fn get_account_with_commitment(
	rpc_client: &SolanaClient,
	nonce_pubkey: &Pubkey,
	commitment_config: CommitmentConfig,
) -> Result<Account, NonceError> {
	rpc_client
		.get_account_with_config(
			nonce_pubkey,
			RpcAccountInfoConfig {
				commitment: Some(commitment_config),
				..Default::default()
			},
		)
		.await
		.map_err(|e| NonceError::Client(format!("{e}")))
		.and_then(|opt| {
			opt.ok_or_else(|| NonceError::Client(format!("AccountNotFound: pubkey={nonce_pubkey}")))
		})
		.and_then(|a| account_identity_ok(&a).map(|()| a))
}

/// Perform basic checks that an account has nonce-like properties.
///
/// # Errors
///
/// Returns [`NonceError::InvalidAccountOwner`] if the account is not owned by
/// the system program. Returns [`NonceError::UnexpectedDataSize`] if the
/// account contains no data.
pub fn account_identity_ok<T: ReadableAccount>(account: &T) -> Result<(), NonceError> {
	if account.owner() != &system_program::id() {
		Err(NonceError::InvalidAccountOwner)
	} else if account.data().is_empty() {
		Err(NonceError::UnexpectedDataSize)
	} else {
		Ok(())
	}
}

/// Deserialize the state of a durable transaction nonce account.
///
/// # Errors
///
/// Returns an error if the account is not owned by the system program or
/// contains no data.
///
/// # Examples
///
/// Determine if a nonce account is initialized:
///
/// ```no_run
/// use anyhow::Result;
/// use solana_client::nonce_utils;
/// use solana_client::rpc_client::SolanaRpcClient;
/// use solana_sdk::nonce::State;
/// use solana_sdk::pubkey::Pubkey;
///
/// fn is_nonce_initialized(
/// 	client: &SolanaRpcClient,
/// 	nonce_account_pubkey: &Pubkey,
/// ) -> Result<bool> {
/// 	// Sign the tx with nonce_account's `blockhash` instead of the
/// 	// network's latest blockhash.
/// 	let nonce_account = client.get_account(nonce_account_pubkey)?;
/// 	let nonce_state = nonce_utils::state_from_account(&nonce_account)?;
///
/// 	Ok(!matches!(nonce_state, State::Uninitialized))
/// }
/// #
/// # let client = SolanaRpcClient::new(String::new());
/// # let nonce_account_pubkey = Pubkey::new_unique();
/// # is_nonce_initialized(&client, &nonce_account_pubkey)?;
/// #
/// # Ok::<(), anyhow::NonceError>(())
/// ```
pub fn state_from_account<T: ReadableAccount + StateMut<Versions>>(
	account: &T,
) -> Result<State, NonceError> {
	account_identity_ok(account)?;
	StateMut::<Versions>::state(account)
		.map_err(|_| NonceError::InvalidAccountData)
		.map(|v| v.state().clone())
}

/// Deserialize the state data of a durable transaction nonce account.
///
/// # Errors
///
/// Returns an error if the account is not owned by the system program or
/// contains no data. Returns an error if the account state is uninitialized or
/// fails to deserialize.
///
/// # Examples
///
/// Create and sign a transaction with a durable nonce:
///
/// ```no_run
/// use solana_client::{
///     rpc_client::SolanaRpcClient,
///     nonce_utils,
/// };
/// use solana_sdk::{
///     message::Message,
///     pubkey::Pubkey,
///     signature::{Keypair, Signer},
///     system_instruction,
///     transaction::Transaction,
/// };
/// use std::path::Path;
/// use anyhow::Result;
/// # use anyhow::anyhow;
///
/// fn create_transfer_tx_with_nonce(
///     client: &SolanaRpcClient,
///     nonce_account_pubkey: &Pubkey,
///     payer: &Keypair,
///     receiver: &Pubkey,
///     amount: u64,
///     tx_path: &Path,
/// ) -> Result<()> {
///
///     let instr_transfer = system_instruction::transfer(
///         &payer.pubkey(),
///         receiver,
///         amount,
///     );
///
///     // In this example, `payer` is `nonce_account_pubkey`'s authority
///     let instr_advance_nonce_account = system_instruction::advance_nonce_account(
///         nonce_account_pubkey,
///         &payer.pubkey(),
///     );
///
///     // The `advance_nonce_account` instruction must be the first issued in
///     // the transaction.
///     let message = Message::new(
///         &[
///             instr_advance_nonce_account,
///             instr_transfer
///         ],
///         Some(&payer.pubkey()),
///     );
///
///     let mut tx = Transaction::new_unsigned(message);
///
///     // Sign the tx with nonce_account's `blockhash` instead of the
///     // network's latest blockhash.
///     let nonce_account = client.get_account(nonce_account_pubkey)?;
///     let nonce_data = nonce_utils::data_from_account(&nonce_account)?;
///     let blockhash = nonce_data.blockhash();
///
///     tx.try_sign(&[payer], blockhash)?;
///
///     // Save the signed transaction locally for later submission.
///     save_tx_to_file(&tx_path, &tx)?;
///
///     Ok(())
/// }
/// #
/// # fn save_tx_to_file(path: &Path, tx: &Transaction) -> Result<()> {
/// #     Ok(())
/// # }
/// #
/// # let client = SolanaRpcClient::new(String::new());
/// # let nonce_account_pubkey = Pubkey::new_unique();
/// # let payer = Keypair::new();
/// # let receiver = Pubkey::new_unique();
/// # create_transfer_tx_with_nonce(&client, &nonce_account_pubkey, &payer, &receiver, 1024, Path::new("new_tx"))?;
/// #
/// # Ok::<(), anyhow::NonceError>(())
/// ```
pub fn data_from_account<T: ReadableAccount + StateMut<Versions>>(
	account: &T,
) -> Result<Data, NonceError> {
	account_identity_ok(account)?;
	state_from_account(account).and_then(|ref s| data_from_state(s).cloned())
}

/// Get the nonce data from its [`State`] value.
///
/// # Errors
///
/// Returns [`NonceError::InvalidStateForOperation`] if `state` is
/// [`State::Uninitialized`].
pub fn data_from_state(state: &State) -> Result<&Data, NonceError> {
	match state {
		State::Uninitialized => Err(NonceError::InvalidStateForOperation),
		State::Initialized(data) => Ok(data),
	}
}
