use serde::{Deserialize, Serialize};

use crate::general::{Icon, Paging};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClubMembers {
  pub items: Vec<ClubMember>,
  pub paging: Paging,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClubMember {
  pub tag: String,
  pub name: String,
  #[serde(rename = "nameColor")]
  pub name_color: String,
  pub role: String,
  pub trophies: i32,
  pub icon: Icon,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Club {
  pub tag: String,
  pub name: String,
  pub description: String,
  pub type_: String,
  pub badge_id: i32,
  pub required_trophies: i32,
  pub trophies: i32,
  pub members: Vec<ClubMember>,
}