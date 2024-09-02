use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid order id")]
    InvalidOrder,
    #[msg("Invalid product id")]
    InvalidProduct,
    #[msg("Invalid treasury address")]
    InvalidTreasury,
}
