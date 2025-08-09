use anchor_lang::prelude::*;

use crate::state::Listing;
use crate::state::User;
use crate::enums::UserType;

#[derive(Accounts)]
#[instruction(listing_id: u64)]
pub struct InitializeListing<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + Listing::INIT_SPACE,
        seeds = [b"listing", user.key().as_ref(), listing_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub user_listing: Account<'info, Listing>,
    #[account(
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, User>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeListing<'info> {
    pub fn init_listing(&mut self, ) -> Result<()> {
        // require!(self.user_account.user_type == UserType::Cook, );
        Ok(())
    }

}