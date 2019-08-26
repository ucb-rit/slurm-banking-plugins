use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  api_token_auth_api: Box<::apis::ApiTokenAuthApi>,
  can_submit_job_api: Box<::apis::CanSubmitJobApi>,
  cpus_api: Box<::apis::CpusApi>,
  jobs_api: Box<::apis::JobsApi>,
  partitions_api: Box<::apis::PartitionsApi>,
  users_api: Box<::apis::UsersApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      api_token_auth_api: Box::new(::apis::ApiTokenAuthApiClient::new(rc.clone())),
      can_submit_job_api: Box::new(::apis::CanSubmitJobApiClient::new(rc.clone())),
      cpus_api: Box::new(::apis::CpusApiClient::new(rc.clone())),
      jobs_api: Box::new(::apis::JobsApiClient::new(rc.clone())),
      partitions_api: Box::new(::apis::PartitionsApiClient::new(rc.clone())),
      users_api: Box::new(::apis::UsersApiClient::new(rc.clone())),
    }
  }

  pub fn api_token_auth_api(&self) -> &::apis::ApiTokenAuthApi{
    self.api_token_auth_api.as_ref()
  }

  pub fn can_submit_job_api(&self) -> &::apis::CanSubmitJobApi{
    self.can_submit_job_api.as_ref()
  }

  pub fn cpus_api(&self) -> &::apis::CpusApi{
    self.cpus_api.as_ref()
  }

  pub fn jobs_api(&self) -> &::apis::JobsApi{
    self.jobs_api.as_ref()
  }

  pub fn partitions_api(&self) -> &::apis::PartitionsApi{
    self.partitions_api.as_ref()
  }

  pub fn users_api(&self) -> &::apis::UsersApi{
    self.users_api.as_ref()
  }


}
