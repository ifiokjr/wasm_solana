use anchor_lang::AccountDeserialize;
use anchor_lang::AccountSerialize;
use anchor_lang::AnchorSerialize;
use anchor_lang::Discriminator;
use anchor_lang::Owner;
use anchor_lang::Result;
use anchor_lang::error::Error;
use anchor_lang::error::ErrorCode;
use anchor_lang::solana_program::account_info::AccountInfo;
use solana_program::system_program;

/// In new versions of anchor the `Account` struct requires the lifetime
/// parameter to live longer than `'info`. This makes it difficult to generate
/// an account from account info.
///
/// The `Account::try_from` method is difficult to use. This is a workaround to
/// make it easier to use data stored by the `AccountInfo`.
pub fn get_verified_account_data<T: AccountSerialize + AccountDeserialize + Owner + Clone>(
	account_info: &AccountInfo<'_>,
) -> Result<T> {
	if system_program::check_id(account_info.owner) && account_info.lamports() == 0 {
		return Err(ErrorCode::AccountNotInitialized.into());
	}

	if account_info.owner != &T::owner() {
		return Err(Error::from(ErrorCode::AccountOwnedByWrongProgram)
			.with_pubkeys((*account_info.owner, T::owner())));
	}

	let mut data: &[u8] = &account_info.try_borrow_data()?;
	T::try_deserialize(&mut data)
}

/// Get the instruction data for the anchor program method.
pub fn get_anchor_instruction_data<T: AnchorSerialize + Discriminator>(
	data: &T,
) -> Result<Vec<u8>> {
	let mut result = vec![];
	result.append(&mut T::DISCRIMINATOR.to_vec());
	result.append(&mut data.try_to_vec()?);

	Ok(result)
}
