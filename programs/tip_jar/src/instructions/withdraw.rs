use anchor_lang::prelude::*;

use crate::constants::TIP_JAR_SEED;
use crate::error::TipJarError;
use crate::state::TipJar;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(
        mut,
        seeds = [TIP_JAR_SEED, tip_jar.owner.as_ref()],
        bump = tip_jar.bump,
        has_one = owner,
    )]
    pub tip_jar: Account<'info, TipJar>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

pub fn handler(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let tip_jar_info = ctx.accounts.tip_jar.to_account_info();
    let owner_info = ctx.accounts.owner.to_account_info();

    let rent_exempt_minimum = Rent::get()?.minimum_balance(tip_jar_info.data_len());
    let available = tip_jar_info
        .lamports()
        .checked_sub(rent_exempt_minimum)
        .ok_or(TipJarError::InsufficientFunds)?;

    require!(amount <= available, TipJarError::InsufficientFunds);

    **tip_jar_info.try_borrow_mut_lamports()? = tip_jar_info
        .lamports()
        .checked_sub(amount)
        .ok_or(TipJarError::Overflow)?;

    **owner_info.try_borrow_mut_lamports()? = owner_info
        .lamports()
        .checked_add(amount)
        .ok_or(TipJarError::Overflow)?;

    msg!(
        "Withdrew {} lamports to owner: {:?}",
        amount,
        ctx.accounts.owner.key()
    );

    Ok(())
}