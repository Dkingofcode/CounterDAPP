#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("G3eJNm4Kwgf13zESTtES6DYFAkgsknBmpJsKRGJay2DN");

#[program]
pub mod solana_dapp_counter_frontend {
    use super::*;

    pub fn initialize(ctx: Context<InitializeSolanaDappCounterFrontend>) -> Result<()> {
        ctx.accounts.solana_dapp_counter_frontend.count = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.solana_dapp_counter_frontend.count =
            ctx.accounts.solana_dapp_counter_frontend.count.saturating_add(1);
        Ok(())
    }

    pub fn decrement(ctx: Context<Update>) -> Result<()> {
        ctx.accounts.solana_dapp_counter_frontend.count =
            ctx.accounts.solana_dapp_counter_frontend.count.saturating_sub(1);
        Ok(())
    }

    pub fn set(ctx: Context<Update>, value: u8) -> Result<()> {
        ctx.accounts.solana_dapp_counter_frontend.count = value;
        Ok(())
    }

    pub fn close(_ctx: Context<CloseSolanaDappCounterFrontend>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeSolanaDappCounterFrontend<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + SolanaDappCounterFrontend::INIT_SPACE
    )]
    pub solana_dapp_counter_frontend: Account<'info, SolanaDappCounterFrontend>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub solana_dapp_counter_frontend: Account<'info, SolanaDappCounterFrontend>,
}

#[derive(Accounts)]
pub struct CloseSolanaDappCounterFrontend<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        close = payer
    )]
    pub solana_dapp_counter_frontend: Account<'info, SolanaDappCounterFrontend>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct SolanaDappCounterFrontend {
    pub count: u8,
}