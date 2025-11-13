use anchor_lang::prelude::*;

declare_id!("4k9UR1GmGVeyiBTwM3BnVxwidWfKYWpFt4CpVs7rdE22");

#[program]
pub mod decenlocker {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
