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
      Ok(())
    }
  }