use anchor_lang::prelude::*;

declare_id!("TvbUu14ofv5yuvgpi35AMrPBWQiHfeG2VgcDnLFB3hw");

pub mod state;
pub use state::*;

pub mod contexts;
pub use contexts::*;

#[program]
pub mod escrow_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn make(ctx: Context<Make>, seed: u64, receive: u64, deposit: u64) -> Result<()> {
        ctx.accounts.deposit(deposit)?;
        ctx.accounts.save_escrow(seed, receive, &ctx.bumps)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
