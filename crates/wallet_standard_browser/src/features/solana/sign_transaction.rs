#![allow(unsafe_code)]

use async_trait::async_trait;
use js_sys::Array;
use serde::Deserialize;
use serde::Serialize;
use solana_sdk::transaction::Transaction;
use solana_sdk::transaction::TransactionVersion;
use solana_sdk::transaction::VersionedTransaction;
use typed_builder::TypedBuilder;
use wallet_standard::SOLANA_SIGN_TRANSACTION;
use wallet_standard::SolanaSignTransactionOutput;
use wallet_standard::SolanaSignTransactionProps;
use wallet_standard::SolanaSignTransactionPropsWithBytes;
use wallet_standard::WalletError;
use wallet_standard::WalletResult;
use wallet_standard::WalletSolanaSignTransaction;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;

use crate::BrowserWallet;
use crate::BrowserWalletAccountInfo;
use crate::impl_feature_from_js;

#[wasm_bindgen]
extern "C" {
	#[derive(Clone, Debug)]
	pub type BrowserSolanaSignTransactionOutput;
	/// Signed, serialized transaction, as raw bytes.
	/// Returning a transaction rather than signatures allows multisig wallets,
	/// program wallets, and other wallets that use meta-transactions to return
	/// a modified, signed transaction.
	#[wasm_bindgen(method, getter, js_name = signedTransaction)]
	pub fn _signed_transaction(this: &BrowserSolanaSignTransactionOutput) -> Vec<u8>;
	#[derive(Clone, Debug)]
	pub type SolanaSignTransactionFeature;
	/// Version of the feature API.
	#[wasm_bindgen(method, getter)]
	pub fn version(this: &SolanaSignTransactionFeature) -> String;
	#[wasm_bindgen(method, getter, js_name = supportedTransactionVersions)]
	pub fn _supported_transaction_versions(this: &SolanaSignTransactionFeature) -> Array;
	/// Sign transactions using the account's secret key.
	///
	/// @param inputs Inputs for signing transactions.
	///
	/// @return Outputs of signing transactions.
	#[allow(unused_qualifications)]
	#[wasm_bindgen(method, catch, variadic, js_name = signTransaction)]
	pub async fn _sign_transaction(
		this: &SolanaSignTransactionFeature,
		args: Array,
	) -> Result<JsValue, JsValue>;
}

impl SolanaSignTransactionOutput for BrowserSolanaSignTransactionOutput {
	fn signed_transaction_bytes(&self) -> Vec<u8> {
		self._signed_transaction()
	}

	fn signed_transaction(&self) -> WalletResult<VersionedTransaction> {
		let bytes = self.signed_transaction_bytes();

		if let Ok(value) = bincode::deserialize(&bytes) {
			Ok(value)
		} else {
			// check if the wallet returns a legacy transaction and convert to a versioned
			// transaction.
			let transaction: Transaction = bincode::deserialize::<Transaction>(&bytes)
				.map_err(|_| WalletError::WalletSignTransaction)?;

			Ok(transaction.into())
		}
	}
}

impl SolanaSignTransactionFeature {
	pub fn supported_transaction_versions(&self) -> WalletResult<Vec<TransactionVersion>> {
		let array = self._supported_transaction_versions();
		let versions = array
			.iter()
			.map(|value| {
				let version: TransactionVersion = serde_wasm_bindgen::from_value(value)?;
				Ok(version)
			})
			.collect::<WalletResult<Vec<_>>>();

		versions
	}
}

impl_feature_from_js!(SolanaSignTransactionFeature, SOLANA_SIGN_TRANSACTION);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SolanaSignTransactionInput {
	/// Account to use.
	#[serde(with = "serde_wasm_bindgen::preserve")]
	pub account: BrowserWalletAccountInfo,
	#[serde(flatten)]
	pub props: SolanaSignTransactionPropsWithBytes,
}

impl SolanaSignTransactionFeature {
	pub async fn sign_transaction(
		&self,
		account: BrowserWalletAccountInfo,
		props: SolanaSignTransactionProps,
	) -> WalletResult<BrowserSolanaSignTransactionOutput> {
		self.sign_transactions(vec![(account, props)])
			.await?
			.first()
			.cloned()
			.ok_or(WalletError::WalletSignTransaction)
	}

	pub async fn sign_transactions(
		&self,
		inputs: Vec<(BrowserWalletAccountInfo, SolanaSignTransactionProps)>,
	) -> WalletResult<Vec<BrowserSolanaSignTransactionOutput>> {
		if inputs.is_empty() {
			return Err(WalletError::InvalidArguments);
		}

		let supported_transaction_versions = self.supported_transaction_versions()?;
		let inputs = inputs
			.into_iter()
			.map(|(account, props)| {
				// Exit early if any of the versioned transactions are not
				// supported.
				if !supported_transaction_versions.contains(&props.transaction.version()) {
					return Err(WalletError::UnsupportedTransactionVersion);
				}

				let input = SolanaSignTransactionInput::builder()
					.account(account)
					.props(SolanaSignTransactionPropsWithBytes {
						transaction: bincode::serialize(&props.transaction)
							.map_err(|_| WalletError::WalletSignTransaction)?,
						chain: props.chain,
						options: props.options,
					})
					.build();

				Ok(input)
			})
			.collect::<WalletResult<Vec<_>>>();

		let js_inputs: Array = serde_wasm_bindgen::to_value(&inputs)?.dyn_into()?;
		let js_results: Array = self._sign_transaction(js_inputs).await?.dyn_into()?;

		Ok(js_results.into_iter().map(JsCast::unchecked_into).collect())
	}
}

#[async_trait(?Send)]
impl WalletSolanaSignTransaction for BrowserWallet {
	type Output = BrowserSolanaSignTransactionOutput;

	async fn sign_transaction(
		&self,
		props: SolanaSignTransactionProps,
	) -> WalletResult<Self::Output> {
		let Some(ref wallet_account) = self.wallet_account else {
			return Err(WalletError::WalletAccount);
		};

		self.wallet
			.get_feature::<SolanaSignTransactionFeature>()?
			.sign_transaction(wallet_account.clone(), props)
			.await
	}

	async fn sign_transactions(
		&self,
		inputs: Vec<SolanaSignTransactionProps>,
	) -> WalletResult<Vec<Self::Output>> {
		let Some(ref wallet_account) = self.wallet_account else {
			return Err(WalletError::WalletAccount);
		};

		let inputs = inputs
			.into_iter()
			.map(|props| (wallet_account.clone(), props))
			.collect();

		self.wallet
			.get_feature::<SolanaSignTransactionFeature>()?
			.sign_transactions(inputs)
			.await
	}
}
