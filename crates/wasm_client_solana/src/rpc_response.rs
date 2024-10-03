use std::collections::HashMap;
use std::fmt;
use std::net::SocketAddr;
use std::str::FromStr;

use derive_more::derive::Deref;
use derive_more::derive::DerefMut;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde_with::DisplayFromStr;
use serde_with::serde_as;
use serde_with::skip_serializing_none;
use solana_sdk::clock::Epoch;
use solana_sdk::clock::Slot;
use solana_sdk::clock::UnixTimestamp;
use solana_sdk::fee_calculator::FeeCalculator;
use solana_sdk::fee_calculator::FeeRateGovernor;
use solana_sdk::hash::Hash;
use solana_sdk::inflation::Inflation;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::Result;
use solana_sdk::transaction::TransactionError;
use thiserror::Error;

use crate::Context;
use crate::impl_websocket_notification;
use crate::solana_account_decoder::UiAccount;
use crate::solana_account_decoder::parse_token::UiTokenAmount;
use crate::solana_transaction_status::ConfirmedTransactionStatusWithSignature;
use crate::solana_transaction_status::TransactionConfirmationStatus;
use crate::solana_transaction_status::UiConfirmedBlock;
use crate::solana_transaction_status::UiInnerInstructions;
use crate::solana_transaction_status::UiTransactionReturnData;

/// Wrapper for rpc return types of methods that provide responses both with and
/// without context. Main purpose of this is to fix methods that lack context
/// information in their return type, without breaking backwards compatibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OptionalContext<T> {
	Context(Response<T>),
	NoContext(T),
}

impl<T> OptionalContext<T> {
	pub fn parse_value(self) -> T {
		match self {
			Self::Context(response) => response.value,
			Self::NoContext(value) => value,
		}
	}
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcResponseContext {
	pub slot: Slot,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub api_version: Option<RpcApiVersion>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RpcApiVersion(semver::Version);

impl std::ops::Deref for RpcApiVersion {
	type Target = semver::Version;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl Default for RpcApiVersion {
	fn default() -> Self {
		Self(solana_version::Version::default().as_semver_version())
	}
}

impl Serialize for RpcApiVersion {
	fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

impl<'de> Deserialize<'de> for RpcApiVersion {
	fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let s: String = Deserialize::deserialize(deserializer)?;
		Ok(RpcApiVersion(
			semver::Version::from_str(&s).map_err(serde::de::Error::custom)?,
		))
	}
}

impl RpcResponseContext {
	pub fn new(slot: Slot) -> Self {
		Self {
			slot,
			api_version: Some(RpcApiVersion::default()),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Response<T> {
	pub context: RpcResponseContext,
	pub value: T,
}

#[skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RpcBlockCommitment<T> {
	pub commitment: Option<T>,
	pub total_stake: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcBlockhashFeeCalculator {
	#[serde_as(as = "DisplayFromStr")]
	pub blockhash: Hash,
	pub fee_calculator: FeeCalculator,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
struct A(#[serde_as(as = "DisplayFromStr")] Pubkey);

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcBlockhash {
	#[serde_as(as = "DisplayFromStr")]
	pub blockhash: Hash,
	pub last_valid_block_height: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcFees {
	#[serde_as(as = "DisplayFromStr")]
	pub blockhash: Hash,
	pub fee_calculator: FeeCalculator,
	pub last_valid_slot: Slot,
	pub last_valid_block_height: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeprecatedRpcFees {
	#[serde_as(as = "DisplayFromStr")]
	pub blockhash: Hash,
	pub fee_calculator: FeeCalculator,
	pub last_valid_slot: Slot,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Fees {
	#[serde_as(as = "DisplayFromStr")]
	pub blockhash: Hash,
	pub fee_calculator: FeeCalculator,
	pub last_valid_block_height: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcFeeCalculator {
	pub fee_calculator: FeeCalculator,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcFeeRateGovernor {
	pub fee_rate_governor: FeeRateGovernor,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RpcInflationGovernor {
	pub initial: f64,
	pub terminal: f64,
	pub taper: f64,
	pub foundation: f64,
	pub foundation_term: f64,
}

impl PartialEq for RpcInflationGovernor {
	fn eq(&self, other: &Self) -> bool {
		approx_eq(self.initial, other.initial)
			&& approx_eq(self.terminal, other.terminal)
			&& approx_eq(self.taper, other.taper)
			&& approx_eq(self.foundation, other.foundation)
			&& approx_eq(self.foundation_term, other.foundation_term)
	}
}

impl Eq for RpcInflationGovernor {}

pub fn approx_eq(a: f64, b: f64) -> bool {
	const EPSILON: f64 = 1e-6;
	(a - b).abs() < EPSILON
}

impl From<Inflation> for RpcInflationGovernor {
	fn from(inflation: Inflation) -> Self {
		Self {
			initial: inflation.initial,
			terminal: inflation.terminal,
			taper: inflation.taper,
			foundation: inflation.foundation,
			foundation_term: inflation.foundation_term,
		}
	}
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RpcInflationRate {
	pub total: f64,
	pub validator: f64,
	pub foundation: f64,
	pub epoch: Epoch,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcKeyedAccount {
	#[serde_as(as = "DisplayFromStr")]
	pub pubkey: Pubkey,
	pub account: UiAccount,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlotInfo {
	pub slot: Slot,
	pub parent: Slot,
	pub root: Slot,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SlotTransactionStats {
	pub num_transaction_entries: u64,
	pub num_successful_transactions: u64,
	pub num_failed_transactions: u64,
	pub max_transactions_per_entry: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum SlotUpdate {
	FirstShredReceived {
		slot: Slot,
		timestamp: u64,
	},
	Completed {
		slot: Slot,
		timestamp: u64,
	},
	CreatedBank {
		slot: Slot,
		parent: Slot,
		timestamp: u64,
	},
	Frozen {
		slot: Slot,
		timestamp: u64,
		stats: SlotTransactionStats,
	},
	Dead {
		slot: Slot,
		timestamp: u64,
		err: String,
	},
	OptimisticConfirmation {
		slot: Slot,
		timestamp: u64,
	},
	Root {
		slot: Slot,
		timestamp: u64,
	},
}

impl SlotUpdate {
	pub fn slot(&self) -> Slot {
		match self {
			Self::FirstShredReceived { slot, .. }
			| Self::Completed { slot, .. }
			| Self::CreatedBank { slot, .. }
			| Self::Frozen { slot, .. }
			| Self::Dead { slot, .. }
			| Self::OptimisticConfirmation { slot, .. }
			| Self::Root { slot, .. } => *slot,
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase", untagged)]
pub enum RpcSignatureResult {
	ProcessedSignature(ProcessedSignatureResult),
	ReceivedSignature(ReceivedSignatureResult),
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcLogsResponse {
	#[serde_as(as = "DisplayFromStr")]
	pub signature: Signature, // Signature as base58 string
	pub err: Option<TransactionError>,
	pub logs: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct LogsNotificationResponse {
	pub context: Context,
	pub value: RpcLogsResponse,
}

impl_websocket_notification!(LogsNotificationResponse, "logs");

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProcessedSignatureResult {
	pub err: Option<TransactionError>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ReceivedSignatureResult {
	ReceivedSignature,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcContactInfo {
	/// Pubkey of the node as a base-58 string
	#[serde_as(as = "DisplayFromStr")]
	pub pubkey: Pubkey,
	/// Gossip port
	pub gossip: Option<SocketAddr>,
	/// Tvu UDP port
	pub tvu: Option<SocketAddr>,
	/// Tpu UDP port
	pub tpu: Option<SocketAddr>,
	/// Tpu QUIC port
	pub tpu_quic: Option<SocketAddr>,
	/// Tpu UDP forwards port
	pub tpu_forwards: Option<SocketAddr>,
	/// Tpu QUIC forwards port
	pub tpu_forwards_quic: Option<SocketAddr>,
	/// Tpu UDP vote port
	pub tpu_vote: Option<SocketAddr>,
	/// Server repair UDP port
	pub serve_repair: Option<SocketAddr>,
	/// JSON RPC port
	pub rpc: Option<SocketAddr>,
	/// `WebSocket` `PubSub` port
	pub pubsub: Option<SocketAddr>,
	/// Software version
	pub version: Option<String>,
	/// First 4 bytes of the `FeatureSet` identifier
	pub feature_set: Option<u32>,
	/// Shred version
	pub shred_version: Option<u16>,
}

/// Map of leader base58 identity pubkeys to the slot indices relative to the
/// first epoch slot
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Deref, DerefMut)]
pub struct RpcLeaderSchedule(#[serde(with = "pubkey_string_map")] pub HashMap<Pubkey, Vec<usize>>);

mod pubkey_string_map {
	use serde::ser::SerializeMap;

	use super::*;

	pub fn serialize<S>(
		map: &HashMap<Pubkey, Vec<usize>>,
		serializer: S,
	) -> std::result::Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut ser_map = serializer.serialize_map(Some(map.len()))?;
		for (k, v) in map {
			ser_map.serialize_entry(&k.to_string(), v)?;
		}
		ser_map.end()
	}

	pub fn deserialize<'de, D>(
		deserializer: D,
	) -> std::result::Result<HashMap<Pubkey, Vec<usize>>, D::Error>
	where
		D: Deserializer<'de>,
	{
		let string_map: HashMap<String, Vec<usize>> = HashMap::deserialize(deserializer)?;
		string_map
			.into_iter()
			.map(|(k, v)| Ok((Pubkey::from_str(&k).map_err(serde::de::Error::custom)?, v)))
			.collect()
	}
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcBlockProductionRange {
	pub first_slot: Slot,
	pub last_slot: Slot,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcBlockProduction {
	/// Map of leader base58 identity pubkeys to a tuple of `(number of leader
	/// slots, number of blocks produced)`
	pub by_identity: HashMap<String, (usize, usize)>,
	pub range: RpcBlockProductionRange,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct RpcVersionInfo {
	/// The current version of solana-core
	pub solana_core: String,
	/// first 4 bytes of the `FeatureSet` identifier
	pub feature_set: Option<u32>,
}

impl fmt::Debug for RpcVersionInfo {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.solana_core)
	}
}

impl fmt::Display for RpcVersionInfo {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		if let Some(version) = self.solana_core.split_whitespace().next() {
			// Display just the semver if possible
			write!(f, "{version}")
		} else {
			write!(f, "{}", self.solana_core)
		}
	}
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct RpcIdentity {
	/// The current node identity pubkey
	#[serde_as(as = "DisplayFromStr")]
	pub identity: Pubkey,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcVote {
	/// Vote account address, as base-58 encoded string
	#[serde_as(as = "DisplayFromStr")]
	pub vote_pubkey: Pubkey,
	pub slots: Vec<Slot>,
	#[serde_as(as = "DisplayFromStr")]
	pub hash: Hash,
	pub timestamp: Option<UnixTimestamp>,
	#[serde_as(as = "DisplayFromStr")]
	pub signature: Signature,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcVoteAccountStatus {
	pub current: Vec<RpcVoteAccountInfo>,
	pub delinquent: Vec<RpcVoteAccountInfo>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcVoteAccountInfo {
	/// Vote account address, as base-58 encoded string
	#[serde_as(as = "DisplayFromStr")]
	pub vote_pubkey: Pubkey,
	/// The validator identity, as base-58 encoded string
	#[serde_as(as = "DisplayFromStr")]
	pub node_pubkey: Pubkey,
	/// The current stake, in lamports, delegated to this vote account
	pub activated_stake: u64,
	/// An 8-bit integer used as a fraction (`commission/MAX_U8`) for rewards
	/// payout
	pub commission: u8,
	/// Whether this account is staked for the current epoch
	pub epoch_vote_account: bool,
	/// Latest history of earned credits for up to
	/// `MAX_RPC_VOTE_ACCOUNT_INFO_EPOCH_CREDITS_HISTORY` epochs   each tuple
	/// is (Epoch, credits, `prev_credits`)
	pub epoch_credits: Vec<(Epoch, u64, u64)>,
	/// Most recent slot voted on by this vote account (0 if no votes exist)
	#[serde(default)]
	pub last_vote: u64,

	/// Current root slot for this vote account (0 if no root slot exists)
	#[serde(default)]
	pub root_slot: Slot,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcSignatureConfirmation {
	pub confirmations: usize,
	pub status: Result<()>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcSimulateTransactionResult {
	pub err: Option<TransactionError>,
	pub logs: Option<Vec<String>>,
	pub accounts: Option<Vec<Option<UiAccount>>>,
	pub units_consumed: Option<u64>,
	pub return_data: Option<UiTransactionReturnData>,
	pub inner_instructions: Option<Vec<UiInnerInstructions>>,
	pub replacement_blockhash: Option<RpcBlockhash>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcStorageTurn {
	#[serde_as(as = "DisplayFromStr")]
	pub blockhash: Hash,
	pub slot: Slot,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcAccountBalance {
	#[serde_as(as = "DisplayFromStr")]
	pub address: Pubkey,
	pub lamports: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcSupply {
	pub total: u64,
	pub circulating: u64,
	pub non_circulating: u64,
	pub non_circulating_accounts: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum StakeActivationState {
	Activating,
	Active,
	Deactivating,
	Inactive,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcStakeActivation {
	pub state: StakeActivationState,
	pub active: u64,
	pub inactive: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RpcTokenAccountBalance {
	#[serde_as(as = "DisplayFromStr")]
	pub address: Pubkey,
	#[serde(flatten)]
	pub amount: UiTokenAmount,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcConfirmedTransactionStatusWithSignature {
	#[serde_as(as = "DisplayFromStr")]
	pub signature: Signature,
	pub slot: Slot,
	pub err: Option<TransactionError>,
	pub memo: Option<String>,
	pub block_time: Option<UnixTimestamp>,
	pub confirmation_status: Option<TransactionConfirmationStatus>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcPerfSample {
	pub slot: Slot,
	pub num_transactions: u64,
	pub num_non_vote_transaction: u64,
	pub num_slots: u64,
	pub sample_period_secs: u16,
}

impl RpcPerfSample {
	pub fn num_vote_transactions(&self) -> u64 {
		self.num_transactions - self.num_non_vote_transaction
	}
}

#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcInflationReward {
	pub epoch: Epoch,
	pub effective_slot: Slot,
	pub amount: u64,            // lamports
	pub post_balance: u64,      // lamports
	pub commission: Option<u8>, // Vote account commission when the reward was credited
}

#[derive(Clone, Deserialize, Serialize, Debug, Error, Eq, PartialEq)]
pub enum RpcBlockUpdateError {
	#[error("block store error")]
	BlockStoreError,

	#[error("unsupported transaction version ({0})")]
	UnsupportedTransactionVersion(u8),
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RpcBlockUpdate {
	pub slot: Slot,
	pub block: Option<UiConfirmedBlock>,
	pub err: Option<RpcBlockUpdateError>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BlockNotificationResponse {
	pub context: Context,
	pub value: RpcBlockUpdate,
}

impl_websocket_notification!(BlockNotificationResponse, "block");

impl From<ConfirmedTransactionStatusWithSignature> for RpcConfirmedTransactionStatusWithSignature {
	fn from(value: ConfirmedTransactionStatusWithSignature) -> Self {
		let ConfirmedTransactionStatusWithSignature {
			signature,
			slot,
			err,
			memo,
			block_time,
		} = value;
		Self {
			signature,
			slot,
			err,
			memo,
			block_time,
			confirmation_status: None,
		}
	}
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct RpcSnapshotSlotInfo {
	pub full: Slot,
	pub incremental: Option<Slot>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RpcPrioritizationFee {
	pub slot: Slot,
	pub prioritization_fee: u64,
}
