use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::state::Marketplace;

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"marketplace"],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        mut,
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasury_bump
    )]
    pub treasury: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawFees<'info> {
    pub fn withdraw_fees(&mut self,) -> Result<()> {
        let accounts = Transfer {
            from: self.treasury.to_account_info(),
            to: self.admin.to_account_info(),
        };

        let seeds = &[b"treasury", self.marketplace.to_account_info().key.as_ref(), &[self.marketplace.treasury_bump]];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(self.system_program.to_account_info(), accounts, signer_seeds);
        transfer(cpi_ctx, self.treasury.lamports())?;
        Ok(())
    }
}