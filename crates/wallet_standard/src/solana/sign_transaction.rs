use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::transaction::Transaction;
use solana_sdk::transaction::VersionedTransaction;
use typed_builder::TypedBuilder;

use crate::WalletResult;

pub const SOLANA_SIGN_TRANSACTION: &str = "solana:signTransaction";

pub trait SolanaSignTransactionOutput {
	/// Signed, serialized transaction, as raw bytes.
	/// Returning a transaction rather than signatures allows multisig wallets,
	/// program wallets, and other wallets that use meta-transactions to return
	/// a modified, signed transaction.
	fn signed_transaction_bytes(&self) -> Vec<u8>;
	fn signed_transaction(&self) -> WalletResult<VersionedTransaction>;
}

impl SolanaSignTransactionOutput for VersionedTransaction {
	fn signed_transaction_bytes(&self) -> Vec<u8> {
		bincode::serialize(self).unwrap()
	}

	fn signed_transaction(&self) -> WalletResult<VersionedTransaction> {
		Ok(self.clone())
	}
}

impl SolanaSignTransactionOutput for Transaction {
	fn signed_transaction_bytes(&self) -> Vec<u8> {
		let versioned_transaction = VersionedTransaction::from(self.clone());
		bincode::serialize(&versioned_transaction).unwrap()
	}

	fn signed_transaction(&self) -> WalletResult<VersionedTransaction> {
		Ok(self.clone().into())
	}
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
	/// Additional options for the transaction.
	#[builder(default, setter(into, strip_option(fallback = options_opt)))]
	pub options: Option<SolanaSignTransactionOptions>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SolanaSignTransactionOptions {
	/// Preflight commitment level.
	#[builder(default, setter(strip_option(fallback = preflight_commitment_opt)))]
	pub preflight_commitment: Option<CommitmentLevel>,
	/// The minimum slot that the request can be evaluated at.
	#[builder(default, setter(strip_option(fallback = min_context_slot_opt)))]
	pub min_context_slot: Option<u64>,
}

#[async_trait(?Send)]
pub trait WalletSolanaSignTransaction {
	type Output: SolanaSignTransactionOutput;

	async fn sign_transaction(
		&self,
		props: SolanaSignTransactionProps,
	) -> WalletResult<Self::Output>;
	async fn sign_transactions(
		&self,
		inputs: Vec<SolanaSignTransactionProps>,
	) -> WalletResult<Vec<Self::Output>>;
}
