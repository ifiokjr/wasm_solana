use bincode::deserialize;
use serde::Deserialize;
use serde::Serialize;
use solana_sdk::clock::Epoch;
use solana_sdk::clock::UnixTimestamp;
use solana_sdk::stake::state::Authorized;
use solana_sdk::stake::state::Delegation;
use solana_sdk::stake::state::Lockup;
use solana_sdk::stake::state::Meta;
use solana_sdk::stake::state::Stake;
use solana_sdk::stake::state::StakeStateV2;

use super::parse_account_data::ParsableAccount;
use super::parse_account_data::ParseAccountError;
use super::StringAmount;

pub fn parse_stake(data: &[u8]) -> Result<StakeAccountType, ParseAccountError> {
	let stake_state: StakeStateV2 = deserialize(data)
		.map_err(|_| ParseAccountError::AccountNotParsable(ParsableAccount::Stake))?;
	let parsed_account = match stake_state {
		StakeStateV2::Uninitialized => StakeAccountType::Uninitialized,
		StakeStateV2::Initialized(meta) => {
			StakeAccountType::Initialized(UiStakeAccount {
				meta: meta.into(),
				stake: None,
			})
		}
		StakeStateV2::Stake(meta, stake, _) => {
			StakeAccountType::Delegated(UiStakeAccount {
				meta: meta.into(),
				stake: Some(stake.into()),
			})
		}
		StakeStateV2::RewardsPool => StakeAccountType::RewardsPool,
	};
	Ok(parsed_account)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type", content = "info")]
pub enum StakeAccountType {
	Uninitialized,
	Initialized(UiStakeAccount),
	Delegated(UiStakeAccount),
	RewardsPool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UiStakeAccount {
	pub meta: UiMeta,
	pub stake: Option<UiStake>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiMeta {
	pub rent_exempt_reserve: StringAmount,
	pub authorized: UiAuthorized,
	pub lockup: UiLockup,
}

impl From<Meta> for UiMeta {
	fn from(meta: Meta) -> Self {
		Self {
			rent_exempt_reserve: meta.rent_exempt_reserve.to_string(),
			authorized: meta.authorized.into(),
			lockup: meta.lockup.into(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiLockup {
	pub unix_timestamp: UnixTimestamp,
	pub epoch: Epoch,
	pub custodian: String,
}

impl From<Lockup> for UiLockup {
	fn from(lockup: Lockup) -> Self {
		Self {
			unix_timestamp: lockup.unix_timestamp,
			epoch: lockup.epoch,
			custodian: lockup.custodian.to_string(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiAuthorized {
	pub staker: String,
	pub withdrawer: String,
}

impl From<Authorized> for UiAuthorized {
	fn from(authorized: Authorized) -> Self {
		Self {
			staker: authorized.staker.to_string(),
			withdrawer: authorized.withdrawer.to_string(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UiStake {
	pub delegation: UiDelegation,
	pub credits_observed: u64,
}

impl From<Stake> for UiStake {
	fn from(stake: Stake) -> Self {
		Self {
			delegation: stake.delegation.into(),
			credits_observed: stake.credits_observed,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UiDelegation {
	pub voter: String,
	pub stake: StringAmount,
	pub activation_epoch: StringAmount,
	pub deactivation_epoch: StringAmount,
	#[deprecated(
		since = "1.16.7",
		note = "Please use `solana_sdk::stake::stake::warmup_cooldown_rate()` instead"
	)]
	pub warmup_cooldown_rate: f64,
}

impl From<Delegation> for UiDelegation {
	fn from(delegation: Delegation) -> Self {
		#[allow(deprecated)]
		Self {
			voter: delegation.voter_pubkey.to_string(),
			stake: delegation.stake.to_string(),
			activation_epoch: delegation.activation_epoch.to_string(),
			deactivation_epoch: delegation.deactivation_epoch.to_string(),
			warmup_cooldown_rate: delegation.warmup_cooldown_rate,
		}
	}
}
