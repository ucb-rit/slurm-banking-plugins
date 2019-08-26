use hyper;
use serde;
use serde_json;

#[derive(Debug)]
pub enum Error<T> {
    Hyper(hyper::Error),
    Serde(serde_json::Error),
    ApiError(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T> {
    pub code: hyper::StatusCode,
    pub content: Option<T>,
}

impl<'de, T> From<(hyper::StatusCode, &'de [u8])> for Error<T> 
    where T: serde::Deserialize<'de> {
    fn from(e: (hyper::StatusCode, &'de [u8])) -> Self {
        if e.1.len() == 0 {
            return Error::ApiError(ApiError{
                code: e.0,
                content: None,
            });
        }
        match serde_json::from_slice::<T>(e.1) {
            Ok(t) => Error::ApiError(ApiError{
                code: e.0,
                content: Some(t),
            }),
            Err(e) => {
                Error::from(e)
            }
        }
    }
}

impl<T> From<hyper::Error> for Error<T> {
    fn from(e: hyper::Error) -> Self {
        return Error::Hyper(e)
    }
}

impl<T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        return Error::Serde(e)
    }
}

use super::models::*;

mod api_token_auth_api;
pub use self::api_token_auth_api::{ ApiTokenAuthApi, ApiTokenAuthApiClient };
mod can_submit_job_api;
pub use self::can_submit_job_api::{ CanSubmitJobApi, CanSubmitJobApiClient };
mod cpus_api;
pub use self::cpus_api::{ CpusApi, CpusApiClient };
mod jobs_api;
pub use self::jobs_api::{ JobsApi, JobsApiClient };
mod partitions_api;
pub use self::partitions_api::{ PartitionsApi, PartitionsApiClient };
mod users_api;
pub use self::users_api::{ UsersApi, UsersApiClient };

pub mod configuration;
pub mod client;
