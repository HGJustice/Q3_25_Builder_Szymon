use anchor_lang::prelude::*;
use crate::state::{UserManagement, UserType};

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + UserManagement::INIT_SPACE,
        seeds = [b"user", user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserManagement>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeUser<'info> {
 pub fn init_user(&mut self, username: String, location: String, user_type: UserType, bumps: &InitializeUserBumps) -> Result<()> {
    self.user_account.set_inner( UserManagement { 
        user_address: self.user.key(), 
        username, 
        location, 
        user_type,
        bump: bumps.user_account,
    });
    Ok(())
}
}