use std::convert::TryInto;
use solana_program::program_error::ProgramError;

use crate::error::EscrowError::InvalidInstruction;

pub enum EscrowInstruction {

    /// Starts the trade by creating and populating an escrow account and transferring ownership of the given temp token account to the PDA
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the escrow
    /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
    /// 2. `[]` The initializer's token account for the token they will receive should the trade go through
    /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade.
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token program
    InitEscrow {
        /// The amount party A expects to receive of token Y
        amount: u64
    }
}

impl EscrowInstruction {
    /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // split the input bytes into the first u8 and the remaining u8s, if empty, return error
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        // using the first byte (tag) match it to a instruction
        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            // if there is no such instruction- error
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            // only considers the first 8 bytes of input, if theres not 8 bytes it later errors out at the end
            .get(..8)
            // looks like it just dereferences here? Not sure
            .and_then(|slice| slice.try_into().ok())
            // turns 8 seperate u8s into a single u64
            .map(u64::from_le_bytes)
            // error if the option type is none, aka if there werent 8 bytes at the start
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}