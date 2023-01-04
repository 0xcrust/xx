use {
    crate::state::*,
    anchor_lang::prelude::{Clock, *},
};

/// Read only. Checks if a user is subscribed
#[derive(Accounts)]
#[instruction(subscriber: Pubkey)]
pub struct CheckSubscription<'info> {
    #[account(
        mut,
        seeds = [b"subscriber".as_ref(), plan.key().as_ref(), subscriber.as_ref()],
        bump,
        has_one = plan
    )]
    subscriber_state: Box<Account<'info, Subscriber>>,

    plan: Box<Account<'info, Plan>>,
}

pub fn handler(ctx: Context<CheckSubscription>, _subscriber: Pubkey) -> Result<bool> {
    let now = Clock::get().unwrap().unix_timestamp;
    let end_date = ctx.accounts.subscriber_state.subscription_end_date;

    let user_subscribed: bool = end_date > now;

    match user_subscribed {
        true => Ok(true),
        false => Ok(false),
    }
}
