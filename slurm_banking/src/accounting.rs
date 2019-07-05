extern crate futures;
extern crate rust_decimal;
extern crate swagger;
extern crate tokio_core;

use config::Config;
use futures::future::Future;
use rust_decimal::{Decimal, RoundingStrategy};
use std::collections::HashMap;
use std::str::FromStr;
use super::logging::safe_info;

fn log(message: &str) {
    safe_info(&("slurm_banking_lib: ".to_owned() + message))
}

pub fn price_per_cpu_hour(partition: &str, conf: &Config) -> Option<Decimal> {
    let prices = match conf.get::<HashMap<String, String>>("PartitionPrice") {
        Ok(prices) => prices,
        Err(_) => return None
    };
    match prices.get(partition) {
        Some(price) => Decimal::from_str(price).ok(),
        None => None
    }
}

pub fn qos_multiplier(qos: &str, conf: &Config) -> Option<Decimal> {
    let qos_multipliers = match conf.get::<HashMap<String, String>>("QosMultiplier") {
        Ok(multipliers) => multipliers,
        Err(_) => return None
    };
    match qos_multipliers.get(qos) {
        Some(multiplier) => Decimal::from_str(multiplier).ok(),
        None => None
    }
}

pub fn expected_cost(
    partition: &str,
    qos: &str,
    max_cpus: u32,
    time_limit_minutes: i64,
    conf: &Config
) -> Option<Decimal> {
    let max_cpus = Decimal::from(max_cpus);
    let time_limit_minutes = Decimal::from(time_limit_minutes);
    let time_limit_hours = time_limit_minutes / Decimal::new(60, 0);
    log(&format!("values: {:?} {:?} {:?}", max_cpus, time_limit_minutes, time_limit_hours));
    let hourly_price = match price_per_cpu_hour(partition, conf) {
        Some(hourly_price) => hourly_price,
        None => return None
    };
    log(&format!("hourly price: {:?}", hourly_price));
    let qos_multiplier = match qos_multiplier(qos, conf) {
        Some(qos_multiplier) => qos_multiplier,
        None => Decimal::from(1)
    };
    log(&format!("qos_multiplier: {:?}", qos_multiplier));
    Some((hourly_price * max_cpus * time_limit_hours * qos_multiplier)
        .round_dp_with_strategy(2, RoundingStrategy::RoundHalfUp))
}

pub fn check_sufficient_funds(job_cost: Decimal, user_id: &str, account_id: &str) -> Result<bool, String> {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let hyper_client = hyper::client::Client::new(&core.handle());
    let configuration = swagger::apis::configuration::Configuration::new(hyper_client);
    let api_client = swagger::apis::client::APIClient::new(configuration);
    log("check_sufficient_funds: api client created");
    let job_cost_str = job_cost.to_string();
    let result = core.run(api_client.can_submit_job_api().can_submit_job_read(&job_cost_str, user_id, account_id));
    log(&format!("{:?}", result));
    let result = match result {
        Ok(response) => response,
        Err(_) => return Err("API check for sufficient funds is inaccessible.".to_string())
    };
    Ok(true)
    // match result.success() {
    //     Some(value) => value.clone(),
    //     None => false // Got a response from the API, but not containing the success field
    // }
}

pub fn create_job(job_create_record: swagger::models::Job) -> Result<(), String> {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let hyper_client = hyper::client::Client::new(&core.handle());
    let configuration = swagger::apis::configuration::Configuration::new(hyper_client);
    let api_client = swagger::apis::client::APIClient::new(configuration);
    log("create_job: api client created");
    let result = core.run(api_client.jobs_api().jobs_create(job_create_record));
    log(&format!("create_job response: {:?}", result));
    Ok(())
}

pub fn update_job(jobslurmid: &str, job_update_record: swagger::models::Job) -> Result<(), String> {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let hyper_client = hyper::client::Client::new(&core.handle());
    let configuration = swagger::apis::configuration::Configuration::new(hyper_client);
    let api_client = swagger::apis::client::APIClient::new(configuration);
    log("update_job: api client created");
    let result = core.run(api_client.jobs_api().jobs_update(jobslurmid, job_update_record));
    log(&format!("update_job response: {:?}", result));
    Ok(())
}
