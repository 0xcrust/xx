use anchor_lang::prelude::*;

#[account]
pub struct Subflow {
    // Subflow creator
    pub admin: Pubkey,
    // total number of active services on subflow
    pub active_services: u64,

    // PDA  bump
    pub bump: u8,
    pub max_pause_duration_days: u8,
}

impl Subflow {
    pub const SIZE: usize = 32 + 8 + 1 + 1;
}
