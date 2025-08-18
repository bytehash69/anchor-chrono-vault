use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::Chrono;

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

impl<'info> Initialize<'info> {
    pub fn initialize(
        &mut self,
        lock_duration: u64,
        token_mint: Pubkey,
        bumps: &InitializeBumps,
    ) -> Result<()> {
        let clock = Clock::get()?;
        let current_time = clock.unix_timestamp as u64;
        let unlock_time = current_time + lock_duration;

        self.chrono_account.set_inner(Chrono {
            deposit_time: current_time,
            unlock_time: unlock_time,
            mint: token_mint.key(),
            bump: bumps.chrono_account,
        });

        Ok(())
    }
}
