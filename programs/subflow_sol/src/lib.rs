use anchor_lang::{prelude::*, solana_program::clock};
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

const DAY_IN_SECONDS: u64 = 60 * 60 * 24;

#[program]
pub mod subflow_sol {
    use super::*;

    pub fn initialize_subflow(ctx: Context<InitializeSubflow>) -> Result<()> {
        let subflow = &mut ctx.accounts.subflow;

        subflow.admin = ctx.accounts.authority.key();
        subflow.active_services = 0;
        subflow.bump = *ctx.bumps.get("subflow").unwrap();
        // cannot pause for more than 30 days at a time
        subflow.max_pause_duration_days = 30;

        Ok(())
    }

    pub fn initialize_service(ctx: Context<InitializeService>, name: String, uri: String) -> Result<()> {
        require!(
            name.chars().count <= Service::MAX_NAME_LENGTH,
            SubflowError::MaxServiceNameExceeded
        );
        require!(
            uri.chars().count <= Service::URI_LENGTH,
            SubflowError::MaxURILengthExceeded
        )
        let subflow = &mut ctx.accounts.subflow;
        subflow.active_services = subflow.active_services.checked_add(1).unwrap();

        let service = &mut ctx.accounts.subflow;
        service.subflow = ctx.accounts.subflow.key();
        service.name = name;
        service.image_uri = uri;
        service.authority = ctx.accounts.authority.key();
        service.active_plans = 0;
        service.bump = *ctx.bumps.get("service").unwrap();
        service.vault = ctx.accounts.vault.key();
        service.mint = ctx.accounts.vault.key();

        service.paused = false;
        service.active_pause_start_time = 0;
        service.active_pause_duration = 0;

        Ok(())
    }

    /// When a service is paused the following actions are restricted on it:
    /// - Withdrawal of funds
    /// - New subscriptions to the service. Active subscriptions are still valid
    /// - Addition of new plans to the service.
    ///
    pub fn pause_service(ctx: Context<PauseService>, duration: u8) -> Result<()> {
        let service = &mut ctx.accounts.service;
        let clock = clock::Clock::get().unwrap();
        
        service.paused = true;
        service.active_pause_start_time = clock.unix_timestamp;
        service.active_pause_duration = duration;
    }

    pub fn unpause_service(ctx: Context<UnpauseService>) -> Result<()> {
        let service = &mut ctx.accounts.service;
        let clock = clock::Clock::get().unwrap();

        /// Check if service can be unpaused
        let now_timestamp = clock.unix_timestamp;
        let duration = service.active_pause_duration;
        let start_timestamp = service.active_pause_start_time;

        let duration_in_seconds: u64 = duration
            .checked_mul(DAY_IN_SECONDS).unwrap();
        
        require!(
            now_timestamp > start_timestamp.checked_add(duration_in_seconds).unwrap(),
            SubflowError::CantUnpauseYet
        );

        // else unpause
        service.paused = false;
        service.active_pause_start_time = 0;
        service.active_pause_duration = 0;
    }

    pub fn add_plan(ctx: Context<AddPlan>, interval: u64, cost: u64) -> Result<()> {
        let service = &mut ctx.accounts.service;
        service.active_plans = service.active_plans.checked_add(1).unwrap();

        let plan = &mut ctx.accounts.plan;
        plan.cost_per_interval = cost;
        plan.interval_in_days = interval;
        plan.bump = *ctx.bumps.get("plan").unwrap();
        plan.service = ctx.accounts.service.key();
        plan.active_subscribers = 0;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeSubflow<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Subflow::SIZE,
        seeds = ["subflow".as_bytes().as_ref(), authority.key().as_ref()],
        bump
    )]
    subflow: Box<Account<'info, Subflow>>,

    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(service_name: String)]
pub struct InitializeService<'info> {
    #[account(mut)]
    subflow: Box<Account<'info, Subflow>>,

    #[account(
        init,
        payer = authority,
        space = 8 + Service::SIZE,
        seeds = [
            service_name.as_bytes().as_ref(), subflow.key().as_ref(), 
            authority.key().as_ref()
        ],
        bump
    )]
    service: Box<Account<'info, Service>>,
    #[account(
        init,
        seeds = ["vault".as_bytes().as_ref(), service.key().as_ref()],
        payer = authority,
        token::mint = mint,
        token::authority = authority
    )]
    vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    authority: Signer<'info>,

    mint: Box<Account<'info, Mint>>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(pause_time: u8)]
pub struct PauseService<'info> {
    subflow: Box<Account<'info, Subflow>>,
    #[account(
        mut, has_one = authority, has_one = subflow
        constraint = pause_time <= subflow.max_pause_duration_days @
        SubflowError::ExceededMaxPauseTime,
    )]
    service: Box<Account<'info, Service>>,

    authority: Signer<'info>,
} 

#[derive(Accounts)]
pub struct UnpauseService<'info> {
    subflow: Box<Account<'info, Subflow>>,
    #[account(mut, has_one = authority, has_one = subflow)]
    service: Box<Account<'info, service>>,
    authority: Signer<'info>
}


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
        seeds = [interval.to_le_bytes().as_ref(),service.key().as_ref()],
        bump
    )]
    plan: Box<Account<'info, Plan>>,

    #[account(mut)]
    authority: Signer<'info>,
    system_program: Program<'info, System>
}

#[account]
pub struct Subflow {
    // Subflow creator
    admin: Pubkey,
    // total number of active services on subflow
    active_services: u64,

    // PDA  bump
    bump: u8,
    max_pause_duration_days: u8,
}

impl Subflow {
    const SIZE: usize = 32 + 8 + 1 + 1;
}

#[account]
pub struct Service {
    subflow: Pubkey,
    id: u64,
    name: String,
    image_uri: String,
    authority: Pubkey,
    active_plans: u8,
    bump: u8,
    vault: Pubkey,
    mint: Pubkey,

    /// Options to pause a service
    paused: bool,
    active_pause_start_time: i64,
    active_pause_duration: u8,
}

impl Service {
    const MAX_NAME_LENGTH: usize = 16;
    const URI_LENGTH: usize = 50;
    const SIZE: usize = (3 * 32) + 8 + (4 + Self::MAX_NAME_LENGTH) + (4 + Self::URI_LENGTH)+ 1 + 1
        + 1 + 64 + 1;
}

#[account]
pub struct Plan {
    cost_per_interval: u64,
    interval_in_days: u64,
    bump: u8,
    service: Pubkey,
    active_subscribers: u64,
}

impl Plan {
    const SIZE: usize = 8 + 8 + 1 + 32 + 8;
}

#[account]
pub struct User {
    id: u64,
    plan: Pubkey,
    subscription_end_date: i64,
    bump: u8
}

impl User {
    const SIZE: usize = 8 + 32 + 8 + 1;
}

#[error_code]
pub enum SubflowError {
    MaxServiceNameExceeded,
    MaxURILengthExceeded,
    ExceededMaxPauseTime,
    ServicePaused,
    CantUnpauseYet,
}