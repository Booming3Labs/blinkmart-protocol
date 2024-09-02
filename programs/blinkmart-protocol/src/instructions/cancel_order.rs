use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::{Admin, Order, Product, ADMIN, ORDER, PRODUCT};

pub fn cancel_order(ctx: Context<CancelOrder>, _params: CancelOrderParams) -> Result<()> {
    require!(ctx.accounts.order.bump.ne(&0), ErrorCode::InvalidOrder);

    let order_amount = ctx.accounts.order.amount;

    **ctx
        .accounts
        .admin
        .to_account_info()
        .try_borrow_mut_lamports()? -= order_amount;
    **ctx
        .accounts
        .payer
        .to_account_info()
        .try_borrow_mut_lamports()? += order_amount;

    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CancelOrderParams {
    order_id: String,
    product_id: String,
}

#[derive(Accounts)]
#[instruction(params: CancelOrderParams)]
pub struct CancelOrder<'info> {
    #[account(mut, address = order.payer)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds=[
			PRODUCT.as_bytes(),
            params.product_id.as_bytes()
		],
        bump
    )]
    pub product: Box<Account<'info, Product>>,

    #[account(
        mut,
        seeds=[
			ORDER.as_bytes(),
            params.order_id.as_bytes()
		],
        close=payer,
        bump
    )]
    pub order: Box<Account<'info, Order>>,

    #[account(
        mut,
        seeds=[
			ADMIN.as_bytes(),
		],
        bump
    )]
    pub admin: Box<Account<'info, Admin>>,

    pub system_program: Program<'info, System>,
}
