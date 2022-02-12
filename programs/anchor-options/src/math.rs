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
