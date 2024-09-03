use std::time::Duration;

use futures_timer::Delay;
use solana_sdk::account::Account;
use solana_sdk::clock::Epoch;
use solana_sdk::clock::Slot;
use solana_sdk::clock::UnixTimestamp;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::epoch_info::EpochInfo;
use solana_sdk::epoch_schedule::EpochSchedule;
use solana_sdk::hash::Hash;
use solana_sdk::message::Message;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::VersionedTransaction;

use crate::methods::*;
use crate::rpc_config::GetConfirmedSignaturesForAddress2Config;
use crate::rpc_config::RpcAccountInfoConfig;
use crate::rpc_config::RpcBlockConfig;
use crate::rpc_config::RpcBlockProductionConfig;
use crate::rpc_config::RpcContextConfig;
use crate::rpc_config::RpcEpochConfig;
use crate::rpc_config::RpcGetVoteAccountsConfig;
use crate::rpc_config::RpcKeyedAccount;
use crate::rpc_config::RpcLargestAccountsConfig;
use crate::rpc_config::RpcLeaderScheduleConfig;
use crate::rpc_config::RpcProgramAccountsConfig;
use crate::rpc_config::RpcSendTransactionConfig;
use crate::rpc_config::RpcSignaturesForAddressConfig;
use crate::rpc_config::RpcSimulateTransactionConfig;
use crate::rpc_config::RpcSupplyConfig;
use crate::rpc_config::RpcTokenAccountsFilter;
use crate::rpc_config::RpcTransactionConfig;
use crate::rpc_filter::TokenAccountsFilter;
use crate::rpc_response::RpcAccountBalance;
use crate::rpc_response::RpcBlockProduction;
use crate::rpc_response::RpcConfirmedTransactionStatusWithSignature;
use crate::rpc_response::RpcInflationGovernor;
use crate::rpc_response::RpcInflationRate;
use crate::rpc_response::RpcInflationReward;
use crate::rpc_response::RpcLeaderSchedule;
use crate::rpc_response::RpcPerfSample;
use crate::rpc_response::RpcSupply;
use crate::rpc_response::RpcVersionInfo;
use crate::rpc_response::RpcVoteAccountStatus;
use crate::solana_account_decoder::parse_address_lookup_table::parse_address_lookup_table;
use crate::solana_account_decoder::parse_address_lookup_table::LookupTableAccountType;
use crate::solana_account_decoder::parse_token::TokenAccountType;
use crate::solana_account_decoder::parse_token::UiTokenAccount;
use crate::solana_account_decoder::parse_token::UiTokenAmount;
use crate::solana_account_decoder::UiAccountData;
use crate::solana_account_decoder::UiAccountEncoding;
use crate::solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;
use crate::solana_transaction_status::TransactionConfirmationStatus;
use crate::solana_transaction_status::UiConfirmedBlock;
use crate::solana_transaction_status::UiTransactionEncoding;
use crate::ClientRequest;
use crate::ClientResponse;
use crate::ClientResult;
use crate::HttpProvider;
use crate::SolanaRpcClientError;
use crate::MAX_RETRIES;
use crate::SLEEP_MS;

/// A client of a remote Solana node.
///
/// `RpcClient` communicates with a Solana node over [JSON-RPC], with the
/// [Solana JSON-RPC protocol][jsonprot]. It is the primary Rust interface for
/// querying and transacting with the network from external programs.
///
/// This type builds on the underlying RPC protocol, adding extra features such
/// as timeout handling, retries, and waiting on transaction [commitment
/// levels][cl]. Some methods simply pass through to the underlying RPC
/// protocol. Not all RPC methods are encapsulated by this type, but
/// `SolanaRpcClient` does expose a generic [`send`](SolanaRpcClient::send)
/// method for making any [`ClientRequest`].
///
/// The documentation for most [`SolanaRpcClient`] methods contains an "RPC
/// Reference" section that links to the documentation for the underlying
/// JSON-RPC method. The documentation for `RpcClient` does not reproduce the
/// documentation for the underlying JSON-RPC methods. Thus reading both is
/// necessary for complete understanding.
///
/// `RpcClient`s generally communicate over HTTP on port 8899, a typical server
/// URL being "<http://localhost:8899>".
///
/// Methods that query information from recent [slots], including those that
/// confirm transactions, decide the most recent slot to query based on a
/// [commitment level][cl], which determines how committed or finalized a slot
/// must be to be considered for the query. Unless specified otherwise, the
/// commitment level is [`Finalized`], meaning the slot is definitely
/// permanently committed. The default commitment level can be configured by
/// creating [`SolanaRpcClient`] with an explicit [`CommitmentConfig`], and that
/// default configured commitment level can be overridden by calling the various
/// `_with_commitment` methods, like
/// [`SolanaRpcClient::confirm_transaction_with_commitment`]. In some cases the
/// configured commitment level is ignored and `Finalized` is used instead, as
/// in [`SolanaRpcClient::get_blocks`], where it would be invalid to use the
/// [`Processed`] commitment level. These exceptions are noted in the method
/// documentation.
///
/// [`Finalized`]: CommitmentLevel::Finalized
/// [`Processed`]: CommitmentLevel::Processed
/// [jsonprot]: https://solana.com/docs/rpc
/// [JSON-RPC]: https://www.jsonrpc.org/specification
/// [slots]: https://solana.com/docs/terminology#slot
/// [cl]: https://solana.com/docs/rpc#configuring-state-commitment
///
/// # Errors
///
/// Methods on [`SolanaRpcClient`] return
/// [`ClientResult`], and many of them
/// return [`ClientResponse`].
///
/// Requests may timeout, in which case they return a [`ClientError`].
#[derive(derive_more::Debug, Clone)]
pub struct SolanaRpcClient {
	commitment_config: CommitmentConfig,
	#[debug(skip)]
	provider: HttpProvider,
}

impl<S: Into<String>> From<S> for SolanaRpcClient {
	fn from(value: S) -> Self {
		Self::new(value)
	}
}

impl SolanaRpcClient {
	/// Create an HTTP `SolanaRpcClient`.
	///
	/// The URL is an HTTP URL, usually for port 8899, as in
	/// "<http://localhost:8899>".
	///
	/// The client has a default timeout of 30 seconds, and a default
	/// [commitment level][cl] of [`Finalized`](CommitmentLevel::Finalized).
	///
	/// [cl]: https://solana.com/docs/rpc#configuring-state-commitment
	///
	/// # Examples
	///
	/// ```
	/// # use solana_rpc_client::nonblocking::rpc_client::RpcClient;
	/// let url = "http://localhost:8899".to_string();
	/// let client = RpcClient::new(url);
	/// ```
	pub fn new(endpoint: impl Into<String>) -> Self {
		Self {
			provider: HttpProvider::new(endpoint),
			commitment_config: CommitmentConfig::confirmed(),
		}
	}

	/// Create an HTTP `RpcClient` with specified [commitment level][cl].
	///
	/// [cl]: https://solana.com/docs/rpc#configuring-state-commitment
	///
	/// The URL is an HTTP URL, usually for port 8899, as in
	/// "<http://localhost:8899>".
	///
	/// The client has a default timeout of 30 seconds, and a user-specified
	/// [`CommitmentLevel`] via [`CommitmentConfig`].
	///
	/// # Examples
	///
	/// ```
	/// # use solana_sdk::commitment_config::CommitmentConfig;
	/// # use solana_rpc_client::nonblocking::rpc_client::RpcClient;
	/// let url = "http://localhost:8899".to_string();
	/// let commitment_config = CommitmentConfig::processed();
	/// let client = SolanaRpcClient::new_with_commitment(url, commitment_config);
	/// ```
	pub fn new_with_commitment(endpoint: &str, commitment_config: CommitmentConfig) -> Self {
		Self {
			provider: HttpProvider::new(endpoint),
			commitment_config,
		}
	}

	/// Get the URL.
	pub fn url(&self) -> &str {
		self.provider.url()
	}

	pub fn commitment(&self) -> CommitmentLevel {
		self.commitment_config.commitment
	}

	pub fn commitment_config(&self) -> CommitmentConfig {
		self.commitment_config
	}

	pub async fn send(&self, request: ClientRequest) -> ClientResult<ClientResponse> {
		let result = self.provider.send(&request).await?;
		Ok(result)
	}

	pub async fn get_account_with_config(
		&self,
		pubkey: &Pubkey,
		config: RpcAccountInfoConfig,
	) -> ClientResult<Option<Account>> {
		let request = GetAccountInfoRequest::new_with_config(*pubkey, config).into();
		let response = GetAccountInfoResponse::from(self.send(request).await?);

		match response.value {
			Some(ui_account) => Ok(ui_account.decode()),
			None => Ok(None),
		}
	}

	pub async fn get_account_with_commitment(
		&self,
		pubkey: &Pubkey,
		commitment_config: CommitmentConfig,
	) -> ClientResult<Option<Account>> {
		self.get_account_with_config(
			pubkey,
			RpcAccountInfoConfig {
				commitment: Some(commitment_config),
				encoding: Some(UiAccountEncoding::Base64),
				..Default::default()
			},
		)
		.await
	}

	pub async fn get_account(&self, pubkey: &Pubkey) -> ClientResult<Account> {
		self.get_account_with_commitment(pubkey, self.commitment_config())
			.await?
			.ok_or_else(|| SolanaRpcClientError::new(format!("Account {pubkey} not found.")))
	}

	pub async fn get_account_data(&self, pubkey: &Pubkey) -> ClientResult<Vec<u8>> {
		Ok(self.get_account(pubkey).await?.data)
	}

	pub async fn get_balance_with_commitment(
		&self,
		pubkey: &Pubkey,
		commitment_config: CommitmentConfig,
	) -> ClientResult<u64> {
		let request = GetBalanceRequest::new_with_config(*pubkey, commitment_config).into();

		let response = GetBalanceResponse::from(self.send(request).await?);

		Ok(response.value)
	}

	pub async fn get_balance(&self, pubkey: &Pubkey) -> ClientResult<u64> {
		self.get_balance_with_commitment(pubkey, self.commitment_config())
			.await
	}

	pub async fn request_airdrop(&self, pubkey: &Pubkey, lamports: u64) -> ClientResult<Signature> {
		let request =
			RequestAirdropRequest::new_with_config(*pubkey, lamports, self.commitment_config)
				.into();
		let response = RequestAirdropResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn get_signature_statuses(
		&self,
		signatures: &[Signature],
	) -> ClientResult<Vec<Option<SignatureStatusesValue>>> {
		let request = GetSignatureStatusesRequest::new(signatures.into()).into();
		let response = GetSignatureStatusesResponse::from(self.send(request).await?);

		Ok(response.value)
	}

	pub async fn get_transaction_with_config(
		&self,
		signature: &Signature,
		config: RpcTransactionConfig,
	) -> ClientResult<EncodedConfirmedTransactionWithStatusMeta> {
		let request = GetTransactionRequest::new_with_config(*signature, config).into();
		let response = GetTransactionResponse::from(self.send(request).await?);

		match response.into() {
			Some(result) => Ok(result),
			None => {
				Err(SolanaRpcClientError::new(format!(
					"Signature {signature} not found."
				)))
			}
		}
	}

	pub async fn get_latest_blockhash_with_config(
		&self,
		commitment_config: CommitmentConfig,
	) -> ClientResult<(Hash, u64)> {
		let request = GetLatestBlockhashRequest::new_with_config(commitment_config).into();
		let response = GetLatestBlockhashResponse::from(self.send(request).await?);

		let hash = response
			.value
			.blockhash
			.parse()
			.map_err(|_| SolanaRpcClientError::new("Hash not parsable."))?;

		Ok((hash, response.value.last_valid_block_height))
	}

	pub async fn get_latest_blockhash_with_commitment(
		&self,
		commitment_config: CommitmentConfig,
	) -> ClientResult<(Hash, u64)> {
		self.get_latest_blockhash_with_config(commitment_config)
			.await
	}

	pub async fn get_latest_blockhash(&self) -> ClientResult<Hash> {
		let result = self
			.get_latest_blockhash_with_commitment(self.commitment_config())
			.await?;

		Ok(result.0)
	}

	pub async fn is_blockhash_valid(
		&self,
		blockhash: &Hash,
		commitment_config: CommitmentConfig,
	) -> ClientResult<bool> {
		let request = IsBlockhashValidRequest::new_with_config(
			*blockhash,
			RpcContextConfig {
				commitment: Some(commitment_config),
				min_context_slot: None,
			},
		)
		.into();
		let response = IsBlockhashValidResponse::from(self.send(request).await?);

		Ok(response.value)
	}

	pub async fn get_minimum_balance_for_rent_exemption(
		&self,
		data_len: usize,
	) -> ClientResult<u64> {
		let request = GetMinimumBalanceForRentExemptionRequest::new(data_len).into();
		let response = GetMinimumBalanceForRentExemptionResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn get_fee_for_message(&self, message: &Message) -> ClientResult<u64> {
		let request = GetFeeForMessageRequest::new(message.to_owned()).into();
		let response = GetFeeForMessageResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn send_transaction_with_config(
		&self,
		transaction: &VersionedTransaction,
		config: RpcSendTransactionConfig,
	) -> ClientResult<Signature> {
		let request =
			SendTransactionRequest::new_with_config(transaction.to_owned(), config).into();
		let response = SendTransactionResponse::from(self.send(request).await?);

		let signature: Signature = response.into();

		// A mismatching RPC response signature indicates an issue with the RPC node,
		// and should not be passed along to confirmation methods. The transaction may
		// or may not have been submitted to the cluster, so callers should verify the
		// success of the correct transaction signature independently.
		if signature == transaction.signatures[0] {
			Ok(transaction.signatures[0])
		} else {
			Err(SolanaRpcClientError::new(format!(
				"RPC node returned mismatched signature {:?}, expected {:?}",
				signature, transaction.signatures[0]
			)))
		}
	}

	pub async fn send_transaction(
		&self,
		transaction: &VersionedTransaction,
	) -> ClientResult<Signature> {
		self.send_transaction_with_config(
			transaction,
			RpcSendTransactionConfig {
				preflight_commitment: Some(self.commitment()),
				encoding: Some(UiTransactionEncoding::Base64),
				..Default::default()
			},
		)
		.await
	}

	pub async fn confirm_transaction_with_commitment(
		&self,
		signature: &Signature,
		commitment_config: CommitmentConfig,
	) -> ClientResult<bool> {
		let mut is_success = false;
		for _ in 0..MAX_RETRIES {
			let signature_statuses = self.get_signature_statuses(&[*signature]).await?;

			if let Some(signature_status) = signature_statuses[0].as_ref() {
				if signature_status.confirmation_status.is_some() {
					let current_commitment = signature_status.confirmation_status.as_ref().unwrap();

					let commitment_matches = match commitment_config.commitment {
						CommitmentLevel::Finalized => {
							matches!(current_commitment, TransactionConfirmationStatus::Finalized)
						}
						CommitmentLevel::Confirmed => {
							matches!(
								current_commitment,
								TransactionConfirmationStatus::Finalized
									| TransactionConfirmationStatus::Confirmed
							)
						}
						_ => true,
					};
					if commitment_matches {
						is_success = signature_status.err.is_none();
						break;
					}
				}
			}

			Delay::new(Duration::from_millis(SLEEP_MS)).await;
		}

		Ok(is_success)
	}

	pub async fn confirm_transaction(&self, signature: &Signature) -> ClientResult<bool> {
		self.confirm_transaction_with_commitment(signature, self.commitment_config())
			.await
	}

	pub async fn send_and_confirm_transaction_with_config(
		&self,
		transaction: &VersionedTransaction,
		commitment_config: CommitmentConfig,
		config: RpcSendTransactionConfig,
	) -> ClientResult<Signature> {
		let tx_hash = self
			.send_transaction_with_config(transaction, config)
			.await?;

		self.confirm_transaction_with_commitment(&tx_hash, commitment_config)
			.await?;

		Ok(tx_hash)
	}

	pub async fn send_and_confirm_transaction_with_commitment(
		&self,
		transaction: &VersionedTransaction,
		commitment_config: CommitmentConfig,
	) -> ClientResult<Signature> {
		self.send_and_confirm_transaction_with_config(
			transaction,
			commitment_config,
			RpcSendTransactionConfig {
				preflight_commitment: Some(commitment_config.commitment),
				encoding: Some(UiTransactionEncoding::Base64),
				..Default::default()
			},
		)
		.await
	}

	pub async fn send_and_confirm_transaction(
		&self,
		transaction: &VersionedTransaction,
	) -> ClientResult<Signature> {
		self.send_and_confirm_transaction_with_commitment(transaction, self.commitment_config())
			.await
	}

	pub async fn get_program_accounts_with_config(
		&self,
		pubkey: &Pubkey,
		config: RpcProgramAccountsConfig,
	) -> ClientResult<Vec<(Pubkey, Account)>> {
		let commitment = config
			.account_config
			.commitment
			.unwrap_or_else(|| self.commitment_config());
		let account_config = RpcAccountInfoConfig {
			commitment: Some(commitment),
			..config.account_config
		};
		let config = RpcProgramAccountsConfig {
			account_config,
			..config
		};

		let request = GetProgramAccountsRequest::new_with_config(*pubkey, config).into();
		let response = GetProgramAccountsResponse::from(self.send(request).await?);

		// Parse keyed accounts
		let accounts = response
			.keyed_accounts()
			.ok_or_else(|| SolanaRpcClientError::new("Program account doesn't exist."))?;

		let mut pubkey_accounts: Vec<(Pubkey, Account)> = Vec::with_capacity(accounts.len());
		for RpcKeyedAccount { pubkey, account } in accounts {
			let pubkey = pubkey.parse().map_err(|_| {
				SolanaRpcClientError::new(format!("{pubkey} is not a valid pubkey."))
			})?;
			pubkey_accounts.push((
				pubkey,
				account.decode().ok_or_else(|| {
					SolanaRpcClientError::new(format!("Unable to decode {pubkey}"))
				})?,
			));
		}
		Ok(pubkey_accounts)
	}

	pub async fn get_program_accounts(
		&self,
		pubkey: &Pubkey,
	) -> ClientResult<Vec<(Pubkey, Account)>> {
		self.get_program_accounts_with_config(
			pubkey,
			RpcProgramAccountsConfig {
				account_config: RpcAccountInfoConfig {
					encoding: Some(UiAccountEncoding::Base64),
					..RpcAccountInfoConfig::default()
				},
				..RpcProgramAccountsConfig::default()
			},
		)
		.await
	}

	pub async fn get_slot_with_commitment(
		&self,
		commitment_config: CommitmentConfig,
	) -> ClientResult<Slot> {
		let request = GetSlotRequest::new_with_config(commitment_config).into();
		let response = GetSlotResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn get_slot(&self) -> ClientResult<Slot> {
		self.get_slot_with_commitment(self.commitment_config())
			.await
	}

	pub async fn get_block_with_config(
		&self,
		slot: Slot,
		config: RpcBlockConfig,
	) -> ClientResult<UiConfirmedBlock> {
		let request = GetBlockRequest::new_with_config(slot, config).into();
		let response = GetBlockResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn get_version(&self) -> ClientResult<RpcVersionInfo> {
		let request = GetVersionRequest::new().into();
		let response = GetVersionResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn get_first_available_block(&self) -> ClientResult<Slot> {
		let request = GetFirstAvailableBlockRequest::new().into();
		let response = GetFirstAvailableBlockResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn get_block_time(&self, slot: Slot) -> ClientResult<UnixTimestamp> {
		let request = GetBlockTimeRequest::new(slot).into();
		let response = GetBlockTimeResponse::from(self.send(request).await?);

		let maybe_ts: Option<UnixTimestamp> = response.into();
		match maybe_ts {
			Some(ts) => Ok(ts),
			None => {
				Err(SolanaRpcClientError::new(format!(
					"Block Not Found: slot={slot}"
				)))
			}
		}
	}

	pub async fn get_block_height_with_commitment(
		&self,
		commitment_config: CommitmentConfig,
	) -> ClientResult<u64> {
		let request = GetBlockHeightRequest::new_with_config(commitment_config).into();
		let response = GetBlockHeightResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn get_block_height(&self) -> ClientResult<u64> {
		self.get_block_height_with_commitment(self.commitment_config())
			.await
	}

	pub async fn get_genesis_hash(&self) -> ClientResult<Hash> {
		let request = GetGenesisHashRequest::new().into();
		let response = GetGenesisHashResponse::from(self.send(request).await?);

		let hash_string: String = response.into();
		let hash = hash_string
			.parse()
			.map_err(|_| SolanaRpcClientError::new("Hash is not parseable."))?;

		Ok(hash)
	}

	pub async fn get_epoch_info_with_commitment(
		&self,
		commitment_config: CommitmentConfig,
	) -> ClientResult<EpochInfo> {
		let request = GetEpochInfoRequest::new_with_config(commitment_config).into();
		let response = GetEpochInfoResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn get_epoch_info(&self) -> ClientResult<EpochInfo> {
		self.get_epoch_info_with_commitment(self.commitment_config())
			.await
	}

	pub async fn get_recent_performance_samples(
		&self,
		limit: Option<usize>,
	) -> ClientResult<Vec<RpcPerfSample>> {
		let request = GetRecentPerformanceSamplesRequest::new_with_config(
			GetRecentPerformanceSamplesRequestConfig { limit },
		)
		.into();
		let response = GetRecentPerformanceSamplesResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn get_blocks_with_limit_and_commitment(
		&self,
		start_slot: Slot,
		limit: usize,
		commitment_config: CommitmentConfig,
	) -> ClientResult<Vec<Slot>> {
		let request =
			GetBlocksWithLimitRequest::new_with_config(start_slot, limit, commitment_config).into();
		let response = GetBlocksWithLimitResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn get_blocks_with_limit(
		&self,
		start_slot: Slot,
		limit: usize,
	) -> ClientResult<Vec<Slot>> {
		self.get_blocks_with_limit_and_commitment(start_slot, limit, self.commitment_config())
			.await
	}

	pub async fn get_largest_accounts_with_config(
		&self,
		config: RpcLargestAccountsConfig,
	) -> ClientResult<Vec<RpcAccountBalance>> {
		let config = RpcLargestAccountsConfig {
			commitment: config.commitment,
			..config
		};

		let request = GetLargestAccountsRequest::new_with_config(config).into();
		let response = GetLargestAccountsResponse::from(self.send(request).await?);

		Ok(response.value)
	}

	pub async fn get_supply_with_config(&self, config: RpcSupplyConfig) -> ClientResult<RpcSupply> {
		let request = GetSupplyRequest::new_with_config(config).into();
		let response = GetSupplyResponse::from(self.send(request).await?);

		Ok(response.value)
	}

	pub async fn get_supply_with_commitment(
		&self,
		commitment_config: CommitmentConfig,
	) -> ClientResult<RpcSupply> {
		self.get_supply_with_config(RpcSupplyConfig {
			commitment: Some(commitment_config),
			exclude_non_circulating_accounts_list: false,
		})
		.await
	}

	pub async fn get_supply(&self) -> ClientResult<RpcSupply> {
		self.get_supply_with_commitment(self.commitment_config())
			.await
	}

	pub async fn get_transaction_count_with_config(
		&self,
		config: RpcContextConfig,
	) -> ClientResult<u64> {
		let request = GetTransactionCountRequest::new_with_config(config).into();
		let response = GetTransactionCountResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn get_transaction_count_with_commitment(
		&self,
		commitment_config: CommitmentConfig,
	) -> ClientResult<u64> {
		self.get_transaction_count_with_config(RpcContextConfig {
			commitment: Some(commitment_config),
			min_context_slot: None,
		})
		.await
	}

	pub async fn get_transaction_count(&self) -> ClientResult<u64> {
		self.get_transaction_count_with_commitment(self.commitment_config())
			.await
	}

	pub async fn get_multiple_accounts_with_config(
		&self,
		pubkeys: &[Pubkey],
		config: RpcAccountInfoConfig,
	) -> ClientResult<Vec<Option<Account>>> {
		let config = RpcAccountInfoConfig {
			commitment: config.commitment,
			..config
		};

		let request = GetMultipleAccountsRequest::new_with_config(pubkeys.to_vec(), config).into();
		let response = GetMultipleAccountsResponse::from(self.send(request).await?);

		Ok(response
			.value
			.iter()
			.filter(|maybe_acc| maybe_acc.is_some())
			.map(|acc| acc.clone().unwrap().decode())
			.collect())
	}

	pub async fn get_multiple_accounts_with_commitment(
		&self,
		pubkeys: &[Pubkey],
		commitment_config: CommitmentConfig,
	) -> ClientResult<Vec<Option<Account>>> {
		self.get_multiple_accounts_with_config(
			pubkeys,
			RpcAccountInfoConfig {
				commitment: Some(commitment_config),
				..RpcAccountInfoConfig::default()
			},
		)
		.await
	}

	pub async fn get_multiple_accounts(
		&self,
		pubkeys: &[Pubkey],
	) -> ClientResult<Vec<Option<Account>>> {
		self.get_multiple_accounts_with_commitment(pubkeys, self.commitment_config())
			.await
	}

	pub async fn get_cluster_nodes(&self) -> ClientResult<Vec<RpcContactInfoWasm>> {
		let request = GetClusterNodesRequest::new().into();
		let response = GetClusterNodesResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn get_vote_accounts_with_config(
		&self,
		config: RpcGetVoteAccountsConfig,
	) -> ClientResult<RpcVoteAccountStatus> {
		let request = GetVoteAccountsRequest::new_with_config(config).into();
		let response = GetVoteAccountsResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn get_vote_accounts_with_commitment(
		&self,
		commitment_config: CommitmentConfig,
	) -> ClientResult<RpcVoteAccountStatus> {
		self.get_vote_accounts_with_config(RpcGetVoteAccountsConfig {
			commitment: Some(commitment_config),
			..Default::default()
		})
		.await
	}

	pub async fn get_vote_accounts(&self) -> ClientResult<RpcVoteAccountStatus> {
		self.get_vote_accounts_with_commitment(self.commitment_config())
			.await
	}

	pub async fn get_epoch_schedule(&self) -> ClientResult<EpochSchedule> {
		let request = GetEpochScheduleRequest::new().into();
		let response = GetEpochScheduleResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn get_signatures_for_address_with_config(
		&self,
		address: &Pubkey,
		config: GetConfirmedSignaturesForAddress2Config,
	) -> ClientResult<Vec<RpcConfirmedTransactionStatusWithSignature>> {
		let config = RpcSignaturesForAddressConfig {
			before: config.before.map(|signature| signature.to_string()),
			until: config.until.map(|signature| signature.to_string()),
			limit: config.limit,
			commitment: config.commitment,
			min_context_slot: None,
		};

		let request = GetSignaturesForAddressRequest::new_with_config(*address, config).into();
		let response = GetSignaturesForAddressResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn minimum_ledger_slot(&self) -> ClientResult<Slot> {
		let request = MinimumLedgerSlotRequest::new().into();
		let response = MinimumLedgerSlotResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn get_blocks_with_commitment(
		&self,
		start_slot: Slot,
		end_slot: Option<Slot>,
		commitment_config: CommitmentConfig,
	) -> ClientResult<Vec<Slot>> {
		let request =
			GetBlocksRequest::new_with_config(start_slot, end_slot, commitment_config).into();
		let response = GetBlocksResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn get_blocks(
		&self,
		start_slot: Slot,
		end_slot: Option<Slot>,
	) -> ClientResult<Vec<Slot>> {
		self.get_blocks_with_commitment(start_slot, end_slot, self.commitment_config())
			.await
	}

	pub async fn get_leader_schedule_with_config(
		&self,
		slot: Option<Slot>,
		config: RpcLeaderScheduleConfig,
	) -> ClientResult<Option<RpcLeaderSchedule>> {
		let request = match slot {
			Some(s) => GetLeaderScheduleRequest::new_with_slot_and_config(s, config).into(),
			None => GetLeaderScheduleRequest::new_with_config(config).into(),
		};
		let response = GetLeaderScheduleResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn get_leader_schedule_with_commitment(
		&self,
		slot: Option<Slot>,
		commitment_config: CommitmentConfig,
	) -> ClientResult<Option<RpcLeaderSchedule>> {
		self.get_leader_schedule_with_config(
			slot,
			RpcLeaderScheduleConfig {
				commitment: Some(commitment_config),
				..Default::default()
			},
		)
		.await
	}

	pub async fn get_block_production_with_config(
		&self,
		config: RpcBlockProductionConfig,
	) -> ClientResult<RpcBlockProduction> {
		let request = GetBlockProductionRequest::new_with_config(config).into();
		let response = GetBlockProductionResponse::from(self.send(request).await?);

		Ok(response.value)
	}

	pub async fn get_block_production_with_commitment(
		&self,
		commitment_config: CommitmentConfig,
	) -> ClientResult<RpcBlockProduction> {
		self.get_block_production_with_config(RpcBlockProductionConfig {
			commitment: Some(commitment_config),
			..Default::default()
		})
		.await
	}

	pub async fn get_block_production(&self) -> ClientResult<RpcBlockProduction> {
		self.get_block_production_with_commitment(self.commitment_config())
			.await
	}

	pub async fn get_inflation_governor_with_commitment(
		&self,
		commitment_config: CommitmentConfig,
	) -> ClientResult<RpcInflationGovernor> {
		let request = GetInflationGovernorRequest::new_with_config(commitment_config).into();
		let response = GetInflationGovernorResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn get_inflation_governor(&self) -> ClientResult<RpcInflationGovernor> {
		self.get_inflation_governor_with_commitment(self.commitment_config())
			.await
	}

	pub async fn get_inflation_rate(&self) -> ClientResult<RpcInflationRate> {
		let request = GetInflationRateRequest::new().into();
		let response = GetInflationRateResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn get_inflation_reward_with_config(
		&self,
		addresses: &[Pubkey],
		epoch: Option<Epoch>,
	) -> ClientResult<Vec<Option<RpcInflationReward>>> {
		let request = GetInflationRewardRequest::new_with_config(
			addresses.to_vec(),
			RpcEpochConfig {
				commitment: Some(self.commitment_config()),
				epoch,
				..Default::default()
			},
		)
		.into();
		let response = GetInflationRewardResponse::from(self.send(request).await?);

		Ok(response.into())
	}

	pub async fn get_inflation_reward(
		&self,
		addresses: &[Pubkey],
	) -> ClientResult<Vec<Option<RpcInflationReward>>> {
		self.get_inflation_reward_with_config(addresses, None).await
	}

	pub async fn get_token_account_with_commitment(
		&self,
		pubkey: &Pubkey,
		commitment_config: CommitmentConfig,
	) -> ClientResult<Option<UiTokenAccount>> {
		let config = RpcAccountInfoConfig {
			encoding: Some(UiAccountEncoding::JsonParsed),
			commitment: Some(commitment_config),
			data_slice: None,
			min_context_slot: None,
		};

		let request = GetAccountInfoRequest::new_with_config(*pubkey, config).into();
		let response = GetAccountInfoResponse::from(self.send(request).await?);

		if let Some(acc) = response.value {
			if let UiAccountData::Json(account_data) = acc.data {
				let token_account_type: TokenAccountType =
					match serde_json::from_value(account_data.parsed) {
						Ok(t) => t,
						Err(e) => return Err(SolanaRpcClientError::new(e.to_string())),
					};

				if let TokenAccountType::Account(token_account) = token_account_type {
					return Ok(Some(token_account));
				}
			}
		}

		Err(SolanaRpcClientError::new(format!(
			"AccountNotFound: pubkey={pubkey}"
		)))
	}

	pub async fn get_token_account(&self, pubkey: &Pubkey) -> ClientResult<Option<UiTokenAccount>> {
		self.get_token_account_with_commitment(pubkey, self.commitment_config())
			.await
	}

	pub async fn get_token_accounts_by_owner_with_commitment(
		&self,
		owner: &Pubkey,
		token_account_filter: TokenAccountsFilter,
		commitment_config: CommitmentConfig,
	) -> ClientResult<Vec<RpcKeyedAccount>> {
		let token_account_filter = match token_account_filter {
			TokenAccountsFilter::Mint(mint) => RpcTokenAccountsFilter::Mint(mint.to_string()),
			TokenAccountsFilter::ProgramId(program_id) => {
				RpcTokenAccountsFilter::ProgramId(program_id.to_string())
			}
		};

		let config = RpcAccountInfoConfig {
			encoding: Some(UiAccountEncoding::JsonParsed),
			commitment: Some(commitment_config),
			data_slice: None,
			min_context_slot: None,
		};

		let request =
			GetTokenAccountsByOwnerRequest::new_with_config(*owner, token_account_filter, config)
				.into();
		let response = GetTokenAccountsByOwnerResponse::from(self.send(request).await?);

		Ok(response.value)
	}

	pub async fn get_token_accounts_by_owner(
		&self,
		owner: &Pubkey,
		token_account_filter: TokenAccountsFilter,
	) -> ClientResult<Vec<RpcKeyedAccount>> {
		self.get_token_accounts_by_owner_with_commitment(
			owner,
			token_account_filter,
			self.commitment_config(),
		)
		.await
	}

	pub async fn get_token_account_balance_with_commitment(
		&self,
		pubkey: &Pubkey,
		commitment_config: CommitmentConfig,
	) -> ClientResult<UiTokenAmount> {
		let request =
			GetTokenAccountBalanceRequest::new_with_config(*pubkey, commitment_config).into();
		let response = GetTokenAccountBalanceResponse::from(self.send(request).await?);

		Ok(response.value)
	}

	pub async fn get_token_account_balance(&self, pubkey: &Pubkey) -> ClientResult<UiTokenAmount> {
		self.get_token_account_balance_with_commitment(pubkey, self.commitment_config())
			.await
	}

	pub async fn get_token_supply_with_commitment(
		&self,
		mint: &Pubkey,
		commitment_config: CommitmentConfig,
	) -> ClientResult<UiTokenAmount> {
		let request = GetTokenSupplyRequest::new_with_config(*mint, commitment_config).into();
		let response = GetTokenSupplyResponse::from(self.send(request).await?);

		Ok(response.value)
	}

	pub async fn get_token_supply(&self, mint: &Pubkey) -> ClientResult<UiTokenAmount> {
		self.get_token_supply_with_commitment(mint, self.commitment_config())
			.await
	}

	pub async fn simulate_transaction_with_config(
		&self,
		transaction: &VersionedTransaction,
		config: RpcSimulateTransactionConfig,
	) -> ClientResult<SimulateTransactionResponse> {
		let request =
			SimulateTransactionRequest::new_with_config(transaction.to_owned(), config).into();
		let response = SimulateTransactionResponse::from(self.send(request).await?);

		Ok(response)
	}

	pub async fn simulate_transaction(
		&self,
		transaction: &VersionedTransaction,
	) -> ClientResult<SimulateTransactionResponse> {
		self.simulate_transaction_with_config(
			transaction,
			RpcSimulateTransactionConfig {
				encoding: Some(UiTransactionEncoding::Base64),
				replace_recent_blockhash: true,
				..Default::default()
			},
		)
		.await
	}

	pub async fn get_health(&self) -> ClientResult<GetHealthResponse> {
		let request = GetHealthRequest::new().into();
		let response = GetHealthResponse::from(self.send(request).await?);

		Ok(response)
	}

	/// Returns the identity pubkey for the current node.
	///
	/// # RPC Reference
	///
	/// This method corresponds directly to the [`getIdentity`] RPC method.
	///
	/// [`getIdentity`]: https://solana.com/docs/rpc/http/getidentity
	pub async fn get_identity(&self) -> ClientResult<GetIdentityResponse> {
		let request = GetIdentityRequest::new().into();
		let response = GetIdentityResponse::from(self.send(request).await?);

		Ok(response)
	}

	/// Returns commitment for particular block
	pub async fn get_block_commitment(
		&self,
		slot: u64,
	) -> ClientResult<GetBlockCommitmentResponse> {
		let request = GetBlockCommitmentRequest::new(slot).into();
		let response = GetBlockCommitmentResponse::from(self.send(request).await?);

		Ok(response)
	}

	/// Returns the highest slot information that the node has snapshots for.
	/// This will find the highest full snapshot slot, and the highest
	/// incremental snapshot slot based on the full snapshot slot, if there is
	/// one.
	///
	/// *VERSION RESTRICTION*
	/// This method is only available in solana-core v1.9 or newer. Please use
	/// getSnapshotSlot for solana-core v1.8 and below.
	pub async fn get_highest_snapshot_slot(&self) -> ClientResult<GetHighestSnapshotSlotResponse> {
		let request = GetHighestSnapshotSlotRequest::new().into();
		let response = GetHighestSnapshotSlotResponse::from(self.send(request).await?);

		Ok(response)
	}

	/// Get the max slot seen from retransmit stage.
	pub async fn get_max_retransmit_slot(&self) -> ClientResult<GetMaxRetransmitSlotResponse> {
		let request = GetMaxRetransmitSlotRequest::new().into();
		let response = GetMaxRetransmitSlotResponse::from(self.send(request).await?);

		Ok(response)
	}

	/// Returns the current slot leader
	pub async fn get_slot_leader(&self) -> ClientResult<GetSlotLeaderResponse> {
		let request = GetSlotLeaderRequest::new().into();
		let response = GetSlotLeaderResponse::from(self.send(request).await?);

		Ok(response)
	}

	/// Returns the slot leaders for a given slot range
	pub async fn get_slot_leaders(
		&self,
		start_slot: u64,
		limit: u64,
	) -> ClientResult<GetSlotLeadersResponse> {
		let request = GetSlotLeadersRequest::new(start_slot, limit).into();
		let response = GetSlotLeadersResponse::from(self.send(request).await?);

		Ok(response)
	}

	pub async fn get_stake_activation(
		&self,
		pubkey: Pubkey,
	) -> ClientResult<GetStakeActivationResponse> {
		let request = GetStakeActivationRequest::new(pubkey).into();
		let response = GetStakeActivationResponse::from(self.send(request).await?);

		Ok(response)
	}

	pub async fn get_stake_activation_with_config(
		&self,
		pubkey: Pubkey,
		config: RpcEpochConfig,
	) -> ClientResult<GetStakeActivationResponse> {
		let request = GetStakeActivationRequest::new_with_config(pubkey, config).into();
		let response = GetStakeActivationResponse::from(self.send(request).await?);

		Ok(response)
	}

	pub async fn get_token_accounts_by_delegate_with_config(
		&self,
		pubkey: Pubkey,
		account_type: AccountType,
		account_key: Pubkey,
		config: RpcAccountInfoConfig,
	) -> ClientResult<GetTokenAccountsByDelegateResponse> {
		let request = GetTokenAccountsByDelegateRequest {
			pubkey,
			account_type,
			account_key,
			config: Some(config),
		};
		let response = GetTokenAccountsByDelegateResponse::from(self.send(request.into()).await?);

		Ok(response)
	}

	pub async fn get_token_accounts_by_delegate(
		&self,
		pubkey: Pubkey,
		account_type: AccountType,
		account_key: Pubkey,
	) -> ClientResult<GetTokenAccountsByDelegateResponse> {
		let request = GetTokenAccountsByDelegateRequest {
			pubkey,
			account_type,
			account_key,
			config: None,
		};
		let response = GetTokenAccountsByDelegateResponse::from(self.send(request.into()).await?);

		Ok(response)
	}

	pub async fn get_token_largets_accounts(
		&self,
		pubkey: Pubkey,
	) -> ClientResult<GetTokenLargestAccountsResponse> {
		let request = GetTokenLargestAccountsRequest::new(pubkey).into();
		let response = GetTokenLargestAccountsResponse::from(self.send(request).await?);

		Ok(response)
	}

	pub async fn get_token_largets_accounts_with_config(
		&self,
		pubkey: Pubkey,
		config: CommitmentConfig,
	) -> ClientResult<GetTokenLargestAccountsResponse> {
		let request = GetTokenLargestAccountsRequest::new_with_config(pubkey, config).into();
		let response = GetTokenLargestAccountsResponse::from(self.send(request).await?);

		Ok(response)
	}

	pub async fn get_address_lookup_table(
		&self,
		pubkey: &Pubkey,
	) -> ClientResult<LookupTableAccountType> {
		let account = self.get_account(pubkey).await?;
		let table_type = parse_address_lookup_table(&account.data)
			.map_err(|error| SolanaRpcClientError::new(error.to_string()))?;

		Ok(table_type)
	}

	/// Wait for the new block which is `n` blocks in the future.
	pub async fn wait_for_new_block(&self, n: u8) -> ClientResult<()> {
		let (_, last_valid_block_height) = self
			.get_latest_blockhash_with_commitment(self.commitment_config())
			.await?;

		for _ in 0..MAX_RETRIES {
			let (_, latest) = self
				.get_latest_blockhash_with_commitment(self.commitment_config())
				.await?;

			if latest >= last_valid_block_height + u64::from(n) {
				break;
			}

			Delay::new(Duration::from_millis(SLEEP_MS)).await;
		}

		Ok(())
	}
}
