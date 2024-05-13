use {
    borsh::BorshDeserialize,
    common::FailOnDivisionPayload,
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        instruction::Instruction,
        msg,
        program::invoke,
        pubkey::Pubkey,
    },
};

// declare and export the program's entrypoint
solana_program::entrypoint!(process_instruction);

// program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Called FailOnDivisionCpi");

    let payload1 = FailOnDivisionPayload::try_from_slice(instruction_data)?;
    let mut payload2 = payload1.clone();
    payload2.divisor = payload1.divisor + 1;

    let accounts_iter = &mut accounts.iter();
    let fail_on_division_program_id = next_account_info(accounts_iter)?;

    let instruction1 = Instruction::new_with_borsh(*fail_on_division_program_id.key, &payload1, vec![]);
    invoke(&instruction1, &[fail_on_division_program_id.clone()])?;

    let instruction2 = Instruction::new_with_borsh(*fail_on_division_program_id.key, &payload2, vec![]);
    invoke(&instruction2, &[fail_on_division_program_id.clone()])
}
