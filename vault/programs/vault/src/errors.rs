use anchor_lang::prelude::*;


#[error_code]
pub enum VaultError {
    #[msg("Bro, rent to chhod de")]
    MaxWithdrawableAmountError,
}
