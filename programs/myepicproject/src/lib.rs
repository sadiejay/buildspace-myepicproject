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


// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
}