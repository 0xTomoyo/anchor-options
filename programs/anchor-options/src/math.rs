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
        let decimals = (base_decimals as i32) + pyth_exponent - (collateral_decimals as i32);
        if decimals >= 0 {
            return (collateral * ((10 as u64).pow(decimals.abs() as u32))) / strike_price;
        } else {
            return (collateral / strike_price) / (10 as u64).pow(decimals.abs() as u32);
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
        let decimals = (base_decimals as i32) + pyth_exponent - (collateral_decimals as i32);
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
        let decimals = (base_decimals as i32) + pyth_exponent - (collateral_decimals as i32);
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
