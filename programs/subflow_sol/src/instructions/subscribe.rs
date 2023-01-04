use {
    crate::{error::SubflowError, state::*},
    anchor_lang::prelude::{Clock, *},
    anchor_spl::token::{Token, TokenAccount, Transfer},
};

const DAY_IN_SECONDS: u64 = 60 * 60 * 24;

#[derive(Accounts)]
pub struct Subscribe<'info> {
    #[account(
        has_one = vault,
        constraint = service.paused == false @ SubflowError::ServicePaused
    )]
    service: Box<Account<'info, Service>>,

    #[account(mut, has_one = service)]
    plan: Box<Account<'info, Plan>>,

    #[account(mut)]
    vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    subscriber: Signer<'info>,
    #[account(
        mut,
        constraint = subscriber_token_account.mint == service.mint @ SubflowError::WrongMint,
        constraint = subscriber_token_account.owner == subscriber.key()
    )]
    subscriber_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
        init,
        payer = subscriber,
        space = 8 + Subscriber::SIZE,
        seeds = ["subscriber".as_bytes().as_ref(), plan.key().as_ref(), subscriber.key().as_ref()],
        bump
    )]
    subscriber_state: Box<Account<'info, Subscriber>>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<Subscribe>) -> Result<()> {
    let plan = &mut ctx.accounts.plan;
    let transfer_size = plan.cost_per_interval;

    let transfer_ix = Transfer {
        from: ctx.accounts.subscriber_token_account.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        authority: ctx.accounts.subscriber.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_ix);

    anchor_spl::token::transfer(cpi_ctx, transfer_size)?;

    let plan_interval = plan.interval_in_days;
    let plan_interval_in_seconds = plan_interval.checked_mul(DAY_IN_SECONDS).unwrap();
    let now = Clock::get().unwrap().unix_timestamp;

    let subscriber_state = &mut ctx.accounts.subscriber_state;
    subscriber_state.plan = plan.key();
    subscriber_state.subscription_end_date =
        now.checked_add(plan_interval_in_seconds as i64).unwrap();
    subscriber_state.bump = *ctx.bumps.get("subscriber_state").unwrap();
    subscriber_state.subscriber = ctx.accounts.subscriber.key();

    Ok(())
}
