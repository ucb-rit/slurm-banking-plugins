extern crate config;

use super::logging::safe_info;
use config::{Config, File};
use std::collections::HashMap;

/// The price config file path, without the file extension
/// For example, for /etc/slurm/bank-config.toml write /etc/slurm/bank-config
static PRICES_CONFIG_FILE_PATH: &str = "/etc/slurm/bank-config";

fn log(message: &str) {
    safe_info(&("slurm_banking_lib: ".to_owned() + message))
}

pub fn get_base_path(conf: &Config) -> String {
    let default_value = "http://localhost:8080".to_string();
    let api_conf = match conf.get::<HashMap<String, String>>("API") {
        Ok(api_conf) => api_conf,
        Err(_) => return default_value,
    };
    api_conf.get("url").unwrap_or(&default_value).clone()
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
        }
        Err(e) => {
            log(&format!(
                "Failed to load prices config file: {}",
                PRICES_CONFIG_FILE_PATH
            ));
            Err(e)
        }
    }
}
