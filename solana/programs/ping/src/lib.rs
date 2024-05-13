use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

// declare and export the program's entrypoint
solana_program::entrypoint!(process_instruction);

// program entrypoint's implementation
pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    // log a message to the blockchain
    msg!("Called Ping");

    // gracefully exit the program
    Ok(())
}
