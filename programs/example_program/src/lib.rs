use anchor_lang::prelude::*;

declare_id!("D1NW5bwpfVQC86ercmzqGVizp8NCuMvAVTLEK3LSCo4E");

#[program]
pub mod example_program {
	use super::*;

	pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
		msg!("Greetings from: {:?}", ctx.program_id);
		Ok(())
	}
}

#[derive(Accounts)]
pub struct Initialize<'info> {
	/// CHECK: for testing purposes
	pub unchecked: UncheckedAccount<'info>,
}
