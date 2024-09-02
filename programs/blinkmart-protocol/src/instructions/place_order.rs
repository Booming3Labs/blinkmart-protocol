use anchor_lang::prelude::*;

use anchor_lang::system_program;

use crate::error::ErrorCode;
use crate::{Admin, Order, Product, ADMIN, ORDER, PRODUCT};
pub fn place_order(ctx: Context<PlaceOrder>, params: PlaceOrderParams) -> Result<()> {
    require!(ctx.accounts.order.bump.eq(&0), ErrorCode::InvalidOrder);

    let amount = params
        .order_quantity
        .checked_mul(ctx.accounts.product.sales_price)
        .unwrap();

    ctx.accounts.order.set_inner(Order {
        order_id: params.order_id,
        payer: ctx.accounts.payer.key(),
        amount,
        bump: ctx.bumps.order,
    });

    system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.admin.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct PlaceOrderParams {
    order_id: String,
    product_id: String,
    payer: Pubkey,
    order_quantity: u64,
}

#[derive(Accounts)]
#[instruction(params: PlaceOrderParams)]
pub struct PlaceOrder<'info> {
    #[account(mut)]
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
        init,
        space=Order::LEN,
        payer=payer,
        seeds=[
			ORDER.as_bytes(),
            params.order_id.as_bytes()
		],
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
