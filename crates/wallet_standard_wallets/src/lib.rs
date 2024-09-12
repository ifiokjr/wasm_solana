#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/readme.md"))]

pub use memory::*;

mod memory;

pub mod prelude {
	pub use wallet_standard::prelude::*;
	pub use wasm_client_solana::prelude::*;
}
