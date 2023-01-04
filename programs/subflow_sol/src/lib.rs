pub mod error;
pub mod id;
mod instructions;
pub mod state;

pub use error::SubflowError;
pub use id::ID;

use anchor_lang::prelude::*;
use instructions::*;

#[program]
pub mod subflow_sol {
    use super::*;

    pub fn initialize(ctx: Context<InitializeSubflow>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn create_service(ctx: Context<CreateService>, name: String, uri: String) -> Result<()> {
        create_service::handler(ctx, name, uri)
    }

    pub fn add_plan(ctx: Context<AddPlan>, interval: u64, cost: u64) -> Result<()> {
        add_plan::handler(ctx, interval, cost)
    }

    pub fn pause(ctx: Context<PauseService>, duration: u8) -> Result<()> {
        pause::handler(ctx, duration)
    }

    pub fn unpause(ctx: Context<UnpauseService>) -> Result<()> {
        unpause::handler(ctx)
    }

    pub fn subscribe(ctx: Context<Subscribe>) -> Result<()> {
        subscribe::handler(ctx)
    }

    pub fn renew(ctx: Context<Renew>) -> Result<()> {
        renew::handler(ctx)
    }

    pub fn check_status(ctx: Context<CheckSubscription>, subscriber_key: Pubkey) -> Result<bool> {
        check_status::handler(ctx, subscriber_key)
    }
}
