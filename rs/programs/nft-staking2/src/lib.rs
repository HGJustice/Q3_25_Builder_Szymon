#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

declare_id!("5wbeXZK9s7wnhSfHZLVqwhea2e3gn2gGzJX9APwCvE7");

mod state;
mod instructions;
use instructions::*;


#[program]
pub mod nft_staking2 {
    use super::*;

    pub fn initialize(ctx: Context<InitalizeConfig>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

