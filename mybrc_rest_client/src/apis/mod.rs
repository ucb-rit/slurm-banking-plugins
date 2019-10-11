use reqwest;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

mod api_token_auth_api;
pub use self::api_token_auth_api::{ApiTokenAuthApi, ApiTokenAuthApiClient};
mod can_submit_job_api;
pub use self::can_submit_job_api::{CanSubmitJobApi, CanSubmitJobApiClient};
mod jobs_api;
pub use self::jobs_api::{JobsApi, JobsApiClient};
mod partitions_api;
pub use self::partitions_api::{PartitionsApi, PartitionsApiClient};
mod upload_cpu_data_api;
pub use self::upload_cpu_data_api::{UploadCpuDataApi, UploadCpuDataApiClient};
mod users_api;
pub use self::users_api::{UsersApi, UsersApiClient};

pub mod client;
pub mod configuration;
