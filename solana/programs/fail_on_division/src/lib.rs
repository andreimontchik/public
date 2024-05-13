use {
    borsh::BorshDeserialize,
    common::FailOnDivisionPayload,
    solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
        pubkey::Pubkey,
    },
};

// declare and export the program's entrypoint
solana_program::entrypoint!(process_instruction);

// program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Called FailOnDivision.");

    let payload = FailOnDivisionPayload::try_from_slice(instruction_data)?;
    msg!("FailOnDivision payload: {:?}", payload);
    let remainder = payload.dividend % payload.divisor;
    if remainder != payload.remainder {
        msg!(
            "FailOnDivision: the division reminder {} is not {}. The transaction is successful.",
            remainder,
            payload.remainder
        );
        Ok(())
    } else {
        msg!(
            "FailOnDivision: the division reminder {} as expected. Failing the transaction.",
            remainder
        );
        Err(ProgramError::Custom(remainder as u32))
    }
}
