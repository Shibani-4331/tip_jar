use anchor_lang::prelude::*;

#[error_code]
pub enum TipJarError {
    #[msg("Only the owner of this tip jar can withdraw from it.")]
    Unauthorized,

    #[msg("Withdrawal amount exceeds the jar's available balance.")]
    InsufficientFunds,

    #[msg("Arithmetic overflow occurred.")]
    Overflow,
}