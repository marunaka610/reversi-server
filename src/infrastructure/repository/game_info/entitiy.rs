

use diesel::{Queryable,Insertable};
use super::super::super::{
  super::{
    schema::{
      game_infos,
    },
  },
};

// ゲーム情報エンティティ
#[derive(Queryable, Debug, Clone, Copy)]
pub struct GameInfoEntitiy {
  pub game_id: i32,
  pub state: i32,
}

/// フィールド取得メソッド定義
impl GameInfoEntitiy {
  /// ID
  fn game_id(&self) -> &i32 {
    &self.game_id
  }

  /// 状態
  fn state(&self) -> &i32 {
    &self.state
  }
}


// ゲーム情報エンティティ追加用
#[derive(Insertable)]
#[table_name = "game_infos"]
pub struct NewGameInfo<'a> {
  pub game_id: &'a i32,
  pub state: &'a i32,
}