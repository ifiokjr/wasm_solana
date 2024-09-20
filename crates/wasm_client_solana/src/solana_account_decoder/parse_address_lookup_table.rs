use serde::Deserialize;
use serde::Serialize;
use serde_with::DisplayFromStr;
use serde_with::serde_as;
use serde_with::skip_serializing_none;
use solana_sdk::address_lookup_table::AddressLookupTableAccount;
use solana_sdk::address_lookup_table::state::AddressLookupTable;
use solana_sdk::instruction::InstructionError;
use solana_sdk::pubkey::Pubkey;

use super::parse_account_data::ParsableAccount;
use super::parse_account_data::ParseAccountError;

pub fn parse_address_lookup_table(
	data: &[u8],
) -> Result<LookupTableAccountType, ParseAccountError> {
	AddressLookupTable::deserialize(data)
		.map(|address_lookup_table| {
			LookupTableAccountType::LookupTable(address_lookup_table.into())
		})
		.or_else(|err| {
			match err {
				InstructionError::UninitializedAccount => Ok(LookupTableAccountType::Uninitialized),
				_ => {
					Err(ParseAccountError::AccountNotParsable(
						ParsableAccount::AddressLookupTable,
					))
				}
			}
		})
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", tag = "type", content = "info")]
pub enum LookupTableAccountType {
	Uninitialized,
	LookupTable(UiLookupTable),
}

impl LookupTableAccountType {
	pub fn optional_address_lookup_table_account(
		&self,
		pubkey: &Pubkey,
	) -> Option<AddressLookupTableAccount> {
		match self {
			LookupTableAccountType::Uninitialized => None,
			LookupTableAccountType::LookupTable(table) => {
				Some(AddressLookupTableAccount {
					key: *pubkey,
					addresses: table.addresses.clone(),
				})
			}
		}
	}
}

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiLookupTable {
	pub deactivation_slot: u64,
	pub last_extended_slot: u64,
	pub last_extended_slot_start_index: u8,
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub authority: Option<Pubkey>,
	#[serde_as(as = "Vec<DisplayFromStr>")]
	pub addresses: Vec<Pubkey>,
}

impl From<AddressLookupTable<'_>> for UiLookupTable {
	fn from(address_lookup_table: AddressLookupTable) -> Self {
		Self {
			deactivation_slot: address_lookup_table.meta.deactivation_slot,
			last_extended_slot: address_lookup_table.meta.last_extended_slot,
			last_extended_slot_start_index: address_lookup_table
				.meta
				.last_extended_slot_start_index,
			authority: address_lookup_table.meta.authority,
			addresses: address_lookup_table.addresses.iter().copied().collect(),
		}
	}
}

#[cfg(test)]
mod test {
	use std::borrow::Cow;

	use solana_sdk::address_lookup_table::state::LOOKUP_TABLE_META_SIZE;
	use solana_sdk::address_lookup_table::state::LookupTableMeta;
	use solana_sdk::pubkey::Pubkey;

	use super::*;

	#[test]
	fn test_parse_address_lookup_table() {
		let authority = Pubkey::new_unique();
		let deactivation_slot = 1;
		let last_extended_slot = 2;
		let last_extended_slot_start_index = 3;
		let lookup_table_meta = LookupTableMeta {
			deactivation_slot,
			last_extended_slot,
			last_extended_slot_start_index,
			authority: Some(authority),
			..LookupTableMeta::default()
		};
		let num_addresses = 42;
		let mut addresses = Vec::with_capacity(num_addresses);
		addresses.resize_with(num_addresses, Pubkey::new_unique);
		let lookup_table = AddressLookupTable {
			meta: lookup_table_meta,
			addresses: Cow::Owned(addresses),
		};
		let lookup_table_data = AddressLookupTable::serialize_for_tests(lookup_table).unwrap();

		let parsing_result = parse_address_lookup_table(&lookup_table_data).unwrap();
		if let LookupTableAccountType::LookupTable(ui_lookup_table) = parsing_result {
			assert_eq!(ui_lookup_table.deactivation_slot, deactivation_slot);
			assert_eq!(ui_lookup_table.last_extended_slot, last_extended_slot);
			assert_eq!(
				ui_lookup_table.last_extended_slot_start_index,
				last_extended_slot_start_index
			);
			assert_eq!(ui_lookup_table.authority, Some(authority));
			assert_eq!(ui_lookup_table.addresses.len(), num_addresses);
		}

		assert_eq!(
			parse_address_lookup_table(&[0u8; LOOKUP_TABLE_META_SIZE]).unwrap(),
			LookupTableAccountType::Uninitialized
		);
		assert!(parse_address_lookup_table(&[]).is_err());
	}
}
