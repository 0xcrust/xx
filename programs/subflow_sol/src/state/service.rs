use anchor_lang::prelude::*;

#[account]
pub struct Service {
    pub subflow: Pubkey,
    pub id: u64,
    pub name: String,
    pub image_uri: String,
    pub authority: Pubkey,
    pub active_plans: u8,
    pub bump: u8,
    pub vault: Pubkey,
    pub mint: Pubkey,

    /// Options to pause a service
    pub paused: bool,
    pub active_pause_start_time: i64,
    pub active_pause_duration: u8,
}

impl Service {
    pub const MAX_NAME_LENGTH: usize = 16;
    pub const URI_LENGTH: usize = 50;
    pub const SIZE: usize =
        (3 * 32) + 8 + (4 + Self::MAX_NAME_LENGTH) + (4 + Self::URI_LENGTH) + 1 + 1 + 1 + 64 + 1;
}
