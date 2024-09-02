use crate::error::ErrorCode;
use anchor_lang::prelude::*;

use crate::{Admin, Product, ADMIN, PRODUCT};

pub fn delisting(ctx: Context<Delisting>, _params: DelistingParams) -> Result<()> {
    let mut withdraw_amount = ctx.accounts.product.sales_amount;

    if withdraw_amount.ne(&0) {
        let transaction_fee = ctx.accounts.admin.transaction_fees; // 500 -> 5%
        withdraw_amount = withdraw_amount
            .checked_mul(transaction_fee as u64)
            .unwrap()
            .checked_div(10000)
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
    }
    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct DelistingParams {
    product_id: String,
}

#[derive(Accounts)]
#[instruction(params: DelistingParams)]
pub struct Delisting<'info> {
    #[account(mut, address = product.administrator)]
    pub administrator: Signer<'info>,

    /// CHECK:
    #[account(mut, address = product.treasury @ ErrorCode::InvalidTreasury)]
    pub treasury: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds=[
			PRODUCT.as_bytes(),
            params.product_id.as_bytes()
		],
        close = administrator,
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

    pub system_program: Program<'info, System>,
}
