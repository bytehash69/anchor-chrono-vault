use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{Chrono, Errors};

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

impl<'info> VaultAction<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                authority: self.user.to_account_info(),
                from: self.user_ata.to_account_info(),
                to: self.vault.to_account_info(),
            },
        );

        transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp as u64;

        require!(
            current_time >= self.chrono_account.unlock_time,
            Errors::TokensStillLocked
        );

        let seeds: &[&[&[u8]]] = &[&[
            b"chrono",
            self.user.key.as_ref(),
            &[self.chrono_account.bump],
        ]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            Transfer {
                authority: self.chrono_account.to_account_info(),
                from: self.vault.to_account_info(),
                to: self.user_ata.to_account_info(),
            },
            seeds,
        );

        transfer(cpi_ctx, amount)?;
        Ok(())
    }
}
