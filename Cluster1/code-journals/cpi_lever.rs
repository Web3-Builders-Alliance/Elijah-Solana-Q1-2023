// Starts with implementing of all necessary objects from Solana program. 
// also the program uses the "Borsh" crate to serialize/deserialize data
use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
    account_info::{
        next_account_info, AccountInfo
    },
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

// creates two structs that deriving 2 borsh traits and Debug trait
// that allows to print data with ":?" to console
//the struct represents a name of the last user that switched the power status
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct SetPowerStatus {
    pub name: String,
}

// this struct represents the power status of the "power" account;
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PowerStatus {
    pub is_on: bool,
}

// Here we exclude the "entrypoint" feature to avoid a conflict
// between programs that rely on each other while making CPIs
#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

// here the fn accepts common parameters for any native solana program
// that I explained in the previous journal deeply kinda
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    // uses match pattern to match the "instruct_data" deserialization result
    // calls the "initialize" fn if a bool was passed in
    // otherwise returns empty object and goes to the next match
    match PowerStatus::try_from_slice(&instruction_data) {
        Ok(power_status) => return initialize(program_id, accounts, power_status),
        Err(_) => {},
    }

    // matches the result of the "instruction_date" deserialization
    // if a string was passed in, calles the "swtich_power" fn to switch the power status;
    match SetPowerStatus::try_from_slice(&instruction_data) {
        Ok(set_power_status) => return switch_power(accounts, set_power_status.name),
        Err(_) => {},
    }

    // returns an error if the instruction data passed in the fn is invalid;
    Err(ProgramError::InvalidInstructionData)
}

pub fn initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    power_status: PowerStatus,
) -> ProgramResult {

    // fn starts with the itrerator creation, then it uses "next_account_info"
    // to access each "account_info" in the account_infos vector
    let accounts_iter = &mut accounts.iter();
    let power = next_account_info(accounts_iter)?;
    let user = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // Trying to get the length of the vec of bytes serizlized from the "power_status", the size of an account (capacity),
    // then tries to get the amount of lamports to become a rent exempt for the particular length.
    let account_span = (power_status.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    // Here a CPI occurs, we're calling the system_program from this program to create an account.
    // First of all, we pass all the arguments that needed to create an account,
    // then we pass all the accounts accessed by the other program (system_program in our case).
    invoke(
        &system_instruction::create_account(
            &user.key,
            &power.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[
            user.clone(), power.clone(), system_program.clone()  // accounts are cloned here to pass the data itself, 
        ]                                                    // not just pointers to the data
    )?;

    // here the serialization occurs, we pass in the mutably borrowed value to update the data;
    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?;

    Ok(())
}

// this function will change the "Power Status" of a "power" account
// accepts the "power" account and a string that represents the name of the user
pub fn switch_power(
    accounts: &[AccountInfo],
    name: String,
) -> ProgramResult {

    // creates an iterator over the account_infos, then gets the first acc ("power")
    let accounts_iter = &mut accounts.iter();
    let power = next_account_info(accounts_iter)?;
    
    // here the deserialization occurs of the status occurs, we get a mut value to change the status
    let mut power_status = PowerStatus::try_from_slice(&power.data.borrow())?;

    // switches the "is_on" bool (pulls the "power switch")
    power_status.is_on = !power_status.is_on;

    // serializes data back to an arr of bytes
    power_status.serialize(&mut &mut power.data.borrow_mut()[..])?;

    // prints the name of the user pulled the "power switch"
    msg!("{} is pulling the power switch!", &name);

    // logs the power switch status
    match power_status.is_on {
        true => msg!("The power is now on."),
        false => msg!("The power is now off!"),
    };

    Ok(())
}

/* The main one is borrowing (references to Pubkey, array of data bytes; 
mutable references of account_info, immutable reference to a string just to print it out).
It also uses the "clone()"" method to create a copy of data to pass into a function when it's necessary.

Others, like vectors (account_infos), ifs, macros (msg, entrypoint) and others are very basic and common 
for a native sol program.

The program EITHER switches the "power status" of a "power" account OR creates a new "power" account
depending on the instruction data passed. 

To create an account, it uses the CPI with the "invoke" associated function
(in this cases - the system program call to create a new account).

I would better match not over the "instruction_data" context itself, but a u8 passed in to determine the function to call.
*/