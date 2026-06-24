use anchor_lang::prelude::*;

use crate::constants::TIP_JAR_SEED;
use crate::state::TipJar;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 8 + 1,
        seeds = [TIP_JAR_SEED, owner.key().as_ref()],
        bump
    )]
    pub tip_jar: Account<'info, TipJar>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let tip_jar = &mut ctx.accounts.tip_jar;

    tip_jar.owner = ctx.accounts.owner.key();
    tip_jar.total_tips = 0;
    tip_jar.bump = ctx.bumps.tip_jar;

    msg!("Tip jar initialized for owner: {:?}", tip_jar.owner);

    Ok(())
}
