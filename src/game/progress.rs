use chrono::{DateTime, Local};
// use chrono::{Date, DateTime, Local, Utc};
// use  std::io;
// use serde_json::{json};
use diesel::Queryable;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ProgressJson {
  pub game_id: u128,
  pub piecies: u128,
}

#[derive(Queryable)]
pub struct Progress {
  pub game_id: u128,
  pub piecies: u128,
  pub time: DateTime<Local>,
}

impl Progress {
  pub fn new(game_id: u128, piecies: u128) -> Progress {
    Progress {
      game_id: game_id,
      piecies: piecies,
      time: Local::now(),
    }
  }

  // pub fn new2(json : String)-> Progress {
  //   let aaa : [[Piece; 8]; 8] = serde_json::from_str(&json).unwrap();
  //   Progress {
  //     game_id: game_id,
  //     piecies: piecies,
  //     time: Local::now(),
  //   }
  // }
}

// enum Piece {
//   None = 0,
//   Black = 1,
//   White = -1,
// }
