/*
 * myBRC REST API
 *
 * REST API for myBRC
 *
 * The version of the OpenAPI document: v1
 *
 * Generated by: https://openapi-generator.tech
 */

use std::borrow::Borrow;
use std::rc::Rc;

use reqwest;

use super::{configuration, Error};

pub struct UsersApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl UsersApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> UsersApiClient {
        UsersApiClient { configuration }
    }
}

pub trait UsersApi {
    fn users_list(&self, page: i32) -> Result<crate::models::InlineResponse2003, Error>;
}

impl UsersApi for UsersApiClient {
    fn users_list(&self, page: i32) -> Result<crate::models::InlineResponse2003, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/users/", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("page", &page.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref auth_conf) = configuration.basic_auth {
            req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}
