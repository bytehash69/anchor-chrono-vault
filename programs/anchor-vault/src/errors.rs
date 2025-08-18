use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("Lock time is not completed yet")]
    TokensStillLocked,
    #[msg("Invalid time")]
    InvalidTime,
}
