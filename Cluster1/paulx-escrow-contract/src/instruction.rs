use std::convert::TryInto;
use solana_program::{
    program_error::ProgramError, 
    // rent::Rent, 
    // instruction::{Instruction, AccountMeta}, 
    // sysvar,
};

use crate::error::EscrowError::InvalidInstruction;

pub enum EscrowInstruction {
    InitEscrow{
        amount: u64
    },
    Exchange{
        amount: u64
    },
    // resets timelock and timeout
    ResetTimeLock {

    },
    // cancel escrow
    Cancel {

    }
}

impl EscrowInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            1 => Self::Exchange {
                amount: Self::unpack_amount(rest)?
            },
            2 => Self::ResetTimeLock {  },
            3 => Self::Cancel {  },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}