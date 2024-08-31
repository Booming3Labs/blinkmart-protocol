use anchor_lang::prelude::*;

use crate::{Product, PRODUCT};

pub fn delisting(_ctx: Context<Delisting>, _params: DelistingParams) -> Result<()> {
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

    pub system_program: Program<'info, System>,
}
