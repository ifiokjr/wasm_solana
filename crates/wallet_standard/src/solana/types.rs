use solana_sdk::pubkey::Pubkey;

use crate::Wallet;
use crate::WalletAccountInfo;
use crate::WalletError;
use crate::WalletResult;
use crate::WalletSolanaSignAndSendTransaction;
use crate::WalletSolanaSignIn;
use crate::WalletSolanaSignMessage;
use crate::WalletSolanaSignTransaction;
use crate::WalletStandard;

pub trait WalletAccountInfoSolanaPubkey {
	fn pubkey(&self) -> Pubkey;
}

pub trait WalletSolanaPubkey {
	fn try_pubkey(&self) -> WalletResult<Pubkey>;

	fn pubkey(&self) -> Pubkey {
		self.try_pubkey().unwrap_or_default()
	}
}

impl<T> WalletAccountInfoSolanaPubkey for T
where
	T: WalletAccountInfo,
{
	fn pubkey(&self) -> Pubkey {
		Pubkey::try_from(self.public_key()).unwrap_or_default()
	}
}

impl<T> WalletSolanaPubkey for T
where
	T: WalletSolana,
{
	fn try_pubkey(&self) -> WalletResult<Pubkey> {
		self.try_public_key()
			.ok_or(WalletError::WalletNotConnected)
			.and_then(|bytes| Pubkey::try_from(bytes).map_err(|_| WalletError::WalletPublicKey))
	}
}

pub trait WalletSolana:
	WalletSolanaSignMessage
	+ WalletSolanaSignTransaction
	+ WalletSolanaSignAndSendTransaction
	+ WalletSolanaSignIn
	+ WalletStandard
{
}

impl<T> WalletSolana for T where
	T: WalletSolanaSignMessage
		+ WalletSolanaSignTransaction
		+ WalletSolanaSignAndSendTransaction
		+ WalletSolanaSignIn
		+ WalletStandard
{
}
