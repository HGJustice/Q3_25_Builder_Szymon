use anchor_lang::prelude::*;
use crate::state::enums::UserType;

#[account]
#[derive(InitSpace)]
pub struct UserManagement {
    pub user_address: Pubkey,
    #[max_len(30)]
    pub username: String,
    #[max_len(50)]
    pub location: String,
    pub user_type: UserType,
    pub bump: u8,
}