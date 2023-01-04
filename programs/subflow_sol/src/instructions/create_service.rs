use {
    crate::{error::SubflowError, state::*},
    anchor_lang::prelude::*,
    anchor_spl::token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
#[instruction(service_name: String)]
pub struct CreateService<'info> {
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
        payer = authority,
        seeds = ["vault".as_bytes().as_ref(), service.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = authority
    )]
    vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    authority: Signer<'info>,

    mint: Box<Account<'info, Mint>>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CreateService>, name: String, uri: String) -> Result<()> {
    require!(
        name.chars().count() <= Service::MAX_NAME_LENGTH,
        SubflowError::MaxServiceNameExceeded
    );
    require!(
        uri.chars().count() <= Service::URI_LENGTH,
        SubflowError::MaxURILengthExceeded
    );
    let subflow = &mut ctx.accounts.subflow;
    subflow.active_services = subflow.active_services.checked_add(1).unwrap();

    let service = &mut ctx.accounts.service;
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
