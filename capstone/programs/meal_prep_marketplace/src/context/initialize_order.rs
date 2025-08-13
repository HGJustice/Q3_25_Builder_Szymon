use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::state::{Listing, Marketplace, Order, User};
use crate::enums::{OrderStatusCook, OrderStatusCustomer};

#[derive(Accounts)]
#[instruction(listing_id: u64)]
pub struct InitalizeOrder<'info> {
    #[account(mut)]
    pub customer: Signer<'info>,
    #[account(
        seeds = [b"user", cook.user_address.as_ref()],
        bump = cook.bump,
    )]
    pub cook: Account<'info, User>,
    #[account(
        seeds = [b"listing", cook.user_address.key().as_ref(), listing_id.to_le_bytes().as_ref()],
        bump = listing.bump
    )]
    pub listing: Account<'info, Listing>,
    #[account(
        init,
        payer = customer,
        space = 8 + Order::INIT_SPACE,
        seeds = [b"order", customer.key().as_ref(), listing.key().as_ref()],
        bump,
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
        seeds = [b"vault", customer.key().as_ref(), listing.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitalizeOrder<'info> {

    pub fn init_order(&mut self, listing_id: u64, meal_quantity: u64, delivery_location: Option<String>, collection_location: Option<String>, bumps: &InitalizeOrderBumps) -> Result<()> {

        let total_order_cost = meal_quantity * self.listing.price_per_meal;
        let rent = Rent::get()?.minimum_balance(self.vault.to_account_info().data_len());
        let total_vault_cost = total_order_cost + rent;

        let cpi_accounts = Transfer {
            from: self.customer.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);
        transfer(cpi_ctx, total_vault_cost)?;


        self.order.set_inner(Order { 
            customer_address: self.customer.key(), 
            cook_address: self.cook.key(), 
            listing_id, 
            total_cost: total_order_cost, 
            meal_quantity, 
            delivery_location, 
            collection_location, 
            cook_order_status: OrderStatusCook::Confirmed,
            customer_order_status: OrderStatusCustomer::Waiting,
            bump: bumps.order, 
            vault_bump: bumps.vault
        });

        self.marketplace.total_orders += 1;

        Ok(())
    }
}