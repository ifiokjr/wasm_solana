use std::sync::Arc;

use async_trait::async_trait;
use derive_more::Deref;
use derive_more::DerefMut;
use futures::lock::Mutex;
use send_wrapper::SendWrapper;
use serde_json::Value;
use solana_program_test::ProgramTestContext;
use solana_sdk::signature::Signature;
use wasm_client_solana::ClientError;
use wasm_client_solana::ClientResponse;
use wasm_client_solana::ClientResult;
use wasm_client_solana::Context;
use wasm_client_solana::GetAccountInfoRequest;
use wasm_client_solana::GetAccountInfoResponse;
use wasm_client_solana::LOCALNET;
use wasm_client_solana::RequestAirdropRequest;
use wasm_client_solana::RequestAirdropResponse;
use wasm_client_solana::RpcProvider;
use wasm_client_solana::SendTransactionRequest;
use wasm_client_solana::SendTransactionResponse;
use wasm_client_solana::SimulateTransactionRequest;
use wasm_client_solana::SimulateTransactionResponse;
use wasm_client_solana::SimulateTransactionResponseValue;
use wasm_client_solana::solana_account_decoder::UiAccount;

use crate::ProgramTestContextExtension;

#[derive(Clone, Deref, DerefMut)]
pub struct TestRpcProvider(Arc<Mutex<ProgramTestContext>>);

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
				value => {
					todo!("This method: `{value}` is not yet implementd for the `TestRpcProvider`.")
				}
			};

			Ok::<Value, ClientError>(result)
		};

		SendWrapper::new(future).await
	}
}
