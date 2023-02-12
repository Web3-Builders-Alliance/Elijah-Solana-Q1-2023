use anchor_lang::{
    prelude::*,
    system_program::{self, Transfer}
};

declare_id!("GWZPQsbLKGnB8rQptKGDFsZ3PtbsYPFxhXm8NyhoftbY");

#[program]
pub mod deposit {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let vault_account = &mut ctx.accounts.vault;

        vault_account.initializer_key = *ctx.accounts.initializer.key;

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        system_program::transfer(ctx.accounts.into_deposit_vault_context(), amount)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let receiver = &mut ctx.accounts.initializer;
        let vault = &mut ctx.accounts.vault;

        **vault.to_account_info().try_borrow_mut_lamports()? -= amount;
        **receiver.try_borrow_mut_lamports()? += amount;

        Ok(())
    }

    
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(
        init_if_needed, payer = initializer, space = Vault::SIZE, 
        seeds=[b"vault", initializer.key().as_ref()], 
        bump
    )]
    pub vault: Account<'info, Vault>,
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub initializer: AccountInfo<'info>,
    #[account(
        mut,
        constraint = vault.initializer_key == initializer.key()
    )]
    pub vault: Account<'info, Vault>
}

#[account]
pub struct Vault {
    pub initializer_key: Pubkey,
}

impl Vault {
    const SIZE: usize = 80; // descriminator (8 bytes), 2 pubkeys (2 * 32 bytes) & u64 (8 bytes)
}

impl<'info> Deposit<'info> {
    fn into_deposit_vault_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.initializer.to_account_info().clone(),
            to: self.vault.to_account_info().clone()
        };
        CpiContext::new(self.system_program.to_account_info(), cpi_accounts)
    }
}