use crate::error::ErrorCode;
use crate::{Admin, Order, Product, ADMIN, ORDER, PRODUCT};

use anchor_lang::prelude::*;

pub fn confirm_receipt(ctx: Context<ConfirmReceipt>, _params: ConfirmReceiptParams) -> Result<()> {
    require!(ctx.accounts.order.bump.ne(&0), ErrorCode::InvalidOrder);

    ctx.accounts.product.sales_amount += ctx.accounts.order.amount;

    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ConfirmReceiptParams {
    order_id: String,
    product_id: String,
}

#[derive(Accounts)]
#[instruction(params: ConfirmReceiptParams)]
pub struct ConfirmReceipt<'info> {
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
