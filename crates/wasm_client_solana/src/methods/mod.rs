pub use account_info::*;
pub use balance::*;
pub use block::*;
pub use block_commitment::*;
pub use block_height::*;
pub use block_production::*;
pub use block_time::*;
pub use blockhash_valid::*;
pub use blocks::*;
pub use blocks_with_limit::*;
pub use cluster_nodes::*;
pub use epoch_info::*;
pub use epoch_schedule::*;
pub use fee_for_message::*;
pub use first_available_block::*;
pub use genesis_hash::*;
pub use health::*;
pub use highest_snapshot_slot::*;
pub use identity::*;
pub use inflation_governor::*;
pub use inflation_rate::*;
pub use inflation_reward::*;
pub use largest_accounts::*;
pub use latest_blockhash::*;
pub use leader_schedule::*;
pub use max_retransmit_slot::*;
pub use minimum_balance_for_rent_exemption::*;
pub use minimum_ledger_slot::*;
pub use multiple_accounts::*;
pub use program_accounts::*;
pub use recent_performance_samples::*;
pub use request_airdrop::*;
pub use send_transaction::*;
use serde::Deserialize;
use serde::Serialize;
// pub use serde_utils::*;
pub use signature_statuses::*;
pub use signatures_for_address::*;
pub use simulate_transaction::*;
pub use slot::*;
pub use slot_leader::*;
pub use slot_leaders::*;
pub use stake_activation::*;
pub use supply::*;
pub use token_account_balance::*;
pub use token_accounts_by_delegate::*;
pub use token_accounts_by_owner::*;
pub use token_largest_accounts::*;
pub use token_supply::*;
pub use transaction::*;
pub use transaction_count::*;
pub use version::*;
pub use vote_accounts::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
	slot: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockProductionRange {
	pub first_slot: u64,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_slot: Option<u64>,
}

mod account_info;
mod balance;
mod block;
mod block_commitment;
mod block_height;
mod block_production;
mod block_time;
mod blockhash_valid;
mod blocks;
mod blocks_with_limit;
mod cluster_nodes;
mod epoch_info;
mod epoch_schedule;
mod fee_for_message;
mod first_available_block;
mod genesis_hash;
mod health;
mod highest_snapshot_slot;
mod identity;
mod inflation_governor;
mod inflation_rate;
mod inflation_reward;
mod largest_accounts;
mod latest_blockhash;
mod leader_schedule;
mod max_retransmit_slot;
mod minimum_balance_for_rent_exemption;
mod minimum_ledger_slot;
mod multiple_accounts;
mod program_accounts;
mod recent_performance_samples;
mod request_airdrop;
mod send_transaction;
pub mod serde_utils;
mod signature_statuses;
mod signatures_for_address;
mod simulate_transaction;
mod slot;
mod slot_leader;
mod slot_leaders;
mod stake_activation;
mod supply;
mod token_account_balance;
mod token_accounts_by_delegate;
mod token_accounts_by_owner;
mod token_largest_accounts;
mod token_supply;
mod transaction;
mod transaction_count;
mod version;
mod vote_accounts;
