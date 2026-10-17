Исходный код смарт-контракта (src/lib.rs):
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello, Solana! Мой первый смарт-контракт успешно выполнен.");
    msg!("Program ID: {}", program_id);
    msg!("Количество переданных аккаунтов: {}", accounts.len());

    Ok(())
}
