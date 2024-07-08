use reqwest::header::AUTHORIZATION;

use crate::{battle_log::BattleLog, general::ApiResult, player::PlayerData};

pub struct Connection {
  pub key: String,
}

impl Connection {
  pub fn new(key: String) -> Connection {
    Connection {
      key: "Bearer ".to_owned() + key.as_str(),
    }
  }

  pub async fn get_player(&self, tag: &str) -> Result<ApiResult<PlayerData>, reqwest::Error> {
    let client = reqwest::Client::new();
    let tag = tag.replace("#", "%23");
    let res = client
      .get(format!("https://api.brawlstars.com/v1/players/{}", tag))
      .header(AUTHORIZATION, self.key.as_str())
      .send()
      .await
      .unwrap();
    res.json::<ApiResult<PlayerData>>().await
  }

  pub async fn get_battle_log(&self, tag: &str) -> Result<ApiResult<BattleLog>, reqwest::Error> {
    let client = reqwest::Client::new();
    let tag = tag.replace("#", "%23");
    let res = client
      .get(format!("https://api.brawlstars.com/v1/players/{}/battlelog", tag))
      .header(AUTHORIZATION, self.key.as_str())
      .send()
      .await
      .unwrap();
    res.json::<ApiResult<BattleLog>>().await
  }
}