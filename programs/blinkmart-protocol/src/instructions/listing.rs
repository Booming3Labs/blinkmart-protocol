use anchor_lang::prelude::*;

use crate::{Product, PRODUCT};

pub fn listing(ctx: Context<Listing>, params: ListingParams) -> Result<()> {
    ctx.accounts.product.set_inner(Product {
        product_id: params.product_id,
        administrator: params.administrator,
        // administrator: ctx.accounts.authority.key(),
        treasury: params.treasury,
        sales_price: params.sales_price,
        // inventory: params.inventory,
        bump: ctx.bumps.product,
        sales_amount: 0,
    });
    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ListingParams {
    product_id: String,
    administrator: Pubkey,
    treasury: Pubkey,
    sales_price: u64,
    // inventory: u64,
}

#[derive(Accounts)]
#[instruction(params: ListingParams)]
pub struct Listing<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        space= Product::LEN,
        payer = authority,
        seeds=[
			PRODUCT.as_bytes(),
            params.product_id.as_bytes(),
            // &hash::hash(params.product_id.as_bytes()).to_bytes()
		],
        bump
    )]
    pub product: Box<Account<'info, Product>>,

    pub system_program: Program<'info, System>,
}
