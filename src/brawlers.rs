use serde::{Deserialize, Serialize};

use crate::general::{Accessory, ClientError, Paging, StarPower};

#[derive(Serialize, Deserialize, Debug)]
pub struct Brawlers {
  pub items: Vec<Brawler>,
  pub paging: Paging,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Brawler {
  pub id: i32,
  pub name: String,
  #[serde(rename = "starPowers")]
  pub star_powers: Vec<StarPower>,
  pub gadgets: Vec<Accessory>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum BrawlerResult {
  Brawler(Brawler),
  Error(ClientError),
}
