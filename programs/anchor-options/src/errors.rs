use anchor_lang::prelude::*;

#[error]
pub enum ErrorCode {
    #[msg("Option Expired")]
    OptionExpired,

    #[msg("Pyth Error")]
    PythError,

    #[msg("Invalid Product")]
    InvalidProduct,

    #[msg("Invalid Oracle")]
    InvalidOracle,

    #[msg("Price Error")]
    PriceError,

    #[msg("Option Not Expired")]
    OptionNotExpired,
}
