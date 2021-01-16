

use diesel::{Queryable,Insertable};
use super::super::super::{
  super::{
    schema::{
      progresses,
    },
  },
};
use chrono::NaiveDateTime;

// ゲーム情報エンティティ
#[derive(Queryable, Debug, Clone)]
pub struct ProgressEntitiy {
  pub game_id: i32,
  pub piecies: Vec<u8>,
  pub time: NaiveDateTime,
}

impl ProgressEntitiy {
  
  fn game_id(&self) -> &i32 {
    &self.game_id
  }

  fn piecies(&self) -> &Vec<u8> {
    &self.piecies
  }

  fn time(&self) -> &NaiveDateTime {
    &self.time
  }
}


// 経過エンティティ
#[derive(Insertable)]
#[table_name = "progresses"]
pub struct NewProgress<'a> {
  pub game_id: &'a i32,
  pub piecies: &'a Vec<u8>,
  pub time: &'a chrono::NaiveDateTime,
}