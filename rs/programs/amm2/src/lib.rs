#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

mod instructions;
mod state;
use instructions::*;
declare_id!("5aEzWFQr8DK2e2gi632aHA56aYVEeJaccYsJj2e3m8y6");


#[program]
pub mod amm2 {
    use super::*;

    pub fn initialize(ctx: Context<Initalize>, seed: u64, fees: u16, authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.init(seed, fees, authority, ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
       Ok(())
    }
}


