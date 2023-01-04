use {crate::state::*, anchor_lang::prelude::*};

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

pub fn handler<'info>(ctx: Context<InitializeSubflow>) -> Result<()> {
    let subflow = &mut ctx.accounts.subflow;

    subflow.admin = ctx.accounts.authority.key();
    subflow.active_services = 0;
    subflow.bump = *ctx.bumps.get("subflow").unwrap();
    // cannot pause for more than 30 days at a time
    subflow.max_pause_duration_days = 30;
    Ok(())
}
