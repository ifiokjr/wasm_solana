#![allow(unsafe_code)]

use async_trait::async_trait;
use js_sys::Array;
use solana_sdk::signature::Signature;
use solana_sdk::transaction::TransactionVersion;
use wallet_standard::SolanaSignAndSendTransactionInput;
use wallet_standard::SolanaSignAndSendTransactionProps;
use wallet_standard::SolanaSignatureOutput;
use wallet_standard::WalletError;
use wallet_standard::WalletResult;
use wallet_standard::WalletSolanaSignAndSendTransaction;
use wallet_standard::SOLANA_SIGN_AND_SEND_TRANSACTION;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

use crate::impl_feature_from_js;
use crate::BrowserWallet;
use crate::BrowserWalletAccountInfo;

#[wasm_bindgen]
extern "C" {
	#[derive(Clone, Debug)]
	pub type BrowserSolanaSignAndSendTransactionOutput;
	/// Transaction signature, as raw bytes.
	#[wasm_bindgen(method, getter, js_name = signature)]
	pub fn _signature(this: &BrowserSolanaSignAndSendTransactionOutput) -> Vec<u8>;
	#[derive(Clone, Debug)]
	pub type SolanaSignAndSendTransactionFeature;
	/// Version of the feature API.
	#[wasm_bindgen(method, getter)]
	pub fn version(this: &SolanaSignAndSendTransactionFeature) -> String;
	#[wasm_bindgen(method, getter, js_name = supported_transaction_versions)]
	pub fn supported_transaction_versions_getter(
		this: &SolanaSignAndSendTransactionFeature,
	) -> Array;
	/// Sign transactions using the account's secret key and send them to the
	/// chain.
	///
	/// @param inputs Inputs for signing and sending transactions.
	///
	/// @return Outputs of signing and sending transactions.
	#[allow(unused_qualifications)]
	#[wasm_bindgen(method, catch, variadic, js_name = signAndSendTransaction)]
	pub async fn _sign_and_send_transaction(
		this: &SolanaSignAndSendTransactionFeature,
		args: Array,
	) -> Result<JsValue, JsValue>;
}

impl SolanaSignatureOutput for BrowserSolanaSignAndSendTransactionOutput {
	fn try_signature(&self) -> WalletResult<Signature> {
		self._signature()
			.try_into()
			.map_err(|_| WalletError::InvalidSignature)
	}

	fn signature(&self) -> Signature {
		self.try_signature().unwrap_throw()
	}
}

impl SolanaSignAndSendTransactionFeature {
	pub fn supported_transaction_versions(&self) -> WalletResult<Vec<TransactionVersion>> {
		let array = self.supported_transaction_versions_getter();
		let versions = array
			.iter()
			.map(|value| {
				let version: TransactionVersion = serde_wasm_bindgen::from_value(value)?;
				Ok(version)
			})
			.collect::<WalletResult<Vec<_>>>();

		versions
	}

	pub async fn sign_and_send_transaction(
		&self,
		account: BrowserWalletAccountInfo,
		props: SolanaSignAndSendTransactionProps,
	) -> WalletResult<BrowserSolanaSignAndSendTransactionOutput> {
		let input = SolanaSignAndSendTransactionInput::builder()
			.account(account)
			.props(props)
			.build();

		self.sign_and_send_transactions(vec![input])
			.await?
			.first()
			.cloned()
			.ok_or(WalletError::WalletSignTransaction)
	}

	pub async fn sign_and_send_transactions(
		&self,
		inputs: Vec<SolanaSignAndSendTransactionInput<BrowserWalletAccountInfo>>,
	) -> WalletResult<Vec<BrowserSolanaSignAndSendTransactionOutput>> {
		if inputs.is_empty() {
			return Err(WalletError::InvalidArguments);
		}

		let supported_transaction_versions = self.supported_transaction_versions()?;

		for input in &inputs {
			// Exit early if any of the versioned transactions are not supported.
			if !supported_transaction_versions.contains(&input.props.transaction.version()) {
				return Err(WalletError::UnsupportedTransactionVersion);
			}
		}

		let js_inputs: Array = serde_wasm_bindgen::to_value(&inputs)?.dyn_into()?;
		let js_results: Array = self
			._sign_and_send_transaction(js_inputs)
			.await?
			.dyn_into()?;

		Ok(js_results
			.into_iter()
			.map(wasm_bindgen::JsCast::unchecked_into)
			.collect())
	}
}

impl_feature_from_js!(
	SolanaSignAndSendTransactionFeature,
	SOLANA_SIGN_AND_SEND_TRANSACTION
);

#[async_trait(?Send)]
impl WalletSolanaSignAndSendTransaction for BrowserWallet {
	type Output = BrowserSolanaSignAndSendTransactionOutput;

	async fn sign_and_send_transaction(
		&self,
		props: SolanaSignAndSendTransactionProps,
	) -> WalletResult<Self::Output> {
		let Some(ref wallet_account) = self.wallet_account else {
			return Err(WalletError::WalletAccount);
		};

		self.wallet
			.get_feature::<SolanaSignAndSendTransactionFeature>()?
			.sign_and_send_transaction(wallet_account.clone(), props)
			.await
	}

	async fn sign_and_send_transactions(
		&self,
		inputs: Vec<SolanaSignAndSendTransactionProps>,
	) -> WalletResult<Vec<Self::Output>> {
		let Some(ref wallet_account) = self.wallet_account else {
			return Err(WalletError::WalletAccount);
		};

		let inputs = inputs
			.into_iter()
			.map(|props| {
				SolanaSignAndSendTransactionInput::builder()
					.account(wallet_account.clone())
					.props(props)
					.build()
			})
			.collect();

		self.wallet
			.get_feature::<SolanaSignAndSendTransactionFeature>()?
			.sign_and_send_transactions(inputs)
			.await
	}
}
