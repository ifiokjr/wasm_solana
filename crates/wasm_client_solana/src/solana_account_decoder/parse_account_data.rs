use std::collections::HashMap;

use heck::ToKebabCase;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use solana_sdk::address_lookup_table;
use solana_sdk::clock::UnixTimestamp;
use solana_sdk::instruction::InstructionError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::stake;
use solana_sdk::system_program;
use solana_sdk::sysvar;
use solana_sdk::vote;
use spl_token_2022::extension::interest_bearing_mint::InterestBearingConfig;
use spl_token_2022::extension::scaled_ui_amount::ScaledUiAmountConfig;
use thiserror::Error;

use super::parse_address_lookup_table::parse_address_lookup_table;
use super::parse_bpf_loader::parse_bpf_upgradeable_loader;
use super::parse_config::parse_config;
use super::parse_nonce::parse_nonce;
use super::parse_stake::parse_stake;
use super::parse_sysvar::parse_sysvar;
#[allow(deprecated)]
use super::parse_token::parse_token_v2;
use super::parse_vote::parse_vote;

lazy_static! {
	static ref ADDRESS_LOOKUP_PROGRAM_ID: Pubkey = address_lookup_table::program::id();
	static ref BPF_UPGRADEABLE_LOADER_PROGRAM_ID: Pubkey = solana_sdk::bpf_loader_upgradeable::id();
	static ref CONFIG_PROGRAM_ID: Pubkey = solana_program::config::program::id();
	static ref STAKE_PROGRAM_ID: Pubkey = stake::program::id();
	static ref SYSTEM_PROGRAM_ID: Pubkey = system_program::id();
	static ref SYSVAR_PROGRAM_ID: Pubkey = sysvar::id();
	static ref VOTE_PROGRAM_ID: Pubkey = vote::program::id();
	pub static ref PARSABLE_PROGRAM_IDS: HashMap<Pubkey, ParsableAccount> = {
		let mut m = HashMap::new();
		m.insert(
			*ADDRESS_LOOKUP_PROGRAM_ID,
			ParsableAccount::AddressLookupTable,
		);
		m.insert(
			*BPF_UPGRADEABLE_LOADER_PROGRAM_ID,
			ParsableAccount::BpfUpgradeableLoader,
		);
		m.insert(*CONFIG_PROGRAM_ID, ParsableAccount::Config);
		m.insert(*SYSTEM_PROGRAM_ID, ParsableAccount::Nonce);
		m.insert(spl_token::id(), ParsableAccount::SplToken);
		m.insert(spl_token_2022::id(), ParsableAccount::SplToken2022);
		m.insert(*STAKE_PROGRAM_ID, ParsableAccount::Stake);
		m.insert(*SYSVAR_PROGRAM_ID, ParsableAccount::Sysvar);
		m.insert(*VOTE_PROGRAM_ID, ParsableAccount::Vote);
		m
	};
}

#[derive(Error, Debug)]
pub enum ParseAccountError {
	#[error("{0:?} account not parsable")]
	AccountNotParsable(ParsableAccount),

	#[error("Program not parsable")]
	ProgramNotParsable,

	#[error("Additional data required to parse: {0}")]
	AdditionalDataMissing(String),

	#[error("Instruction error")]
	InstructionError(#[from] InstructionError),

	#[error("Serde json error")]
	SerdeJsonError(#[from] serde_json::error::Error),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParsedAccount {
	pub program: String,
	pub parsed: Value,
	pub space: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ParsableAccount {
	AddressLookupTable,
	BpfUpgradeableLoader,
	Config,
	Nonce,
	SplToken,
	SplToken2022,
	Stake,
	Sysvar,
	Vote,
}

#[deprecated(since = "2.0.0", note = "Use `AccountAdditionalDataV2` instead")]
#[derive(Clone, Copy, Default)]
pub struct AccountAdditionalData {
	pub spl_token_decimals: Option<u8>,
}

#[derive(Clone, Copy, Default)]
pub struct AccountAdditionalDataV2 {
	pub spl_token_additional_data: Option<SplTokenAdditionalData>,
}

#[derive(Clone, Copy, Default)]
pub struct SplTokenAdditionalData {
	pub decimals: u8,
	pub interest_bearing_config: Option<(InterestBearingConfig, UnixTimestamp)>,
}

impl SplTokenAdditionalData {
	pub fn with_decimals(decimals: u8) -> Self {
		Self {
			decimals,
			..Default::default()
		}
	}
}

#[derive(Clone, Copy, Default)]
pub struct SplTokenAdditionalDataV2 {
	pub decimals: u8,
	pub interest_bearing_config: Option<(InterestBearingConfig, UnixTimestamp)>,
	pub scaled_ui_amount_config: Option<(ScaledUiAmountConfig, UnixTimestamp)>,
}

impl From<SplTokenAdditionalData> for SplTokenAdditionalDataV2 {
	fn from(v: SplTokenAdditionalData) -> Self {
		Self {
			decimals: v.decimals,
			interest_bearing_config: v.interest_bearing_config,
			scaled_ui_amount_config: None,
		}
	}
}

impl SplTokenAdditionalDataV2 {
	pub fn with_decimals(decimals: u8) -> Self {
		Self {
			decimals,
			..Default::default()
		}
	}
}

#[deprecated(since = "2.0.0", note = "Use `parse_account_data_v2` instead")]
#[allow(deprecated)]
pub fn parse_account_data(
	pubkey: &Pubkey,
	program_id: &Pubkey,
	data: &[u8],
	additional_data: Option<AccountAdditionalData>,
) -> Result<ParsedAccount, ParseAccountError> {
	parse_account_data_v2(
		pubkey,
		program_id,
		data,
		additional_data.map(|d| {
			AccountAdditionalDataV2 {
				spl_token_additional_data: d
					.spl_token_decimals
					.map(SplTokenAdditionalData::with_decimals),
			}
		}),
	)
}

#[allow(deprecated)]
pub fn parse_account_data_v2(
	pubkey: &Pubkey,
	program_id: &Pubkey,
	data: &[u8],
	additional_data: Option<AccountAdditionalDataV2>,
) -> Result<ParsedAccount, ParseAccountError> {
	let program_name = PARSABLE_PROGRAM_IDS
		.get(program_id)
		.ok_or(ParseAccountError::ProgramNotParsable)?;
	let additional_data = additional_data.unwrap_or_default();
	let parsed_json = match program_name {
		ParsableAccount::AddressLookupTable => {
			serde_json::to_value(parse_address_lookup_table(data)?)?
		}
		ParsableAccount::BpfUpgradeableLoader => {
			serde_json::to_value(parse_bpf_upgradeable_loader(data)?)?
		}
		ParsableAccount::Config => serde_json::to_value(parse_config(data, pubkey)?)?,
		ParsableAccount::Nonce => serde_json::to_value(parse_nonce(data)?)?,
		ParsableAccount::SplToken | ParsableAccount::SplToken2022 => {
			serde_json::to_value(parse_token_v2(
				data,
				additional_data.spl_token_additional_data.as_ref(),
			)?)?
		}
		ParsableAccount::Stake => serde_json::to_value(parse_stake(data)?)?,
		ParsableAccount::Sysvar => serde_json::to_value(parse_sysvar(data, pubkey)?)?,
		ParsableAccount::Vote => serde_json::to_value(parse_vote(data)?)?,
	};
	Ok(ParsedAccount {
		program: format!("{program_name:?}").to_kebab_case(),
		parsed: parsed_json,
		space: data.len() as u64,
	})
}

#[cfg(test)]
mod test {
	use solana_sdk::nonce::State;
	use solana_sdk::nonce::state::Data;
	use solana_sdk::nonce::state::Versions;
	use solana_sdk::vote::program::id as vote_program_id;
	use solana_sdk::vote::state::VoteState;
	use solana_sdk::vote::state::VoteStateVersions;

	use super::*;

	#[test]
	fn test_parse_account_data() {
		let account_pubkey = solana_sdk::pubkey::new_rand();
		let other_program = solana_sdk::pubkey::new_rand();
		let data = vec![0; 4];
		assert!(parse_account_data_v2(&account_pubkey, &other_program, &data, None).is_err());

		let vote_state = VoteState::default();
		let mut vote_account_data: Vec<u8> = vec![0; VoteState::size_of()];
		let versioned = VoteStateVersions::new_current(vote_state);
		VoteState::serialize(&versioned, &mut vote_account_data).unwrap();
		let parsed = parse_account_data_v2(
			&account_pubkey,
			&vote_program_id(),
			&vote_account_data,
			None,
		)
		.unwrap();
		assert_eq!(parsed.program, "vote".to_string());
		assert_eq!(parsed.space, VoteState::size_of() as u64);

		let nonce_data = Versions::new(State::Initialized(Data::default()));
		let nonce_account_data = bincode::serialize(&nonce_data).unwrap();
		let parsed = parse_account_data_v2(
			&account_pubkey,
			&system_program::id(),
			&nonce_account_data,
			None,
		)
		.unwrap();
		assert_eq!(parsed.program, "nonce".to_string());
		assert_eq!(parsed.space, State::size() as u64);
	}
}
