use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::entrypoint;
use solana_program::entrypoint::ProgramResult;
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_sdk::account_info::{next_account_info, AccountInfo};

/// Define the type of state stored in accounts
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
}

/// Define instructions that could be passed to the entrypoint
#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum Instructions {
    GreetUser { username: String },
    GreetAnonymous,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

/// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello world Rust program entrypoint");

    // Interacting accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of tiems the account has been greeted
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    greeting_account.counter += 1;
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Greeted {} time(s)!", greeting_account.counter);

    Ok(())
}
