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

pub fn calculate_expired_value(
    options: u64,
    strike_price: u64,
    is_put: bool,
    expiry_price: u64,
) -> (u64, bool) {
    if is_put && (strike_price > expiry_price) {
        let payout = (strike_price - expiry_price) * options;
        return (payout, true);
    } else if !is_put && (expiry_price > strike_price) {
        let payout = ((expiry_price - strike_price) * options) / expiry_price;
        return (payout, true);
    } else {
        return (0, false);
    }
}
