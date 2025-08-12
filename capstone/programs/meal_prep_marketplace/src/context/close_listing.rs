use anchor_lang::prelude::*;

use crate::state::{Listing, Marketplace, User};
use crate::errors::ErrorCode;

#[derive(Accounts)]
#[instruction(listing_id: u64)]
pub struct CloseListing<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, User>,
    #[account(
        mut,
        seeds = [b"listing", user.key().as_ref(), listing_id.to_le_bytes().as_ref()],
        bump = user_listing.bump,
        close = user,
    )]
    pub user_listing: Account<'info, Listing>,
    #[account(
        mut,
        seeds = [b"marketplace"],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,
}

impl<'info> CloseListing<'info> {
    pub fn close_listing(&mut self, listing_id: u64) -> Result<()> {
        require!(self.user_listing.listing_owner == self.user.key(), ErrorCode::OnlyOwnerCanCloseListing);

        self.marketplace.total_listings -= 1;
        self.user_account.listings_count -= 1;
        Ok(())
    }
}