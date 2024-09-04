use core::str::FromStr;

use serde::Deserialize;
use serde::Deserializer;
use serde::Serializer;

pub mod pubkey {
	use solana_sdk::pubkey::Pubkey;

	use super::*;

	pub fn serialize<S>(pubkey: &Pubkey, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let string = pubkey.to_string();
		serializer.serialize_str(string.as_str())
	}

	pub fn deserialize<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
	where
		D: Deserializer<'de>,
	{
		Deserialize::deserialize(deserializer).map(|v| Pubkey::from_str(v).unwrap())
	}
}

pub mod hash {
	use solana_sdk::hash::Hash;

	use super::*;

	pub fn serialize<S>(hash: &Hash, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let string = hash.to_string();
		serializer.serialize_str(string.as_str())
	}

	pub fn deserialize<'de, D>(deserializer: D) -> Result<Hash, D::Error>
	where
		D: Deserializer<'de>,
	{
		Deserialize::deserialize(deserializer).map(|v| Hash::from_str(v).unwrap())
	}
}

pub mod signature {
	use solana_sdk::signature::Signature;

	use super::*;

	pub fn serialize<S>(signature: &Signature, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let string = signature.to_string();
		serializer.serialize_str(string.as_str())
	}

	pub fn deserialize<'de, D>(deserializer: D) -> Result<Signature, D::Error>
	where
		D: Deserializer<'de>,
	{
		Deserialize::deserialize(deserializer).map(|v| Signature::from_str(v).unwrap())
	}
}

#[cfg(test)]
mod tests {
	use assert2::check;
	use serde::Deserialize;
	use serde::Serialize;
	use solana_sdk::hash::Hash;
	use solana_sdk::pubkey::Pubkey;
	use solana_sdk::signature::Signature;

	use super::*;

	#[test]
	fn serde_solana() -> anyhow::Result<()> {
		#[derive(Serialize, Deserialize)]
		struct TestStruct {
			#[serde(with = "pubkey")]
			pubkey: Pubkey,
			#[serde(with = "hash")]
			hash: Hash,
			#[serde(with = "signature")]
			signature: Signature,
		}

		let initial = TestStruct {
			pubkey: Pubkey::new_unique(),
			hash: Hash::new_unique(),
			signature: Signature::new_unique(),
		};

		let string = serde_json::to_string(&initial)?;
		let output: TestStruct = serde_json::from_str(&string)?;

		check!(output.pubkey == initial.pubkey);
		check!(output.hash == initial.hash);
		check!(output.signature == initial.signature);

		Ok(())
	}
}
