use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Admin {
    pub administrator: Pubkey,
    pub treasury: Pubkey,
    pub operation: Pubkey,
    pub transaction_fees: u16,
    pub bump: u8,
}

impl Admin {
    pub const LEN: usize = 8 + Admin::INIT_SPACE;
}
