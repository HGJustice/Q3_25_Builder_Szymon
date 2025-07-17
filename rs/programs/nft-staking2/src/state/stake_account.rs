use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakedAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub staked_at: i64,
    pub bump: u8,
}