pub mod constants;
pub mod error;
mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
use instructions::*;
pub use state::*;

declare_id!("GSa7icw4GkCW5Lnqdz53BVhX3V9tx8MPY7XZxR8BAixS");

#[program]
pub mod blinkmart_protocol {   
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
        instructions::initialize(ctx, params)
    }

    pub fn listing(ctx: Context<Listing>, params: ListingParams) -> Result<()> {
        instructions::listing(ctx, params)
    }

    pub fn delisting(ctx: Context<Delisting>, params: DelistingParams) -> Result<()> {
        instructions::delisting(ctx, params)
    }

    pub fn place_order(ctx: Context<PlaceOrder>, params: PlaceOrderParams) -> Result<()> {
        instructions::place_order(ctx, params)
    }

    pub fn cancel_order(ctx: Context<CancelOrder>, params: CancelOrderParams) -> Result<()> {
        instructions::cancel_order(ctx, params)
    }

    pub fn confirm_receipt(
        ctx: Context<ConfirmReceipt>,
        params: ConfirmReceiptParams,
    ) -> Result<()> {
        instructions::confirm_receipt(ctx, params)
    }

    pub fn seller_withdraw(
        ctx: Context<SellerWithdraw>,
        params: SellerWithdrawParams,
    ) -> Result<()> {
        instructions::seller_withdraw(ctx, params)
    }
}
