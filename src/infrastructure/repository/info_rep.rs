use diesel::{Queryable,Insertable};
use reversi_server::schema::game_infos;
use reversi_server::schema::game_infos::dsl::*;
use super::super::{
  db_connect,
  database::*
};
use diesel::prelude::*;
use juniper::{
  //
  graphql_object,
  graphql_subscription,
};

// ゲーム情報エンティティ
#[derive(Queryable, Debug)]
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

// 全検索
pub fn find_all() -> Vec<GameInfoEntitiy> {
  let connection = db_connect::establish_connection();
  let results = game_infos
    .load::<GameInfoEntitiy>(&connection)
    .expect("Error loading posts");
    results
}
// 1件検索
pub fn find_unique() -> Vec<GameInfoEntitiy> {
  let connection = db_connect::establish_connection();
  let results = game_infos
    .filter(game_id.eq(1))
    .limit(1)
    .load::<GameInfoEntitiy>(&connection)
    .expect("Error loading posts");
    results
}


pub fn insert(gameid_param: &i32, state_param: &i32) -> usize {

  let connection = db_connect::establish_connection();
  let a :i32 = 2;
  let b :i32 = 2;
  let new = NewGameInfo{game_id : gameid_param, state : state_param};

  diesel::insert_into(game_infos::table)
    .values(&new)
    .execute(&connection)
    .expect("Error saving new post")
}




#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let x = GameInfoEntitiy::new(1, 2);
    assert_eq!(1, x.game_id);
    // assert_eq!(GameState::BlackTurn, x.state);
  }

  #[test]
  fn find_all_test() {
    let results : Vec<GameInfoEntitiy> = find_all();
    dbg!(results.len());
    for item  in results {
      dbg!(item);
    }
  }

  #[test]
  fn insert_test() {
    let id :i32 = 4;
    let st :i32 = 4;
    insert(&id, &st);
  }
}
