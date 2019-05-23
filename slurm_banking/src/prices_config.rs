extern crate config;

use config::{Config, File};
use super::logging::safe_info;

/// The price config file path, without the file extension
/// For example, for /etc/slurm/prices.toml write /etc/slurm/prices
static PRICES_CONFIG_FILE_PATH: &str = "/etc/slurm/prices";

fn log(message: &str) {
    safe_info(&("slurm_banking_lib: ".to_owned() + message))
}

pub fn load_config_from_file(conf: &mut Config) -> Result<(), config::ConfigError> {
    log(&format!(
        "Looking for prices config file: {}",
        PRICES_CONFIG_FILE_PATH
    ));
    match conf.merge(File::with_name(PRICES_CONFIG_FILE_PATH)) {
        Ok(_) => {
            log("Successfully loaded prices config file");
            Ok(())
        },
        Err(e) => {
            log(&format!(
                "Failed to load prices config file: {}", 
                PRICES_CONFIG_FILE_PATH
            ));
            Err(e)
        }
    }
}