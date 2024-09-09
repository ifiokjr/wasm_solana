pub use memory::*;

mod memory;

pub mod prelude {
	pub use wallet_standard::prelude::*;
	pub use wasm_client_solana::prelude::*;
}
