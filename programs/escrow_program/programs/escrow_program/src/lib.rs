use anchor_lang::prelude::*;

declare_id!("TvbUu14ofv5yuvgpi35AMrPBWQiHfeG2VgcDnLFB3hw");

pub mod state;
pub use state::*;

#[program]
pub mod escrow_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
