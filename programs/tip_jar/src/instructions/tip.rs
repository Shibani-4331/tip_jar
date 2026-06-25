use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program::invoke, system_instruction};

use crate::constants::TIP_JAR_SEED;
use crate::error::TipJarError;
use crate::state::TipJar;

#[derive(Accounts)]
pub struct Tip<'info> {
    #[account(
        mut,
        seeds = [TIP_JAR_SEED, tip_jar.owner.as_ref()],
        bump = tip_jar.bump,
    )]
    pub tip_jar: Account<'info, TipJar>,

    #[account(mut)]
    pub tipper: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Tip>, amount: u64) -> Result<()> {
    let transfer_ix = system_instruction::transfer(
        &ctx.accounts.tipper.key(),
        &ctx.accounts.tip_jar.key(),
        amount,
    );
    invoke(
        &transfer_ix,
        &[
            ctx.accounts.tipper.to_account_info(),
            ctx.accounts.tip_jar.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;
    let tip_jar = &mut ctx.accounts.tip_jar;
    tip_jar.total_tips = tip_jar
        .total_tips
        .checked_add(amount)
        .ok_or(TipJarError::Overflow)?;

    msg!(
        "Tip of {} lamports received. New total: {}",
        amount,
        tip_jar.total_tips
    );
    Ok(())
}