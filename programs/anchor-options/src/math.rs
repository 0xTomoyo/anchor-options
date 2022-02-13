/// Calculate the amount of options mintable given the margin
pub fn calculate_option_amount(
    collateral: u64,
    strike_price: u64,
    is_put: bool,
    collateral_decimals: u8,
    base_decimals: u8,
    pyth_exponent: i32,
) -> u64 {
    if is_put {
        // options = collateral / strike
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

/// Calculate the amount of margin required to mint options
pub fn calculate_collateral_amount(
    options: u64,
    strike_price: u64,
    is_put: bool,
    collateral_decimals: u8,
    base_decimals: u8,
    pyth_exponent: i32,
) -> u64 {
    if is_put {
        // colateral = options * strike
        let decimals = (base_decimals as i32) + pyth_exponent.abs() - (collateral_decimals as i32);
        let units = (10 as u128).pow(decimals.abs() as u32);
        if decimals >= 0 {
            return (((options as u128) * (strike_price as u128)) / units) as u64;
        } else {
            return (((options as u128) * units) / (strike_price as u128)) as u64;
        }
    } else {
        return options;
    }
}

/// Calculate the payout of an expired option
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
        // payout = (strike_price - expiry_price) * options
        let decimals = (base_decimals as i32) + pyth_exponent.abs() - (collateral_decimals as i32);
        let payout = ((strike_price - expiry_price) as u128) * (options as u128);
        let units = (10 as u128).pow(decimals.abs() as u32);
        if decimals >= 0 {
            return (payout / units) as u64;
        } else {
            return (payout * units) as u64;
        }
    } else if !is_put && (expiry_price > strike_price) {
        // payout = ((expiry_price - strike_price) * options) / expiry_price
        return (((expiry_price - strike_price) as u128) * (options as u128)
            / (expiry_price as u128)) as u64;
    } else {
        return 0;
    }
}

/// Calculate the amount of underlying collateral for an option
pub fn calculate_collateral(options: u64, total_collateral: u64, total_options: u64) -> u64 {
    // (options * total_collateral) / total_options
    return (((options as u128) * (total_collateral as u128)) / (total_options as u128)) as u64;
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOL_DECIMALS: u8 = 9;
    const SRM_DECIMALS: u8 = 6;
    const TEST_DECIMALS: u8 = 2;
    const USDC_DECIMALS: u8 = 6;

    const PYTH_USD_EXPONENT: i32 = -8;

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

    #[test]
    fn test_calculate_collateral_amount() {
        let collateral = calculate_collateral_amount(
            50_000000000,
            100_00000000,
            false,
            SOL_DECIMALS,
            SOL_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(collateral, 50_000000000);

        let collateral = calculate_collateral_amount(
            50_000000000,
            100_00000000,
            true,
            USDC_DECIMALS,
            SOL_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(collateral, 5000_000000);

        let collateral = calculate_collateral_amount(
            10_000000,
            2_00000000,
            false,
            SRM_DECIMALS,
            SRM_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(collateral, 10_000000);

        let collateral = calculate_collateral_amount(
            10_000000,
            2_00000000,
            true,
            USDC_DECIMALS,
            SRM_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(collateral, 20_000000);

        let collateral = calculate_collateral_amount(
            10_00,
            6_00000000,
            false,
            TEST_DECIMALS,
            TEST_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(collateral, 10_00);

        let collateral = calculate_collateral_amount(
            10_00,
            6_00000000,
            true,
            USDC_DECIMALS,
            TEST_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(collateral, 60_000000);
    }

    #[test]
    fn test_calculate_expired_value() {
        let payout = calculate_expired_value(
            50_000000000,
            100_00000000,
            120_00000000,
            false,
            SOL_DECIMALS,
            SOL_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(payout, 8_333333333);

        let payout = calculate_expired_value(
            50_000000000,
            100_00000000,
            80_00000000,
            false,
            SOL_DECIMALS,
            SOL_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(payout, 0);

        let payout = calculate_expired_value(
            50_000000000,
            100_00000000,
            120_00000000,
            true,
            USDC_DECIMALS,
            SOL_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(payout, 0);

        let payout = calculate_expired_value(
            50_000000000,
            100_00000000,
            80_00000000,
            true,
            USDC_DECIMALS,
            SOL_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(payout, 1000_000000);

        let payout = calculate_expired_value(
            10_000000,
            2_00000000,
            3_00000000,
            false,
            SRM_DECIMALS,
            SRM_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(payout, 3_333333);

        let payout = calculate_expired_value(
            10_000000,
            2_00000000,
            1_00000000,
            false,
            SRM_DECIMALS,
            SRM_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(payout, 0);

        let payout = calculate_expired_value(
            10_000000,
            2_00000000,
            3_00000000,
            true,
            USDC_DECIMALS,
            SRM_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(payout, 0);

        let payout = calculate_expired_value(
            10_000000,
            2_00000000,
            1_00000000,
            true,
            USDC_DECIMALS,
            SRM_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(payout, 10_000000);

        let payout = calculate_expired_value(
            10_00,
            6_00000000,
            7_00000000,
            false,
            TEST_DECIMALS,
            TEST_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(payout, 1_42);

        let payout = calculate_expired_value(
            10_00,
            6_00000000,
            5_00000000,
            false,
            TEST_DECIMALS,
            TEST_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(payout, 0);

        let payout = calculate_expired_value(
            10_00,
            6_00000000,
            7_00000000,
            true,
            USDC_DECIMALS,
            TEST_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(payout, 0);

        let payout = calculate_expired_value(
            10_00,
            6_00000000,
            5_00000000,
            true,
            USDC_DECIMALS,
            TEST_DECIMALS,
            PYTH_USD_EXPONENT,
        );
        assert_eq!(payout, 10_000000);
    }

    #[test]
    fn test_calculate_collateral() {
        let collateral = calculate_collateral(10_000000000, 10_000000000, 10_000000000);
        assert_eq!(collateral, 10_000000000);

        let collateral = calculate_collateral(5_000000000, 10_000000000, 10_000000000);
        assert_eq!(collateral, 5_000000000);

        let collateral = calculate_collateral(10_000000000, 6_000000000, 10_000000000);
        assert_eq!(collateral, 6_000000000);

        let collateral = calculate_collateral(5_000000000, 6_000000000, 10_000000000);
        assert_eq!(collateral, 3_000000000);
    }
}
