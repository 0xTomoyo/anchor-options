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

    /// Deposit collateral and mint options
    pub fn mint(ctx: Context<MintOptions>, collateral: u64) -> ProgramResult {
        instructions::mint::handler(ctx, collateral)
    }

    /// Burn long and short options to withdraw collateral
    pub fn burn(ctx: Context<BurnOptions>, options: u64) -> ProgramResult {
        instructions::burn::handler(ctx, options)
    }

    /// Settles an option by recording the expiry price
    pub fn settle(ctx: Context<SettleOption>) -> ProgramResult {
        instructions::settle::handler(ctx)
    }
}
