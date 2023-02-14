use anchor_lang::prelude::*; // it's required for any anchor programs
                             // brings everything needs for anchor 


declare_id!("ECWPhR3rJbaPfyNFgphnjxSEexbTArc7vxD8fnW6tgKw"); // simply defines the program's id


#[program] // defines a module with the entrypoint to the program with instructions
pub mod anchor_program_example {
    use super::*;

    // Context type provides some "non-argument" inputs to the program
    // using the next sequence: "program_id", "deserialized/validated accs", "remaining accs"
    pub fn check_accounts(_ctx: Context<CheckingAccounts>) -> Result<()> { 

        Ok(())
    }
}

#[derive(Accounts)] // "Accounts" trait implements deserialization on the next struct
                    // anchor uses "Borsh" under the hood to (de)serialize data
pub struct CheckingAccounts<'info> {
    payer: Signer<'info>, // signer type makes the account a signer of the associated instruction
    #[account(mut)] //"mut" attribute check if the account is mutable
    /// CHECK: This account's data is empty
    account_to_create: AccountInfo<'info>, // just an account info, should be used with "/// CHECK" 
                                           // to explain why an account doesn't need to be checked
    #[account(mut)]
    /// CHECK: This account's data is empty
    account_to_change: AccountInfo<'info>,
    system_program: Program<'info, System>, // the Program type with the System generic validates
                                            // if the given program is the System Program in this case
}

/* The "check_accounts" instruction doesn't have any code implementation inside
because anchor validates/checks the context accounts under the hood, one simply needs to provide
a proper type of the account or constaint for checks to occur.

One of the concept's being used here in comparison to the "native" variant of the same program is
"Lifetimes". This ensures that the passed links lives as long as the shortest reference does. Some Rust basics.

Another important concept is being used here is the "attributes". For ex., "#[accounts()]" allows to 
use some constraints to proceed some account checkings.

The program is super simple so I don't see the way to improve it. It doesn't do anything
besides checking the provided accounts.
 */