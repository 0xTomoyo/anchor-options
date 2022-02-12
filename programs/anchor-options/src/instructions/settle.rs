use anchor_lang::prelude::*;

use crate::errors::ErrorCode;
use crate::state::*;

#[event]
pub struct SettleEvent {
    pub market: Pubkey,
    pub expiry_price: u64,
}

#[derive(Accounts)]
pub struct SettleOption<'info> {
    /// Option account
    pub market: Box<Account<'info, OptionMarket>>,

    /// The account where a Pyth oracle keeps the updated price of the token
    #[account(
        constraint = market.pyth_oracle_price == pyth_oracle_price.key()
    )]
    pub pyth_oracle_price: AccountInfo<'info>,
}

/// Settles an option by recording the expiry price
pub fn handler(ctx: Context<SettleOption>) -> ProgramResult {
    if ctx.accounts.market.expiry_timestamp > Clock::get()?.unix_timestamp {
        return Err(ErrorCode::OptionNotExpired.into());
    }

    if ctx.accounts.market.expiry_price != 0 {
        return Ok(());
    }

    let oracle_data = ctx.accounts.pyth_oracle_price.try_borrow_data()?;

    let oracle = match pyth_client::load_price(&oracle_data) {
        Ok(val) => val,
        Err(_) => return Err(ErrorCode::PythError.into()),
    };
    let price = match oracle.get_current_price() {
        None => return Err(ErrorCode::PriceError.into()),
        Some(val) => val,
    };

    if price.price < 0 {
        return Err(ErrorCode::PriceError.into());
    }

    ctx.accounts.market.expiry_price = price.price as u64;

    emit!(SettleEvent {
        market: ctx.accounts.market.key(),
        expiry_price: price.price as u64,
    });

    Ok(())
}
