#[macro_export]
macro_rules! base_create_request_builder {
	($program:path, $program_struct:path, $name_prefix:ident, $accounts:ident) => {
		$crate::__private::paste::paste! {
			#[derive($crate::__private::typed_builder::TypedBuilder)]
			#[builder(mutators(
					/// Add signers to the request method. This can be added multiple times in the builder.
			    pub fn signers(
						&mut self,
						mut signers: Vec<&'a dyn $crate::__private::solana_sdk::signer::Signer>
					) {
						self.all_signers.append(&mut signers);
			    }
					/// Add signers to the request method. This can be added multiple times in the builder.
			    pub fn signer(
						&mut self,
						mut signer: &'a impl $crate::__private::solana_sdk::signer::Signer
					) {
						self.all_signers.push(signer);
			    }
			    /// Add instructions to the request method. This can be added multiple times in the builder.
			    pub fn instructions(
						&mut self,
						mut instructions: Vec<$crate::__private::solana_sdk::instruction::Instruction>
					) {
						self.pre_instructions.append(&mut instructions);
			    }
			    /// Add an instruction to the request method. This can be added multiple times in the builder.
			    pub fn instruction(
						&mut self,
						instruction: $crate::__private::solana_sdk::instruction::Instruction
					) {
						self.pre_instructions.push(instruction);
			    }
			))]
			pub struct [<$name_prefix Request>]<
				'a,
				W: $crate::WalletAnchor + 'a,
			> {
				/// This is the anchor client for interacting with this program.
				pub program_client: &'a $program_struct<W>,
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
				#[builder(via_mutators(init = vec![]))]
				pub all_signers: Vec<&'a dyn $crate::__private::solana_sdk::signer::Signer>,
				/// Instructions that are run prior to the current anchor program instruction.
				#[builder(via_mutators(init = vec![]))]
				pub pre_instructions: Vec<$crate::__private::solana_sdk::instruction::Instruction>,
				#[builder(default)]
				/// Instructions that are run after the anchor program instruction.
				pub post_instructions: Vec<$crate::__private::solana_sdk::instruction::Instruction>,
				/// Options to be passed into the transaction being signed or sent.
				#[builder(default)]
				pub options: $crate::__private::wallet_standard::SolanaSignAndSendTransactionOptions,
			}

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
					self.program_client.rpc()
				}

				fn signers(&self) -> Vec<&'a dyn $crate::__private::solana_sdk::signer::Signer> {
					self.all_signers.clone()
				}

				fn instructions(&self) -> Vec<$crate::__private::solana_sdk::instruction::Instruction> {
					use $crate::__private::anchor_lang::InstructionData;
					use $crate::__private::anchor_lang::ToAccountMetas;

					let mut accounts = self.accounts.to_account_metas(None);
					let mut instructions = self.pre_instructions.clone();

					accounts.append(&mut self.remaining_accounts.clone());

					instructions.push($crate::__private::solana_sdk::instruction::Instruction {
						program_id: self.program_client.id(),
						accounts,
						data: self.args.data(),
					});

					instructions.append(&mut self.post_instructions.clone());

					instructions
				}
			}

			impl<'a, W: $crate::WalletAnchor + 'a> [<$name_prefix Request>]<'a, W> {
				/// Compose multiple instructions from the current anchor program client.
				pub fn compose(&self) -> [<$program_struct Composer>]<'a, W> {
					use $crate::AnchorRequestMethods;

					[<$program_struct Composer>] {
						program_client: self.program_client,
						instructions: self.instructions(),
						signers: self.signers(),
					}
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
			pub type [<$name_prefix RequestBuilderOptionalArgs>]<'a, W> =
				[<$name_prefix RequestBuilder>]<
					'a,
					W,
					(
						(&'a $program_struct<W>,),
						(&'a W,),
						(::$program::instruction::$name_prefix,),
						(),
						(),
						(std::vec::Vec<&'a dyn $crate::prelude::Signer>,),
						(std::vec::Vec<$crate::__private::solana_sdk::instruction::Instruction>,),
						(),
						(),
					),
				>;
			impl<W: $crate::WalletAnchor> $program_struct<W> {
				pub fn [<$name_prefix:snake>](&self) -> [<$name_prefix RequestBuilderOptionalArgs>]<'_, W> {
					[<$name_prefix Request>]::builder()
						.program_client(self)
						.wallet(self.wallet())
						.args(::$program::instruction::$name_prefix {})
				}
			}

			impl<'a, W: $crate::WalletAnchor + 'a> [<$program_struct Composer>]<'a, W> {
				pub fn [<$name_prefix:snake>](self) -> [<$name_prefix RequestBuilderOptionalArgs>]<'a, W> {
					[<$name_prefix Request>]::builder()
						.program_client(self.program_client)
						.wallet(self.program_client.wallet())
						.args(::$program::instruction::$name_prefix {})
						.instructions(self.instructions)
						.signers(self.signers)
				}
			}
		}
	};

	($program:path, $program_struct:path, $name_prefix:ident, $accounts:ident, "required:args") => {
		$crate::base_create_request_builder!($program, $program_struct, $name_prefix, $accounts);
		$crate::__private::paste::paste! {
			pub type [<$name_prefix RequestBuilderRequiredArgs>]<'a, W> =
				[<$name_prefix RequestBuilder>]<
					'a,
					W,
					(
						(&'a $program_struct<W>,),
						(&'a W,),
						(),
						(),
						(),
						(std::vec::Vec<&'a dyn $crate::prelude::Signer>,),
						(std::vec::Vec<$crate::__private::solana_sdk::instruction::Instruction>,),
						(),
						(),
					),
				>;

			impl<W: $crate::WalletAnchor> $program_struct<W> {
				pub fn [<$name_prefix:snake>](&self) -> [<$name_prefix RequestBuilderRequiredArgs>]<'_, W> {
					[<$name_prefix Request>]::builder()
						.program_client(self)
						.wallet(self.wallet())

				}
			}

			impl<'a, W: $crate::WalletAnchor + 'a> [<$program_struct Composer>]<'a, W> {
				pub fn [<$name_prefix:snake>](self) -> [<$name_prefix RequestBuilderRequiredArgs>]<'a, W> {
					[<$name_prefix Request>]::builder()
						.program_client(self.program_client)
						.wallet(self.program_client.wallet())
						.instructions(self.instructions)
						.signers(self.signers)
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

#[macro_export]
macro_rules! create_program_client_macro {
	($program_crate:path, $program_client_struct:ident) => {
		$crate::__private::paste::paste! {
			macro_rules! [<$program_client_struct:snake _request_builder>] {
				($name_prefix: ident,$accounts: ident,"optional:args") => {
					$crate::create_request_builder!(
						$program_crate,
						$program_client_struct,
						$name_prefix,
						$accounts,
						"optional:args"
					);
				};
				($name_prefix: ident,$accounts: ident,"required:args") => {
					$crate::create_request_builder!(
						$program_crate,
						$program_client_struct,
						$name_prefix,
						$accounts,
						"required:args"
					);
				};
				($name_prefix: ident,$accounts: ident) => {
					$crate::create_request_builder!(
						$program_crate,
						$program_client_struct,
						$name_prefix,
						$accounts,
						"required:args"
					);
				};
				($name_prefix: ident,"optional:args") => {
					$crate::create_request_builder!(
						$program_crate,
						$program_client_struct,
						$name_prefix,
						$name_prefix,
						"optional:args"
					);
				};
				($name_prefix: ident) => {
					$crate::create_request_builder!(
						$program_crate,
						$program_client_struct,
						$name_prefix,
						$name_prefix,
						"required:args"
					);
				};
			}
		}
	};
}

/// Create a program client struct with the provided name.
///
/// ```rust
/// use wasm_client_anchor::create_program_client!(example_program, ExampleProgramClient);
/// use wasm_client_solana::SolanaRpcClient;
/// use wasm_client_solana::DEVNET;
/// use wallet_standard_wallets::MemoryWallet;
///
/// let rpc = SolanaRpcClient::new(DEVNET);
/// let wallet = MemoryWallet::new(rpc.clone(), accounts: &[Keypair::new()]);
/// let example_program_client = ExampleProgramClient::builder().rpc(rpc).wallet(wallet).build();
/// ```
#[macro_export]
macro_rules! create_program_client {
	($id:expr, $program_client_name:ident) => {
		$crate::__private::paste::paste! {
			pub trait [<Into $program_client_name>]<W: $crate::WalletAnchor> {
				fn [<into_ $program_client_name:snake>](self) -> $program_client_name<W>;
			}

			impl<W: $crate::WalletAnchor> [<Into $program_client_name>]<W> for $crate::AnchorProgram<W> {
				fn [<into_ $program_client_name:snake>](self) -> $program_client_name<W> {
					self.into()
				}
			}

			#[derive(::std::fmt::Debug, ::core::clone::Clone)]
			pub struct $program_client_name<W: $crate::WalletAnchor>($crate::AnchorProgram<W>);

			impl<W: $crate::WalletAnchor> core::ops::Deref for $program_client_name<W> {
				type Target = $crate::AnchorProgram<W>;

				fn deref(&self) -> &Self::Target {
					&self.0
				}
			}

			impl<W: $crate::WalletAnchor> From<$crate::AnchorProgram<W>> for $program_client_name<W> {
				fn from(program: $crate::AnchorProgram<W>) -> Self {
					$program_client_name(program)
				}
			}

			impl<W: $crate::WalletAnchor> $program_client_name<W> {
				/// Start the `AnchorProgram` builder with the `program_id` already set to
				/// the default.
				pub fn builder() -> $crate::AnchorProgramPartialBuilder<W> {
					$crate::AnchorProgram::builder().program_id($id)
				}

				/// Start the `AnchorProgram` builder with a custom `program_id`.
				pub fn builder_with_program(
					program_id: &$crate::__private::solana_sdk::pubkey::Pubkey,
				) -> $crate::AnchorProgramPartialBuilder<W> {
					$crate::AnchorProgram::builder().program_id(*program_id)
				}

				/// Get the program
				pub fn program(&self) -> &$crate::AnchorProgram<W> {
					self
				}

				/// Request an airdrop to the payer account
				pub async fn request_airdrop(
					&self,
					pubkey: &$crate::__private::solana_sdk::pubkey::Pubkey,
					lamports: u64,
				) -> $crate::AnchorClientResult<$crate::__private::solana_sdk::signature::Signature> {
					let signature = self.rpc().request_airdrop(pubkey, lamports).await?;
					Ok(signature)
				}
			}

			/// This struct is used to compose different request methods together.
			pub struct [<$program_client_name Composer>]<'a, W: $crate::WalletAnchor + 'a> {
				/// This is the anchor client for interacting with this program.
				program_client: &'a $program_client_name<W>,
				instructions: Vec<$crate::__private::solana_sdk::instruction::Instruction>,
				signers: Vec<&'a dyn $crate::__private::solana_sdk::signer::Signer>,
			}

			impl<'a, W: $crate::WalletAnchor + 'a> [<$program_client_name Composer>]<'a, W> {
				/// Generate a custom anchor request for instruction that you want to
				/// declare yourself.
				pub fn request(&self) -> $crate::AnchorRequestBuilderPartial<'_, W> {
					$crate::AnchorRequest::builder()
						.rpc(self.program_client.rpc())
						.program_id(self.program_client.id())
						.wallet(self.program_client.wallet())
				}

				/// Sometimes you don't want to interact with the program directly, but just
				/// need to send a transaction using the wallet.
				pub fn empty_request(&self) -> $crate::EmptyAnchorRequestBuilderPartial<'_, W> {
					$crate::EmptyAnchorRequest::builder()
						.rpc(self.program_client.rpc())
						.program_id(self.program_client.id())
						.wallet(self.program_client.wallet())
				}

			}
		}
	};
}
