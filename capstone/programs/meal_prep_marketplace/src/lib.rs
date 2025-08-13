#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

pub mod context;
pub mod state;
pub mod enums;
pub mod errors;

use enums::*;
use context::*;


declare_id!("7oCizVtCCp4VuFjWf4wbNzxpPckLZTSABP9YyBuD8oqa");

#[program]
pub mod meal_prep_marketplace {
    use super::*;

    pub fn initialize_marketplace(ctx: Context<Initialize>, fees: u16) -> Result<()> {
        ctx.accounts.init_marketplace(fees, &ctx.bumps)?;
        Ok(())
    }

    pub fn initialize_user(ctx: Context<InitializeUser>, username: String, location: String, user_type: UserType) -> Result<()> {
        ctx.accounts.init_user(username, location, user_type, &ctx.bumps)?;
        Ok(())
    }

    pub fn cook_creates_listing(ctx: Context<InitializeListing>, meal_name: String, description: String, price_per_meal: u64, meals_available: u64, delivery_available: bool, delivery_fee: Option<u32>) -> Result<()> {
        ctx.accounts.init_listing(meal_name, description, price_per_meal, meals_available, delivery_available, delivery_fee, &ctx.bumps)?;
        Ok(())
    }

    pub fn close_listing(ctx: Context<CloseListing>, listing_id: u64) -> Result<()> {
        ctx.accounts.close_listing(listing_id)?;
        Ok(())
    }

    pub fn create_order(ctx: Context<InitalizeOrder>, listing_id: u64, meal_quantity: u64, delivery_location: Option<String>, collection_location: Option<String>) -> Result<()> {
        ctx.accounts.init_order(listing_id, meal_quantity, delivery_location, collection_location, &ctx.bumps)?;
        Ok(())
    }


    pub fn cook_change_order_status(ctx: Context<UpdateOrderStatus>, listing_id: u64, new_order_status: OrderStatusCook) -> Result<()> {
        ctx.accounts.cook_updates_order_status(listing_id, new_order_status)?;
        Ok(())
    }

    pub fn customer_change_order_status(ctx: Context<UpdateOrderStatus>, listing_id: u64, new_order_status: OrderStatusCustomer) -> Result<()> {
        ctx.accounts.customer_updates_order_status(listing_id, new_order_status)?;
        Ok(())
    }
}