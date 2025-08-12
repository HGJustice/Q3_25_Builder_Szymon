use anchor_lang::prelude::*;

use crate::state::Marketplace;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init, 
        payer = admin, 
        space = 8 + Marketplace::INIT_SPACE,
        seeds = [b"marketplace"],
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump,
    )]
    pub treasury: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init_marketplace(&mut self, fees: u16, bumps: &InitializeBumps) -> Result<()> {
        self.marketplace.set_inner(Marketplace { 
            admin: self.admin.key(), 
            total_listings: 0, 
            total_orders: 0,
            treasury_bump: bumps.treasury, 
            fees,
            bump: bumps.marketplace 
        });
        Ok(())
    }
}