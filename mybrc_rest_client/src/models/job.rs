/* 
 * Snippets API
 *
 * Test description
 *
 * OpenAPI spec version: v1
 * Contact: contact@snippets.local
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Job {
  #[serde(rename = "jobnumber")]
  jobnumber: Option<i32>,
  #[serde(rename = "jobslurmid")]
  jobslurmid: i32,
  #[serde(rename = "submitdate")]
  submitdate: String,
  #[serde(rename = "startdate")]
  startdate: Option<String>,
  #[serde(rename = "enddate")]
  enddate: Option<String>,
  #[serde(rename = "userid")]
  userid: i32,
  #[serde(rename = "accountid")]
  accountid: i32,
  #[serde(rename = "amount")]
  amount: String,
  #[serde(rename = "jobstatus")]
  jobstatus: i32,
  #[serde(rename = "partition")]
  partition: i32,
  #[serde(rename = "qos")]
  qos: i32,
  #[serde(rename = "created")]
  created: Option<String>,
  #[serde(rename = "updated")]
  updated: Option<String>
}

impl Job {
  pub fn new(jobslurmid: i32, submitdate: String, userid: i32, accountid: i32, amount: String, jobstatus: i32, partition: i32, qos: i32) -> Job {
    Job {
      jobnumber: None,
      jobslurmid: jobslurmid,
      submitdate: submitdate,
      startdate: None,
      enddate: None,
      userid: userid,
      accountid: accountid,
      amount: amount,
      jobstatus: jobstatus,
      partition: partition,
      qos: qos,
      created: None,
      updated: None
    }
  }

  pub fn set_jobnumber(&mut self, jobnumber: i32) {
    self.jobnumber = Some(jobnumber);
  }

  pub fn with_jobnumber(mut self, jobnumber: i32) -> Job {
    self.jobnumber = Some(jobnumber);
    self
  }

  pub fn jobnumber(&self) -> Option<&i32> {
    self.jobnumber.as_ref()
  }

  pub fn reset_jobnumber(&mut self) {
    self.jobnumber = None;
  }

  pub fn set_jobslurmid(&mut self, jobslurmid: i32) {
    self.jobslurmid = jobslurmid;
  }

  pub fn with_jobslurmid(mut self, jobslurmid: i32) -> Job {
    self.jobslurmid = jobslurmid;
    self
  }

  pub fn jobslurmid(&self) -> &i32 {
    &self.jobslurmid
  }


  pub fn set_submitdate(&mut self, submitdate: String) {
    self.submitdate = submitdate;
  }

  pub fn with_submitdate(mut self, submitdate: String) -> Job {
    self.submitdate = submitdate;
    self
  }

  pub fn submitdate(&self) -> &String {
    &self.submitdate
  }


  pub fn set_startdate(&mut self, startdate: String) {
    self.startdate = Some(startdate);
  }

  pub fn with_startdate(mut self, startdate: String) -> Job {
    self.startdate = Some(startdate);
    self
  }

  pub fn startdate(&self) -> Option<&String> {
    self.startdate.as_ref()
  }

  pub fn reset_startdate(&mut self) {
    self.startdate = None;
  }

  pub fn set_enddate(&mut self, enddate: String) {
    self.enddate = Some(enddate);
  }

  pub fn with_enddate(mut self, enddate: String) -> Job {
    self.enddate = Some(enddate);
    self
  }

  pub fn enddate(&self) -> Option<&String> {
    self.enddate.as_ref()
  }

  pub fn reset_enddate(&mut self) {
    self.enddate = None;
  }

  pub fn set_userid(&mut self, userid: i32) {
    self.userid = userid;
  }

  pub fn with_userid(mut self, userid: i32) -> Job {
    self.userid = userid;
    self
  }

  pub fn userid(&self) -> &i32 {
    &self.userid
  }


  pub fn set_accountid(&mut self, accountid: i32) {
    self.accountid = accountid;
  }

  pub fn with_accountid(mut self, accountid: i32) -> Job {
    self.accountid = accountid;
    self
  }

  pub fn accountid(&self) -> &i32 {
    &self.accountid
  }


  pub fn set_amount(&mut self, amount: String) {
    self.amount = amount;
  }

  pub fn with_amount(mut self, amount: String) -> Job {
    self.amount = amount;
    self
  }

  pub fn amount(&self) -> &String {
    &self.amount
  }


  pub fn set_jobstatus(&mut self, jobstatus: i32) {
    self.jobstatus = jobstatus;
  }

  pub fn with_jobstatus(mut self, jobstatus: i32) -> Job {
    self.jobstatus = jobstatus;
    self
  }

  pub fn jobstatus(&self) -> &i32 {
    &self.jobstatus
  }


  pub fn set_partition(&mut self, partition: i32) {
    self.partition = partition;
  }

  pub fn with_partition(mut self, partition: i32) -> Job {
    self.partition = partition;
    self
  }

  pub fn partition(&self) -> &i32 {
    &self.partition
  }


  pub fn set_qos(&mut self, qos: i32) {
    self.qos = qos;
  }

  pub fn with_qos(mut self, qos: i32) -> Job {
    self.qos = qos;
    self
  }

  pub fn qos(&self) -> &i32 {
    &self.qos
  }


  pub fn set_created(&mut self, created: String) {
    self.created = Some(created);
  }

  pub fn with_created(mut self, created: String) -> Job {
    self.created = Some(created);
    self
  }

  pub fn created(&self) -> Option<&String> {
    self.created.as_ref()
  }

  pub fn reset_created(&mut self) {
    self.created = None;
  }

  pub fn set_updated(&mut self, updated: String) {
    self.updated = Some(updated);
  }

  pub fn with_updated(mut self, updated: String) -> Job {
    self.updated = Some(updated);
    self
  }

  pub fn updated(&self) -> Option<&String> {
    self.updated.as_ref()
  }

  pub fn reset_updated(&mut self) {
    self.updated = None;
  }

}


