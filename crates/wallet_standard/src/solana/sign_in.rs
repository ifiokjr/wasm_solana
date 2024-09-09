use std::fmt::Write;

use async_trait::async_trait;
use serde::Deserialize;
use serde::Serialize;
use typed_builder::TypedBuilder;

use super::SolanaSignMessageOutput;
use super::WalletAccountInfoSolanaPubkey;
use crate::SolanaSignatureOutput;
use crate::WalletAccountInfo;
use crate::WalletError;
use crate::WalletResult;

pub const SOLANA_SIGN_IN: &str = "solana:signIn";

pub trait SolanaSignInOutput: SolanaSignatureOutput + SolanaSignMessageOutput {
	type Account: WalletAccountInfo;
	/// Account that was signed in.
	/// The address of the account may be different from the provided input
	/// Address.
	fn account(&self) -> Self::Account;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SolanaSignInInput {
	/// Optional EIP-4361 Domain.
	/// If not provided, the wallet must determine the Domain to include in the
	/// message.
	#[builder(default, setter(into, strip_option))]
	pub domain: Option<String>,
	/// Optional EIP-4361 Address.
	/// If not provided, the wallet must determine the Address to include in the
	/// message.
	#[builder(default, setter(into, strip_option))]
	pub address: Option<String>,
	/// Optional EIP-4361 Statement.
	/// If not provided, the wallet must not include Statement in the message.
	#[builder(default, setter(into, strip_option))]
	pub statement: Option<String>,
	/// Optional EIP-4361 URI.
	/// If not provided, the wallet must not include URI in the message.
	#[builder(default, setter(into, strip_option))]
	pub uri: Option<String>,
	/// Optional EIP-4361 Version.
	/// If not provided, the wallet must not include Version in the message.
	#[builder(default, setter(into, strip_option))]
	pub version: Option<String>,
	/// Optional EIP-4361 Chain ID.
	/// If not provided, the wallet must not include Chain ID in the message.
	#[builder(default, setter(into, strip_option))]
	pub chain_id: Option<String>,
	/// Optional EIP-4361 Nonce.
	/// If not provided, the wallet must not include Nonce in the message.
	#[builder(default, setter(into, strip_option))]
	pub nonce: Option<String>,
	/// Optional EIP-4361 Issued At.
	/// If not provided, the wallet must not include Issued At in the message.
	#[builder(default, setter(into, strip_option))]
	pub issued_at: Option<String>,
	/// Optional EIP-4361 Expiration Time.
	/// If not provided, the wallet must not include Expiration Time in the
	/// message.
	#[builder(default, setter(into, strip_option))]
	pub expiration_time: Option<String>,
	/// Optional EIP-4361 Not Before.
	/// If not provided, the wallet must not include Not Before in the message.
	#[builder(default, setter(into, strip_option))]
	pub not_before: Option<String>,
	/// Optional EIP-4361 Request ID.
	/// If not provided, the wallet must not include Request ID in the message.
	#[builder(default, setter(into, strip_option))]
	pub request_id: Option<String>,
	/// Optional EIP-4361 Resources.
	/// If not provided, the wallet must not include Resources in the message.
	#[builder(default, setter(into, strip_option))]
	pub resources: Option<Vec<String>>,
}

#[async_trait(?Send)]
pub trait WalletSolanaSignIn {
	type Output: SolanaSignInOutput;

	async fn sign_in(&self, input: SolanaSignInInput) -> WalletResult<Self::Output>;
	async fn sign_in_many(&self, inputs: Vec<SolanaSignInInput>)
	-> WalletResult<Vec<Self::Output>>;
}

/// Check tha the input and output of the sign in are valid.
pub fn verify_sign_in(
	input: &SolanaSignInInput,
	output: &impl SolanaSignInOutput,
) -> WalletResult<()> {
	let account = output.account();

	let Some(input_address) = input.address.as_ref() else {
		return Err(WalletError::WalletSignIn);
	};

	if account.address().as_str() != input_address {
		return Err(WalletError::WalletSignIn);
	}

	verify_output_text(input, output)?;

	let signature = output.try_signature()?;
	let signed_message = output.signed_message();
	let pubkey = account.pubkey();

	if pubkey.to_string().as_str() != input_address {
		return Err(WalletError::WalletSignIn);
	}

	if signature.verify(&pubkey.to_bytes(), &signed_message) {
		Ok(())
	} else {
		Err(WalletError::WalletSignIn)
	}
}

#[allow(unused_assignments)]
fn verify_output_text(
	input: &SolanaSignInInput,
	output: &impl SolanaSignInOutput,
) -> WalletResult<()> {
	let input_text = create_sign_in_message_text(input)?;
	let output_text = String::from_utf8(output.signed_message())
		.map_err(|_| WalletError::ParseString("could not parse the signed_message".into()))?;

	if input_text == output_text {
		return Ok(());
	}

	let domain = input
		.domain
		.as_ref()
		.ok_or(WalletError::WalletSignInFields("Domain is required".into()))?;
	let address = input
		.address
		.as_ref()
		.ok_or(WalletError::WalletSignInFields(
			"Address is required".into(),
		))?;

	let Some((_, output_text)) = output_text
		.split_once(format!("{domain} wants you to sign in with your Solana account:").as_str())
	else {
		return Err(WalletError::WalletSignIn);
	};

	let Some((_, mut output_text)) = output_text.split_once(address) else {
		return Err(WalletError::WalletSignIn);
	};

	if let Some(ref statement) = input.statement {
		let Some((_, remaining)) = output_text.split_once(statement) else {
			return Err(WalletError::WalletSignIn);
		};

		output_text = remaining;
	}

	// Use macros for concise field appending
	macro_rules! confirm_field_exists {
		($field:expr, $name:literal) => {
			if let Some(ref value) = $field {
				let Some((_, remaining)) =
					output_text.split_once(format!("{}: {}", $name, value).as_str())
				else {
					return Err(WalletError::WalletSignIn);
				};

				output_text = remaining;
			}
		};
	}

	confirm_field_exists!(input.uri, "URI");
	confirm_field_exists!(input.version, "Version");
	confirm_field_exists!(input.chain_id, "Chain ID");
	confirm_field_exists!(input.nonce, "Nonce");
	confirm_field_exists!(input.issued_at, "Issued At");
	confirm_field_exists!(input.expiration_time, "Expiration Time");
	confirm_field_exists!(input.not_before, "Not Before");
	confirm_field_exists!(input.request_id, "Request ID");

	// TODO check resources

	Ok(())
}

/// ```markup
/// ${domain} wants you to sign in with your Solana account:
/// ${address}
///
/// ${statement}
///
/// URI: ${uri}
/// Version: ${version}
/// Chain ID: ${chain}
/// Nonce: ${nonce}
/// Issued At: ${issued_at}
/// Expiration Time: ${expiration_time}
/// Not Before: ${not_before}
/// Request ID: ${request_id}
/// Resources:
/// - ${resources[0]}
/// - ${resources[1]}
/// ...
/// - ${resources[n]}
/// ```
pub fn create_sign_in_message_text(input: &SolanaSignInInput) -> WalletResult<String> {
	let mut message = String::with_capacity(256); // Pre-allocate for efficiency
	let domain = input
		.domain
		.as_ref()
		.ok_or(WalletError::WalletSignInFields("Domain is required".into()))?;
	let address = input
		.address
		.as_ref()
		.ok_or(WalletError::WalletSignInFields(
			"Address is required".into(),
		))?;

	write!(
		&mut message,
		"{domain} wants you to sign in with your Solana account:\n{address}"
	)?;

	if let Some(ref statement) = input.statement {
		write!(&mut message, "\n\n{statement}")?;
	}

	let mut fields = Vec::with_capacity(10); // Estimate number of fields for allocation

	// Use macros for concise field appending
	macro_rules! push_field {
		($field:expr, $name:literal) => {
			if let Some(ref value) = $field {
				fields.push(format!("{}: {}", $name, value));
			}
		};
	}

	push_field!(input.uri, "URI");
	push_field!(input.version, "Version");
	push_field!(input.chain_id, "Chain ID");
	push_field!(input.nonce, "Nonce");
	push_field!(input.issued_at, "Issued At");
	push_field!(input.expiration_time, "Expiration Time");
	push_field!(input.not_before, "Not Before");
	push_field!(input.request_id, "Request ID");

	if let Some(ref resources) = input.resources {
		fields.push("Resources:".to_string());
		for resource in resources {
			fields.push(format!("- {resource}"));
		}
	}

	if !fields.is_empty() {
		write!(&mut message, "\n\n{}", fields.join("\n")).unwrap();
	}

	Ok(message)
}
