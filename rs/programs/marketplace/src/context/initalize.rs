use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::state::Marketplace;


#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin, 
        space = 8 + Marketplace::INIT_SPACE,
        seeds = [b"marketplace", name.as_str().as_bytes()],
        bump,
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump,
    )]
    pub treasury: SystemAccount<'info>,
    #[account(
        init, 
        payer = admin, 
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = marketplace,
    )]
    pub rewards: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, name: String, fees: u16, bumps: &InitializeBumps ) -> Result<()> {
        self.marketplace.set_inner(Marketplace { 
            admin: self.admin.key(),
            fees, 
            bump: bumps.marketplace, 
            tresasury_bump: bumps.treasury, 
            reward_bump: bumps.rewards, 
            name 
        });
        Ok(())
    }
}