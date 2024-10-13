use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::WalletError;
use crate::WalletResult;
use crate::WalletSolanaSignAndSendTransaction;
use crate::WalletSolanaSignIn;
use crate::WalletSolanaSignMessage;
use crate::WalletSolanaSignTransaction;
use crate::WalletStandard;

pub trait WalletSolanaPubkey {
	/// In order to prevent clashes with the built in
	/// [`solana_sdk::signer::Signer`] this is named differently.
	fn try_solana_pubkey(&self) -> WalletResult<Pubkey>;

	/// In order to prevent clashes with the built in
	/// [`solana_sdk::signer::Signer`] this is named differently.
	fn solana_pubkey(&self) -> Pubkey {
		self.try_solana_pubkey().unwrap_or_default()
	}
}

impl WalletSolanaPubkey for Keypair {
	fn try_solana_pubkey(&self) -> WalletResult<Pubkey> {
		let pubkey = Signer::try_pubkey(self)?;
		Ok(pubkey)
	}
}

impl<T> WalletSolanaPubkey for T
where
	T: WalletSolana,
{
	fn try_solana_pubkey(&self) -> WalletResult<Pubkey> {
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
