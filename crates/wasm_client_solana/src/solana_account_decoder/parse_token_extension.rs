use serde::Deserialize;
use serde::Serialize;
use serde_with::DisplayFromStr;
use serde_with::serde_as;
use serde_with::skip_serializing_none;
use solana_program::pubkey::Pubkey;
use solana_sdk::clock::UnixTimestamp;
use solana_sdk::program_pack::Pack;
use spl_token_2022::extension::BaseState;
use spl_token_2022::extension::BaseStateWithExtensions;
use spl_token_2022::extension::ExtensionType;
use spl_token_2022::extension::StateWithExtensions;
use spl_token_2022::extension::{self};
use spl_token_group_interface::state::TokenGroup;
use spl_token_group_interface::state::TokenGroupMember;
use spl_token_metadata_interface::state::TokenMetadata;

use super::parse_token::UiAccountState;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", tag = "extension", content = "state")]
pub enum UiExtension {
	Uninitialized,
	TransferFeeConfig(UiTransferFeeConfig),
	TransferFeeAmount(UiTransferFeeAmount),
	MintCloseAuthority(UiMintCloseAuthority),
	ConfidentialTransferMint(UiConfidentialTransferMint),
	ConfidentialTransferAccount(UiConfidentialTransferAccount),
	DefaultAccountState(UiDefaultAccountState),
	ImmutableOwner,
	MemoTransfer(UiMemoTransfer),
	NonTransferable,
	InterestBearingConfig(UiInterestBearingConfig),
	CpiGuard(UiCpiGuard),
	PermanentDelegate(UiPermanentDelegate),
	NonTransferableAccount,
	ConfidentialMintBurn(UiConfidentialMintBurn),
	ConfidentialTransferFeeConfig(UiConfidentialTransferFeeConfig),
	ConfidentialTransferFeeAmount(UiConfidentialTransferFeeAmount),
	TransferHook(UiTransferHook),
	TransferHookAccount(UiTransferHookAccount),
	MetadataPointer(UiMetadataPointer),
	TokenMetadata(UiTokenMetadata),
	GroupPointer(UiGroupPointer),
	GroupMemberPointer(UiGroupMemberPointer),
	TokenGroup(UiTokenGroup),
	TokenGroupMember(UiTokenGroupMember),
	UnparseableExtension,
	ScaledUiAmountConfig(UiScaledUiAmountConfig),
	PausableConfig(UiPausableConfig),
	PausableAccount,
}

pub fn parse_extension<S: BaseState + Pack>(
	extension_type: &ExtensionType,
	account: &StateWithExtensions<S>,
) -> UiExtension {
	match extension_type {
		ExtensionType::Uninitialized => UiExtension::Uninitialized,
		ExtensionType::TransferFeeConfig => {
			account
				.get_extension::<extension::transfer_fee::TransferFeeConfig>()
				.map(|&extension| UiExtension::TransferFeeConfig(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::TransferFeeAmount => {
			account
				.get_extension::<extension::transfer_fee::TransferFeeAmount>()
				.map(|&extension| UiExtension::TransferFeeAmount(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::MintCloseAuthority => {
			account
				.get_extension::<extension::mint_close_authority::MintCloseAuthority>()
				.map(|&extension| UiExtension::MintCloseAuthority(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::ConfidentialTransferMint => {
			account
				.get_extension::<extension::confidential_transfer::ConfidentialTransferMint>()
				.map(|&extension| UiExtension::ConfidentialTransferMint(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::ConfidentialTransferFeeConfig => {
			account
				.get_extension::<extension::confidential_transfer_fee::ConfidentialTransferFeeConfig>(
				)
				.map(|&extension| UiExtension::ConfidentialTransferFeeConfig(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::ConfidentialTransferAccount => {
			account
				.get_extension::<extension::confidential_transfer::ConfidentialTransferAccount>()
				.map(|&extension| UiExtension::ConfidentialTransferAccount(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::ConfidentialTransferFeeAmount => {
			account
				.get_extension::<extension::confidential_transfer_fee::ConfidentialTransferFeeAmount>(
				)
				.map(|&extension| UiExtension::ConfidentialTransferFeeAmount(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::DefaultAccountState => {
			account
				.get_extension::<extension::default_account_state::DefaultAccountState>()
				.map(|&extension| UiExtension::DefaultAccountState(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::ImmutableOwner => UiExtension::ImmutableOwner,
		ExtensionType::MemoTransfer => {
			account
				.get_extension::<extension::memo_transfer::MemoTransfer>()
				.map(|&extension| UiExtension::MemoTransfer(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::NonTransferable => UiExtension::NonTransferable,
		ExtensionType::InterestBearingConfig => {
			account
				.get_extension::<extension::interest_bearing_mint::InterestBearingConfig>()
				.map(|&extension| UiExtension::InterestBearingConfig(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::CpiGuard => {
			account
				.get_extension::<extension::cpi_guard::CpiGuard>()
				.map(|&extension| UiExtension::CpiGuard(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::PermanentDelegate => {
			account
				.get_extension::<extension::permanent_delegate::PermanentDelegate>()
				.map(|&extension| UiExtension::PermanentDelegate(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::NonTransferableAccount => UiExtension::NonTransferableAccount,
		ExtensionType::MetadataPointer => {
			account
				.get_extension::<extension::metadata_pointer::MetadataPointer>()
				.map(|&extension| UiExtension::MetadataPointer(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::TokenMetadata => {
			account
				.get_variable_len_extension::<TokenMetadata>()
				.map(|extension| UiExtension::TokenMetadata(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::TransferHook => {
			account
				.get_extension::<extension::transfer_hook::TransferHook>()
				.map(|&extension| UiExtension::TransferHook(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::TransferHookAccount => {
			account
				.get_extension::<extension::transfer_hook::TransferHookAccount>()
				.map(|&extension| UiExtension::TransferHookAccount(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::GroupPointer => {
			account
				.get_extension::<extension::group_pointer::GroupPointer>()
				.map(|&extension| UiExtension::GroupPointer(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::GroupMemberPointer => {
			account
				.get_extension::<extension::group_member_pointer::GroupMemberPointer>()
				.map(|&extension| UiExtension::GroupMemberPointer(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::TokenGroup => {
			account
				.get_extension::<TokenGroup>()
				.map(|&extension| UiExtension::TokenGroup(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::TokenGroupMember => {
			account
				.get_extension::<TokenGroupMember>()
				.map(|&extension| UiExtension::TokenGroupMember(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::ConfidentialMintBurn => {
			account
				.get_extension::<extension::confidential_mint_burn::ConfidentialMintBurn>()
				.map(|&extension| UiExtension::ConfidentialMintBurn(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::ScaledUiAmount => {
			account
				.get_extension::<extension::scaled_ui_amount::ScaledUiAmountConfig>()
				.map(|&extension| UiExtension::ScaledUiAmountConfig(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::Pausable => {
			account
				.get_extension::<extension::pausable::PausableConfig>()
				.map(|&extension| UiExtension::PausableConfig(extension.into()))
				.unwrap_or(UiExtension::UnparseableExtension)
		}
		ExtensionType::PausableAccount => UiExtension::PausableAccount,
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiTransferFee {
	pub epoch: u64,
	pub maximum_fee: u64,
	pub transfer_fee_basis_points: u16,
}

impl From<extension::transfer_fee::TransferFee> for UiTransferFee {
	fn from(transfer_fee: extension::transfer_fee::TransferFee) -> Self {
		Self {
			epoch: u64::from(transfer_fee.epoch),
			maximum_fee: u64::from(transfer_fee.maximum_fee),
			transfer_fee_basis_points: u16::from(transfer_fee.transfer_fee_basis_points),
		}
	}
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiTransferFeeConfig {
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub transfer_fee_config_authority: Option<Pubkey>,
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub withdraw_withheld_authority: Option<Pubkey>,
	pub withheld_amount: u64,
	pub older_transfer_fee: UiTransferFee,
	pub newer_transfer_fee: UiTransferFee,
}

impl From<extension::transfer_fee::TransferFeeConfig> for UiTransferFeeConfig {
	fn from(transfer_fee_config: extension::transfer_fee::TransferFeeConfig) -> Self {
		let transfer_fee_config_authority: Option<Pubkey> =
			transfer_fee_config.transfer_fee_config_authority.into();
		let withdraw_withheld_authority: Option<Pubkey> =
			transfer_fee_config.withdraw_withheld_authority.into();

		Self {
			transfer_fee_config_authority,
			withdraw_withheld_authority,
			withheld_amount: u64::from(transfer_fee_config.withheld_amount),
			older_transfer_fee: transfer_fee_config.older_transfer_fee.into(),
			newer_transfer_fee: transfer_fee_config.newer_transfer_fee.into(),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiTransferFeeAmount {
	pub withheld_amount: u64,
}

impl From<extension::transfer_fee::TransferFeeAmount> for UiTransferFeeAmount {
	fn from(transfer_fee_amount: extension::transfer_fee::TransferFeeAmount) -> Self {
		Self {
			withheld_amount: u64::from(transfer_fee_amount.withheld_amount),
		}
	}
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiMintCloseAuthority {
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub close_authority: Option<Pubkey>,
}

impl From<extension::mint_close_authority::MintCloseAuthority> for UiMintCloseAuthority {
	fn from(mint_close_authority: extension::mint_close_authority::MintCloseAuthority) -> Self {
		let close_authority: Option<Pubkey> = mint_close_authority.close_authority.into();
		Self { close_authority }
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiDefaultAccountState {
	pub account_state: UiAccountState,
}

impl From<extension::default_account_state::DefaultAccountState> for UiDefaultAccountState {
	fn from(default_account_state: extension::default_account_state::DefaultAccountState) -> Self {
		let account_state =
			spl_token_2022::state::AccountState::try_from(default_account_state.state)
				.unwrap_or_default();
		Self {
			account_state: account_state.into(),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiMemoTransfer {
	pub require_incoming_transfer_memos: bool,
}

impl From<extension::memo_transfer::MemoTransfer> for UiMemoTransfer {
	fn from(memo_transfer: extension::memo_transfer::MemoTransfer) -> Self {
		Self {
			require_incoming_transfer_memos: memo_transfer.require_incoming_transfer_memos.into(),
		}
	}
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiInterestBearingConfig {
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub rate_authority: Option<Pubkey>,
	pub initialization_timestamp: UnixTimestamp,
	pub pre_update_average_rate: i16,
	pub last_update_timestamp: UnixTimestamp,
	pub current_rate: i16,
}

impl From<extension::interest_bearing_mint::InterestBearingConfig> for UiInterestBearingConfig {
	fn from(
		interest_bearing_config: extension::interest_bearing_mint::InterestBearingConfig,
	) -> Self {
		let rate_authority: Option<Pubkey> = interest_bearing_config.rate_authority.into();

		Self {
			rate_authority,
			initialization_timestamp: UnixTimestamp::from(
				interest_bearing_config.initialization_timestamp,
			),
			pre_update_average_rate: i16::from(interest_bearing_config.pre_update_average_rate),
			last_update_timestamp: UnixTimestamp::from(
				interest_bearing_config.last_update_timestamp,
			),
			current_rate: i16::from(interest_bearing_config.current_rate),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiCpiGuard {
	pub lock_cpi: bool,
}

impl From<extension::cpi_guard::CpiGuard> for UiCpiGuard {
	fn from(cpi_guard: extension::cpi_guard::CpiGuard) -> Self {
		Self {
			lock_cpi: cpi_guard.lock_cpi.into(),
		}
	}
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiPermanentDelegate {
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub delegate: Option<Pubkey>,
}

impl From<extension::permanent_delegate::PermanentDelegate> for UiPermanentDelegate {
	fn from(permanent_delegate: extension::permanent_delegate::PermanentDelegate) -> Self {
		let delegate: Option<Pubkey> = permanent_delegate.delegate.into();
		Self { delegate }
	}
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiConfidentialTransferMint {
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub authority: Option<Pubkey>,
	pub auto_approve_new_accounts: bool,
	pub auditor_elgamal_pubkey: Option<String>,
}

impl From<extension::confidential_transfer::ConfidentialTransferMint>
	for UiConfidentialTransferMint
{
	fn from(
		confidential_transfer_mint: extension::confidential_transfer::ConfidentialTransferMint,
	) -> Self {
		let authority: Option<Pubkey> = confidential_transfer_mint.authority.into();
		Self {
			authority,
			auto_approve_new_accounts: confidential_transfer_mint.auto_approve_new_accounts.into(),
			auditor_elgamal_pubkey: {
				#[cfg(not(target_arch = "wasm32"))]
				let auditor_elgamal_pubkey: Option<
					spl_token_2022::solana_zk_sdk::encryption::pod::elgamal::PodElGamalPubkey,
				> = confidential_transfer_mint.auditor_elgamal_pubkey.into();
				// TODO FIXME
				#[cfg(target_arch = "wasm32")]
				let auditor_elgamal_pubkey: Option<String> = None;

				auditor_elgamal_pubkey.map(|pubkey| pubkey.to_string())
			},
		}
	}
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiConfidentialMintBurn {
	pub confidential_supply: String,
	pub decryptable_supply: String,
	pub supply_elgamal_pubkey: String,
}

impl From<extension::confidential_mint_burn::ConfidentialMintBurn> for UiConfidentialMintBurn {
	fn from(
		confidential_mint_burn: extension::confidential_mint_burn::ConfidentialMintBurn,
	) -> Self {
		UiConfidentialMintBurn {
			confidential_supply: confidential_mint_burn.confidential_supply.to_string(),
			decryptable_supply: confidential_mint_burn.decryptable_supply.to_string(),
			supply_elgamal_pubkey: confidential_mint_burn.supply_elgamal_pubkey.to_string(),
		}
	}
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiConfidentialTransferFeeConfig {
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub authority: Option<Pubkey>,
	pub withdraw_withheld_authority_elgamal_pubkey: String,
	pub harvest_to_mint_enabled: bool,
	pub withheld_amount: String,
}

impl From<extension::confidential_transfer_fee::ConfidentialTransferFeeConfig>
	for UiConfidentialTransferFeeConfig
{
	fn from(
		confidential_transfer_fee_config: extension::confidential_transfer_fee::ConfidentialTransferFeeConfig,
	) -> Self {
		let authority: Option<Pubkey> = confidential_transfer_fee_config.authority.into();
		Self {
			authority,
			withdraw_withheld_authority_elgamal_pubkey: confidential_transfer_fee_config
				.withdraw_withheld_authority_elgamal_pubkey
				.to_string(),
			harvest_to_mint_enabled: confidential_transfer_fee_config
				.harvest_to_mint_enabled
				.into(),
			withheld_amount: format!("{}", confidential_transfer_fee_config.withheld_amount),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiConfidentialTransferAccount {
	pub approved: bool,
	pub elgamal_pubkey: String,
	pub pending_balance_lo: String,
	pub pending_balance_hi: String,
	pub available_balance: String,
	pub decryptable_available_balance: String,
	pub allow_confidential_credits: bool,
	pub allow_non_confidential_credits: bool,
	pub pending_balance_credit_counter: u64,
	pub maximum_pending_balance_credit_counter: u64,
	pub expected_pending_balance_credit_counter: u64,
	pub actual_pending_balance_credit_counter: u64,
}

impl From<extension::confidential_transfer::ConfidentialTransferAccount>
	for UiConfidentialTransferAccount
{
	fn from(
		confidential_transfer_account: extension::confidential_transfer::ConfidentialTransferAccount,
	) -> Self {
		Self {
			approved: confidential_transfer_account.approved.into(),
			elgamal_pubkey: format!("{}", confidential_transfer_account.elgamal_pubkey),
			pending_balance_lo: format!("{}", confidential_transfer_account.pending_balance_lo),
			pending_balance_hi: format!("{}", confidential_transfer_account.pending_balance_hi),
			available_balance: format!("{}", confidential_transfer_account.available_balance),
			decryptable_available_balance: format!(
				"{}",
				confidential_transfer_account.decryptable_available_balance
			),
			allow_confidential_credits: confidential_transfer_account
				.allow_confidential_credits
				.into(),
			allow_non_confidential_credits: confidential_transfer_account
				.allow_non_confidential_credits
				.into(),
			pending_balance_credit_counter: confidential_transfer_account
				.pending_balance_credit_counter
				.into(),
			maximum_pending_balance_credit_counter: confidential_transfer_account
				.maximum_pending_balance_credit_counter
				.into(),
			expected_pending_balance_credit_counter: confidential_transfer_account
				.expected_pending_balance_credit_counter
				.into(),
			actual_pending_balance_credit_counter: confidential_transfer_account
				.actual_pending_balance_credit_counter
				.into(),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiConfidentialTransferFeeAmount {
	pub withheld_amount: String,
}

impl From<extension::confidential_transfer_fee::ConfidentialTransferFeeAmount>
	for UiConfidentialTransferFeeAmount
{
	fn from(
		confidential_transfer_fee_amount: extension::confidential_transfer_fee::ConfidentialTransferFeeAmount,
	) -> Self {
		Self {
			withheld_amount: format!("{}", confidential_transfer_fee_amount.withheld_amount),
		}
	}
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiMetadataPointer {
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub authority: Option<Pubkey>,
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub metadata_address: Option<Pubkey>,
}

impl From<extension::metadata_pointer::MetadataPointer> for UiMetadataPointer {
	fn from(metadata_pointer: extension::metadata_pointer::MetadataPointer) -> Self {
		let authority: Option<Pubkey> = metadata_pointer.authority.into();
		let metadata_address: Option<Pubkey> = metadata_pointer.metadata_address.into();

		Self {
			authority,
			metadata_address,
		}
	}
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiTokenMetadata {
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub update_authority: Option<Pubkey>,
	#[serde_as(as = "DisplayFromStr")]
	pub mint: Pubkey,
	pub name: String,
	pub symbol: String,
	pub uri: String,
	pub additional_metadata: Vec<(String, String)>,
}

impl From<TokenMetadata> for UiTokenMetadata {
	fn from(token_metadata: TokenMetadata) -> Self {
		let update_authority: Option<Pubkey> = token_metadata.update_authority.into();
		Self {
			update_authority,
			mint: token_metadata.mint,
			name: token_metadata.name,
			symbol: token_metadata.symbol,
			uri: token_metadata.uri,
			additional_metadata: token_metadata.additional_metadata,
		}
	}
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiTransferHook {
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub authority: Option<Pubkey>,
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub program_id: Option<Pubkey>,
}

impl From<extension::transfer_hook::TransferHook> for UiTransferHook {
	fn from(transfer_hook: extension::transfer_hook::TransferHook) -> Self {
		let authority: Option<Pubkey> = transfer_hook.authority.into();
		let program_id: Option<Pubkey> = transfer_hook.program_id.into();

		Self {
			authority,
			program_id,
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiTransferHookAccount {
	pub transferring: bool,
}

impl From<extension::transfer_hook::TransferHookAccount> for UiTransferHookAccount {
	fn from(transfer_hook: extension::transfer_hook::TransferHookAccount) -> Self {
		Self {
			transferring: transfer_hook.transferring.into(),
		}
	}
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiGroupPointer {
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub authority: Option<Pubkey>,
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub group_address: Option<Pubkey>,
}

impl From<extension::group_pointer::GroupPointer> for UiGroupPointer {
	fn from(group_pointer: extension::group_pointer::GroupPointer) -> Self {
		let authority: Option<Pubkey> = group_pointer.authority.into();
		let group_address: Option<Pubkey> = group_pointer.group_address.into();

		Self {
			authority,
			group_address,
		}
	}
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiGroupMemberPointer {
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub authority: Option<Pubkey>,
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub member_address: Option<Pubkey>,
}

impl From<extension::group_member_pointer::GroupMemberPointer> for UiGroupMemberPointer {
	fn from(member_pointer: extension::group_member_pointer::GroupMemberPointer) -> Self {
		let authority: Option<Pubkey> = member_pointer.authority.into();
		let member_address: Option<Pubkey> = member_pointer.member_address.into();

		Self {
			authority,
			member_address,
		}
	}
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiTokenGroup {
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub update_authority: Option<Pubkey>,
	#[serde_as(as = "DisplayFromStr")]
	pub mint: Pubkey,
	pub size: u64,
	pub max_size: u64,
}

impl From<TokenGroup> for UiTokenGroup {
	fn from(token_group: TokenGroup) -> Self {
		let update_authority: Option<Pubkey> = token_group.update_authority.into();
		Self {
			update_authority,
			mint: token_group.mint,
			size: token_group.size.into(),
			max_size: token_group.max_size.into(),
		}
	}
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiTokenGroupMember {
	#[serde_as(as = "DisplayFromStr")]
	pub mint: Pubkey,
	#[serde_as(as = "DisplayFromStr")]
	pub group: Pubkey,
	pub member_number: u64,
}

impl From<TokenGroupMember> for UiTokenGroupMember {
	fn from(member: TokenGroupMember) -> Self {
		Self {
			mint: member.mint,
			group: member.group,
			member_number: member.member_number.into(),
		}
	}
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiScaledUiAmountConfig {
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub authority: Option<Pubkey>,
	pub multiplier: String,
	pub new_multiplier_effective_timestamp: i64,
	pub new_multiplier: String,
}

impl From<extension::scaled_ui_amount::ScaledUiAmountConfig> for UiScaledUiAmountConfig {
	fn from(scaled_ui_amount_config: extension::scaled_ui_amount::ScaledUiAmountConfig) -> Self {
		let authority: Option<Pubkey> = scaled_ui_amount_config.authority.into();
		let multiplier: f64 = scaled_ui_amount_config.multiplier.into();
		let new_multiplier_effective_timestamp: i64 = scaled_ui_amount_config
			.new_multiplier_effective_timestamp
			.into();
		let new_multiplier: f64 = scaled_ui_amount_config.new_multiplier.into();

		Self {
			authority,
			multiplier: multiplier.to_string(),
			new_multiplier_effective_timestamp,
			new_multiplier: new_multiplier.to_string(),
		}
	}
}

#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UiPausableConfig {
	#[serde_as(as = "Option<DisplayFromStr>")]
	pub authority: Option<Pubkey>,
	pub paused: bool,
}

impl From<extension::pausable::PausableConfig> for UiPausableConfig {
	fn from(pausable_config: extension::pausable::PausableConfig) -> Self {
		let authority: Option<Pubkey> = pausable_config.authority.into();
		Self {
			authority,
			paused: pausable_config.paused.into(),
		}
	}
}
