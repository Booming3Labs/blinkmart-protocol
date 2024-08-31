use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid order id")]
    InvalidOrderId,
    #[msg("Invalid product id")]
    InvalidProductId,
    #[msg("Invalid treasury address")]
    InvalidTreasury,
}
