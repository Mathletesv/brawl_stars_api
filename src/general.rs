use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientError {
  pub reason: Option<String>,
  pub message: Option<String>,
  #[serde(rename = "type")]
  pub error_type: Option<String>,
  pub detail: Option<String>,
}

impl fmt::Display for ClientError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "ClientError: {:?}", self)
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Paging {
  pub cursors: Cursors,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cursors {
  pub before: Option<String>,
  pub after: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StarPower {
  pub id: i32,
  pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ApiResult<T> {
  Ok(T),
  Error(ClientError),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Accessory {
  pub id: i32,
  pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Icon {
  pub id: i32
}