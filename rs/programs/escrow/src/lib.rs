#![allow(unexpected_cfgs)]
#![allow(deprecated)]
use anchor_lang::prelude::*;

mod instructions;
use instructions::*;
mod state;
declare_id!("Fgs3MEq2ZyTk6n3C7wTMD68XnzjsjLw3iNz2Hso1Pkvk");

#[program]
pub mod escrow {
    use super::*;

    pub fn init_escrow(ctx: Context<Make>, seed: u64, deposit: u64, ) -> Result<()> {
        ctx.accounts.init_escrow(seed, deposit, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)
    } 

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_withdraw()
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.withdraw_and_close()
    }
}