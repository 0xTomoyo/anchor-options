use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::errors::ErrorCode;
use crate::state::*;

#[event]
pub struct OptionEvent {
    market: Pubkey,
    base_mint: Pubkey,
    collateral_mint: Pubkey,
    short_note_mint: Pubkey,
    long_note_mint: Pubkey,
    pyth_oracle_price: Pubkey,
    strike_price: u64,
    expiry_timestamp: i64,
    is_put: bool,
}

#[derive(Accounts)]
#[instruction(strike_price: u64, expiry_timestamp: i64, is_put: bool)]
pub struct InitializeOption<'info> {
    /// Option account
    #[account(
        init,
        payer = payer,
    )]
    pub market: Account<'info, OptionMarket>,

    /// PDA which has authority over all assets in the market
    #[account(
        seeds = [b"market_authority", market.key().as_ref()],
        bump,
    )]
    pub market_authority: AccountInfo<'info>,

    /// Mint account for the base token
    pub base_mint: Account<'info, Mint>,

    /// Mint account for the collateral token (should be same as base_mint if the option is a call)
    #[account(
        constraint = is_put || (base_mint.key() == collateral_mint.key())
    )]
    pub collateral_mint: Account<'info, Mint>,

    /// Mint account for notes that represent a short option
    #[account(
        init,
        payer = payer,
        seeds = [b"short_note_mint", market.key().as_ref()],
        bump,
        mint::decimals = base_mint.decimals,
        mint::authority = market_authority,
    )]
    pub short_note_mint: Account<'info, Mint>,

    /// Mint account for notes that represent a long option
    #[account(
        init,
        payer = payer,
        seeds = [b"long_note_mint", market.key().as_ref()],
        bump,
        mint::decimals = base_mint.decimals,
        mint::authority = market_authority,
    )]
    pub long_note_mint: Account<'info, Mint>,

    /// Vault with custody over the collateral tokens
    #[account(
        init,
        payer = payer,
        seeds = [b"vault", market.key().as_ref()],
        bump,
        token::mint = collateral_mint,
        token::authority = market_authority
    )]
    pub vault: Account<'info, TokenAccount>,

    /// The account where a Pyth oracle keeps the updated price of the token
    pub pyth_oracle_price: AccountInfo<'info>,

    /// The account containing metadata about the Pyth oracle
    pub pyth_oracle_product: AccountInfo<'info>,

    /// Signer
    pub payer: Signer<'info>,

    /// Rent
    pub rent: Sysvar<'info, Rent>,

    /// System program
    pub system_program: Program<'info, System>,

    /// Token program
    pub token_program: Program<'info, Token>,
}

/// Initialize a new option
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
    if read_pyth_product_attribute(&product.attr, b"quote_currency").is_none() {
        return Err(ErrorCode::InvalidProduct.into());
    }
    if product.px_acc.val[..] != ctx.accounts.pyth_oracle_price.key().to_bytes() {
        return Err(ErrorCode::InvalidOracle.into());
    }

    ctx.accounts.market.market_authority = ctx.accounts.market_authority.key();
    ctx.accounts.market.base_mint = ctx.accounts.base_mint.key();
    ctx.accounts.market.collateral_mint = ctx.accounts.collateral_mint.key();
    ctx.accounts.market.short_note_mint = ctx.accounts.short_note_mint.key();
    ctx.accounts.market.long_note_mint = ctx.accounts.long_note_mint.key();
    ctx.accounts.market.vault = ctx.accounts.vault.key();
    ctx.accounts.market.bumps = OptionBumps {
        market_authority: *ctx.bumps.get("market_authority").unwrap(),
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
        market: ctx.accounts.market.key(),
        base_mint: ctx.accounts.base_mint.key(),
        collateral_mint: ctx.accounts.collateral_mint.key(),
        short_note_mint: ctx.accounts.short_note_mint.key(),
        long_note_mint: ctx.accounts.long_note_mint.key(),
        pyth_oracle_price: ctx.accounts.pyth_oracle_price.key(),
    });

    Ok(())
}

fn read_pyth_product_attribute<'d>(data: &'d [u8], attribute: &[u8]) -> Option<&'d [u8]> {
    let mut idx = 0;

    while idx < data.len() {
        let key_len = data[idx] as usize;
        idx += 1;

        if key_len == 0 {
            continue;
        }

        let key = &data[idx..idx + key_len];
        idx += key_len;

        let val_len = data[idx] as usize;
        idx += 1;

        let value = &data[idx..idx + val_len];
        idx += val_len;

        if key == attribute {
            return Some(value);
        }
    }

    None
}
