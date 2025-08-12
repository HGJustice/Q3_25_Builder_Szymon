use anchor_lang::prelude::*;

use crate::state::{Listing, Marketplace, User};
use crate::enums::UserType;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct InitializeListing<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, User>,
    #[account(
        init,
        payer = user,
        space = 8 + Listing::INIT_SPACE,
        seeds = [b"listing", user.key().as_ref(), user_account.listings_count.to_le_bytes().as_ref()],
        bump,
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

impl<'info> InitializeListing<'info> {
    pub fn init_listing(&mut self, meal_name: String, description: String, price_per_meal: u64, meals_available: u64, delivery_available: bool, delivery_fee: Option<u32>, bumps: &InitializeListingBumps) -> Result<()> {
        require!(self.user_account.user_type == UserType::Cook, ErrorCode::OnlyCooksCanCreateListings);

        self.user_listing.set_inner(Listing { 
            listing_id: self.user_account.listings_count, 
            listing_owner: self.user.key(), 
            meal_name, 
            description,
            price_per_meal,
            meals_available,
            delivery_available, 
            delivery_fee, 
            bump: bumps.user_listing,
        });

        self.user_account.listings_count += 1;
        self.marketplace.total_listings += 1;
       
        Ok(())
    }
}