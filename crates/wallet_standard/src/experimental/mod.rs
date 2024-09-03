pub use decrypt::*;
pub use encrypt::*;

mod decrypt;
mod encrypt;

/// Default encryption algorithm in `NaCl`.
/// Curve25519 scalar multiplication, Salsa20 secret-key encryption, and
/// Poly1305 one-time authentication.
pub const CIPHER_X25519_XSALSA20_POLY1305: &str = "x25519-xsalsa20-poly1305";
