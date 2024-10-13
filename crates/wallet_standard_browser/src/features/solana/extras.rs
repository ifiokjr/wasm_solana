use solana_sdk::pubkey::Pubkey;
use wallet_standard::WalletAccountInfo;
use wallet_standard::WalletError;
use wallet_standard::WalletResult;
use wallet_standard::WalletSolanaPubkey;

use crate::BrowserWalletAccountInfo;
use crate::BrowserWalletInfo;
use crate::SolanaSignAndSendTransactionFeature;
use crate::SolanaSignMessageFeature;
use crate::SolanaSignTransactionFeature;

impl BrowserWalletInfo {
	pub fn is_solana_standard_compatible(&self) -> bool {
		self.is_standard_compatible()
			&& self.is_feature_supported::<SolanaSignMessageFeature>()
			&& self.is_feature_supported::<SolanaSignTransactionFeature>()
			&& self.is_feature_supported::<SolanaSignAndSendTransactionFeature>()
	}
}

impl WalletSolanaPubkey for BrowserWalletAccountInfo {
	fn try_solana_pubkey(&self) -> WalletResult<Pubkey> {
		Pubkey::try_from(self.public_key()).map_err(|_| WalletError::WalletPublicKey)
	}
}
