use {
    crate::{error::SubflowError, state::*},
    anchor_lang::prelude::{Clock, *},
};

const DAY_IN_SECONDS: u64 = 60 * 60 * 24;

#[derive(Accounts)]
pub struct UnpauseService<'info> {
    subflow: Box<Account<'info, Subflow>>,
    #[account(mut, has_one = authority, has_one = subflow)]
    service: Box<Account<'info, Service>>,
    authority: Signer<'info>,
}

pub fn handler(ctx: Context<UnpauseService>) -> Result<()> {
    let service = &mut ctx.accounts.service;
    let clock = Clock::get().unwrap();

    // Check if service can be unpaused
    let now_timestamp = clock.unix_timestamp;
    let duration = service.active_pause_duration;
    let start_timestamp = service.active_pause_start_time;

    let duration_in_seconds: u64 = (duration as u64).checked_mul(DAY_IN_SECONDS).unwrap();

    require!(
        now_timestamp
            > start_timestamp
                .checked_add(duration_in_seconds as i64)
                .unwrap(),
        SubflowError::CantUnpauseYet
    );

    // else unpause
    service.paused = false;
    service.active_pause_start_time = 0;
    service.active_pause_duration = 0;
    Ok(())
}
