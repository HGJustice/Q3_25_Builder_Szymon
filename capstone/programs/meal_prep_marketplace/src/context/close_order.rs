use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::state::{User, Listing, Marketplace, Order};
use crate::enums::{OrderStatusCook, OrderStatusCustomer};
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(listing_id: u64)]
pub struct CloseOrder<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"user", cook.user_address.as_ref()],
        bump = cook.bump,
    )]
    pub cook: Account<'info, User>,
        #[account(
        mut,
        seeds = [b"listing", user.key().as_ref(), listing_id.to_le_bytes().as_ref()],
        bump = user_listing.bump,
    )]
    pub user_listing: Account<'info, Listing>,
    #[account(
        mut, 
        seeds = [b"order", order.customer_address.as_ref(), user_listing.key().as_ref()],
        bump = order.bump,
        close = cook,
    )]
    pub order: Account<'info, Order>,
    #[account(
        mut,
        seeds = [b"marketplace"],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,
        #[account(
        mut, 
        seeds = [b"vault", user.key().as_ref(), user_listing.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>
} 

impl<'info> CloseOrder<'info> {
    pub fn close_order(&mut self, ) -> Result<()> {
        require!(self.order.cook_order_status == OrderStatusCook::Complete, ErrorCode::CantWithdrawCookHasntCompletedOrder);
        require!(self.order.customer_order_status == OrderStatusCustomer::Collected, ErrorCode::CantWithdrawCustomerHasntCollectedItem);

        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.cook.to_account_info(),
        };
        
        let seeds = &[
            b"vault",
            self.order.customer_address.as_ref(),
            self.user_listing.key().as_ref(),
            &[self.order.vault_bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(self.system_program.to_account_info(), cpi_accounts, signer_seeds);
        transfer(cpi_ctx, self.vault.lamports())?;

        Ok(())
    }
}