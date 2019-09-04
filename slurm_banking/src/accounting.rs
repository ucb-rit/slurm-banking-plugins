extern crate futures;
extern crate rust_decimal;
extern crate swagger;
extern crate tokio_core;

use super::logging::safe_info;
use config::Config;
use futures::future::{Either, Future};
use rust_decimal::{Decimal, RoundingStrategy};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;

static REQUEST_TIMEOUT_MS: u64 = 10 * 1000;

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
    job_cost: Decimal,
    user_id: &str,
    account_id: &str,
) -> Result<bool, String> {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let timeout = tokio_core::reactor::Timeout::new(
        Duration::from_millis(REQUEST_TIMEOUT_MS),
        &core.handle(),
    )
    .unwrap();

    let hyper_client = hyper::client::Client::new(&core.handle());
    let mut configuration = swagger::apis::configuration::Configuration::new(hyper_client);
    configuration.base_path = base_path;
    let api_client = swagger::apis::client::APIClient::new(configuration);
    log("check_sufficient_funds: api client created");
    let job_cost_str = job_cost.to_string();

    // Reference (timeouts): https://stackoverflow.com/a/45314194
    let work = api_client
        .can_submit_job_api()
        .can_submit_job_read(&job_cost_str, user_id, account_id)
        .select2(timeout)
        .then(|res| match res {
            Ok(Either::A((result, _timeout))) => Ok(result),
            Ok(Either::B((_timeout, _result))) => Err("Timed out"),
            Err(Either::A((err, _timeout))) => {
                log(&format!("{:?}", err));
                Err("Request error")
            }
            Err(Either::B((_timeout_err, _result))) => Err("Timed out"),
        });

    let result = core.run(work);
    log(&format!("{:?}", result));
    let result = match result {
        Ok(response) => response,
        Err(err) => return Err(err.to_string()),
    };
    match result.success() {
        Some(value) => Ok(*value),
        None => Ok(false), // Got a response from the API, but not containing the success field
    }
}

pub fn create_job(
    base_path: String,
    job_create_record: swagger::models::Job,
) -> Result<(), String> {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let timeout = tokio_core::reactor::Timeout::new(
        Duration::from_millis(REQUEST_TIMEOUT_MS),
        &core.handle(),
    )
    .unwrap();

    let hyper_client = hyper::client::Client::new(&core.handle());
    let mut configuration = swagger::apis::configuration::Configuration::new(hyper_client);
    configuration.base_path = base_path;
    let api_client = swagger::apis::client::APIClient::new(configuration);
    log("create_job: api client created");

    // Reference (timeouts): https://stackoverflow.com/a/45314194
    let work = api_client
        .jobs_api()
        .jobs_create(job_create_record)
        .select2(timeout)
        .then(|res| match res {
            Ok(Either::A((result, _timeout))) => Ok(result),
            Ok(Either::B((_timeout, _result))) => Err("Timed out"),
            Err(Either::A((err, _timeout))) => {
                log(&format!("{:?}", err));
                Err("Request error")
            }
            Err(Either::B((_timeout_err, _result))) => Err("Timed out"),
        });

    let result = core.run(work);
    log(&format!("create_job response: {:?}", result));
    Ok(())
}

pub fn update_job(
    base_path: String,
    jobslurmid: &str,
    job_update_record: swagger::models::Job,
) -> Result<(), String> {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let timeout = tokio_core::reactor::Timeout::new(
        Duration::from_millis(REQUEST_TIMEOUT_MS),
        &core.handle(),
    )
    .unwrap();

    let hyper_client = hyper::client::Client::new(&core.handle());
    let mut configuration = swagger::apis::configuration::Configuration::new(hyper_client);
    configuration.base_path = base_path;
    let api_client = swagger::apis::client::APIClient::new(configuration);
    log("update_job: api client created");

    // Reference (timeouts): https://stackoverflow.com/a/45314194
    let work = api_client
        .jobs_api()
        .jobs_update(jobslurmid, job_update_record)
        .select2(timeout)
        .then(|res| match res {
            Ok(Either::A((result, _timeout))) => Ok(result),
            Ok(Either::B((_timeout, _result))) => Err("Timed out"),
            Err(Either::A((err, _timeout))) => {
                log(&format!("{:?}", err));
                Err("Request error")
            }
            Err(Either::B((_timeout_err, _result))) => Err("Timed out"),
        });

    let result = core.run(work);
    log(&format!("update_job response: {:?}", result));
    Ok(())
}
