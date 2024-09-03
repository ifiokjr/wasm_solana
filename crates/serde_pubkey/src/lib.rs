use core::str::FromStr;

use serde::Deserializer;
use serde::Serializer;
use solana_program::pubkey::Pubkey;

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
	serde::Deserialize::deserialize(deserializer).map(|v| Pubkey::from_str(v).unwrap())
}
