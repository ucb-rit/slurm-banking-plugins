extern crate openapi;
extern crate rust_decimal;

use super::logging::safe_info;
use config::Config;
use rust_decimal::{Decimal, RoundingStrategy};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;

static REQUEST_TIMEOUT_SECS: u64 = 5;

fn log(message: &str) {
    safe_info(&("slurm_banking_lib: ".to_owned() + message))
}

pub fn price_per_cpu_hour(partition: &str, conf: &Config) -> Option<Decimal> {
    let prices = match conf.get::<HashMap<String, String>>("PartitionPrice") {
        Ok(prices) => prices,
        Err(_) => return None,
    };
    match prices.get(partition) {
        Some(price) => Decimal::from_str(price).ok(),
        None => None,
    }
}

pub fn qos_multiplier(qos: &str, conf: &Config) -> Option<Decimal> {
    let qos_multipliers = match conf.get::<HashMap<String, String>>("QosMultiplier") {
        Ok(multipliers) => multipliers,
        Err(_) => return None,
    };
    match qos_multipliers.get(qos) {
        Some(multiplier) => Decimal::from_str(multiplier).ok(),
        None => None,
    }
}

pub fn expected_cost(
    partition: &str,
    qos: &str,
    max_cpus: u32,
    time_limit_seconds: i64,
    conf: &Config,
) -> Option<Decimal> {
    let max_cpus = Decimal::from(max_cpus);
    let time_limit_seconds = Decimal::from(time_limit_seconds);
    let time_limit_hours = time_limit_seconds / Decimal::new(60 * 60, 0);
    let hourly_price = match price_per_cpu_hour(partition, conf) {
        Some(hourly_price) => hourly_price,
        None => return None,
    };
    let qos_multiplier = match qos_multiplier(qos, conf) {
        Some(qos_multiplier) => qos_multiplier,
        None => Decimal::from(1),
    };
    Some(
        (hourly_price * max_cpus * time_limit_hours * qos_multiplier)
            .round_dp_with_strategy(2, RoundingStrategy::RoundHalfUp),
    )
}

pub fn check_sufficient_funds(
    base_path: String,
    auth_token: &str,
    job_cost: Decimal,
    user_id: &str,
    account_id: &str,
) -> Result<bool, openapi::apis::Error> {
    let mut configuration = openapi::apis::configuration::Configuration::new();
    configuration.base_path = base_path;
    configuration.client = reqwest::Client::builder()
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .build()?;
    let api_client = openapi::apis::client::APIClient::new(configuration);
    log("check_sufficient_funds: api client created");
    let job_cost_str = job_cost.to_string();

    let result = api_client.can_submit_job_api().can_submit_job_read(
        &job_cost_str,
        user_id,
        account_id,
        &auth_token,
    );

    log(&format!("{:?}", result));
    let result = match result {
        Ok(response) => response,
        Err(err) => return Err(err),
    };
    match result.success {
        Some(value) => Ok(value),
        None => Ok(false), // Got a response from the API, but not containing the success field
    }
}

pub fn create_job(
    base_path: String,
    auth_token: &str,
    job_create_record: openapi::models::Job,
) -> Result<(), openapi::apis::Error> {
    let mut configuration = openapi::apis::configuration::Configuration::new();
    configuration.base_path = base_path;
    configuration.client = reqwest::Client::builder()
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .build()?;
    let api_client = openapi::apis::client::APIClient::new(configuration);
    log("create_job: api client created");

    let result = api_client
        .jobs_api()
        .jobs_create(job_create_record, auth_token);

    log(&format!("create_job response: {:?}", result));
    Ok(())
}

pub fn update_job(
    base_path: String,
    auth_token: &str,
    jobslurmid: &str,
    job_update_record: openapi::models::Job,
) -> Result<(), openapi::apis::Error> {
    let mut configuration = openapi::apis::configuration::Configuration::new();
    configuration.base_path = base_path;
    configuration.client = reqwest::Client::builder()
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .build()?;
    let api_client = openapi::apis::client::APIClient::new(configuration);
    log("update_job: api client created");

    let result = api_client
        .jobs_api()
        .jobs_update(jobslurmid, job_update_record, auth_token);

    log(&format!("update_job response: {:?}", result));
    Ok(())
}
