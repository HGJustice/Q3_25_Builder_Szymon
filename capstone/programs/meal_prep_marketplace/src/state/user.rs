use anchor_lang::prelude::*;
use crate::enums::UserType;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub user_address: Pubkey,
    #[max_len(30)]
    pub username: String,
    #[max_len(50)]
    pub location: String,
    pub user_type: UserType,
    pub listings_count: u64, // option 32
    pub bump: u8,
}