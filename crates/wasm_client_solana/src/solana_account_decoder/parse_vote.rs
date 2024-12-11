use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use solana_sdk::clock::Epoch;
use solana_sdk::clock::Slot;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::vote::state::BlockTimestamp;
use solana_sdk::vote::state::Lockout;
use solana_sdk::vote::state::VoteState;

use super::parse_account_data::ParseAccountError;
use super::StringAmount;

pub fn parse_vote(data: &[u8]) -> Result<VoteAccountType, ParseAccountError> {
	let mut vote_state = VoteState::deserialize(data).map_err(ParseAccountError::from)?;
	let epoch_credits = vote_state
		.epoch_credits()
		.iter()
		.map(|(epoch, credits, previous_credits)| {
			UiEpochCredits {
				epoch: *epoch,
				credits: credits.to_string(),
				previous_credits: previous_credits.to_string(),
			}
		})
		.collect();
	let votes = vote_state
		.votes
		.iter()
		.map(|lockout| {
			UiLockout {
				slot: lockout.slot(),
				confirmation_count: lockout.confirmation_count(),
			}
		})
		.collect();
	let authorized_voters = vote_state
		.authorized_voters()
		.iter()
		.map(|(epoch, authorized_voter)| {
			UiAuthorizedVoters {
				epoch: *epoch,
				authorized_voter: *authorized_voter,
			}
		})
		.collect();
	let prior_voters = vote_state
		.prior_voters()
		.buf()
		.iter()
		.filter(|(pubkey, ..)| pubkey != &Pubkey::default())
		.map(
			|(authorized_pubkey, epoch_of_last_authorized_switch, target_epoch)| {
				UiPriorVoters {
					authorized_pubkey: *authorized_pubkey,
					epoch_of_last_authorized_switch: *epoch_of_last_authorized_switch,
					target_epoch: *target_epoch,
				}
			},
		)
		.collect();
	Ok(VoteAccountType::Vote(UiVoteState {
		node_pubkey: vote_state.node_pubkey,
		authorized_withdrawer: vote_state.authorized_withdrawer,
		commission: vote_state.commission,
		votes,
		root_slot: vote_state.root_slot,
		authorized_voters,
		prior_voters,
		epoch_credits,
		last_timestamp: vote_state.last_timestamp,
	}))
}

/// A wrapper enum for consistency across programs
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", tag = "type", content = "info")]
pub enum VoteAccountType {
	Vote(UiVoteState),
}

/// A duplicate representation of `VoteState` for pretty JSON serialization
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiVoteState {
	#[serde_as(as = "DisplayFromStr")]
	node_pubkey: Pubkey,
	#[serde_as(as = "DisplayFromStr")]
	authorized_withdrawer: Pubkey,
	commission: u8,
	votes: Vec<UiLockout>,
	root_slot: Option<Slot>,
	authorized_voters: Vec<UiAuthorizedVoters>,
	prior_voters: Vec<UiPriorVoters>,
	epoch_credits: Vec<UiEpochCredits>,
	last_timestamp: BlockTimestamp,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct UiLockout {
	slot: Slot,
	confirmation_count: u32,
}

impl From<&Lockout> for UiLockout {
	fn from(lockout: &Lockout) -> Self {
		Self {
			slot: lockout.slot(),
			confirmation_count: lockout.confirmation_count(),
		}
	}
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct UiAuthorizedVoters {
	epoch: Epoch,
	#[serde_as(as = "DisplayFromStr")]
	authorized_voter: Pubkey,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct UiPriorVoters {
	#[serde_as(as = "DisplayFromStr")]
	authorized_pubkey: Pubkey,
	epoch_of_last_authorized_switch: Epoch,
	target_epoch: Epoch,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct UiEpochCredits {
	epoch: Epoch,
	credits: StringAmount,
	previous_credits: StringAmount,
}

#[cfg(test)]
mod test {
	use solana_sdk::vote::state::VoteStateVersions;

	use super::*;

	#[test]
	fn test_parse_vote() {
		let vote_state = VoteState::default();
		let mut vote_account_data: Vec<u8> = vec![0; VoteState::size_of()];
		let versioned = VoteStateVersions::new_current(vote_state);
		VoteState::serialize(&versioned, &mut vote_account_data).unwrap();
		let expected_vote_state = UiVoteState {
			node_pubkey: Pubkey::default(),
			authorized_withdrawer: Pubkey::default(),
			..UiVoteState::default()
		};
		assert_eq!(
			parse_vote(&vote_account_data).unwrap(),
			VoteAccountType::Vote(expected_vote_state)
		);

		let bad_data = vec![0; 4];
		assert!(parse_vote(&bad_data).is_err());
	}
}
