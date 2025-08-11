use anchor_lang::prelude::*;

use crate::state::{listing, Listing, Marketplace};

#[derive(Accounts)]
#[instruction(listing_id: u64)]
pub struct CloseListing<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
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
    pub system_program: Program<'info, System>,
}


impl<'info> CloseListing<'info> {

}