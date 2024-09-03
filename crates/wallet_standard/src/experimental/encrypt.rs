#![allow(unsafe_code)]

use std::future::Future;

use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::WalletResult;

pub const EXPERIMENTAL_ENCRYPT: &str = "experimental:encrypt";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ExperimentalEncryptProps {
	/// Cipher to use for encryption.
	#[builder(setter(into))]
	pub cipher: String,
	/// Public key to derive a shared key to encrypt the data using.
	#[builder(setter(into))]
	#[serde(with = "serde_bytes")]
	pub public_key: Vec<u8>,
	/// Cleartext to encrypt.
	#[serde(with = "serde_bytes")]
	pub cleartext: Vec<u8>,
	/// Multiple of padding bytes to use for encryption, defaulting to 0.
	///
	/// Valid values `0 | 8 | 16 | 32 | 64 | 128 | 256 | 512 | 1024 | 2048`
	#[builder(default, setter(into, strip_option))]
	pub padding: Option<u8>,
}

pub trait ExperimentalEncryptOutput {
	/// Ciphertext that was encrypted.
	fn cipher_text(&self) -> Vec<u8>;
	/// Nonce that was used for encryption.
	fn nonce(&self) -> Vec<u8>;
}

pub trait WalletExperimentalEncrypt {
	type Output: ExperimentalEncryptOutput;

	fn encrypt_many(
		&self,
		props: Vec<ExperimentalEncryptProps>,
	) -> impl Future<Output = WalletResult<Vec<Self::Output>>>;
	fn encrypt(
		&self,
		props: ExperimentalEncryptProps,
	) -> impl Future<Output = WalletResult<Self::Output>>;
}
