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

		::solana_program_test::processor!(entry)
	}};
}
