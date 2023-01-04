use anchor_lang::prelude::*;

#[account]
pub struct Subscriber {
    pub subscriber: Pubkey,
    pub plan: Pubkey,
    pub subscription_end_date: i64,
    pub bump: u8,
}

impl Subscriber {
    pub const SIZE: usize = 32 + 32 + 8 + 1;
}
