use serde::{Deserialize, Serialize};

use crate::{connection::Connection, general::{ApiResult, ClientError, Paging}};

#[derive(Serialize, Deserialize, Debug)]
pub struct BattleLog {
  pub items: Vec<Battle>,
  pub paging: Paging,
}

impl BattleLog {
  pub async fn get(tag: &str, connection: &Connection) -> Result<Vec<Battle>, ClientError> {
    match connection.get_battle_log(tag).await.unwrap() {
      ApiResult::Ok(battle_log) => Ok(battle_log.items),
      ApiResult::Error(err) => Err(err),
    }
  }
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
  pub result: Option<String>,
  pub rank: Option<i32>,
  pub duration: Option<i32>,
  pub star_player: Option<BattlePlayer>,
  pub teams: Vec<Vec<BattlePlayer>>,
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