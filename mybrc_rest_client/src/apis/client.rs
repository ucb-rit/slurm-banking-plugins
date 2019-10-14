use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
    api_token_auth_api: Box<dyn crate::apis::ApiTokenAuthApi>,
    can_submit_job_api: Box<dyn crate::apis::CanSubmitJobApi>,
    jobs_api: Box<dyn crate::apis::JobsApi>,
    partitions_api: Box<dyn crate::apis::PartitionsApi>,
    upload_cpu_data_api: Box<dyn crate::apis::UploadCpuDataApi>,
    users_api: Box<dyn crate::apis::UsersApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            api_token_auth_api: Box::new(crate::apis::ApiTokenAuthApiClient::new(rc.clone())),
            can_submit_job_api: Box::new(crate::apis::CanSubmitJobApiClient::new(rc.clone())),
            jobs_api: Box::new(crate::apis::JobsApiClient::new(rc.clone())),
            partitions_api: Box::new(crate::apis::PartitionsApiClient::new(rc.clone())),
            upload_cpu_data_api: Box::new(crate::apis::UploadCpuDataApiClient::new(rc.clone())),
            users_api: Box::new(crate::apis::UsersApiClient::new(rc.clone())),
        }
    }

    pub fn api_token_auth_api(&self) -> &dyn crate::apis::ApiTokenAuthApi {
        self.api_token_auth_api.as_ref()
    }

    pub fn can_submit_job_api(&self) -> &dyn crate::apis::CanSubmitJobApi {
        self.can_submit_job_api.as_ref()
    }

    pub fn jobs_api(&self) -> &dyn crate::apis::JobsApi {
        self.jobs_api.as_ref()
    }

    pub fn partitions_api(&self) -> &dyn crate::apis::PartitionsApi {
        self.partitions_api.as_ref()
    }

    pub fn upload_cpu_data_api(&self) -> &dyn crate::apis::UploadCpuDataApi {
        self.upload_cpu_data_api.as_ref()
    }

    pub fn users_api(&self) -> &dyn crate::apis::UsersApi {
        self.users_api.as_ref()
    }
}
