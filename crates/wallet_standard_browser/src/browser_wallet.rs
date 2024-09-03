use async_trait::async_trait;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use typed_builder::TypedBuilder;
use wallet_standard::AsyncSigner;
use wallet_standard::SolanaSignatureOutput;
use wallet_standard::Wallet;
use wallet_standard::WalletError;
use wallet_standard::WalletResult;
use wallet_standard::WalletSolanaSignMessage;

use crate::BrowserWalletAccountInfo;
use crate::BrowserWalletInfo;

#[derive(Clone, Debug, PartialEq, Eq, TypedBuilder)]
pub struct BrowserWallet {
	/// The currently selected wallet account.
	#[builder(default, setter(strip_option))]
	pub wallet_account: Option<BrowserWalletAccountInfo>,
	/// The currently selected wallet.
	pub wallet: BrowserWalletInfo,
}

impl Wallet for BrowserWallet {
	type Account = BrowserWalletAccountInfo;
	type Wallet = BrowserWalletInfo;

	fn wallet(&self) -> Self::Wallet {
		self.wallet.clone()
	}

	fn wallet_account(&self) -> Option<Self::Account> {
		self.wallet_account.clone()
	}
}

impl From<BrowserWalletInfo> for BrowserWallet {
	fn from(value: BrowserWalletInfo) -> Self {
		BrowserWallet::builder().wallet(value).build()
	}
}

impl From<&BrowserWalletInfo> for BrowserWallet {
	fn from(value: &BrowserWalletInfo) -> Self {
		value.clone().into()
	}
}

#[async_trait(?Send)]
impl AsyncSigner for BrowserWallet {
	fn try_pubkey(&self) -> WalletResult<Pubkey> {
		self.try_public_key()
			.and_then(|bytes| Pubkey::try_from(bytes).ok())
			.ok_or(WalletError::WalletPublicKey)
	}

	async fn try_sign_message(&self, message: &[u8]) -> WalletResult<Signature> {
		let result = WalletSolanaSignMessage::sign_message(self, message).await?;

		result.try_signature()
	}
}
