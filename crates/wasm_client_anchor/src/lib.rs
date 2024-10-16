#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/readme.md"))]

pub use anchor::*;
pub use events::*;

mod anchor;
mod events;
pub mod macros;
pub mod utils;

pub mod prelude {
	pub use anchor_lang::AccountDeserialize;
	pub use anchor_lang::AccountSerialize;
	pub use anchor_lang::Accounts;
	pub use anchor_lang::AccountsClose;
	pub use anchor_lang::AccountsExit;
	pub use anchor_lang::Bump;
	pub use anchor_lang::Bumps;
	pub use anchor_lang::CheckId;
	pub use anchor_lang::CheckOwner;
	pub use anchor_lang::Discriminator;
	pub use anchor_lang::Event;
	pub use anchor_lang::Id;
	pub use anchor_lang::Ids;
	pub use anchor_lang::InstructionData;
	pub use anchor_lang::Key;
	pub use anchor_lang::Lamports;
	pub use anchor_lang::Owner;
	pub use anchor_lang::Owners;
	pub use anchor_lang::Space;
	pub use anchor_lang::ToAccountInfo;
	pub use anchor_lang::ToAccountInfos;
	pub use anchor_lang::ToAccountMetas;
	pub use anchor_lang::ZeroCopy;
	pub use solana_sdk::signer::Signer;
	pub use wasm_client_solana::prelude::*;

	pub use crate::AnchorProgramClient;
	pub use crate::AnchorRequestMethods;
	pub use crate::AnchorRpcClient;
	pub use crate::WalletAnchor;
}

#[doc(hidden)]
pub mod __private {
	pub use anchor_lang;
	pub use async_trait;
	pub use paste;
	pub use solana_sdk;
	pub use typed_builder;
	pub use wallet_standard;
	pub use wasm_client_solana;
}
