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
pub struct InlineResponse2003 {
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "next", skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(rename = "previous", skip_serializing_if = "Option::is_none")]
    pub previous: Option<String>,
    #[serde(rename = "results")]
    pub results: Vec<crate::models::ScgUser>,
}

impl InlineResponse2003 {
    pub fn new(count: i32, results: Vec<crate::models::ScgUser>) -> InlineResponse2003 {
        InlineResponse2003 {
            count,
            next: None,
            previous: None,
            results,
        }
    }
}
