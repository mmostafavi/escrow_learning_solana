// inside error.rs
use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Account Not Rent Exempt")]
    NotRentExempt,

    #[error("Expected Amount Missmacth")]
    ExpectedAmountMissmatch,

    #[error("Amount Over Flow")]
    AmountOverFlow,
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
