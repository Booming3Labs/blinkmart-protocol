use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Product {
    #[max_len(40)]
    pub product_id: String,
    pub administrator: Pubkey,
    pub treasury: Pubkey,
    pub sales_price: u64,
    pub sales_amount: u64,
    // pub inventory: u64,
    pub bump: u8,
}

impl Product {
    pub const LEN: usize = 8 + Product::INIT_SPACE;
}
