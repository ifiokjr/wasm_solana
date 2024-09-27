#[macro_export]
macro_rules! base_create_request_builder {
	($program:path, $program_struct:path, $name_prefix:ident, $accounts:ident) => {
		$crate::__private::paste::paste! {
			pub type [<$name_prefix RequestBuilderPartial>]<'a, W> =
				[<$name_prefix RequestBuilder>]<
					'a,
					W,
					(
						(&'a $program_struct<W>,),
						(&'a W,),
						(),
						(),
						(),
						(),
						(),
						(),
						(),
					),
				>;

			#[derive($crate::__private::typed_builder::TypedBuilder)]
			pub struct [<$name_prefix Request>]<
				'a,
				W: $crate::WalletAnchor + 'a,
			> {
				/// This is the launchpad program.
				pub launchpad: &'a $program_struct<W>,
				/// This is the wallet / payer that will always sign the transaction. It should implement [`wasm_client_anchor::WalletAnchor`] to allow for async signing via wallets.
				pub wallet: &'a W,
				/// Provide the args to the anchor program endpoint. This will be transformed into the instruction data when processing the transaction.
				#[builder(setter(into))]
				pub args: ::$program::instruction::$name_prefix,
				/// Provide the anchor accounts that will be used for the anchor instruction
				pub accounts: ::$program::accounts::$accounts,
				/// Additional accounts which might be needed in a transfer hook / or in a future transaction when the transaction is saved on chain for a later date.
				#[builder(default)]
				pub remaining_accounts: Vec<$crate::__private::solana_sdk::instruction::AccountMeta>,
				/// Signers that can sign the data synchronously
				#[builder(default)]
				pub signers: Vec<&'a dyn $crate::__private::solana_sdk::signer::Signer>,
				#[builder(default)]
				/// Instructions that are run before the anchor instruction.
				pub instructions: Vec<$crate::__private::solana_sdk::instruction::Instruction>,
				#[builder(default)]
				/// Instructions that are run after the anchor instruction is completed.
				pub extra_instructions: Vec<$crate::__private::solana_sdk::instruction::Instruction>,
				/// Options to be passed into the transaction being signed or sent.
				#[builder(default)]
				pub options: $crate::__private::wallet_standard::SolanaSignAndSendTransactionOptions,
			}

			impl<'a, W: $crate::WalletAnchor + 'a> [<$name_prefix Request>]<'a, W> {}

			#[$crate::__private::async_trait::async_trait(?Send)]
			impl<'a, W: $crate::WalletAnchor + 'a> $crate::AnchorRequestMethods<'a, W>
				for [<$name_prefix Request>]<'a, W>
			{
				fn options(&self) -> $crate::__private::wallet_standard::SolanaSignAndSendTransactionOptions {
					self.options.clone()
				}

				fn wallet(&self) -> &'a W {
					self.wallet
				}

				fn rpc(&self) -> &'a $crate::__private::wasm_client_solana::SolanaRpcClient {
					self.launchpad.rpc()
				}

				fn signers(&self) -> Vec<&'a dyn $crate::__private::solana_sdk::signer::Signer> {
					self.signers.clone()
				}

				fn instructions(&self) -> Vec<$crate::__private::solana_sdk::instruction::Instruction> {
					use $crate::__private::anchor_lang::InstructionData;
					use $crate::__private::anchor_lang::ToAccountMetas;

					let mut accounts = self.accounts.to_account_metas(None);
					let mut instructions = self.instructions.clone();

					accounts.append(&mut self.remaining_accounts.clone());

					instructions.push($crate::__private::solana_sdk::instruction::Instruction {
						program_id: self.launchpad.id(),
						accounts,
						data: self.args.data(),
					});

					instructions.append(&mut self.extra_instructions.clone());

					instructions
				}
			}
		}
	};
}

#[macro_export]
#[allow(unused_macro_rules)]
macro_rules! create_request_builder {
	($program:path, $program_struct:path, $name_prefix:ident, $accounts:ident, "optional:args") => {
		$crate::base_create_request_builder!($program, $program_struct, $name_prefix, $accounts);
		$crate::__private::paste::paste! {
			pub type [<$name_prefix RequestBuilderArgsPartial>]<'a, W> =
				[<$name_prefix RequestBuilder>]<
					'a,
					W,
					(
						(&'a $program_struct<W>,),
						(&'a W,),
						(::$program::instruction::$name_prefix,),
						(),
						(),
						(),
						(),
						(),
						(),
					),
				>;
			impl<W: $crate::WalletAnchor> $program_struct<W> {
				pub fn [<$name_prefix:snake>](&self) -> [<$name_prefix RequestBuilderArgsPartial>]<'_, W> {
					[<$name_prefix Request>]::builder()
						.launchpad(self)
						.wallet(self.wallet())
						.args(::$program::instruction::$name_prefix {})
				}
			}
		}
	};
	($program:path, $program_struct:path, $name_prefix:ident, $accounts:ident, "required:args") => {
		$crate::base_create_request_builder!($program, $program_struct, $name_prefix, $accounts);
		$crate::__private::paste::paste! {
			impl<W: $crate::WalletAnchor> $program_struct<W> {
				pub fn [<$name_prefix:snake>](&self) -> [<$name_prefix RequestBuilderPartial>]<'_, W> {
					[<$name_prefix Request>]::builder()
						.launchpad(self)
						.wallet(self.wallet())

				}
			}
		}
	};
	($program:path, $program_struct:path, $name_prefix:ident, $accounts:ident) => {
		$crate::create_request_builder!($program, $program_struct, $name_prefix, $accounts, "required:args");
	};
	($program:path, $program_struct:path, $name_prefix:ident, "optional:args") => {
		$crate::create_request_builder!($program, $program_struct, $name_prefix, $name_prefix, "optional:args");
	};
	($program:path, $program_struct:path, $name_prefix:ident) => {
		$crate::create_request_builder!($program, $program_struct, $name_prefix, $name_prefix, "required:args");
	};
}
