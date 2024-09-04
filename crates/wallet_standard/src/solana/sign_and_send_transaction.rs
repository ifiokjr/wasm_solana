use std::future::Future;

use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::transaction::VersionedTransaction;
use typed_builder::TypedBuilder;

use crate::SolanaSignatureOutput;
use crate::WalletAccountInfo;
use crate::WalletResult;

pub const SOLANA_SIGN_AND_SEND_TRANSACTION: &str = "solana:signAndSendTransaction";

pub trait SolanaSignAndSendTransactionOutput: SolanaSignatureOutput {}
impl<T> SolanaSignAndSendTransactionOutput for T where T: SolanaSignatureOutput {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SolanaSignAndSendTransactionInput<Account: WalletAccountInfo> {
	/// Account to use.
	#[cfg_attr(feature = "browser", serde(with = "serde_wasm_bindgen::preserve"))]
	pub account: Account,
	#[serde(flatten)]
	pub props: SolanaSignAndSendTransactionProps,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SolanaSignAndSendTransactionProps {
	/// The versioned transaction.
	#[builder(setter(into))]
	pub transaction: VersionedTransaction,
	/// Chain to use.
	#[builder(default, setter(into, strip_option))]
	pub chain: Option<String>,
	#[builder(default, setter(into, strip_option))]
	pub options: Option<SolanaSignAndSendTransactionOptions>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SolanaSignAndSendTransactionOptions {
	/// Preflight commitment level.
	#[builder(default, setter(into, strip_option))]
	pub preflight_commitment: Option<CommitmentLevel>,
	/// The minimum slot that the request can be evaluated at.
	#[builder(default, setter(into, strip_option))]
	pub min_context_slot: Option<u64>,
	/// Mode for signing and sending transactions.
	#[builder(default, setter(into, strip_option))]
	pub mode: Option<SolanaSignAndSendTransactionMode>,
	/// Desired commitment level. If provided, confirm the transaction after
	/// sending.
	#[builder(default, setter(into, strip_option))]
	pub commitment: Option<CommitmentLevel>,
	/// Disable transaction verification at the RPC.
	#[builder(default, setter(into, strip_option))]
	pub skip_preflight: Option<bool>,
	/// Maximum number of times for the RPC node to retry sending the
	/// transaction to the leader.
	#[builder(default, setter(into, strip_option))]
	pub max_retries: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SolanaSignAndSendTransactionMode {
	/// Sign and send the transaction.
	Parallel,
	/// Sign the transaction and return it.
	Serial,
}

#[async_trait(?Send)]
pub trait WalletSolanaSignAndSendTransaction {
	type Output: SolanaSignAndSendTransactionOutput;

	async fn sign_and_send_transaction(
		&self,
		props: SolanaSignAndSendTransactionProps,
	) -> WalletResult<Self::Output>;
	async fn sign_and_send_transactions(
		&self,
		inputs: Vec<SolanaSignAndSendTransactionProps>,
	) -> WalletResult<Vec<Self::Output>>;
}
