#![allow(clippy::arithmetic_side_effects)]

pub mod parse_account_data;
pub mod parse_address_lookup_table;
pub mod parse_bpf_loader;
#[allow(deprecated)]
pub mod parse_config;
pub mod parse_nonce;
pub mod parse_stake;
pub mod parse_sysvar;
pub mod parse_token;
pub mod parse_token_extension;
pub mod parse_vote;
pub mod validator_info;

use std::io::Read;
use std::io::Write;

use base64::Engine;
use base64::prelude::BASE64_STANDARD;
use serde::Deserialize;
use serde::Serialize;
use serde_with::DisplayFromStr;
use serde_with::serde_as;
use serde_with::skip_serializing_none;
use solana_sdk::account::ReadableAccount;
use solana_sdk::account::WritableAccount;
use solana_sdk::clock::Epoch;
use solana_sdk::fee_calculator::FeeCalculator;
use solana_sdk::pubkey::Pubkey;
use typed_builder::TypedBuilder;

use self::parse_account_data::AccountAdditionalDataV2;
use self::parse_account_data::ParsedAccount;
use self::parse_account_data::parse_account_data_v2;

pub type StringAmount = String;
pub type StringDecimals = String;
pub const MAX_BASE58_BYTES: usize = 128;

/// A duplicate representation of an Account for pretty JSON serialization
#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct UiAccount {
	pub lamports: u64,
	pub data: UiAccountData,
	#[serde_as(as = "DisplayFromStr")]
	pub owner: Pubkey,
	#[builder(setter(into, strip_bool(fallback = executable_bool)))]
	pub executable: bool,
	pub rent_epoch: Epoch,
	#[builder(default, setter(into, strip_option(fallback = space_opt)))]
	pub space: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum UiAccountData {
	LegacyBinary(String), // Legacy. Retained for RPC backwards compatibility
	Json(ParsedAccount),
	Binary(String, UiAccountEncoding),
}

impl UiAccountData {
	/// Returns decoded account data in binary format if possible
	pub fn decode(&self) -> Option<Vec<u8>> {
		match self {
			UiAccountData::Json(_) => None,
			UiAccountData::LegacyBinary(blob) => bs58::decode(blob).into_vec().ok(),
			UiAccountData::Binary(blob, encoding) => {
				match encoding {
					UiAccountEncoding::Base58 => bs58::decode(blob).into_vec().ok(),
					UiAccountEncoding::Base64 => BASE64_STANDARD.decode(blob).ok(),
					#[cfg(feature = "zstd")]
					UiAccountEncoding::Base64Zstd => {
						BASE64_STANDARD.decode(blob).ok().and_then(|zstd_data| {
							let mut data = vec![];
							zstd::stream::read::Decoder::new(zstd_data.as_slice())
								.and_then(|mut reader| reader.read_to_end(&mut data))
								.map(|_| data)
								.ok()
						})
					}
					UiAccountEncoding::Binary | UiAccountEncoding::JsonParsed => None,
				}
			}
		}
	}
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum UiAccountEncoding {
	Binary, // Legacy. Retained for RPC backwards compatibility
	Base58,
	Base64,
	JsonParsed,
	#[cfg(feature = "zstd")]
	#[serde(rename = "base64+zstd")]
	Base64Zstd,
}

impl UiAccount {
	fn encode_bs58<T: ReadableAccount>(
		account: &T,
		data_slice_config: Option<UiDataSliceConfig>,
	) -> String {
		let slice = slice_data(account.data(), data_slice_config);
		if slice.len() <= MAX_BASE58_BYTES {
			bs58::encode(slice).into_string()
		} else {
			"error: data too large for bs58 encoding".to_string()
		}
	}

	pub fn encode<T: ReadableAccount>(
		pubkey: &Pubkey,
		account: &T,
		encoding: UiAccountEncoding,
		additional_data: Option<AccountAdditionalDataV2>,
		data_slice_config: Option<UiDataSliceConfig>,
	) -> Self {
		let space = account.data().len();
		let data = match encoding {
			UiAccountEncoding::Binary => {
				let data = Self::encode_bs58(account, data_slice_config);
				UiAccountData::LegacyBinary(data)
			}
			UiAccountEncoding::Base58 => {
				let data = Self::encode_bs58(account, data_slice_config);
				UiAccountData::Binary(data, encoding)
			}
			UiAccountEncoding::Base64 => {
				UiAccountData::Binary(
					BASE64_STANDARD.encode(slice_data(account.data(), data_slice_config)),
					encoding,
				)
			}
			#[cfg(feature = "zstd")]
			UiAccountEncoding::Base64Zstd => {
				let mut encoder = zstd::stream::write::Encoder::new(Vec::new(), 0).unwrap();
				match encoder
					.write_all(slice_data(account.data(), data_slice_config))
					.and_then(|()| encoder.finish())
				{
					Ok(zstd_data) => {
						UiAccountData::Binary(BASE64_STANDARD.encode(zstd_data), encoding)
					}
					Err(_) => {
						UiAccountData::Binary(
							BASE64_STANDARD.encode(slice_data(account.data(), data_slice_config)),
							UiAccountEncoding::Base64,
						)
					}
				}
			}
			UiAccountEncoding::JsonParsed => {
				if let Ok(parsed_data) =
					parse_account_data_v2(pubkey, account.owner(), account.data(), additional_data)
				{
					UiAccountData::Json(parsed_data)
				} else {
					UiAccountData::Binary(
						BASE64_STANDARD.encode(slice_data(account.data(), data_slice_config)),
						UiAccountEncoding::Base64,
					)
				}
			}
		};
		UiAccount {
			lamports: account.lamports(),
			data,
			owner: *account.owner(),
			executable: account.executable(),
			rent_epoch: account.rent_epoch(),
			space: Some(space as u64),
		}
	}

	pub fn decode<T: WritableAccount>(&self) -> Option<T> {
		let data = self.data.decode()?;
		Some(T::create(
			self.lamports,
			data,
			self.owner,
			self.executable,
			self.rent_epoch,
		))
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiFeeCalculator {
	pub lamports_per_signature: StringAmount,
}

impl From<FeeCalculator> for UiFeeCalculator {
	fn from(fee_calculator: FeeCalculator) -> Self {
		Self {
			lamports_per_signature: fee_calculator.lamports_per_signature.to_string(),
		}
	}
}

impl Default for UiFeeCalculator {
	fn default() -> Self {
		Self {
			lamports_per_signature: "0".to_string(),
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UiDataSliceConfig {
	pub offset: usize,
	pub length: usize,
}

fn slice_data(data: &[u8], data_slice_config: Option<UiDataSliceConfig>) -> &[u8] {
	if let Some(UiDataSliceConfig { offset, length }) = data_slice_config {
		if offset >= data.len() {
			&[]
		} else if length > data.len() - offset {
			&data[offset..]
		} else {
			&data[offset..offset + length]
		}
	} else {
		data
	}
}

#[cfg(test)]
mod test {
	use solana_sdk::account::Account;
	use solana_sdk::account::AccountSharedData;

	use super::*;

	#[test]
	fn test_slice_data() {
		let data = vec![1, 2, 3, 4, 5];
		let slice_config = Some(UiDataSliceConfig {
			offset: 0,
			length: 5,
		});
		assert_eq!(slice_data(&data, slice_config), &data[..]);

		let slice_config = Some(UiDataSliceConfig {
			offset: 0,
			length: 10,
		});
		assert_eq!(slice_data(&data, slice_config), &data[..]);

		let slice_config = Some(UiDataSliceConfig {
			offset: 1,
			length: 2,
		});
		assert_eq!(slice_data(&data, slice_config), &data[1..3]);

		let slice_config = Some(UiDataSliceConfig {
			offset: 10,
			length: 2,
		});
		assert_eq!(slice_data(&data, slice_config), &[] as &[u8]);
	}

	#[test]
	fn test_encode_account_when_data_exceeds_base58_byte_limit() {
		let data = vec![42; MAX_BASE58_BYTES + 2];
		let account = AccountSharedData::from(Account {
			data,
			..Account::default()
		});

		// Whole account
		assert_eq!(
			UiAccount::encode_bs58(&account, None),
			"error: data too large for bs58 encoding"
		);

		// Slice of account that's still too large
		assert_eq!(
			UiAccount::encode_bs58(
				&account,
				Some(UiDataSliceConfig {
					length: MAX_BASE58_BYTES + 1,
					offset: 1
				})
			),
			"error: data too large for bs58 encoding"
		);

		// Slice of account that fits inside `MAX_BASE58_BYTES`
		assert_ne!(
			UiAccount::encode_bs58(
				&account,
				Some(UiDataSliceConfig {
					length: MAX_BASE58_BYTES,
					offset: 1
				})
			),
			"error: data too large for bs58 encoding"
		);

		// Slice of account that's too large, but whose intersection with the account
		// still fits
		assert_ne!(
			UiAccount::encode_bs58(
				&account,
				Some(UiDataSliceConfig {
					length: MAX_BASE58_BYTES + 1,
					offset: 2
				})
			),
			"error: data too large for bs58 encoding"
		);
	}

	#[cfg(feature = "zstd")]
	#[test]
	fn test_base64_zstd() {
		use assert2::check;

		let encoded_account = UiAccount::encode(
			&Pubkey::default(),
			&AccountSharedData::from(Account {
				data: vec![0; 1024],
				..Account::default()
			}),
			UiAccountEncoding::Base64Zstd,
			None,
			None,
		);

		insta::assert_compact_json_snapshot!(encoded_account.data, @r###"["KLUv/QBYTQAAEAAAAQD7K4AF", "base64+zstd"]"###);

		let decoded_account = encoded_account.decode::<Account>().unwrap();
		check!(decoded_account.data() == &vec![0; 1024]);

		let decoded_account = encoded_account.decode::<AccountSharedData>().unwrap();
		check!(decoded_account.data() == &vec![0; 1024]);
	}
}
