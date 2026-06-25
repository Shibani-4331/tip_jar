pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("713h5SKmZ33JimUM8TrF8es2HdiacpUU2eZosuqs7GSS");

#[program]
pub mod tip_jar {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    pub fn tip(ctx: Context<Tip>, amount: u64)-> Result<()>{
        instructions::tip::handler(ctx, amount)
    }
}
