use std::future::Future;

use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::WalletResult;

pub const EXPERIMENTAL_DECRYPT: &str = "experimental:decrypt";

pub trait ExperimentalDecryptOutput {
	/// `cleartext` that was decrypted.
	fn cleartext(&self) -> Vec<u8>;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ExperimentalDecryptProps {
	/// Cipher to use for decryption.
	#[builder(setter(into))]
	cipher: String,
	/// Public key to derive a shared key to decrypt the data using.
	#[builder(setter(into))]
	#[serde(with = "serde_bytes")]
	public_key: Vec<u8>,
	/// Ciphertext to decrypt.
	#[builder(setter(into))]
	#[serde(with = "serde_bytes")]
	pub cipher_text: Vec<u8>,
	/// Nonce to use for decryption.
	#[builder(setter(into))]
	#[serde(with = "serde_bytes")]
	pub nonce: Vec<u8>,
	/// Multiple of padding bytes to use for decryption, defaulting to 0.
	///
	/// Valid values `0 | 8 | 16 | 32 | 64 | 128 | 256 | 512 | 1024 | 2048`
	#[builder(default, setter(into, strip_option))]
	padding: Option<u8>,
}

#[async_trait(?Send)]
pub trait WalletExperimentalDecrypt {
	type Output: ExperimentalDecryptOutput;

	async fn decrypt_many(
		&self,
		props: Vec<ExperimentalDecryptProps>,
	) -> WalletResult<Vec<Self::Output>>;
	async fn decrypt(&self, props: ExperimentalDecryptProps) -> WalletResult<Self::Output>;
}
