extern crate futures;
extern crate rust_decimal;
extern crate swagger;
extern crate tokio_core;

use futures::future::Future;
use rust_decimal::{Decimal, RoundingStrategy};
use std::collections::HashMap;
use std::str::FromStr;
use super::logging::safe_info;

fn log(message: &str) {
    safe_info(&("slurm_banking_lib: ".to_owned() + message))
}

pub fn price_per_cpu_hour(partition: &str, prices: &HashMap<String, String>) -> Option<Decimal> {
    match prices.get(partition) {
        Some(price) => Decimal::from_str(price).ok(),
        None => None,
    }
}

pub fn deduct_service_units(_account: &str, _user_id: u32, _amount: Decimal) -> Result<(), String> {
    let core = tokio_core::reactor::Core::new().unwrap();
    let hyper_client = hyper::client::Client::new(&core.handle());
    let configuration = swagger::apis::configuration::Configuration::new(hyper_client);
    let api_client = swagger::apis::client::APIClient::new(configuration);
    let job_list = api_client.jobs_api().jobs_list(0).wait();
    log(&format!("{:?}", job_list));
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
        .map(|price| price * max_cpus * time_limit_minutes / Decimal::new(60, 0).round_dp_with_strategy(2, RoundingStrategy::RoundHalfUp))
}
