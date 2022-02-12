use anchor_lang::prelude::*;

/// Option account
#[account]
#[derive(Default)]
pub struct OptionMarket {
    /// PDA which has authority over all assets in the market
    pub market_authority: Pubkey,

    /// The mint for the base token
    pub base_mint: Pubkey,

    /// The mint for the token used as collateral
    pub collateral_mint: Pubkey,

    /// The mint for this option's deposit notes, represents a short option
    pub short_note_mint: Pubkey,

    /// The mint for this option's option tokens, represents a long option
    pub long_note_mint: Pubkey,

    /// The account with custody over the collateral tokens
    pub vault: Pubkey,

    /// The bump seed values for pdas
    pub bumps: OptionBumps,

    /// The account where a Pyth oracle keeps the updated price of the token
    pub pyth_oracle_price: Pubkey,

    /// Strike price, must be in the same decimals as the Pyth oracle
    pub strike_price: u64,

    /// Expiry price, must be in the same decimals as the Pyth oracle
    pub expiry_price: u64,

    /// Expiry timestamp
    pub expiry_timestamp: i64,

    /// False if the option is a put, True if the option is a call
    pub is_put: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct OptionBumps {
    pub market_authority: u8,
    pub short_note_mint: u8,
    pub long_note_mint: u8,
    pub vault: u8,
}
