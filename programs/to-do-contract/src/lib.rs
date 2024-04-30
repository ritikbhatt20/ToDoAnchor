use anchor_lang::prelude::*;

declare_id!("AeXPYzizpsK4pjooMkfy5f35aujposyF5sH7odrT9hM5");

#[program]
pub mod to_do_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
