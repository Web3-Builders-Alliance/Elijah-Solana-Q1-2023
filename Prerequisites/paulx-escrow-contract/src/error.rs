use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    #[error("Invalid instruction")]
    InvalidInstruction,

    #[error("The account is not a rent exempt")]
    NotRentExempt,

    #[error("Expected amount mismatched")]
    ExpectedAmountMismatch,

    #[error("Amount Overflow")]
    AmountOverflow
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}