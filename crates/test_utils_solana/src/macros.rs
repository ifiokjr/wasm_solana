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

/// Assert that the banks client simulation errors with the provided anchor
/// error.
#[macro_export]
macro_rules! assert_banks_client_simulated_error {
	($result:ident, $expected_error:path) => {{
		let error_name = $expected_error.name();

		$crate::__private::check!(
			$result.result.unwrap().is_err(),
			"the simulation was expected to error"
		);
		$crate::__private::check!(
			$result
				.simulation_details
				.unwrap()
				.logs
				.iter()
				.any(|log| log.contains(format!("Error Code: {error_name}").as_str())),
			"`{:#?}` not found in logs",
			$expected_error,
		);
	}};
}

/// Assert that the banks client errors with the expected anchor error code.
#[macro_export]
macro_rules! assert_banks_client_metadata_error {
	($result:ident, $expected_error:path) => {{
		let error_name = $expected_error.name();

		$crate::__private::check!(
			$result.result.is_err(),
			"the simulation was expected to error"
		);
		$crate::__private::check!(
			$result
				.metadata
				.unwrap()
				.log_messages
				.iter()
				.any(|log| log.contains(format!("Error Code: {error_name}").as_str())),
			"`{:#?}` not found in logs",
			$expected_error,
		);
	}};
}
