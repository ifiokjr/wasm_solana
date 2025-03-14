//! This module is responsible for parsing events emitted by the anchor program.

use std::marker::PhantomData;
use std::task::Poll;

use anchor_lang::Event;
use futures::ready;
use futures::Stream;
use pin_project::pin_project;
use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use typed_builder::TypedBuilder;
use wasm_client_solana::rpc_response::LogsNotificationResponse;
use wasm_client_solana::Subscription;

use crate::AnchorClientError;
use crate::AnchorClientResult;

/// The events stream for anchor logs from programs.
#[derive(Clone, TypedBuilder)]
#[pin_project]
pub struct EventSubscription<T: Event> {
	/// The underlying log subscription.
	#[pin]
	subscription: Subscription<LogsNotificationResponse>,
	/// The program id
	program_id: Pubkey,
	/// The stack of the currently processed log stack./// This stack helps in
	/// tracking nested program invocations.
	#[builder(default)]
	stack: Option<ProgramLogIterator<T>>,
}

impl<T: Event> EventSubscription<T> {
	pub async fn unsubscribe(&self) -> AnchorClientResult<()> {
		self.subscription.unsubscribe().await?;

		Ok(())
	}
}

const PROGRAM_LOG: &str = "Program log: ";
const PROGRAM_DATA: &str = "Program data: ";

#[serde_as]
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnchorEventContext {
	#[serde_as(as = "DisplayFromStr")]
	pub signature: Signature,
	pub slot: u64,
	pub subscription_id: u64,
}

impl<T: Event> Stream for EventSubscription<T> {
	type Item = (AnchorEventContext, T);

	fn poll_next(
		self: std::pin::Pin<&mut Self>,
		cx: &mut std::task::Context<'_>,
	) -> Poll<Option<Self::Item>> {
		let program_id = self.program_id;
		let mut this = self.project();

		if let Some(stack) = this.stack {
			match stack.next() {
				Some(value) => return Poll::Ready(Some((stack.context(), value))),
				None => {
					*this.stack = None;
				}
			}
		}

		let Some(result) = ready!(this.subscription.as_mut().poll_next(cx)) else {
			return Poll::Ready(None);
		};

		let signature = result.params.result.value.signature;
		let slot = result.params.result.context.slot;
		let subscription_id = result.params.subscription;
		let logs = result.params.result.value.logs;
		let context = AnchorEventContext {
			signature,
			slot,
			subscription_id,
		};
		let mut iterator = ProgramLogIterator::<T>::new(program_id, logs, context);

		let Some(value) = iterator.next() else {
			// There are no relevant values in the logs.
			return Poll::Pending;
		};

		*this.stack = Some(iterator);

		Poll::Ready(Some((context, value)))
	}
}

#[derive(Debug, Clone)]
pub struct ProgramLogIterator<T: Event> {
	program_id: Pubkey,
	index: usize,
	logs: Vec<String>,
	stack: Vec<Pubkey>,
	context: AnchorEventContext,
	phantom: PhantomData<T>,
}

impl<T: Event> Iterator for ProgramLogIterator<T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.index == 0 {
			self.update_program_for_log(self.index, None).ok();
			self.index = 1;
		}

		loop {
			// Exit if no longs remaining at the current index.
			let log = self.logs.get(self.index)?;
			self.index += 1;

			let current_program = self.program();

			let ParsedLogEntry {
				event,
				program: new_program,
				did_pop,
			} = if Some(&self.program_id) == current_program {
				match handle_program_log::<T>(&self.program_id, log).ok() {
					Some(entry) => entry,
					None => continue,
				}
			} else {
				let (program, did_pop) = handle_system_log(&self.program_id, log);
				ParsedLogEntry::builder()
					.program(program)
					.did_pop(did_pop)
					.build()
			};

			if let Some(new_program) = new_program {
				self.push(new_program);
			}

			if did_pop {
				self.pop();
				// index is intentionally peeking at the next invocation.
				self.update_program_for_log(self.index, Some("invoke [1]"))
					.ok();
			}

			if event.is_none() {
				continue;
			}

			break event;
		}
	}
}

impl<T: Event> ProgramLogIterator<T> {
	pub fn new(program: Pubkey, logs: Vec<String>, context: AnchorEventContext) -> Self {
		Self {
			program_id: program,
			index: 0,
			logs,
			stack: vec![],
			context,
			phantom: PhantomData,
		}
	}

	pub fn context(&self) -> AnchorEventContext {
		self.context
	}

	/// Update the internal
	fn update_program_for_log(
		&mut self,
		log_index: usize,
		ends_with: Option<&str>,
	) -> AnchorClientResult<()> {
		let log = self.logs.get(log_index);

		let Some(log) = log else {
			return Ok(());
		};

		if let Some(ends_with) = ends_with {
			if !log.ends_with(ends_with) {
				return Ok(());
			}
		}

		let re = regex::Regex::new(r"^Program (.*) invoke.*$").unwrap();
		let captures = re
			.captures(log)
			.ok_or_else(|| AnchorClientError::LogParse(log.to_string()))?;
		let program: Pubkey = captures
			.get(1)
			.ok_or_else(|| AnchorClientError::LogParse(log.to_string()))?
			.as_str()
			.parse()?;

		self.push(program);

		Ok(())
	}

	pub fn program(&self) -> Option<&Pubkey> {
		self.stack.last()
	}

	pub fn push(&mut self, program: Pubkey) {
		self.stack.push(program);
	}

	pub fn pop(&mut self) {
		self.stack.pop();
	}
}

#[derive(TypedBuilder)]
#[builder(field_defaults(default))]
pub struct ParsedLogEntry<T: Event> {
	/// The event data being parsed if any is found.
	pub event: Option<T>,
	/// The new program log stack.
	pub program: Option<Pubkey>,
	/// Whether to pop the previous program log stack.
	pub did_pop: bool,
}

pub fn handle_program_log<T: Event>(
	program_id: &Pubkey,
	log: &str,
) -> AnchorClientResult<ParsedLogEntry<T>> {
	use anchor_lang::__private::base64;
	use base64::engine::general_purpose::STANDARD;
	use base64::Engine;

	// Log emitted from the current program.
	if let Some(log) = log
		.strip_prefix(PROGRAM_LOG)
		.or_else(|| log.strip_prefix(PROGRAM_DATA))
	{
		let Ok(log_bytes) = STANDARD.decode(log) else {
			log::warn!("Could not base64 decode log: {}", log);
			return Ok(ParsedLogEntry::builder().build());
		};

		let event = log_bytes
			.starts_with(T::DISCRIMINATOR)
			.then(|| {
				let mut data = &log_bytes[T::DISCRIMINATOR.len()..];
				T::deserialize(&mut data).map_err(|e| AnchorClientError::LogParse(e.to_string()))
			})
			.transpose()?;

		Ok(ParsedLogEntry::builder().event(event).build())
	}
	// System log.
	else {
		let (program, did_pop) = handle_system_log(program_id, log);

		Ok(ParsedLogEntry::builder()
			.program(program)
			.did_pop(did_pop)
			.build())
	}
}

pub fn handle_system_log(program_id: &Pubkey, log: &str) -> (Option<Pubkey>, bool) {
	if log.starts_with(&format!("Program {program_id} log:")) {
		(Some(*program_id), false)

		// `Invoke [1]` instructions are pushed to the stack in
		// `parse_logs_response`, so this ensures we only push CPIs to the
		// stack at this stage
	} else if log.contains("invoke") && !log.ends_with("[1]") {
		(Some(Pubkey::default()), false) // Any pubkey will do.
	} else {
		let re = regex::Regex::new(r"^Program (.*) success*$").unwrap();

		if re.is_match(log) {
			(None, true)
		} else {
			(None, false)
		}
	}
}
