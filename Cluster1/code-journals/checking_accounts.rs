// The program starts with brining necessary "solana_program" structs, methods or associated functions
use solana_program::{
    account_info::{ AccountInfo, next_account_info }, // "next_account_info" allows 
                                                      // to iterate over a slice of account_infos
    entrypoint, 
    entrypoint::ProgramResult, // every solana program should return a result
                               // either an "Ok(())" or an "Error"
    msg,      // the message we want to display for users
    program_error::ProgramError,  // essentially an enum of error that Solana 
                                  // runtime can return in different cases
    pubkey::Pubkey,
    system_program,
};

// Solana program can have olny 1 entrypoint (in our case - the "process_instruction" function)
entrypoint!(process_instruction);


fn process_instruction(
    program_id: &Pubkey,    // program address/pubkey 
    accounts: &[AccountInfo], // a slice of account_infos that 
    _instruction_data: &[u8], // some date as a slice of bytes 
) -> ProgramResult {
    // returns true if a given pubkey is the program_id, otherwise false (and error)
    if system_program::check_id(program_id) {
        return Err(ProgramError::IncorrectProgramId)
    };

    // checks the amount of accounts passed to the insturction
    // returns an error if less than 4
    if accounts.len() < 4 {
        msg!("This instruction requires 4 accounts:");
        msg!("  payer, account_to_create, account_to_change, system_program");
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    // accounts passed in the accounts vector must be in the expected order
    let accounts_iter = &mut accounts.iter(); // creates an iterator over the accounts
    

    // here we accessing the items in the account_infos vector
    // and we know the sequence of accounts passed in the instruction
    let _payer = next_account_info(accounts_iter)?;
    // question mark operator is the "unwrap()" shorthand essentially
    // returns either a value or an error
    let account_to_create = next_account_info(accounts_iter)?; 
    let account_to_change = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // You can make sure an account has NOT been initialized.
    
    // simply displays the accounts_to_create pubkey
    msg!("New account: {}", account_to_create.key);

    // checks if the account_to_create has been initialized already, returns an error if true
    if account_to_create.lamports() != 0 {
        msg!("The program expected the account to create to not yet be initialized.");
        return Err(ProgramError::AccountAlreadyInitialized)
    };
    // (logic to create an account...)

    // checks if the account_to_change hasn't been initialized yet, returns an error if true
    msg!("Account to change: {}", account_to_change.key);
    if account_to_change.lamports() == 0 {
        msg!("The program expected the account to change to be initialized.");
        return Err(ProgramError::UninitializedAccount)
    };

    // checks if the program is the owner of the account_to_change, returns an error if it's not
    // it's neccessary for program to own the account to be able to mutate the "account_to_change" data
    if account_to_change.owner != program_id {
        msg!("Account to change does not have the correct program id.");
        return Err(ProgramError::IncorrectProgramId)
    };

    // simply checks if the "system_program" pubkey provided by the 
    // "key" field is similar to the constant
    if system_program.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId)
    };

    Ok(()) // returning an Ok variant of the ProgramResult if no error was returned
}

/* The main concept's being used here is borrowing (references to Pubkey, array of data bytes; 
    mutable references of account_info). Others, like vectors (account_infos), ifs, macros and others are very basic.

The contract simply checks if the data passed to insruction passes the checks to pefrorm safe actions.

The program is kinda simple and I'm not sure if we can make it mor eefficient and safe
Here're no code duplications and ownership moves that could have vulnerabilities potentially while used.
 */