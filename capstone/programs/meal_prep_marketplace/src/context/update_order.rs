use anchor_lang::prelude::*;

use crate::state::{User, Order, Listing};
use crate::enums::{OrderStatusCook, OrderStatusCustomer, UserType};
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(listing_id: u64)]
pub struct UpdateOrderStatus<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, User>,
    #[account(
        seeds = [b"user", cook.user_address.as_ref()],
        bump = cook.bump,
    )]
    pub cook: Account<'info, User>,
    #[account(
        seeds = [b"listing", cook.user_address.key().as_ref(), listing_id.to_le_bytes().as_ref()],
        bump = listing.bump,
    )]
    pub listing: Account<'info, Listing>,
    #[account(
        mut, 
        seeds = [b"order", order.customer_address.as_ref(), listing.key().as_ref()],
        bump = order.bump,
    )]
    pub order: Account<'info, Order>,
}

impl<'info> UpdateOrderStatus<'info> {

    pub fn cook_updates_order_status(&mut self, listing_id: u64, new_order_status: OrderStatusCook ) -> Result<()> {
        require!(self.user_account.user_type == UserType::Cook, ErrorCode::OnlyCookCanUpdateCookOrderStatus);

        self.order.cook_order_status = new_order_status;

        Ok(())
    }


    pub fn customer_updates_order_status(&mut self, listing_id: u64, new_order_status: OrderStatusCustomer) -> Result<()> {
        require!(self.user_account.user_type == UserType::Customer, ErrorCode::OnlyCustomerCanUpdateCustomerOrderStatus);

        self.order.customer_order_status = new_order_status;
        Ok(())
    }

}