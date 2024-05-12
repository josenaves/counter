use borsh::{BorshDeserialize, BorshSerialize};
// use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct UserArgs {
    pub value: u32,
}

pub enum CounterInstructions {
    Increment(UserArgs),
    Decrement(UserArgs),
    Update(UserArgs),
    Reset,
}

impl CounterInstructions {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => Self::Increment(UserArgs::try_from_slice(rest).unwrap()),
            1 => Self::Decrement(UserArgs::try_from_slice(rest).unwrap()),
            2 => Self::Update(UserArgs::try_from_slice(rest).unwrap()),
            3 => Self::Reset,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
