use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod subflow_sol {
    use super::*;

    pub fn initialize_subflow(ctx: Context<InitializeSubflow>) -> Result<()> {
        let subflow = &mut ctx.accounts.subflow;

        subflow.admin = ctx.accounts.authority.key();
        subflow.active_services = 0;
        subflow.bump = *ctx.bumps.get("subflow").unwrap();

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
}

impl Subflow {
    const SIZE: usize = 32 + 8 + 1;
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

    /* Options to pause a service */
    // paused: bool,
    // pause_start_time: i64,
    // max_pause_duration_days: u8,
}

impl Service {
    const MAX_NAME_LENGTH: usize = 16;
    const URI_LENGTH: usize = 50;
    const SIZE: usize = 32 + 8 + (4 + Self::MAX_NAME_LENGTH) + (4 + Self::URI_LENGTH) + 32 + 1 + 1;
}

#[account]
pub struct Plan {
    id: u64,
    cost: u64,
    interval_in_days: u64,
    bump: u8,
}

impl Plan {
    const SIZE: usize = 8 + 8 + 8 + 1;
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
}