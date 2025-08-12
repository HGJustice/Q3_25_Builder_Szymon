use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing {
    pub listing_id: u64,
    pub listing_owner: Pubkey,
    #[max_len(30)]
    pub meal_name: String,
    #[max_len(50)]
    pub description: String,
    pub price_per_meal: u64,
    pub meals_available: u64,
    pub delivery_available: bool,
    pub delivery_fee: Option<u32>,
    pub bump: u8,
}