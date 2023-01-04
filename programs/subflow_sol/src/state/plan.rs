use anchor_lang::prelude::*;

#[account]
pub struct Plan {
    pub cost_per_interval: u64,
    pub interval_in_days: u64,
    pub bump: u8,
    pub service: Pubkey,
}

impl Plan {
    pub const SIZE: usize = 8 + 8 + 1 + 32;
}
