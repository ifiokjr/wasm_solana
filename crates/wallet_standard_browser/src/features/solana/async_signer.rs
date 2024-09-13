use async_trait::async_trait;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use wallet_standard::AsyncSigner;
use wallet_standard::SolanaSignatureOutput;
use wallet_standard::WalletResult;
use wallet_standard::WalletSolanaPubkey;
use wallet_standard::WalletSolanaSignMessage;

use crate::BrowserWallet;
use crate::BrowserWalletInfo;
use crate::SolanaSignAndSendTransactionFeature;
use crate::SolanaSignMessageFeature;
use crate::SolanaSignTransactionFeature;

#[async_trait(?Send)]
impl AsyncSigner for BrowserWallet {
	fn try_pubkey(&self) -> WalletResult<Pubkey> {
		WalletSolanaPubkey::try_pubkey(self)
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Signature> {
		let result = WalletSolanaSignMessage::sign_message(self, message).await?;

		result.try_signature()
	}
}

impl BrowserWalletInfo {
	pub fn is_solana_standard_compatible(&self) -> bool {
		self.is_standard_compatible()
			&& self.is_feature_supported::<SolanaSignMessageFeature>()
			&& self.is_feature_supported::<SolanaSignTransactionFeature>()
			&& self.is_feature_supported::<SolanaSignAndSendTransactionFeature>()
	}
}
