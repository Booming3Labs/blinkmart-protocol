use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Order {
    #[max_len(20)]
    pub order_id: String,
    pub payer: Pubkey,
    pub amount: u64,
    pub bump: u8,
}

impl Order {
    pub const LEN: usize = 8 + Order::INIT_SPACE;
}
