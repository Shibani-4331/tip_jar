use anchor_lang::prelude::*;

#[account]
pub struct TipJar {
    pub owner: Pubkey,
    pub total_tips: u64,
    pub bump: u8,
}

impl TipJar {
    pub const SPACE: usize = 8 + 32 + 8 + 1;
}