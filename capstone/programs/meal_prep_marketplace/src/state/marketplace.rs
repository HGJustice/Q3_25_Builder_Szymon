use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Marketplace {
    pub admin: Pubkey,
    pub total_listings: u64,
    pub total_orders: u64,
    pub treasury_bump: u8, 
    pub fees: u16,
    pub bump: u8,
}