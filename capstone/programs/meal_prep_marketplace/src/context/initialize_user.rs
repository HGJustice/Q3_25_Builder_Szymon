use anchor_lang::prelude::*;
use crate::state::{User};
use crate::enums::UserType;

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + User::INIT_SPACE,
        seeds = [b"user", user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, User>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeUser<'info> {
 pub fn init_user(&mut self, username: String, location: String, user_type: UserType, bumps: &InitializeUserBumps) -> Result<()> {
    self.user_account.set_inner( User { 
        user_address: self.user.key(), 
        username, 
        location, 
        user_type, 
        listings_count: 0, // maybe option<32> if user is customer
        bump: bumps.user_account,
    });
    Ok(())
}
}