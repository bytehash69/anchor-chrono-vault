use anchor_lang::prelude::*;

declare_id!("B8nJgmX8ZqbTbmp9UakSihNbCWbDocxFDjfMqjQ5hUnW");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
