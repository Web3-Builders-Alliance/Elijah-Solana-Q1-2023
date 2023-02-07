use anchor_lang::prelude::*;

declare_id!("Gab9ScfzUPHtbrTQw2NBPwvVXiiZY5YNvf7bJkvjeWAM");

#[program]
pub mod deposit {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
