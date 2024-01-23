use borsh::BorshDeserialize;
use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct ValueArgs {
    pub value: u32,
}

pub enum CounterInstructions {
    Increment(ValueArgs),
    Decrement(ValueArgs),
    Update(ValueArgs),
    Reset,
}

impl CounterInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => Self::Increment(ValueArgs::try_from_slice(rest).unwrap()),
            1 => Self::Decrement(ValueArgs::try_from_slice(rest).unwrap()),
            2 => Self::Update(ValueArgs::try_from_slice(rest).unwrap()),
            3 => Self::Reset,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
