#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

pub mod context;
use context::*;
pub mod state;
declare_id!("7oCizVtCCp4VuFjWf4wbNzxpPckLZTSABP9YyBuD8oqa");

#[program]
pub mod meal_prep_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fees: u16) -> Result<()> {
        ctx.accounts.init(fees, &ctx.bumps)?;
        Ok(())
    }
}

