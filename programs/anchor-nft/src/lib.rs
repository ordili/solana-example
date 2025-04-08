use anchor_lang::prelude::*;

declare_id!("EUjvWpSZLYnTkqN6pWgSVhcrGdCtSkR6x7spo4Z2Ghpd");

#[program]
pub mod anchor_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
