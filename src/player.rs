use serde::{Deserialize, Serialize};

use crate::{battle_log::{Battle, BattleLog}, connection::Connection, general::{Accessory, ApiResult, ClientError, Icon, StarPower}};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
  pub data: PlayerData,
  pub battles: Vec<Battle>,
}

impl Player {
  pub async fn new(tag: &str, connection: &Connection) -> Result<Player, ClientError> {
    match connection.get_player(tag).await.unwrap() {
      ApiResult::Ok(data) => Ok(Player { data: data, battles: Vec::new() }),
      ApiResult::Error(error) => Err(error),
    }
  }

  pub fn get_brawler(&self, brawler_name: &str) -> Option<&BrawlerStat> {
    self.data.brawlers.iter().find(|b| b.name == brawler_name)
  }

  pub async fn get_battles(&mut self, connection: &Connection) -> Result<Vec<Battle>, ClientError> {
    BattleLog::get(&self.data.tag, connection).await
  }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerData {
  pub tag: String,
  pub name: String,
  pub name_color: Option<String>,
  pub icon: Icon,
  pub trophies: i32,
  pub highest_trophies: i32,
  pub highest_power_play_points: Option<i32>,
  pub exp_level: i32,
  pub exp_points: i32,
  pub is_qualified_from_championship_challenge: bool,
  #[serde(rename = "3vs3Victories")]
  pub trios_victories: i32,
  pub solo_victories: i32,
  pub duo_victories: i32,
  pub best_robo_rumble_time: i32,
  pub best_time_as_big_brawler: i32,
  pub club: PlayerClub,
  pub brawlers: Vec<BrawlerStat>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BrawlerStat {
  pub id: i32,
  pub name: String,
  pub power: i32,
  pub rank: i32,
  pub trophies: i32,
  pub highest_trophies: i32,
  pub gears: Vec<GearStat>,
  pub star_powers: Vec<StarPower>,
  pub gadgets: Vec<Accessory>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GearStat {
  pub id: i32,
  pub name: String,
  pub level: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerClub {
  pub tag: Option<String>,
  pub name: Option<String>,
}