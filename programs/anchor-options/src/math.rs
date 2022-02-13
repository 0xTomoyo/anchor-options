// Calculate the amount of options mintable given the margin
pub fn calculate_option_amount(
    collateral: u64,
    strike_price: u64,
    is_put: bool,
    collateral_decimals: u8,
    base_decimals: u8,
    pyth_exponent: i32,
) -> u64 {
    if is_put {
        let decimals = (base_decimals as i32) + pyth_exponent.abs() - (collateral_decimals as i32);
        let units = (10 as u128).pow(decimals.abs() as u32);
        if decimals >= 0 {
            return (((collateral as u128) * units) / (strike_price as u128)) as u64;
        } else {
            return (((collateral as u128) / (strike_price as u128)) / units) as u64;
        }
    } else {
        return collateral;
    }
}

// Calculate the amount of margin required to mint options
pub fn calculate_collateral_amount(
    options: u64,
    strike_price: u64,
    is_put: bool,
    collateral_decimals: u8,
    base_decimals: u8,
    pyth_exponent: i32,
) -> u64 {
    if is_put {
        let decimals = (base_decimals as i32) + pyth_exponent.abs() - (collateral_decimals as i32);
        if decimals >= 0 {
            return (options * strike_price) / (10 as u64).pow(decimals.abs() as u32);
        } else {
            return (options * (10 as u64).pow(decimals.abs() as u32)) / strike_price;
        }
    } else {
        return options;
    }
}

// Calculate the payout of an expired option
pub fn calculate_expired_value(
    options: u64,
    strike_price: u64,
    expiry_price: u64,
    is_put: bool,
    collateral_decimals: u8,
    base_decimals: u8,
    pyth_exponent: i32,
) -> u64 {
    if is_put && (strike_price > expiry_price) {
        let decimals = (base_decimals as i32) + pyth_exponent.abs() - (collateral_decimals as i32);
        let payout = (strike_price - expiry_price) * options;
        if decimals >= 0 {
            return payout / (10 as u64).pow(decimals.abs() as u32);
        } else {
            return payout * (10 as u64).pow(decimals.abs() as u32);
        }
    } else if !is_put && (expiry_price > strike_price) {
        return ((expiry_price - strike_price) * options) / expiry_price;
    } else {
        return 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOL_DECIMALS: u8 = 9;
    const SRM_DECIMALS: u8 = 6;
    const TEST_DECIMALS: u8 = 2;
    const USDC_DECIMALS: u8 = 6;

    const PYTH_USD_EXPONENT: i32 = -8;

    const SOL_PRICE: u64 = 95_67553600;
    const SRM_PRICE: u64 = 2_27358175;
    const TEST_PRICE: u64 = 5_00000000;

    #[test]
    fn test_calculate_option_amount() {
        let options = calculate_option_amount(
            50_000000000,
            100_00000000,
            false,
            SOL_DECIMALS,
            SOL_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(options, 50_000000000);

        let options = calculate_option_amount(
            5000_000000,
            100_00000000,
            true,
            USDC_DECIMALS,
            SOL_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(options, 50_000000000);

        let options = calculate_option_amount(
            10_000000,
            2_00000000,
            false,
            SRM_DECIMALS,
            SRM_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(options, 10_000000);

        let options = calculate_option_amount(
            20_000000,
            2_00000000,
            true,
            USDC_DECIMALS,
            SRM_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(options, 10_000000);

        let options = calculate_option_amount(
            10_00,
            6_00000000,
            false,
            TEST_DECIMALS,
            TEST_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(options, 10_00);

        let options = calculate_option_amount(
            60_000000,
            6_00000000,
            true,
            USDC_DECIMALS,
            TEST_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(options, 10_00);
    }
}
