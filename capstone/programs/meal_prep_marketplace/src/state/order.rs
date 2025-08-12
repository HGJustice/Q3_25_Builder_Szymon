use anchor_lang::prelude::*;

use crate::enums::OrderStatus;

#[account]
#[derive(InitSpace)]
pub struct Order {
    pub customer_address: Pubkey,
    pub cook_address: Pubkey,
    pub listing_id: u64,
    pub total_cost: u64,
    pub meal_quantity: u64,
    #[max_len(50)]
    pub delivery_location: Option<String>,
    #[max_len(50)]
    pub collection_location: Option<String>,
    pub order_status: OrderStatus,
    pub bump: u8,
    pub vault_bump: u8,
}