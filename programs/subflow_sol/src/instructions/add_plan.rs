use {
    crate::{error::SubflowError, state::*},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(interval: u64)]
pub struct AddPlan<'info> {
    #[account(
        mut, has_one = authority,
        constraint = service.paused == false @ SubflowError::ServicePaused
    )]
    service: Box<Account<'info, Service>>,
    /// The interval is used as one of the
    /// seeds for the plan PDA because a service
    /// ideally shouldn't have different plans
    /// for the same interval.
    #[account(
        init,
        payer = authority,
        space = 8 + Plan::SIZE,
        seeds = ["plan".as_bytes().as_ref(), interval.to_le_bytes().as_ref(), service.key().as_ref()],
        bump
    )]
    plan: Box<Account<'info, Plan>>,

    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AddPlan>, interval: u64, cost: u64) -> Result<()> {
    let service = &mut ctx.accounts.service;
    service.active_plans = service.active_plans.checked_add(1).unwrap();

    let plan = &mut ctx.accounts.plan;
    plan.cost_per_interval = cost;
    plan.interval_in_days = interval;
    plan.bump = *ctx.bumps.get("plan").unwrap();
    plan.service = ctx.accounts.service.key();

    Ok(())
}
