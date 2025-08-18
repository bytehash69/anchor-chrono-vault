use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

declare_id!("B8nJgmX8ZqbTbmp9UakSihNbCWbDocxFDjfMqjQ5hUnW");

#[program]
pub mod anchor_vault {

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        lock_duration: u64,
        token_mint: Pubkey,
    ) -> Result<()> {
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp as u64;
        let unlock_time = current_time + lock_duration;

        ctx.accounts.chrono_account.set_inner(Chrono {
            deposit_time: current_time,
            unlock_time: unlock_time,
            mint: token_mint.key(),
        });

        Ok(())
    }

    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                authority: ctx.accounts.user.to_account_info(),
                from: ctx.accounts.user_ata.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
            },
        );

        transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp as u64;

        require!(
            current_time >= ctx.accounts.chrono_account.unlock_time,
            Errors::TokensStillLocked
        );

        let seeds: &[&[&[u8]]] = &[&[
            b"chrono",
            ctx.accounts.user.key.as_ref(),
            &[ctx.bumps.chrono_account],
        ]];

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                authority: ctx.accounts.chrono_account.to_account_info(),
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.user_ata.to_account_info(),
            },
            seeds,
        );

        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        space = 8 + Chrono::INIT_SPACE,
        seeds = [b"chrono", user.key().as_ref()],
        bump
    )]
    pub chrono_account: Account<'info, Chrono>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = chrono_account,
        associated_token::token_program = token_program
    )]
    pub vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct VaultAction<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"chrono", user.key().as_ref()],
        bump
    )]
    pub chrono_account: Account<'info, Chrono>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = chrono_account,
        associated_token::token_program = token_program
    )]
    pub vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[account]
#[derive(InitSpace)]
pub struct Chrono {
    deposit_time: u64,
    unlock_time: u64,
    mint: Pubkey,
}

#[error_code]
pub enum Errors {
    #[msg("Lock time is not completed yet")]
    TokensStillLocked,
}
