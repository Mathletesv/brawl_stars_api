use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Rotation {
  pub items: Vec<RotationEvent>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RotationEvent {
  pub start_time: String,
  pub end_time: String,
  pub slot_id: i32,
  pub event: Event,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
  pub id: i32,
  pub mode: String,
  pub map: String,
}