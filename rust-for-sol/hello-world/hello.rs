use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg
};

entrypoint!(process_instruction)

pub fn process_instruction(
    //Arguments and their types
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
    // The return type (i.e. what data type the function returns)
) -> ProgramResult{
  // Leave the body empty for now :)
}
