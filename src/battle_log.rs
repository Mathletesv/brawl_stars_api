use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BattleLog {
  pub items: Vec<Battle>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Battle {
  pub battle_time: String,
  pub event: Event,
  pub battle: BattleData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
  pub id: i32,
  pub mode: String,
  pub map: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BattleData {
  pub mode: String,
  #[serde(rename = "type")]
  pub game_type: String,
  pub result: String,
  pub duration: i32,
  pub star_player: Option<BattlePlayer>,
  pub teams: ((BattlePlayer, BattlePlayer, BattlePlayer), (BattlePlayer, BattlePlayer, BattlePlayer)),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BattlePlayer {
  pub tag: String,
  pub name: String,
  pub brawler: Brawler,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Brawler {
  pub id: i32,
  pub name: String,
  pub power: i32,
  pub trophies: i32
}