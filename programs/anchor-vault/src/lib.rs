use anchor_lang::prelude::*;

declare_id!("B8nJgmX8ZqbTbmp9UakSihNbCWbDocxFDjfMqjQ5hUnW");

pub mod errors;
pub mod instructions;
pub mod state;

pub use errors::*;
pub use instructions::*;
pub use state::*;

#[program]
pub mod anchor_vault {

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        lock_duration: u64,
        token_mint: Pubkey,
    ) -> Result<()> {
        ctx.accounts
            .initialize(lock_duration, token_mint, &ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)?;
        Ok(())
    }
}
