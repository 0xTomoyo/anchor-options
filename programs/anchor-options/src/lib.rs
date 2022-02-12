use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod math;
pub mod state;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod anchor_options {
    use super::*;

    /// Initialize a new option
    pub fn init_option(
        ctx: Context<InitializeOption>,
        strike_price: u64,
        expiry_timestamp: i64,
        is_put: bool,
    ) -> ProgramResult {
        instructions::init_option::handler(ctx, strike_price, expiry_timestamp, is_put)
    }
}
