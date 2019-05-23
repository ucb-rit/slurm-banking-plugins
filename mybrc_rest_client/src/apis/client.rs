use std::rc::Rc;

use hyper;
use super::configuration::Configuration;

pub struct APIClient<C: hyper::client::Connect> {
  configuration: Rc<Configuration<C>>,
  accounts_api: Box<::apis::AccountsApi>,
  jobs_api: Box<::apis::JobsApi>,
  useraccountassociations_api: Box<::apis::UseraccountassociationsApi>,
  users_api: Box<::apis::UsersApi>,
}

impl<C: hyper::client::Connect> APIClient<C> {
  pub fn new(configuration: Configuration<C>) -> APIClient<C> {
    let rc = Rc::new(configuration);

    APIClient {
      configuration: rc.clone(),
      accounts_api: Box::new(::apis::AccountsApiClient::new(rc.clone())),
      jobs_api: Box::new(::apis::JobsApiClient::new(rc.clone())),
      useraccountassociations_api: Box::new(::apis::UseraccountassociationsApiClient::new(rc.clone())),
      users_api: Box::new(::apis::UsersApiClient::new(rc.clone())),
    }
  }

  pub fn accounts_api(&self) -> &::apis::AccountsApi{
    self.accounts_api.as_ref()
  }

  pub fn jobs_api(&self) -> &::apis::JobsApi{
    self.jobs_api.as_ref()
  }

  pub fn useraccountassociations_api(&self) -> &::apis::UseraccountassociationsApi{
    self.useraccountassociations_api.as_ref()
  }

  pub fn users_api(&self) -> &::apis::UsersApi{
    self.users_api.as_ref()
  }


}
