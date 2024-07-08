use serde::{Deserialize, Serialize};

use crate::general::{Icon, Paging};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerRanking {
  pub items: Vec<TopPlayer>,
  pub paging: Paging,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TopPlayer {
  pub tag: String,
  pub name: String,
  #[serde(rename = "nameColor")]
  pub name_color: String,
  pub icon: Icon,
  pub trophies: i32,
  pub rank: i32,
  pub club: Option<ClubName>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClubName {
  pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClubRanking {
  pub items: Vec<TopClub>,
  pub paging: Paging,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TopClub {
  pub tag: String,
  pub name: String,
  pub badge_id: i32,
  pub trophies: i32,
  pub rank: i32,
  pub member_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BrawlerRanking {
  pub items: Vec<TopPlayer>,
  pub paging: Paging,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PowerPlaySeasons {
  pub items: Vec<PowerPlaySeason>,
  pub paging: Paging,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerPlaySeason {
  pub id: i32,
  pub start_time: String,
  pub end_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PowerPlayRanking {
  pub items: Vec<TopPlayer>,
  pub paging: Paging,
}