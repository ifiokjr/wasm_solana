use anchor_lang::prelude::*;

declare_id!("D1NW5bwpfVQC86ercmzqGVizp8NCuMvAVTLEK3LSCo4E");

#[program]
pub mod example_program {
	use super::*;

	pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
		msg!("Greetings from: {:?}", ctx.program_id);
		Ok(())
	}

	pub fn another(ctx: Context<Another>, useless: u32) -> Result<()> {
		msg!("another useless: {}, program: {}", useless, ctx.program_id);
		Ok(())
	}
}

#[derive(Accounts)]
pub struct Initialize<'info> {
	/// CHECK: for testing purposes
	pub unchecked: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct Another<'info> {
	/// signer for testing purposes
	pub signer: Signer<'info>,
}

impl From<u32> for instruction::Another {
	fn from(useless: u32) -> Self {
		instruction::Another { useless }
	}
}
