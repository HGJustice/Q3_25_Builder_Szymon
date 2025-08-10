use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseListing<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    
}