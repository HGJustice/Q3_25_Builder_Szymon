use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Customers cannot create listings, only cooks!")]
    OnlyCooksCanCreateListings,
    #[msg("Only the owner can close their listing and claim lamports back")]
    OnlyOwnerCanCloseListing,
    #[msg("Only cook can update their OrderStatus")]
    OnlyCookCanUpdateCookOrderStatus,
    #[msg("Only customer can update their OrderStatus")]
    OnlyCustomerCanUpdateCustomerOrderStatus,
    UnauthorizedCookUpdate,
    UnauthorizedCustomerUpdate,
    CantWithdrawCustomerHasntCollectedItem,
    CantWithdrawCookHasntCompletedOrder,

}