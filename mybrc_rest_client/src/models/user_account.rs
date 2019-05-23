/* 
 * myBRC REST API
 *
 * REST API for myBRC
 *
 * OpenAPI spec version: v1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAccount {
  #[serde(rename = "useraccountassociationid")]
  useraccountassociationid: Option<i32>,
  #[serde(rename = "userid")]
  userid: Option<i32>,
  #[serde(rename = "accountid")]
  accountid: Option<String>,
  #[serde(rename = "userallocation")]
  userallocation: i32,
  #[serde(rename = "userbalance")]
  userbalance: i32
}

impl UserAccount {
  pub fn new(userallocation: i32, userbalance: i32) -> UserAccount {
    UserAccount {
      useraccountassociationid: None,
      userid: None,
      accountid: None,
      userallocation: userallocation,
      userbalance: userbalance
    }
  }

  pub fn set_useraccountassociationid(&mut self, useraccountassociationid: i32) {
    self.useraccountassociationid = Some(useraccountassociationid);
  }

  pub fn with_useraccountassociationid(mut self, useraccountassociationid: i32) -> UserAccount {
    self.useraccountassociationid = Some(useraccountassociationid);
    self
  }

  pub fn useraccountassociationid(&self) -> Option<&i32> {
    self.useraccountassociationid.as_ref()
  }

  pub fn reset_useraccountassociationid(&mut self) {
    self.useraccountassociationid = None;
  }

  pub fn set_userid(&mut self, userid: i32) {
    self.userid = Some(userid);
  }

  pub fn with_userid(mut self, userid: i32) -> UserAccount {
    self.userid = Some(userid);
    self
  }

  pub fn userid(&self) -> Option<&i32> {
    self.userid.as_ref()
  }

  pub fn reset_userid(&mut self) {
    self.userid = None;
  }

  pub fn set_accountid(&mut self, accountid: String) {
    self.accountid = Some(accountid);
  }

  pub fn with_accountid(mut self, accountid: String) -> UserAccount {
    self.accountid = Some(accountid);
    self
  }

  pub fn accountid(&self) -> Option<&String> {
    self.accountid.as_ref()
  }

  pub fn reset_accountid(&mut self) {
    self.accountid = None;
  }

  pub fn set_userallocation(&mut self, userallocation: i32) {
    self.userallocation = userallocation;
  }

  pub fn with_userallocation(mut self, userallocation: i32) -> UserAccount {
    self.userallocation = userallocation;
    self
  }

  pub fn userallocation(&self) -> &i32 {
    &self.userallocation
  }


  pub fn set_userbalance(&mut self, userbalance: i32) {
    self.userbalance = userbalance;
  }

  pub fn with_userbalance(mut self, userbalance: i32) -> UserAccount {
    self.userbalance = userbalance;
    self
  }

  pub fn userbalance(&self) -> &i32 {
    &self.userbalance
  }


}



