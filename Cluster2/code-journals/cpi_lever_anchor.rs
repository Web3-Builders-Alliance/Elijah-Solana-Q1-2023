use anchor_lang::prelude::*; // it's required for any anchor programs
                             // brings everything needs for anchor 

declare_id!("EnjN3cm7xYqYHNUZbQfhJYj5S5RBrSU9tc5aHwQ6LqvT"); // defines th program id
 // I assume it should match the ID mentioned in the "anchor.toml" file


#[program] // program entrypoint with instructions
pub mod lever {
    use super::*; 
    pub fn initialize(_ctx: Context<InitializeLever>) -> Result<()> {
        // the fn doesn't have any code implemetation since the initialization occurs
        // during the checks that "InitializeLever" constraints do
        Ok(())
    }

    // besides context, the fn accepts an adition data which is a string 
    pub fn switch_power(ctx: Context<SetPowerStatus>, name: String) -> Result<()> { 
        
        let power = &mut ctx.accounts.power; // power is a mut ref to the "power" account
        power.is_on = !power.is_on; // switching the power state

        msg!("{} is pulling the power switch!", &name);

        match power.is_on { // match pattern to log a message depending on the power state
            true => msg!("The power is now on."),
            false => msg!("The power is now off!"),
        };

        Ok(()) // returns Ok(()) if no error has occurred
    }
}


#[derive(Accounts)]
pub struct InitializeLever<'info> {
    #[account(init, payer = user, space = 8 + 8)] // creates an account with the user as payer and
    #[account(mut)]                 // with a particular space allocated (first 8 bytes are for the descrimintator)
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>, // I found out that the "system_program" must be named as 
                                                // the "system_program" in case we use "init" attribute
}

#[derive(Accounts)]
pub struct SetPowerStatus<'info> { // the struct contains an account of the "PowerStatus" type
    #[account(mut)]
    pub power: Account<'info, PowerStatus>,
}

#[account]
pub struct PowerStatus { // the struct contains a simple bool as data
    pub is_on: bool,     // "On/Off"
}

/* I wasn't explaining the very basics/details since I mentioned almost all of them in the previous journal. 

The program basically initializes a power account and switches it's power status. 
It also displays a name passed into the fn.

The program is simple, I like how clean and simple it looks. The native variant of it looks way
more complicated and confusing. The power of anchor is obvious for me from now on.
 */