use anchor_lang::prelude::*;

use crate::{Admin, ADMIN};

pub fn initialize(ctx: Context<Initialize>, params: InitializeParams) -> Result<()> {
    ctx.accounts.admin.set_inner(Admin {
        administrator: params.administrator,
        treasury: params.treasury,
        operation: params.operation,
        transaction_fees: params.transaction_fees,
        bump: ctx.bumps.admin,
    });

    Ok(())
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitializeParams {
    administrator: Pubkey,
    treasury: Pubkey,
    operation: Pubkey,
    transaction_fees: u16,
}

#[derive(Accounts)]
#[instruction(params: InitializeParams)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        space=Admin::LEN,
        payer = payer,
        seeds=[
			ADMIN.as_bytes(),
		],
        bump
    )]
    pub admin: Box<Account<'info, Admin>>,

    pub system_program: Program<'info, System>,
}
