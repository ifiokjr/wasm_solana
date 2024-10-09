use std::sync::Arc;

use async_trait::async_trait;
use derive_more::Deref;
use derive_more::DerefMut;
use futures::lock::Mutex;
use send_wrapper::SendWrapper;
use serde_json::Value;
use solana_program_test::ProgramTestContext;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use wasm_client_solana::ClientError;
use wasm_client_solana::ClientResponse;
use wasm_client_solana::ClientResult;
use wasm_client_solana::Context;
use wasm_client_solana::GetAccountInfoRequest;
use wasm_client_solana::GetAccountInfoResponse;
use wasm_client_solana::GetBalanceRequest;
use wasm_client_solana::GetBalanceResponse;
use wasm_client_solana::GetLatestBlockhashResponse;
use wasm_client_solana::GetSignatureStatusesRequest;
use wasm_client_solana::GetSignatureStatusesResponse;
use wasm_client_solana::LOCALNET;
use wasm_client_solana::RequestAirdropRequest;
use wasm_client_solana::RequestAirdropResponse;
use wasm_client_solana::RpcProvider;
use wasm_client_solana::SendTransactionRequest;
use wasm_client_solana::SendTransactionResponse;
use wasm_client_solana::SimulateTransactionRequest;
use wasm_client_solana::SimulateTransactionResponse;
use wasm_client_solana::SimulateTransactionResponseValue;
use wasm_client_solana::SolanaRpcClient;
use wasm_client_solana::rpc_response::RpcBlockhash;
use wasm_client_solana::solana_account_decoder::UiAccount;
use wasm_client_solana::solana_transaction_status::TransactionConfirmationStatus;
use wasm_client_solana::solana_transaction_status::TransactionStatus;

use crate::ProgramTestContextExtension;

#[derive(Clone, Deref, DerefMut)]
pub struct TestRpcProvider(pub Arc<Mutex<ProgramTestContext>>);

impl TestRpcProvider {
	/// Create a new [`TestRpcProvider`] from the [`ProgramTestContext`].
	pub fn new(ctx: ProgramTestContext) -> Self {
		ctx.into()
	}

	/// Get the wrapped inner [`ProgramTestContext`].
	pub fn inner(&self) -> Arc<Mutex<ProgramTestContext>> {
		self.0.clone()
	}

	/// Wrap the current `RpcProvider` in an `Arc` struct.
	pub fn arc(&self) -> Arc<Self> {
		Arc::new(self.clone())
	}

	/// Create a new instance of the [`SolanaRpcClient`].
	pub fn to_rpc_client(&self) -> SolanaRpcClient {
		SolanaRpcClient::new_with_provider(self.arc(), CommitmentConfig::finalized())
	}
}

impl From<ProgramTestContext> for TestRpcProvider {
	fn from(value: ProgramTestContext) -> Self {
		Self(Arc::new(Mutex::new(value)))
	}
}

#[async_trait]
impl RpcProvider for TestRpcProvider {
	fn url(&self) -> String {
		LOCALNET.to_string()
	}

	async fn send(&self, method: &'static str, request: Value) -> ClientResult<Value> {
		let future = async move {
			let mut client = self.0.lock().await;
			let banks = &mut client.banks_client;

			let result = match method {
				"getAccountInfo" => {
					let request: GetAccountInfoRequest = serde_json::from_value(request).unwrap();
					let account = banks.get_account(request.pubkey).await.unwrap();
					let result = GetAccountInfoResponse {
						context: Context {
							slot: client.get_slot().await.unwrap(),
						},
						value: account.map(|account| UiAccount::encode(&request.pubkey, &account, wasm_client_solana::solana_account_decoder::UiAccountEncoding::Base64, None, None)),
					};
					let response = ClientResponse {
						jsonrpc: "2.0".into(),
						id: 0,
						result,
					};

					serde_json::to_value(response).unwrap()
				}
				"getBalance" => {
					let request: GetBalanceRequest = serde_json::from_value(request).unwrap();
					let value = banks.get_balance(request.pubkey).await.unwrap();
					let result = GetBalanceResponse {
						context: Context {
							slot: client.get_slot().await.unwrap(),
						},
						value,
					};
					let response = ClientResponse {
						jsonrpc: "2.0".into(),
						id: 0,
						result,
					};

					serde_json::to_value(response).unwrap()
				}
				"getLatestBlockhash" => {
					let blockhash = banks.get_latest_blockhash().await.unwrap();
					let last_valid_block_height = banks.get_root_block_height().await.unwrap();
					let result = GetLatestBlockhashResponse {
						context: Context {
							slot: client.get_slot().await.unwrap(),
						},
						value: RpcBlockhash {
							blockhash,
							last_valid_block_height,
						},
					};
					let response = ClientResponse {
						jsonrpc: "2.0".into(),
						id: 0,
						result,
					};

					serde_json::to_value(response).unwrap()
				}
				"getSignatureStatuses" => {
					let request: GetSignatureStatusesRequest =
						serde_json::from_value(request).unwrap();
					let statuses = banks
						.get_transaction_statuses(request.signatures)
						.await
						.unwrap();

					let result = GetSignatureStatusesResponse {
						context: Context {
							slot: client.get_slot().await.unwrap(),
						},
						value: statuses
							.into_iter()
							.map(|maybe_status| {
								maybe_status.map(|status| {
									TransactionStatus {
										slot: status.slot,
										confirmations: status.confirmations.map(|v| v as u64),
										err: status.err,
										confirmation_status: status.confirmation_status.map(|v| {
											match v {
										    solana_banks_interface::TransactionConfirmationStatus::Processed => TransactionConfirmationStatus::Processed,
										    solana_banks_interface::TransactionConfirmationStatus::Confirmed => TransactionConfirmationStatus::Confirmed,
										    solana_banks_interface::TransactionConfirmationStatus::Finalized => TransactionConfirmationStatus::Finalized,
											}
										})
									}
								})
							})
							.collect(),
					};
					let response = ClientResponse {
						jsonrpc: "2.0".into(),
						id: 0,
						result,
					};

					serde_json::to_value(response).unwrap()
				}
				"requestAirdrop" => {
					let request: RequestAirdropRequest = serde_json::from_value(request).unwrap();
					client
						.fund_account(&request.pubkey, request.lamports)
						.await
						.unwrap();
					let result = RequestAirdropResponse(Signature::default());
					let response = ClientResponse {
						jsonrpc: "2.0".into(),
						id: 0,
						result,
					};

					serde_json::to_value(response).unwrap()
				}
				"sendTransaction" => {
					let request: SendTransactionRequest = serde_json::from_value(request).unwrap();
					let signature = request
						.transaction
						.signatures
						.first()
						.copied()
						.unwrap_or(Signature::default());
					banks.send_transaction(request.transaction).await.unwrap();
					let result: SendTransactionResponse = SendTransactionResponse(signature);
					let response = ClientResponse {
						jsonrpc: "2.0".into(),
						id: 0,
						result,
					};

					serde_json::to_value(response).unwrap()
				}
				"simulateTransaction" => {
					let request: SimulateTransactionRequest =
						serde_json::from_value(request).unwrap();
					let simulation = banks
						.simulate_transaction(request.transaction)
						.await
						.unwrap();

					let result = SimulateTransactionResponse {
						context: Context {
							slot: client.get_slot().await.unwrap(),
						},
						value: SimulateTransactionResponseValue {
							err: simulation.result.and_then(|value| value.err().clone()),
							logs: simulation
								.simulation_details
								.as_ref()
								.map(|v| v.logs.clone()),
							accounts: Some(vec![]),
							units_consumed: simulation
								.simulation_details
								.as_ref()
								.map(|v| v.units_consumed),
							return_data: simulation
								.simulation_details
								.and_then(|v| v.return_data.map(Into::into)),
						},
					};
					let response = ClientResponse {
						jsonrpc: "2.0".into(),
						id: 0,
						result,
					};

					serde_json::to_value(response).unwrap()
				}
				value => {
					todo!("This method: `{value}` is not yet implementd for the `TestRpcProvider`.")
				}
			};

			Ok::<Value, ClientError>(result)
		};

		SendWrapper::new(future).await
	}
}
