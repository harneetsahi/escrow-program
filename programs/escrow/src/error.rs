use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Failed to withdraw tokens from the vault")]
    FailedVaultWithdrawal,

    #[msg("Failed to close the vault")]
    FailedVaultClosure,

    #[msg("Insufficient balance in taker's account")]
    InsufficientTakerBalance,
    }
