use std::fmt;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum RewardType {
	Fee,
	Rent,
	Staking,
	Voting,
}

impl fmt::Display for RewardType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				RewardType::Fee => "fee",
				RewardType::Rent => "rent",
				RewardType::Staking => "staking",
				RewardType::Voting => "voting",
			}
		)
	}
}
