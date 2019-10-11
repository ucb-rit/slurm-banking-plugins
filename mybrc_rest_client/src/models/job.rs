/*
 * myBRC REST API
 *
 * REST API for myBRC
 *
 * The version of the OpenAPI document: v1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Job {
    #[serde(rename = "jobslurmid")]
    pub jobslurmid: String,
    #[serde(rename = "submitdate", skip_serializing_if = "Option::is_none")]
    pub submitdate: Option<String>,
    #[serde(rename = "startdate", skip_serializing_if = "Option::is_none")]
    pub startdate: Option<String>,
    #[serde(rename = "enddate", skip_serializing_if = "Option::is_none")]
    pub enddate: Option<String>,
    #[serde(rename = "userid")]
    pub userid: String,
    #[serde(rename = "accountid")]
    pub accountid: String,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "jobstatus", skip_serializing_if = "Option::is_none")]
    pub jobstatus: Option<String>,
    #[serde(rename = "partition", skip_serializing_if = "Option::is_none")]
    pub partition: Option<String>,
    #[serde(rename = "qos", skip_serializing_if = "Option::is_none")]
    pub qos: Option<String>,
    #[serde(rename = "nodes", skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<crate::models::Node>>,
    #[serde(rename = "num_cpus", skip_serializing_if = "Option::is_none")]
    pub num_cpus: Option<i32>,
    #[serde(rename = "num_req_nodes", skip_serializing_if = "Option::is_none")]
    pub num_req_nodes: Option<i32>,
    #[serde(rename = "num_alloc_nodes", skip_serializing_if = "Option::is_none")]
    pub num_alloc_nodes: Option<i32>,
    #[serde(rename = "raw_time", skip_serializing_if = "Option::is_none")]
    pub raw_time: Option<f32>,
    #[serde(rename = "cpu_time", skip_serializing_if = "Option::is_none")]
    pub cpu_time: Option<f32>,
}

impl Job {
    pub fn new(jobslurmid: String, userid: String, accountid: String) -> Job {
        Job {
            jobslurmid,
            submitdate: None,
            startdate: None,
            enddate: None,
            userid,
            accountid,
            amount: None,
            jobstatus: None,
            partition: None,
            qos: None,
            nodes: None,
            num_cpus: None,
            num_req_nodes: None,
            num_alloc_nodes: None,
            raw_time: None,
            cpu_time: None,
        }
    }
}


