#![allow(unsafe_code)]

use std::future::Future;

use serde::Deserialize;
use serde::Serialize;
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::transaction::Transaction;
use solana_sdk::transaction::VersionedTransaction;
use typed_builder::TypedBuilder;

use crate::WalletAccountInfo;
use crate::WalletResult;

pub const SOLANA_SIGN_TRANSACTION: &str = "solana:signTransaction";

pub trait SolanaSignTransactionOutput {
	/// Signed, serialized transaction, as raw bytes.
	/// Returning a transaction rather than signatures allows multisig wallets,
	/// program wallets, and other wallets that use meta-transactions to return
	/// a modified, signed transaction.
	fn signed_transaction(&self) -> Vec<u8>;
	fn signed_versioned_transaction(&self) -> WalletResult<VersionedTransaction>;
	fn signed_legacy_transaction(&self) -> WalletResult<Transaction>;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SolanaSignTransactionInput<Account: WalletAccountInfo> {
	/// Account to use.
	#[cfg_attr(feature = "browser", serde(with = "serde_wasm_bindgen::preserve"))]
	pub account: Account,
	#[serde(flatten)]
	pub props: SolanaSignTransactionPropsWithBytes,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SolanaSignTransactionPropsWithBytes {
	/// The versioned transaction which will be encoded into bytes and sent to
	/// the wallet.
	#[builder(setter(into))]
	pub transaction: Vec<u8>,
	/// Chain to use.
	#[builder(default, setter(into))]
	pub chain: Option<String>,
	#[builder(default, setter(into, strip_option))]
	pub options: Option<SolanaSignTransactionOptions>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SolanaSignTransactionProps {
	/// The versioned transaction which will be encoded into bytes and sent to
	/// the wallet.
	#[builder(setter(into))]
	pub transaction: VersionedTransaction,
	/// Chain to use.
	#[builder(default, setter(into))]
	pub chain: Option<String>,
	#[builder(default, setter(into, strip_option))]
	pub options: Option<SolanaSignTransactionOptions>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SolanaSignTransactionOptions {
	/// Preflight commitment level.
	pub preflight_commitment: Option<CommitmentLevel>,
	/// The minimum slot that the request can be evaluated at.
	pub min_context_slot: Option<u64>,
}

pub trait WalletSolanaSignTransaction {
	type Output: SolanaSignTransactionOutput;

	fn sign_transaction(
		&self,
		props: SolanaSignTransactionProps,
	) -> impl Future<Output = WalletResult<Self::Output>>;
	fn sign_transactions(
		&self,
		inputs: Vec<SolanaSignTransactionProps>,
	) -> impl Future<Output = WalletResult<Vec<Self::Output>>>;
}
