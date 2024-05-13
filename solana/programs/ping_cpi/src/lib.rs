use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::Instruction,
    msg,
    program::invoke,
    pubkey::Pubkey,
};

// declare and export the program's entrypoint
solana_program::entrypoint!(process_instruction);

// program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Called PingCpi");

    let accounts_iter = &mut accounts.iter();
    let ping_program_id = next_account_info(accounts_iter)?;

    let instruction = Instruction::new_with_bytes(*ping_program_id.key, &vec![], vec![]);

    invoke(&instruction, &[ping_program_id.clone()])
}
