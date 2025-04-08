use anchor_lang::prelude::*;
use std::ops::DerefMut;

declare_id!("4rYU4LZNaM1smqPx3omxBrKkRdUEMQvfFoJ2F3nNYVv5");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let counter = ctx.accounts.counter.deref_mut();
        let bump = ctx.bumps.counter;
        *counter = Counter {
            authority: *ctx.accounts.authority.key,
            count: 0,
            bump,
        };
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        require_keys_eq!(
            *ctx.accounts.authority.key,
            ctx.accounts.counter.authority,
            ErrorCode::Unauthorized
        );
        ctx.accounts.counter.count += 1;
        Ok(())
    }
}

#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
    pub bump: u8,
}

impl Counter {
    pub const SIZE: usize = 8 + 32 + 8 + 1;
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = Counter::SIZE,
        seeds = [b"anchor_counter"],
        bump
    )]
    counter: Account<'info, Counter>,

    #[account(mut)]
    authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        mut,
        seeds = [b"counter"],
        bump = counter.bump
    )]
    counter: Account<'info, Counter>,
    authority: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,

    #[msg("The provided input is invalid.")]
    InvalidInput,

    #[msg("Insufficient funds to perform the operation.")]
    InsufficientFunds,
}
