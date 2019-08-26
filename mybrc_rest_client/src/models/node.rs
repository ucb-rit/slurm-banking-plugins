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
pub struct Node {
  #[serde(rename = "name")]
  name: String
}

impl Node {
  pub fn new(name: String) -> Node {
    Node {
      name: name
    }
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn with_name(mut self, name: String) -> Node {
    self.name = name;
    self
  }

  pub fn name(&self) -> &String {
    &self.name
  }


}



