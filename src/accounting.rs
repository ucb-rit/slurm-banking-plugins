extern crate rust_decimal;

use rust_decimal::Decimal;
use std::collections::HashMap;
use std::str::FromStr;

pub fn price_per_cpu_hour(partition: &str, prices: &HashMap<String, String>) -> Option<Decimal> {
    match prices.get(partition) {
        Some(price) => Decimal::from_str(price).ok(),
        None => None,
    }
}

pub fn deduct_service_units(_account: &str, _user_id: u32, _amount: Decimal) -> Result<(), String> {
    Ok(())
}

pub fn expected_cost(
    partition: &str,
    max_cpus: u32,
    time_limit_minutes: u32,
    prices: &HashMap<String, String>,
) -> Option<Decimal> {
    let max_cpus = Decimal::from(max_cpus);
    let time_limit_minutes = Decimal::from(time_limit_minutes);
    let hourly_price = price_per_cpu_hour(partition, prices);
    hourly_price
        .map(|price| price * max_cpus * time_limit_minutes / Decimal::new(60, 0).round_dp(2))
}
