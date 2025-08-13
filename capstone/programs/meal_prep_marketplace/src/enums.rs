use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum UserType {
    Cook,
    Customer
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum OrderStatusCook {
    Confirmed, 
    Ready, 
    Complete,
    Cancelled
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum OrderStatusCustomer {
    Collected,
    Waiting,
    Cancelled
}