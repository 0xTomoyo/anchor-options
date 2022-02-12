pub fn calculate_option_amount(
    amount: u64,
    strike_price: u64,
    is_put: bool,
    collateral_decimals: u8,
    base_decimals: u8,
    pyth_exponent: i32,
) -> u64 {
    if is_put {
        let decimals = (base_decimals as i32) + pyth_exponent - (collateral_decimals as i32);
        if decimals >= 0 {
            return (amount * ((10 as u64).pow(decimals.abs() as u32))) / strike_price;
        } else {
            return (amount / strike_price) / (10 as u64).pow(decimals.abs() as u32);
        }
    } else {
        return amount;
    }
}
