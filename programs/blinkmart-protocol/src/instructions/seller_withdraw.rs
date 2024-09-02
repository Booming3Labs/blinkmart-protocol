use anchor_lang::prelude::*;

use crate::error::ErrorCode;

use crate::{Admin, Product, ADMIN, PRODUCT};

pub fn seller_withdraw(ctx: Context<SellerWithdraw>, _params: SellerWithdrawParams) -> Result<()> {
    require!(ctx.accounts.product.bump.ne(&0), ErrorCode::InvalidProduct);

    // let transaction_fee = ctx.accounts.admin.transaction_fees; // 500 -> 5%
    let transaction_fee = ctx
        .accounts
        .product
        .sales_amount
        .checked_mul(ctx.accounts.admin.transaction_fees as u64)
        .unwrap()
        .checked_div(10000)
        .unwrap();
    let withdraw_amount = ctx
        .accounts
        .product
        .sales_amount
        .checked_sub(transaction_fee)
        .unwrap();

    **ctx
        .accounts
        .admin
        .to_account_info()
        .try_borrow_mut_lamports()? -= withdraw_amount;
    **ctx
        .accounts
        .treasury
        .to_account_info()
        .try_borrow_mut_lamports()? += withdraw_amount;

    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SellerWithdrawParams {
    product_id: String,
}

#[derive(Accounts)]
#[instruction(params: SellerWithdrawParams)]
pub struct SellerWithdraw<'info> {
    /// CHECK:
    #[account(mut, address = product.treasury @ ErrorCode::InvalidTreasury)]
    pub treasury: UncheckedAccount<'info>,

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
			ADMIN.as_bytes(),
		],
        bump
    )]
    pub admin: Box<Account<'info, Admin>>,

    #[account(mut, address = product.administrator)]
    pub administrator: Signer<'info>,

    pub system_program: Program<'info, System>,
}
