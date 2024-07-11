use std::fmt::Result;

use brawl_stars_api::{battle_log::BattleLog, connection::Connection, general::ApiResult, player::Player};
use reqwest::header::AUTHORIZATION;

use brawl_stars_api::player::PlayerData;

#[tokio::main]
async fn main() -> Result {
    println!("Hello, world!");
    // let client = reqwest::Client::new();
    // let res = client
    //                         .get("https://api.brawlstars.com/v1/players/%2320VUJ29CP")
    //                         .header(AUTHORIZATION, "Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6IjI4YTMxOGY3LTAwMDAtYTFlYi03ZmExLTJjNzQzM2M2Y2NhNSJ9.eyJpc3MiOiJzdXBlcmNlbGwiLCJhdWQiOiJzdXBlcmNlbGw6Z2FtZWFwaSIsImp0aSI6ImI1YTZmNTE3LWNmYzYtNDY4Ni04MzA4LWIzYWRmMmQxZjI4YiIsImlhdCI6MTcxNjQxNDc0MCwic3ViIjoiZGV2ZWxvcGVyLzExYjNiZGNmLWQ4YzAtODM2ZC1lYTUxLTM1MjIwYjQzN2I5MCIsInNjb3BlcyI6WyJicmF3bHN0YXJzIl0sImxpbWl0cyI6W3sidGllciI6ImRldmVsb3Blci9zaWx2ZXIiLCJ0eXBlIjoidGhyb3R0bGluZyJ9LHsiY2lkcnMiOlsiNzEuMTg3LjIzNy42NyJdLCJ0eXBlIjoiY2xpZW50In1dfQ.AYXjC2eCAWsAGMGxWg3TQ2NrlmboYVzicvr2Qd12syj_zf6XVsp6G4IIBofnrb8Ttyw5Kxqp21bonH0bbhHI0A")
    //                         .send()
    //                         .await
    //                         .unwrap();
    // println!("{:?}", res.json::<ApiResult<PlayerData>>().await.unwrap());
    let connection = Connection::new("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiIsImtpZCI6IjI4YTMxOGY3LTAwMDAtYTFlYi03ZmExLTJjNzQzM2M2Y2NhNSJ9.eyJpc3MiOiJzdXBlcmNlbGwiLCJhdWQiOiJzdXBlcmNlbGw6Z2FtZWFwaSIsImp0aSI6ImI1YTZmNTE3LWNmYzYtNDY4Ni04MzA4LWIzYWRmMmQxZjI4YiIsImlhdCI6MTcxNjQxNDc0MCwic3ViIjoiZGV2ZWxvcGVyLzExYjNiZGNmLWQ4YzAtODM2ZC1lYTUxLTM1MjIwYjQzN2I5MCIsInNjb3BlcyI6WyJicmF3bHN0YXJzIl0sImxpbWl0cyI6W3sidGllciI6ImRldmVsb3Blci9zaWx2ZXIiLCJ0eXBlIjoidGhyb3R0bGluZyJ9LHsiY2lkcnMiOlsiNzEuMTg3LjIzNy42NyJdLCJ0eXBlIjoiY2xpZW50In1dfQ.AYXjC2eCAWsAGMGxWg3TQ2NrlmboYVzicvr2Qd12syj_zf6XVsp6G4IIBofnrb8Ttyw5Kxqp21bonH0bbhHI0A".to_owned());
    // let mut me = Player::new("#20VUJ29CP", &connection).await.unwrap();
    // println!("{:?}", me);
    // let battle_log = me.get_battles(&connection).await;
    // println!("{:?}", battle_log);
    let try_two = BattleLog::get("#909GR8902", &connection).await;
    println!("{:?}", try_two);
    Ok(())
}
