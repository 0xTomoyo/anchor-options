use anchor_lang::prelude::*;
use anchor_spl::{
    token,
    token::{Mint, MintTo, Token, TokenAccount, Transfer},
};

use crate::errors::ErrorCode;
use crate::math::*;
use crate::state::*;

#[event]
pub struct MintEvent {
    market: Pubkey,
    depositor: Pubkey,
    deposit_account: Pubkey,
    short_note_account: Pubkey,
    long_note_account: Pubkey,
    collateral: u64,
    options: u64,
}

#[derive(Accounts)]
#[instruction(collateral: u64)]
pub struct MintOptions<'info> {
    /// Option account
    pub market: Box<Account<'info, OptionMarket>>,

    /// PDA which has authority over all assets in the market
    #[account(
        constraint = market.market_authority == market_authority.key()
    )]
    pub market_authority: AccountInfo<'info>,

    /// Mint account for the base token
    #[account(
        constraint = market.base_mint == base_mint.key()
    )]
    pub base_mint: Box<Account<'info, Mint>>,

    /// Mint account for the collateral token (should be same as base_mint if the option is a call)
    #[account(
        constraint = market.collateral_mint == collateral_mint.key()
    )]
    pub collateral_mint: Box<Account<'info, Mint>>,

    /// Mint account for notes that represent a short option
    #[account(
        constraint = market.short_note_mint == short_note_mint.key()
    )]
    pub short_note_mint: Box<Account<'info, Mint>>,

    /// Mint account for notes that represent a long option
    #[account(
        constraint = market.long_note_mint == long_note_mint.key()
    )]
    pub long_note_mint: Box<Account<'info, Mint>>,

    /// Vault with custody over the collateral tokens
    #[account(
        constraint = market.vault == vault.key()
    )]
    pub vault: Box<Account<'info, TokenAccount>>,

    /// The account where a Pyth oracle keeps the updated price of the token
    #[account(
        constraint = market.pyth_oracle_price == pyth_oracle_price.key()
    )]
    pub pyth_oracle_price: AccountInfo<'info>,

    /// The token account to receive the short option notes
    pub short_note_account: Box<Account<'info, TokenAccount>>,

    /// The token account to receive the long option notes
    pub long_note_account: Box<Account<'info, TokenAccount>>,

    /// The token account with the collateral to be deposited
    pub deposit_account: Box<Account<'info, TokenAccount>>,

    /// Signer
    pub depositor: Signer<'info>,

    /// Token program
    pub token_program: Program<'info, Token>,
}

impl<'info> MintOptions<'info> {
    fn transfer_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.deposit_account.to_account_info(),
                to: self.vault.to_account_info(),
                authority: self.depositor.to_account_info(),
            },
        )
    }

    fn short_note_mint_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                to: self.short_note_account.to_account_info(),
                mint: self.short_note_mint.to_account_info(),
                authority: self.market_authority.to_account_info(),
            },
        )
    }

    fn long_note_mint_context(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            MintTo {
                to: self.long_note_account.to_account_info(),
                mint: self.long_note_mint.to_account_info(),
                authority: self.market_authority.to_account_info(),
            },
        )
    }
}

/// Deposit collateral and mint options
pub fn handler(ctx: Context<MintOptions>, collateral: u64) -> ProgramResult {
    if ctx.accounts.market.expiry_timestamp <= Clock::get()?.unix_timestamp {
        return Err(ErrorCode::OptionExpired.into());
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

    let options = calculate_option_amount(
        collateral,
        ctx.accounts.market.strike_price,
        ctx.accounts.market.is_put,
        ctx.accounts.collateral_mint.decimals,
        ctx.accounts.base_mint.decimals,
        price.expo,
    );

    token::transfer(ctx.accounts.transfer_context(), collateral)?;

    let market = ctx.accounts.market.key();
    let seeds = &[
        b"market_authority",
        market.as_ref(),
        &[ctx.accounts.market.bumps.market_authority],
    ];

    token::mint_to(
        ctx.accounts.short_note_mint_context().with_signer(&[seeds]),
        options,
    )?;

    token::mint_to(
        ctx.accounts.long_note_mint_context().with_signer(&[seeds]),
        options,
    )?;

    emit!(MintEvent {
        collateral,
        options,
        market: ctx.accounts.market.key(),
        depositor: ctx.accounts.depositor.key(),
        deposit_account: ctx.accounts.deposit_account.key(),
        short_note_account: ctx.accounts.short_note_account.key(),
        long_note_account: ctx.accounts.long_note_account.key(),
    });

    Ok(())
}
