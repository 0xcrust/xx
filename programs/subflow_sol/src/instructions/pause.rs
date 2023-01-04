use {
    crate::{error::SubflowError, state::*},
    anchor_lang::prelude::{Clock, *},
};

#[derive(Accounts)]
#[instruction(pause_time: u8)]
pub struct PauseService<'info> {
    subflow: Box<Account<'info, Subflow>>,
    #[account(
        mut, has_one = authority, has_one = subflow,
        constraint = pause_time <= subflow.max_pause_duration_days @
        SubflowError::ExceededMaxPauseTime,
    )]
    service: Box<Account<'info, Service>>,

    authority: Signer<'info>,
}

/// When a service is paused the following actions are restricted on it:
/// - Withdrawal of funds
/// - New subscriptions to the service. Active subscriptions are still valid
/// - Addition of new plans to the service.
///
pub fn handler(ctx: Context<PauseService>, duration: u8) -> Result<()> {
    let service = &mut ctx.accounts.service;
    let clock = Clock::get().unwrap();

    service.paused = true;
    service.active_pause_start_time = clock.unix_timestamp;
    service.active_pause_duration = duration;

    Ok(())
}
