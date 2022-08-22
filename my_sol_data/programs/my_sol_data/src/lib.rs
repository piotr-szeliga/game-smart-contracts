mod ins;
mod state;

use anchor_lang::prelude::*;
use ins::{InitMyData, UpdateMyData, *};

//replace the program id that you get after deploying the program
declare_id!("FV7LzNMAD7SGrnrppeMrN1HvrsW7UMPeXH5abx8NUAQY");

#[program]
pub mod my_sol_data
{
    use super::*;
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    pub fn init_data(ctx: Context<InitMyData>) -> ProgramResult {

        let acc = &mut ctx.accounts.data;
        acc.number = 0;
        acc.message = String::from("MyData initialized!");

        // we store the public key of the signer to the owner field
        // of MyData
        acc.owner = ctx.accounts.owner.key();

        Ok(())
    }

    pub fn update_data (ctx : Context<UpdateMyData>, number : u8, message : String) -> ProgramResult {

        let acc = &mut ctx.accounts.data;
        acc.number = number;
        acc.message = message;

        Ok(())
    }
}

// #[derive(Accounts)]
// pub struct InitMyData<'info>
// {
//     #[account(init, payer = owner, space = 8 + 1 + 50 + 32)]
//     pub data: Account<'info, MyData>,
//     #[account(mut)]
//     pub owner: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }
//
// #[derive(Accounts)]
// pub struct UpdateMyData<'info> {
//
//     #[account(mut,has_one=owner)]
//     pub data : Account<'info, MyData>,
//     #[account(mut)]
//     pub owner : Signer<'info>,
// }