use anchor_lang::AccountDeserialize;
use anchor_lang::AnchorSerialize;
use anchor_lang::Discriminator;
use async_trait::async_trait;
use solana_program::pubkey::Pubkey;
use solana_sdk::account::AccountSharedData;
use solana_sdk::account::WritableAccount;
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::rent::Rent;
use test_utils_solana::BanksClient;
use test_utils_solana::BanksClientError;
use test_utils_solana::ProgramTest;
use test_utils_solana::ProgramTestContext;
use test_utils_solana::prelude::*;

pub trait FromAnchorData {
	fn from_anchor_data<T: AnchorSerialize + Discriminator>(data: T, owner: Pubkey) -> Self;
}

impl FromAnchorData for AccountSharedData {
	fn from_anchor_data<T: AnchorSerialize + Discriminator>(data: T, owner: Pubkey) -> Self {
		let mut bytes = Vec::new();
		let discriminator = T::DISCRIMINATOR;
		let anchor_data = data.try_to_vec().expect("cannot serialize anchor data");

		bytes.extend_from_slice(discriminator);
		bytes.extend_from_slice(&anchor_data);

		let rent = Rent::default().minimum_balance(bytes.len());

		Self::create(rent, bytes, owner, false, 0)
	}
}

#[cfg(feature = "test_validator")]
pub trait TestValidatorGenesisExtensions {
	fn add_account_with_anchor<T: AnchorSerialize + Discriminator>(
		&mut self,
		pubkey: Pubkey,
		owner: Pubkey,
		data: T,
	);
}

#[cfg(feature = "test_validator")]
impl TestValidatorGenesisExtensions for test_utils_solana::TestValidatorGenesis {
	fn add_account_with_anchor<T: AnchorSerialize + Discriminator>(
		&mut self,
		address: Pubkey,
		owner: Pubkey,
		data: T,
	) {
		self.add_account(address, AccountSharedData::from_anchor_data(data, owner));
	}
}

#[async_trait(?Send)]
pub trait BanksClientAsyncAnchorExtension {
	async fn get_anchor_account<T: AccountDeserialize>(
		&mut self,
		address: &Pubkey,
	) -> Result<T, BanksClientError>;
}

#[async_trait(?Send)]
impl BanksClientAsyncAnchorExtension for BanksClient {
	async fn get_anchor_account<T: AccountDeserialize>(
		&mut self,
		address: &Pubkey,
	) -> Result<T, BanksClientError> {
		let Some(account) = self
			.get_account_with_commitment(*address, CommitmentLevel::Finalized)
			.await?
		else {
			return Err(BanksClientError::ClientError("account not found"));
		};

		let mut data: &[u8] = &account.data;
		let result = T::try_deserialize(&mut data).map_err(|_| {
			BanksClientError::ClientError("could not deserialize account, invalid data")
		})?;

		Ok(result)
	}
}

pub trait ProgramTestAnchorExtension {
	/// Adds an Anchor account.
	fn add_account_with_anchor<T: AnchorSerialize + Discriminator>(
		&mut self,
		pubkey: Pubkey,
		owner: Pubkey,
		anchor_data: T,
		executable: bool,
	);
	/// Adds an empty anchor account with a discriminator and specified size.
	fn add_empty_account_with_anchor<T: AnchorSerialize + Discriminator>(
		&mut self,
		pubkey: Pubkey,
		owner: Pubkey,
		size: usize,
	);
}

impl ProgramTestAnchorExtension for ProgramTest {
	fn add_account_with_anchor<T: AnchorSerialize + Discriminator>(
		&mut self,
		pubkey: Pubkey,
		owner: Pubkey,
		anchor_data: T,
		executable: bool,
	) {
		let discriminator = &T::DISCRIMINATOR;
		let data = anchor_data
			.try_to_vec()
			.expect("Cannot serialize provided anchor account");
		let mut v = Vec::new();
		v.extend_from_slice(discriminator);
		v.extend_from_slice(&data);
		self.add_account_with_data(pubkey, owner, &v, executable);
	}

	fn add_empty_account_with_anchor<T: AnchorSerialize + Discriminator>(
		&mut self,
		pubkey: Pubkey,
		owner: Pubkey,
		size: usize,
	) {
		let discriminator = T::DISCRIMINATOR;
		let data = vec![0_u8; size];
		let mut v = Vec::new();
		v.extend_from_slice(discriminator);
		v.extend_from_slice(&data);
		self.add_account_with_data(pubkey, owner, &v, false);
	}
}

#[async_trait(?Send)]
pub trait ProgramTestContextAnchorExtension {
	/// Get the anchor account from the provided address.
	async fn get_anchor_account<T: AccountDeserialize>(
		&mut self,
		address: &Pubkey,
	) -> Result<T, BanksClientError>;
	fn add_account_with_anchor<T: AnchorSerialize + Discriminator>(
		&mut self,
		pubkey: &Pubkey,
		owner: &Pubkey,
		anchor_data: T,
	) -> Result<(), BanksClientError>;
}

#[async_trait(?Send)]
impl ProgramTestContextAnchorExtension for ProgramTestContext {
	async fn get_anchor_account<T: AccountDeserialize>(
		&mut self,
		address: &Pubkey,
	) -> Result<T, BanksClientError> {
		self.banks_client.get_anchor_account(address).await
	}

	fn add_account_with_anchor<T: AnchorSerialize + Discriminator>(
		&mut self,
		address: &Pubkey,
		owner: &Pubkey,
		data: T,
	) -> Result<(), BanksClientError> {
		self.set_account(address, &AccountSharedData::from_anchor_data(data, *owner));

		Ok(())
	}
}

/// A wrapper providing supports for anchor programs in
/// [`solana_program_test::processor`]
///
/// The current processor for [`solana_program_test`] doesn't support anchor
/// programs due to lifetime conflicts. This is a wrapper that supports the
/// anchor lifetimes by using [`Box::leak`] on the accounts array.
#[macro_export]
macro_rules! anchor_processor {
	($program:ident) => {{
		fn entry(
			program_id: &::solana_program::pubkey::Pubkey,
			accounts: &[::solana_program::account_info::AccountInfo],
			instruction_data: &[u8],
		) -> ::solana_program::entrypoint::ProgramResult {
			let accounts = Box::leak(Box::new(accounts.to_vec()));

			$program::entry(program_id, accounts, instruction_data)
		}

		$crate::__private::processor!(entry)
	}};
}

pub mod prelude {
	pub use test_utils_solana::prelude::*;
	pub use wasm_client_anchor::prelude::*;

	pub use super::BanksClientAsyncAnchorExtension;
	pub use super::FromAnchorData;
	pub use super::ProgramTestAnchorExtension;
	pub use super::ProgramTestContextAnchorExtension;
	#[cfg(feature = "test_validator")]
	pub use super::TestValidatorGenesisExtensions;
}

#[doc(hidden)]
pub mod __private {
	pub use solana_program_test::processor;
}
