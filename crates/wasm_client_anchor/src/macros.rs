#[macro_export]
macro_rules! base_create_request_builder {
	($program:path, $program_struct:path, $name_prefix:ident, $accounts:ident) => {
		$crate::__private::paste::paste! {
			#[derive($crate::__private::typed_builder::TypedBuilder)]
			#[builder(mutators(
					/// Add signers to the request method. This can be added multiple times in the builder.
			    pub fn signers(
						&mut self,
						mut signers: std::vec::Vec<&'a dyn $crate::__private::solana_sdk::signer::Signer>
					) {
						self.signers_.append(&mut signers);
			    }
					/// Add signers to the request method. This can be added multiple times in the builder.
			    pub fn signer(
						&mut self,
						mut signer: &'a impl $crate::__private::solana_sdk::signer::Signer
					) {
						self.signers_.push(signer);
			    }
			    /// Add instructions to the request method. This can be added multiple times in the builder.
			    pub fn instructions(
						&mut self,
						mut instructions: std::vec::Vec<$crate::__private::solana_sdk::instruction::Instruction>
					) {
						self.instructions_before.append(&mut instructions);
			    }
			    /// Add an instruction to the request method. This can be added multiple times in the builder.
			    pub fn instruction(
						&mut self,
						instruction: $crate::__private::solana_sdk::instruction::Instruction
					) {
						self.instructions_before.push(instruction);
			    }
			    /// Add [`AddressLookupTable`]'s to the request method. This can be added multiple times in the builder.
			    pub fn address_lookup_tables(
						&mut self,
						mut address_lookup_tables: std::vec::Vec<$crate::__private::solana_sdk::address_lookup_table::AddressLookupTableAccount>
					) {
						self.address_lookup_tables_.append(&mut address_lookup_tables);
			    }
			    /// Add an [`AddressLookupTable`] to the request method. This can be added multiple times in the builder.
			    pub fn address_lookup_table(
						&mut self,
						address_lookup_table: $crate::__private::solana_sdk::address_lookup_table::AddressLookupTableAccount
					) {
						self.address_lookup_tables_.push(address_lookup_table);
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
				pub remaining_accounts: std::vec::Vec<$crate::__private::solana_sdk::instruction::AccountMeta>,
				/// Signers that can sign the data synchronously
				#[builder(via_mutators(init = vec![]))]
				pub signers_: std::vec::Vec<&'a dyn $crate::__private::solana_sdk::signer::Signer>,
				/// Instructions that are run prior to the current anchor program instruction.
				#[builder(via_mutators(init = vec![]))]
				pub instructions_before: std::vec::Vec<$crate::__private::solana_sdk::instruction::Instruction>,
				#[builder(default)]
				/// Instructions that are run after the anchor program instruction.
				pub instructions_after: std::vec::Vec<$crate::__private::solana_sdk::instruction::Instruction>,
				/// The address lookup tables to add to the transaction which saves space
				/// when creating the transaction.
				#[builder(via_mutators(init = vec![]))]
				pub address_lookup_tables_: std::vec::Vec<$crate::__private::solana_sdk::address_lookup_table::AddressLookupTableAccount>,
				/// A custom blockhash which can be used for a `DurableNonce` hash.
				#[builder(default, setter(into, strip_option(fallback = blockhash_opt)))]
				pub blockhash: ::core::option::Option<$crate::__private::solana_sdk::hash::Hash>,
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

				fn signers(&self) -> std::vec::Vec<&'a dyn $crate::__private::solana_sdk::signer::Signer> {
					self.signers_.clone()
				}

				fn instructions(&self) -> std::vec::Vec<$crate::__private::solana_sdk::instruction::Instruction> {
					use $crate::__private::anchor_lang::InstructionData;
					use $crate::__private::anchor_lang::ToAccountMetas;

					let mut accounts = self.accounts.to_account_metas(None);
					let mut instructions = self.instructions_before.clone();

					accounts.append(&mut self.remaining_accounts.clone());

					instructions.push($crate::__private::solana_sdk::instruction::Instruction {
						program_id: self.program_client.id(),
						accounts,
						data: self.args.data(),
					});

					instructions.append(&mut self.instructions_after.clone());

					instructions
				}

				fn address_lookup_tables(&self) -> std::vec::Vec<$crate::__private::solana_sdk::address_lookup_table::AddressLookupTableAccount> {
					self.address_lookup_tables_.clone()
				}

				async fn blockhash(&self) -> $crate::AnchorClientResult<$crate::__private::solana_sdk::hash::Hash> {
					let hash = self
						.blockhash
						.unwrap_or(self.rpc().get_latest_blockhash().await?);
					Ok(hash)
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
						address_lookup_tables: self.address_lookup_tables(),
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
						(std::vec::Vec<$crate::__private::solana_sdk::address_lookup_table::AddressLookupTableAccount>,),
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
						.address_lookup_tables(self.address_lookup_tables)
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
						(std::vec::Vec<$crate::__private::solana_sdk::address_lookup_table::AddressLookupTableAccount>,),
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
						.address_lookup_tables(self.address_lookup_tables)
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
/// use memory_wallet::MemoryWallet;
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

			impl<W: $crate::WalletAnchor> $crate::AnchorProgramClient<W> for $program_client_name<W> {
				fn builder() -> $crate::AnchorProgramPartialBuilder<W> {
					$crate::AnchorProgram::builder().program_id($id)
				}
			}
			/// This struct is used to compose different request methods together.
			pub struct [<$program_client_name Composer>]<'a, W: $crate::WalletAnchor + 'a> {
				/// This is the anchor client for interacting with this program.
				program_client: &'a $program_client_name<W>,
				instructions: std::vec::Vec<$crate::__private::solana_sdk::instruction::Instruction>,
				signers: std::vec::Vec<&'a dyn $crate::__private::solana_sdk::signer::Signer>,
				address_lookup_tables: std::vec::Vec<$crate::__private::solana_sdk::address_lookup_table::AddressLookupTableAccount>,
			}

			impl<'a, W: $crate::WalletAnchor + 'a> [<$program_client_name Composer>]<'a, W> {
				/// Generate a custom anchor request for instruction that you want to
				/// declare yourself.
				pub fn request(self) -> $crate::AnchorRequestBuilderPartial<'a, W> {
					$crate::AnchorRequest::builder()
						.rpc(self.program_client.rpc())
						.program_id(self.program_client.id())
						.wallet(self.program_client.wallet())
						.signers(self.signers)
						.instructions(self.instructions)
						.address_lookup_tables(self.address_lookup_tables)
				}

				/// Sometimes you don't want to interact with the program directly, but just
				/// need to send a transaction using the wallet.
				pub fn empty_request(self) -> $crate::EmptyAnchorRequestBuilderPartial<'a, W> {
					$crate::EmptyAnchorRequest::builder()
						.rpc(self.program_client.rpc())
						.program_id(self.program_client.id())
						.wallet(self.program_client.wallet())
						.signers(self.signers)
						.instructions(self.instructions)
						.address_lookup_tables(self.address_lookup_tables)
				}

			}
		}
	};
}
