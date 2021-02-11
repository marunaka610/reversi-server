use diesel::{Queryable,Insertable};
use crate::{
  schema::{
    game_infos,
  },
};

// ゲーム情報エンティティ
#[derive(Queryable, Debug, Clone, Copy)]
pub struct GameInfoEntitiy {
  pub game_id: i32,
  pub state: i32,
  pub start_time: chrono::NaiveDateTime,
  pub end_time: Option<chrono::NaiveDateTime>,
}

// ゲーム情報エンティティ追加用
#[derive(Insertable)]
#[table_name = "game_infos"]
pub struct NewGameInfo<'a> {
  pub state: &'a i32,
  pub start_time: &'a chrono::NaiveDateTime,
}