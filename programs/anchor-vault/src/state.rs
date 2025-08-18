use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Chrono {
    pub deposit_time: u64,
    pub unlock_time: u64,
    pub mint: Pubkey,
    pub bump: u8,
}
