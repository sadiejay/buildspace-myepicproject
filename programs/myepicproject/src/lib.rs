// kinda like import statement
use anchor_lang::prelude::*;

// anchor generated "program id" for Solana to read
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");


// calls Solana program to run module
#[program]

// defines attribute macros
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
      // Get a reference to the account.
      let base_account = &mut ctx.accounts.base_account;
      // Initialize total_gifs.
      base_account.total_gifs = 0;
      Ok(())
    }

        // Another function woo!
    pub fn add_gif(ctx: Context<AddGif>) -> ProgramResult {
        // Get a reference to the account and increment total_gifs.
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs += 1;
        Ok(())
    }
  }
// attribute macro that will specify different account constraints
#[derive(Accounts)]
// Attach certain variables to the StartStuffOff context.
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Specify what data you want in the AddGif Context.
// Getting a handle on the flow of things :)?
#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
}


// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
}