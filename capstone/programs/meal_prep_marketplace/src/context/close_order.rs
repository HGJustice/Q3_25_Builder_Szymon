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
        mut,
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
        close = user,
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
        seeds = [b"vault", order.customer_address.key().as_ref(), user_listing.key().as_ref()],
        bump = order.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump = marketplace.treasury_bump,
    )]
    pub treasury: SystemAccount<'info>,
    pub system_program: Program<'info, System>
} 

impl<'info> CloseOrder<'info> {

    pub fn send_fees(&mut self, ) -> Result<()> {
        let vault_balance = self.vault.lamports();

        let cpi_fees = Transfer {
            from: self.vault.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let seeds = &[
            b"vault", 
            self.order.customer_address.as_ref(), 
            self.user_listing.to_account_info().key.as_ref(),  
            &[self.order.vault_bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let marketplace_fee = vault_balance
            .checked_mul(self.marketplace.fees as u64)
            .unwrap()
            .checked_div(10000)  
            .unwrap();

        let cpi_ctx1 = CpiContext::new_with_signer(self.system_program.to_account_info(), cpi_fees, signer_seeds);
        transfer(cpi_ctx1, marketplace_fee)?;
        Ok(())
    }

  
      pub fn close_order_with_fees(&mut self, listing_id: u64) -> Result<()> {
        require!(
            self.order.cook_order_status == OrderStatusCook::Complete, 
            ErrorCode::CantWithdrawCookHasntCompletedOrder
        );
        require!(
            self.order.customer_order_status == OrderStatusCustomer::Collected, 
            ErrorCode::CantWithdrawCustomerHasntCollectedItem
        );
        
        // Get total vault balance
        let vault_balance = self.vault.lamports();
        
        // Calculate fee based on the actual order cost (not including rent)
        let marketplace_fee = self.order.total_cost - self.marketplace.fees as u64;
        
        // Cook gets everything else (order amount - fee + rent)
        let cook_amount = vault_balance
            .checked_sub(marketplace_fee)
            .unwrap();
        
        let seeds = &[
            b"vault", 
            self.order.customer_address.as_ref(), 
            self.user_listing.to_account_info().key.as_ref(),  
            &[self.order.vault_bump]
        ];
        let signer_seeds = &[&seeds[..]];
        
        // Transfer 1: Marketplace fee to treasury
        if marketplace_fee > 0 {
            let fee_transfer = Transfer {
                from: self.vault.to_account_info(),
                to: self.treasury.to_account_info(),
            };
            
            let fee_ctx = CpiContext::new_with_signer(
                self.system_program.to_account_info(), 
                fee_transfer, 
                signer_seeds
            );
            transfer(fee_ctx, marketplace_fee)?;
        }
        
        // Transfer 2: Everything remaining to cook (this drains vault to 0)
        let cook_transfer = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };
        
        let cook_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(), 
            cook_transfer, 
            signer_seeds
        );
        transfer(cook_ctx, cook_amount)?;
        
        self.marketplace.total_orders -= 1;
        
        Ok(())
    }
}