use bincode::deserialize;
use bincode::serialized_size;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::short_vec;
use solana_sdk::stake::config::Config as StakeConfig;
use solana_sdk::stake::config::{self as stake_config};

use super::parse_account_data::ParsableAccount;
use super::parse_account_data::ParseAccountError;
use super::validator_info;

/// A collection of keys to be stored in Config account data.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ConfigKeys {
	// Each key tuple comprises a unique `Pubkey` identifier,
	// and `bool` whether that key is a signer of the data
	#[serde(with = "short_vec")]
	pub keys: Vec<(Pubkey, bool)>,
}

impl ConfigKeys {
	pub fn serialized_size(keys: Vec<(Pubkey, bool)>) -> u64 {
		serialized_size(&ConfigKeys { keys }).unwrap()
	}
}

fn get_config_data(bytes: &[u8]) -> Result<&[u8], bincode::Error> {
	deserialize::<ConfigKeys>(bytes)
		.and_then(|keys| serialized_size(&keys))
		.map(|offset| &bytes[offset as usize..])
}

pub fn parse_config(data: &[u8], pubkey: &Pubkey) -> Result<ConfigAccountType, ParseAccountError> {
	let parsed_account = if pubkey == &stake_config::id() {
		get_config_data(data)
			.ok()
			.and_then(|data| deserialize::<StakeConfig>(data).ok())
			.map(|config| ConfigAccountType::StakeConfig(config.into()))
	} else {
		deserialize::<ConfigKeys>(data).ok().and_then(|key_list| {
			if !key_list.keys.is_empty() && key_list.keys[0].0 == validator_info::id() {
				parse_config_data::<String>(data, key_list.keys).and_then(|validator_info| {
					Some(ConfigAccountType::ValidatorInfo(UiConfig {
						keys: validator_info.keys,
						config_data: serde_json::from_str(&validator_info.config_data).ok()?,
					}))
				})
			} else {
				None
			}
		})
	};
	parsed_account.ok_or(ParseAccountError::AccountNotParsable(
		ParsableAccount::Config,
	))
}

fn parse_config_data<T>(data: &[u8], keys: Vec<(Pubkey, bool)>) -> Option<UiConfig<T>>
where
	T: serde::de::DeserializeOwned,
{
	let config_data: T = deserialize(get_config_data(data).ok()?).ok()?;
	let keys = keys
		.iter()
		.map(|key| {
			UiConfigKey {
				pubkey: key.0.to_string(),
				signer: key.1,
			}
		})
		.collect();
	Some(UiConfig { keys, config_data })
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type", content = "info")]
pub enum ConfigAccountType {
	StakeConfig(UiStakeConfig),
	ValidatorInfo(UiConfig<Value>),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiConfigKey {
	pub pubkey: String,
	pub signer: bool,
}

#[deprecated(
	since = "1.16.7",
	note = "Please use `solana_sdk::stake::state::warmup_cooldown_rate()` instead"
)]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UiStakeConfig {
	pub warmup_cooldown_rate: f64,
	pub slash_penalty: u8,
}

impl From<StakeConfig> for UiStakeConfig {
	fn from(config: StakeConfig) -> Self {
		Self {
			warmup_cooldown_rate: config.warmup_cooldown_rate,
			slash_penalty: config.slash_penalty,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiConfig<T> {
	pub keys: Vec<UiConfigKey>,
	pub config_data: T,
}
