use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  can_submit_job_api: Box<::apis::CanSubmitJobApi>,
  jobs_api: Box<::apis::JobsApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      can_submit_job_api: Box::new(::apis::CanSubmitJobApiClient::new(rc.clone())),
      jobs_api: Box::new(::apis::JobsApiClient::new(rc.clone())),
    }
  }

  pub fn can_submit_job_api(&self) -> &::apis::CanSubmitJobApi{
    self.can_submit_job_api.as_ref()
  }

  pub fn jobs_api(&self) -> &::apis::JobsApi{
    self.jobs_api.as_ref()
  }


}
