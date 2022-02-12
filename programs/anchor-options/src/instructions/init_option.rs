use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::errors::ErrorCode;
use crate::state::*;

#[event]
pub struct OptionEvent {
    collateral_mint: Pubkey,
    pyth_oracle_price: Pubkey,
    strike_price: u64,
    expiry_timestamp: i64,
    is_put: bool,
}

#[derive(Accounts)]
#[instruction(strike_price: u64, expiry_timestamp: i64, is_put: bool)]
pub struct InitializeOption<'info> {
    #[account(
        init,
        payer = payer,
        seeds = [b"market"],
        bump,
    )]
    pub market: Account<'info, OptionMarket>,
    pub base_mint: Account<'info, Mint>,
    #[account(
        constraint = is_put || (base_mint.key() == collateral_mint.key())
    )]
    pub collateral_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = payer,
        seeds = [b"short_note_mint", market.key().as_ref()],
        bump,
        mint::decimals = base_mint.decimals,
        mint::authority = market,
    )]
    pub short_note_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = payer,
        seeds = [b"long_note_mint", market.key().as_ref()],
        bump,
        mint::decimals = base_mint.decimals,
        mint::authority = market,
    )]
    pub long_note_mint: Account<'info, Mint>,
    #[account(
        init,
        payer = payer,
        seeds = [b"vault", market.key().as_ref()],
        bump,
        token::mint = collateral_mint,
        token::authority = market
    )]
    pub vault: Account<'info, TokenAccount>,
    pub pyth_oracle_price: AccountInfo<'info>,
    pub pyth_oracle_product: AccountInfo<'info>,
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

/// Initialize a new empty option market
pub fn handler(
    ctx: Context<InitializeOption>,
    strike_price: u64,
    expiry_timestamp: i64,
    is_put: bool,
) -> ProgramResult {
    if expiry_timestamp <= Clock::get()?.unix_timestamp {
        return Err(ErrorCode::OptionExpired.into());
    }

    // Validate pyth oracle
    let product_data = ctx.accounts.pyth_oracle_product.try_borrow_data()?;
    let product = match pyth_client::load_product(&product_data) {
        Ok(val) => val,
        Err(_) => return Err(ErrorCode::PythError.into()),
    };
    if crate::utils::read_pyth_product_attribute(&product.attr, b"quote_currency").is_none() {
        return Err(ErrorCode::InvalidProduct.into());
    }
    if product.px_acc.val[..] != ctx.accounts.pyth_oracle_price.key().to_bytes() {
        return Err(ErrorCode::InvalidOracle.into());
    }

    ctx.accounts.market.collateral_mint = ctx.accounts.collateral_mint.key();
    ctx.accounts.market.short_note_mint = ctx.accounts.short_note_mint.key();
    ctx.accounts.market.long_note_mint = ctx.accounts.long_note_mint.key();
    ctx.accounts.market.vault = ctx.accounts.vault.key();
    ctx.accounts.market.bumps = OptionBumps {
        market: *ctx.bumps.get("market").unwrap(),
        short_note_mint: *ctx.bumps.get("short_note_mint").unwrap(),
        long_note_mint: *ctx.bumps.get("long_note_mint").unwrap(),
        vault: *ctx.bumps.get("vault").unwrap(),
    };

    ctx.accounts.market.pyth_oracle_price = ctx.accounts.pyth_oracle_price.key();
    ctx.accounts.market.strike_price = strike_price;
    ctx.accounts.market.expiry_timestamp = expiry_timestamp;
    ctx.accounts.market.is_put = is_put;

    emit!(OptionEvent {
        strike_price,
        expiry_timestamp,
        is_put,
        collateral_mint: ctx.accounts.collateral_mint.key(),
        pyth_oracle_price: ctx.accounts.pyth_oracle_price.key(),
    });

    Ok(())
}
