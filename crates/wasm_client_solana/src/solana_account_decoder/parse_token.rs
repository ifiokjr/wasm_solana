use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;
use serde_with::skip_serializing_none;
use serde_with::DisplayFromStr;
use solana_sdk::pubkey::Pubkey;
use spl_token_2022::extension::BaseStateWithExtensions;
use spl_token_2022::extension::StateWithExtensions;
use spl_token_2022::generic_token_account::GenericTokenAccount;
use spl_token_2022::solana_program::program_option::COption;
use spl_token_2022::solana_program::program_pack::Pack;
use spl_token_2022::state::Account;
use spl_token_2022::state::AccountState;
use spl_token_2022::state::Mint;
use spl_token_2022::state::Multisig;

use super::parse_account_data::ParsableAccount;
use super::parse_account_data::ParseAccountError;
use super::parse_account_data::SplTokenAdditionalData;
use super::parse_token_extension::parse_extension;
use super::parse_token_extension::UiExtension;
use super::StringAmount;
use super::StringDecimals;

// Returns all known SPL Token program ids
pub fn spl_token_ids() -> Vec<Pubkey> {
	vec![spl_token::id(), spl_token_2022::id()]
}

// Check if the provided program id as a known SPL Token program id
pub fn is_known_spl_token_id(program_id: &Pubkey) -> bool {
	*program_id == spl_token::id() || *program_id == spl_token_2022::id()
}

#[deprecated(since = "2.0.0", note = "Use `parse_token_v2` instead")]
pub fn parse_token(
	data: &[u8],
	decimals: Option<u8>,
) -> Result<TokenAccountType, ParseAccountError> {
	let additional_data = decimals.map(SplTokenAdditionalData::with_decimals);
	parse_token_v2(data, additional_data.as_ref())
}

pub fn parse_token_v2(
	data: &[u8],
	additional_data: Option<&SplTokenAdditionalData>,
) -> Result<TokenAccountType, ParseAccountError> {
	if let Ok(account) = StateWithExtensions::<Account>::unpack(data) {
		let additional_data = additional_data.as_ref().ok_or_else(|| {
			ParseAccountError::AdditionalDataMissing(
				"no mint_decimals provided to parse spl-token account".to_string(),
			)
		})?;
		let extension_types = account.get_extension_types().unwrap_or_default();
		let ui_extensions = extension_types
			.iter()
			.map(|extension_type| parse_extension::<Account>(extension_type, &account))
			.collect();
		return Ok(TokenAccountType::Account(UiTokenAccount {
			mint: account.base.mint,
			owner: account.base.owner,
			token_amount: token_amount_to_ui_amount_v2(account.base.amount, additional_data),
			delegate: match account.base.delegate {
				COption::Some(pubkey) => Some(pubkey),
				COption::None => None,
			},
			state: account.base.state.into(),
			is_native: account.base.is_native(),
			rent_exempt_reserve: match account.base.is_native {
				COption::Some(reserve) => {
					Some(token_amount_to_ui_amount_v2(reserve, additional_data))
				}
				COption::None => None,
			},
			delegated_amount: if account.base.delegate.is_none() {
				None
			} else {
				Some(token_amount_to_ui_amount_v2(
					account.base.delegated_amount,
					additional_data,
				))
			},
			close_authority: match account.base.close_authority {
				COption::Some(pubkey) => Some(pubkey),
				COption::None => None,
			},
			extensions: ui_extensions,
		}));
	}
	if let Ok(mint) = StateWithExtensions::<Mint>::unpack(data) {
		let extension_types = mint.get_extension_types().unwrap_or_default();
		let ui_extensions = extension_types
			.iter()
			.map(|extension_type| parse_extension::<Mint>(extension_type, &mint))
			.collect();
		return Ok(TokenAccountType::Mint(UiMint {
			mint_authority: match mint.base.mint_authority {
				COption::Some(pubkey) => Some(pubkey),
				COption::None => None,
			},
			supply: mint.base.supply.to_string(),
			decimals: mint.base.decimals,
			is_initialized: mint.base.is_initialized,
			freeze_authority: match mint.base.freeze_authority {
				COption::Some(pubkey) => Some(pubkey),
				COption::None => None,
			},
			extensions: ui_extensions,
		}));
	}
	if data.len() == Multisig::get_packed_len() {
		let multisig = Multisig::unpack(data)
			.map_err(|_| ParseAccountError::AccountNotParsable(ParsableAccount::SplToken))?;
		Ok(TokenAccountType::Multisig(UiMultisig {
			num_required_signers: multisig.m,
			num_valid_signers: multisig.n,
			is_initialized: multisig.is_initialized,
			signers: multisig
				.signers
				.iter()
				.filter_map(|pubkey| {
					if pubkey == &Pubkey::default() {
						None
					} else {
						Some(pubkey.to_string())
					}
				})
				.collect(),
		}))
	} else {
		Err(ParseAccountError::AccountNotParsable(
			ParsableAccount::SplToken,
		))
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase", tag = "type", content = "info")]
#[allow(clippy::large_enum_variant)]
pub enum TokenAccountType {
	Account(UiTokenAccount),
	Mint(UiMint),
	Multisig(UiMultisig),
}

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiTokenAccount {
	#[serde_as(as = "DisplayFromStr")]
	pub mint: Pubkey,
	#[serde_as(as = "DisplayFromStr")]
	pub owner: Pubkey,
	pub token_amount: UiTokenAmount,
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub delegate: Option<Pubkey>,
	pub state: UiAccountState,
	pub is_native: bool,
	pub rent_exempt_reserve: Option<UiTokenAmount>,
	pub delegated_amount: Option<UiTokenAmount>,
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub close_authority: Option<Pubkey>,
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub extensions: Vec<UiExtension>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum UiAccountState {
	Uninitialized,
	Initialized,
	Frozen,
}

impl From<AccountState> for UiAccountState {
	fn from(state: AccountState) -> Self {
		match state {
			AccountState::Uninitialized => UiAccountState::Uninitialized,
			AccountState::Initialized => UiAccountState::Initialized,
			AccountState::Frozen => UiAccountState::Frozen,
		}
	}
}

pub fn real_number_string(amount: u64, decimals: u8) -> StringDecimals {
	let decimals = decimals as usize;
	if decimals > 0 {
		// Left-pad zeros to decimals + 1, so we at least have an integer zero
		let mut s = format!("{:01$}", amount, decimals + 1);
		// Add the decimal point (Sorry, "," locales!)
		s.insert(s.len() - decimals, '.');
		s
	} else {
		amount.to_string()
	}
}

pub fn real_number_string_trimmed(amount: u64, decimals: u8) -> StringDecimals {
	let mut s = real_number_string(amount, decimals);
	if decimals > 0 {
		let zeros_trimmed = s.trim_end_matches('0');
		s = zeros_trimmed.trim_end_matches('.').to_string();
	}
	s
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UiTokenAmount {
	pub ui_amount: Option<f64>,
	pub decimals: u8,
	pub amount: StringAmount,
	pub ui_amount_string: StringDecimals,
}

impl Eq for UiTokenAmount {}

impl UiTokenAmount {
	pub fn real_number_string(&self) -> String {
		real_number_string(
			u64::from_str(&self.amount).unwrap_or_default(),
			self.decimals,
		)
	}

	pub fn real_number_string_trimmed(&self) -> String {
		if self.ui_amount_string.is_empty() {
			real_number_string_trimmed(
				u64::from_str(&self.amount).unwrap_or_default(),
				self.decimals,
			)
		} else {
			self.ui_amount_string.clone()
		}
	}
}

#[deprecated(since = "2.0.0", note = "Use `token_amount_to_ui_amount_v2` instead")]
pub fn token_amount_to_ui_amount(amount: u64, decimals: u8) -> UiTokenAmount {
	token_amount_to_ui_amount_v2(amount, &SplTokenAdditionalData::with_decimals(decimals))
}

pub fn token_amount_to_ui_amount_v2(
	amount: u64,
	additional_data: &SplTokenAdditionalData,
) -> UiTokenAmount {
	let decimals = additional_data.decimals;
	let (ui_amount, ui_amount_string) = if let Some((interest_bearing_config, unix_timestamp)) =
		additional_data.interest_bearing_config
	{
		let ui_amount_string =
			interest_bearing_config.amount_to_ui_amount(amount, decimals, unix_timestamp);
		(
			ui_amount_string
				.as_ref()
				.and_then(|x| f64::from_str(x).ok()),
			ui_amount_string.unwrap_or(String::new()),
		)
	} else {
		let ui_amount = 10_usize
			.checked_pow(u32::from(decimals))
			.map(|dividend| amount as f64 / dividend as f64);
		(ui_amount, real_number_string_trimmed(amount, decimals))
	};
	UiTokenAmount {
		ui_amount,
		decimals,
		amount: amount.to_string(),
		ui_amount_string,
	}
}

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiMint {
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub mint_authority: Option<Pubkey>,
	pub supply: StringAmount,
	pub decimals: u8,
	pub is_initialized: bool,
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub freeze_authority: Option<Pubkey>,
	#[serde(skip_serializing_if = "Vec::is_empty", default)]
	pub extensions: Vec<UiExtension>,
}

impl From<Mint> for UiMint {
	fn from(value: Mint) -> Self {
		Self {
			mint_authority: value.mint_authority.into(),
			supply: format!("{}", value.supply),
			decimals: value.decimals,
			is_initialized: value.is_initialized,
			freeze_authority: value.freeze_authority.into(),
			extensions: vec![],
		}
	}
}

impl From<&Mint> for UiMint {
	fn from(value: &Mint) -> Self {
		(*value).into()
	}
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiMultisig {
	pub num_required_signers: u8,
	pub num_valid_signers: u8,
	pub is_initialized: bool,
	pub signers: Vec<String>,
}

pub fn get_token_account_mint(data: &[u8]) -> Option<Pubkey> {
	Account::valid_account_data(data)
		.then(|| Pubkey::try_from(data.get(..32)?).ok())
		.flatten()
}

#[cfg(test)]
mod test {
	use spl_pod::optional_keys::OptionalNonZeroPubkey;
	use spl_token_2022::extension::immutable_owner::ImmutableOwner;
	use spl_token_2022::extension::interest_bearing_mint::InterestBearingConfig;
	use spl_token_2022::extension::memo_transfer::MemoTransfer;
	use spl_token_2022::extension::mint_close_authority::MintCloseAuthority;
	use spl_token_2022::extension::BaseStateWithExtensionsMut;
	use spl_token_2022::extension::ExtensionType;
	use spl_token_2022::extension::StateWithExtensionsMut;

	use super::*;
	use crate::solana_account_decoder::parse_token_extension::UiMemoTransfer;
	use crate::solana_account_decoder::parse_token_extension::UiMintCloseAuthority;

	const INT_SECONDS_PER_YEAR: i64 = 6 * 6 * 24 * 36524;

	#[test]
	fn test_parse_token() {
		let mint_pubkey = Pubkey::new_from_array([2; 32]);
		let owner_pubkey = Pubkey::new_from_array([3; 32]);
		let mut account_data = vec![0; Account::get_packed_len()];
		let mut account = Account::unpack_unchecked(&account_data).unwrap();
		account.mint = mint_pubkey;
		account.owner = owner_pubkey;
		account.amount = 42;
		account.state = AccountState::Initialized;
		account.is_native = COption::None;
		account.close_authority = COption::Some(owner_pubkey);
		Account::pack(account, &mut account_data).unwrap();

		assert!(parse_token_v2(&account_data, None).is_err());
		assert_eq!(
			parse_token_v2(
				&account_data,
				Some(&SplTokenAdditionalData::with_decimals(2))
			)
			.unwrap(),
			TokenAccountType::Account(UiTokenAccount {
				mint: mint_pubkey,
				owner: owner_pubkey,
				token_amount: UiTokenAmount {
					ui_amount: Some(0.42),
					decimals: 2,
					amount: "42".to_string(),
					ui_amount_string: "0.42".to_string()
				},
				delegate: None,
				state: UiAccountState::Initialized,
				is_native: false,
				rent_exempt_reserve: None,
				delegated_amount: None,
				close_authority: Some(owner_pubkey),
				extensions: vec![],
			}),
		);

		let mut mint_data = vec![0; Mint::get_packed_len()];
		let mut mint = Mint::unpack_unchecked(&mint_data).unwrap();
		mint.mint_authority = COption::Some(owner_pubkey);
		mint.supply = 42;
		mint.decimals = 3;
		mint.is_initialized = true;
		mint.freeze_authority = COption::Some(owner_pubkey);
		Mint::pack(mint, &mut mint_data).unwrap();

		assert_eq!(
			parse_token_v2(&mint_data, None).unwrap(),
			TokenAccountType::Mint(UiMint {
				mint_authority: Some(owner_pubkey),
				supply: 42.to_string(),
				decimals: 3,
				is_initialized: true,
				freeze_authority: Some(owner_pubkey),
				extensions: vec![],
			}),
		);

		let signer1 = Pubkey::new_from_array([1; 32]);
		let signer2 = Pubkey::new_from_array([2; 32]);
		let signer3 = Pubkey::new_from_array([3; 32]);
		let mut multisig_data = vec![0; Multisig::get_packed_len()];
		let mut signer_pubkeys = [Pubkey::default(); 11];
		signer_pubkeys[0] = signer1;
		signer_pubkeys[1] = signer2;
		signer_pubkeys[2] = signer3;
		let mut multisig = Multisig::unpack_unchecked(&multisig_data).unwrap();
		multisig.m = 2;
		multisig.n = 3;
		multisig.is_initialized = true;
		multisig.signers = signer_pubkeys;
		Multisig::pack(multisig, &mut multisig_data).unwrap();

		assert_eq!(
			parse_token_v2(&multisig_data, None).unwrap(),
			TokenAccountType::Multisig(UiMultisig {
				num_required_signers: 2,
				num_valid_signers: 3,
				is_initialized: true,
				signers: vec![
					signer1.to_string(),
					signer2.to_string(),
					signer3.to_string()
				],
			}),
		);

		let bad_data = vec![0; 4];
		assert!(parse_token_v2(&bad_data, None).is_err());
	}

	#[test]
	fn test_get_token_account_mint() {
		let mint_pubkey = Pubkey::new_from_array([2; 32]);
		let mut account_data = vec![0; Account::get_packed_len()];
		let mut account = Account::unpack_unchecked(&account_data).unwrap();
		account.mint = mint_pubkey;
		account.state = AccountState::Initialized;
		Account::pack(account, &mut account_data).unwrap();

		let expected_mint_pubkey = Pubkey::from([2; 32]);
		assert_eq!(
			get_token_account_mint(&account_data),
			Some(expected_mint_pubkey)
		);
	}

	#[test]
	fn test_ui_token_amount_real_string() {
		assert_eq!(&real_number_string(1, 0), "1");
		assert_eq!(&real_number_string_trimmed(1, 0), "1");
		let token_amount =
			token_amount_to_ui_amount_v2(1, &SplTokenAdditionalData::with_decimals(0));
		assert_eq!(
			token_amount.ui_amount_string,
			real_number_string_trimmed(1, 0)
		);
		assert_eq!(token_amount.ui_amount, Some(1.0));
		assert_eq!(&real_number_string(10, 0), "10");
		assert_eq!(&real_number_string_trimmed(10, 0), "10");
		let token_amount =
			token_amount_to_ui_amount_v2(10, &SplTokenAdditionalData::with_decimals(0));
		assert_eq!(
			token_amount.ui_amount_string,
			real_number_string_trimmed(10, 0)
		);
		assert_eq!(token_amount.ui_amount, Some(10.0));
		assert_eq!(&real_number_string(1, 9), "0.000000001");
		assert_eq!(&real_number_string_trimmed(1, 9), "0.000000001");
		let token_amount =
			token_amount_to_ui_amount_v2(1, &SplTokenAdditionalData::with_decimals(9));
		assert_eq!(
			token_amount.ui_amount_string,
			real_number_string_trimmed(1, 9)
		);
		assert_eq!(token_amount.ui_amount, Some(0.000_000_001));
		assert_eq!(&real_number_string(1_000_000_000, 9), "1.000000000");
		assert_eq!(&real_number_string_trimmed(1_000_000_000, 9), "1");
		let token_amount =
			token_amount_to_ui_amount_v2(1_000_000_000, &SplTokenAdditionalData::with_decimals(9));
		assert_eq!(
			token_amount.ui_amount_string,
			real_number_string_trimmed(1_000_000_000, 9)
		);
		assert_eq!(token_amount.ui_amount, Some(1.0));
		assert_eq!(&real_number_string(1_234_567_890, 3), "1234567.890");
		assert_eq!(&real_number_string_trimmed(1_234_567_890, 3), "1234567.89");
		let token_amount =
			token_amount_to_ui_amount_v2(1_234_567_890, &SplTokenAdditionalData::with_decimals(3));
		assert_eq!(
			token_amount.ui_amount_string,
			real_number_string_trimmed(1_234_567_890, 3)
		);
		assert_eq!(token_amount.ui_amount, Some(1_234_567.89));
		assert_eq!(
			&real_number_string(1_234_567_890, 25),
			"0.0000000000000001234567890"
		);
		assert_eq!(
			&real_number_string_trimmed(1_234_567_890, 25),
			"0.000000000000000123456789"
		);
		let token_amount =
			token_amount_to_ui_amount_v2(1_234_567_890, &SplTokenAdditionalData::with_decimals(20));
		assert_eq!(
			token_amount.ui_amount_string,
			real_number_string_trimmed(1_234_567_890, 20)
		);
		assert_eq!(token_amount.ui_amount, None);
	}

	#[test]
	fn test_ui_token_amount_with_interest() {
		// constant 5%
		let config = InterestBearingConfig {
			initialization_timestamp: 0.into(),
			pre_update_average_rate: 500.into(),
			last_update_timestamp: INT_SECONDS_PER_YEAR.into(),
			current_rate: 500.into(),
			..Default::default()
		};
		let additional_data = SplTokenAdditionalData {
			decimals: 0,
			interest_bearing_config: Some((config, INT_SECONDS_PER_YEAR)),
		};
		let token_amount = token_amount_to_ui_amount_v2(1, &additional_data);
		assert_eq!(token_amount.ui_amount_string, "1.0512710963760241");
		assert!(
			(token_amount.ui_amount.unwrap() - 1.051_271_096_376_024_1_f64).abs() < f64::EPSILON
		);
		let token_amount = token_amount_to_ui_amount_v2(10, &additional_data);
		assert_eq!(token_amount.ui_amount_string, "10.512710963760242");
		assert!(
			(token_amount.ui_amount.unwrap() - 10.512_710_963_760_241_f64).abs() < f64::EPSILON
		);

		// huge case
		let config = InterestBearingConfig {
			initialization_timestamp: 0.into(),
			pre_update_average_rate: 32767.into(),
			last_update_timestamp: 0.into(),
			current_rate: 32767.into(),
			..Default::default()
		};
		let additional_data = SplTokenAdditionalData {
			decimals: 0,
			interest_bearing_config: Some((config, INT_SECONDS_PER_YEAR * 1_000)),
		};
		let token_amount = token_amount_to_ui_amount_v2(u64::MAX, &additional_data);
		assert_eq!(token_amount.ui_amount, Some(f64::INFINITY));
		assert_eq!(token_amount.ui_amount_string, "inf");
	}

	#[test]
	fn test_ui_token_amount_real_string_zero() {
		assert_eq!(&real_number_string(0, 0), "0");
		assert_eq!(&real_number_string_trimmed(0, 0), "0");
		let token_amount =
			token_amount_to_ui_amount_v2(0, &SplTokenAdditionalData::with_decimals(0));
		assert_eq!(
			token_amount.ui_amount_string,
			real_number_string_trimmed(0, 0)
		);
		assert_eq!(token_amount.ui_amount, Some(0.0));
		assert_eq!(&real_number_string(0, 9), "0.000000000");
		assert_eq!(&real_number_string_trimmed(0, 9), "0");
		let token_amount =
			token_amount_to_ui_amount_v2(0, &SplTokenAdditionalData::with_decimals(9));
		assert_eq!(
			token_amount.ui_amount_string,
			real_number_string_trimmed(0, 9)
		);
		assert_eq!(token_amount.ui_amount, Some(0.0));
		assert_eq!(&real_number_string(0, 25), "0.0000000000000000000000000");
		assert_eq!(&real_number_string_trimmed(0, 25), "0");
		let token_amount =
			token_amount_to_ui_amount_v2(0, &SplTokenAdditionalData::with_decimals(20));
		assert_eq!(
			token_amount.ui_amount_string,
			real_number_string_trimmed(0, 20)
		);
		assert_eq!(token_amount.ui_amount, None);
	}

	#[test]
	fn test_parse_token_account_with_extensions() {
		let mint_pubkey = Pubkey::new_from_array([2; 32]);
		let owner_pubkey = Pubkey::new_from_array([3; 32]);

		let account_base = Account {
			mint: mint_pubkey,
			owner: owner_pubkey,
			amount: 42,
			state: AccountState::Initialized,
			is_native: COption::None,
			close_authority: COption::Some(owner_pubkey),
			delegate: COption::None,
			delegated_amount: 0,
		};
		let account_size = ExtensionType::try_calculate_account_len::<Account>(&[
			ExtensionType::ImmutableOwner,
			ExtensionType::MemoTransfer,
		])
		.unwrap();
		let mut account_data = vec![0; account_size];
		let mut account_state =
			StateWithExtensionsMut::<Account>::unpack_uninitialized(&mut account_data).unwrap();

		account_state.base = account_base;
		account_state.pack_base();
		account_state.init_account_type().unwrap();

		assert!(parse_token_v2(&account_data, None).is_err());
		assert_eq!(
			parse_token_v2(
				&account_data,
				Some(&SplTokenAdditionalData::with_decimals(2))
			)
			.unwrap(),
			TokenAccountType::Account(UiTokenAccount {
				mint: mint_pubkey,
				owner: owner_pubkey,
				token_amount: UiTokenAmount {
					ui_amount: Some(0.42),
					decimals: 2,
					amount: "42".to_string(),
					ui_amount_string: "0.42".to_string()
				},
				delegate: None,
				state: UiAccountState::Initialized,
				is_native: false,
				rent_exempt_reserve: None,
				delegated_amount: None,
				close_authority: Some(owner_pubkey),
				extensions: vec![],
			}),
		);

		let mut account_data = vec![0; account_size];
		let mut account_state =
			StateWithExtensionsMut::<Account>::unpack_uninitialized(&mut account_data).unwrap();

		account_state.base = account_base;
		account_state.pack_base();
		account_state.init_account_type().unwrap();

		account_state
			.init_extension::<ImmutableOwner>(true)
			.unwrap();
		let memo_transfer = account_state.init_extension::<MemoTransfer>(true).unwrap();
		memo_transfer.require_incoming_transfer_memos = true.into();

		assert!(parse_token_v2(&account_data, None).is_err());
		assert_eq!(
			parse_token_v2(
				&account_data,
				Some(&SplTokenAdditionalData::with_decimals(2))
			)
			.unwrap(),
			TokenAccountType::Account(UiTokenAccount {
				mint: mint_pubkey,
				owner: owner_pubkey,
				token_amount: UiTokenAmount {
					ui_amount: Some(0.42),
					decimals: 2,
					amount: "42".to_string(),
					ui_amount_string: "0.42".to_string()
				},
				delegate: None,
				state: UiAccountState::Initialized,
				is_native: false,
				rent_exempt_reserve: None,
				delegated_amount: None,
				close_authority: Some(owner_pubkey),
				extensions: vec![
					UiExtension::ImmutableOwner,
					UiExtension::MemoTransfer(UiMemoTransfer {
						require_incoming_transfer_memos: true,
					}),
				],
			}),
		);
	}

	#[test]
	fn test_parse_token_mint_with_extensions() {
		let owner_pubkey = Pubkey::new_from_array([3; 32]);
		let mint_size =
			ExtensionType::try_calculate_account_len::<Mint>(&[ExtensionType::MintCloseAuthority])
				.unwrap();
		let mint_base = Mint {
			mint_authority: COption::Some(owner_pubkey),
			supply: 42,
			decimals: 3,
			is_initialized: true,
			freeze_authority: COption::Some(owner_pubkey),
		};
		let mut mint_data = vec![0; mint_size];
		let mut mint_state =
			StateWithExtensionsMut::<Mint>::unpack_uninitialized(&mut mint_data).unwrap();

		mint_state.base = mint_base;
		mint_state.pack_base();
		mint_state.init_account_type().unwrap();

		assert_eq!(
			parse_token_v2(&mint_data, None).unwrap(),
			TokenAccountType::Mint(UiMint {
				mint_authority: Some(owner_pubkey),
				supply: 42.to_string(),
				decimals: 3,
				is_initialized: true,
				freeze_authority: Some(owner_pubkey),
				extensions: vec![],
			}),
		);

		let mut mint_data = vec![0; mint_size];
		let mut mint_state =
			StateWithExtensionsMut::<Mint>::unpack_uninitialized(&mut mint_data).unwrap();

		let mint_close_authority = mint_state
			.init_extension::<MintCloseAuthority>(true)
			.unwrap();
		mint_close_authority.close_authority =
			OptionalNonZeroPubkey::try_from(Some(owner_pubkey)).unwrap();
		mint_state.base = mint_base;
		mint_state.pack_base();
		mint_state.init_account_type().unwrap();

		assert_eq!(
			parse_token_v2(&mint_data, None).unwrap(),
			TokenAccountType::Mint(UiMint {
				mint_authority: Some(owner_pubkey),
				supply: 42.to_string(),
				decimals: 3,
				is_initialized: true,
				freeze_authority: Some(owner_pubkey),
				extensions: vec![UiExtension::MintCloseAuthority(UiMintCloseAuthority {
					close_authority: Some(owner_pubkey),
				})],
			}),
		);
	}
}
