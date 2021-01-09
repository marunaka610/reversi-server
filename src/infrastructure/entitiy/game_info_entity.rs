use reversi_server::schema::game_infos;
use juniper::{
  graphql_object,
};
use diesel::{Queryable,Insertable};
use super::super::{
  database::*
};

// ゲーム情報エンティティ
#[derive(Queryable, Debug, Clone, Copy)]
pub struct GameInfoEntitiy {
  pub game_id: i32,
  pub state: i32,
}
impl GameInfoEntitiy {
  pub fn new(id: i32, game_state: i32) -> GameInfoEntitiy {
    Self {
      game_id: id,
      state: game_state,
    }
  }
}

/// A humanoid creature in the Star Wars universe.
#[graphql_object(context = Database)]
impl GameInfoEntitiy {
  /// T
  fn game_id(&self) -> &i32 {
    &self.game_id
  }

  /// 
  fn state(&self) -> &i32 {
    &self.state
  }
}


// ゲーム情報エンティティ
#[derive(Insertable)]
#[table_name = "game_infos"]
pub struct NewGameInfo<'a> {
  pub game_id: &'a i32,
  pub state: &'a i32,
}