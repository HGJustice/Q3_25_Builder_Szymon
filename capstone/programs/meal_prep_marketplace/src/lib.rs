#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

pub mod context;
use context::*;
pub mod state;
use state::*;
declare_id!("7oCizVtCCp4VuFjWf4wbNzxpPckLZTSABP9YyBuD8oqa");

#[program]
pub mod meal_prep_marketplace {
    use crate::state::UserType;
    use super::*;

    pub fn initialize_marketplace(ctx: Context<Initialize>, fees: u16) -> Result<()> {
        ctx.accounts.init(fees, &ctx.bumps)?;
        Ok(())
    }

    pub fn initialize_user(ctx: Context<InitializeUser>, username: String, location: String, user_type: UserType) -> Result<()> {
        ctx.accounts.init_user(username, location, user_type, &ctx.bumps)?;
        Ok(())
    }
}

