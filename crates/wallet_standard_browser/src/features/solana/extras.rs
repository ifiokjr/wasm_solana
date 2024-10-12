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
