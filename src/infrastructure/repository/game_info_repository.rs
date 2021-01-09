
use reversi_server::schema::game_infos;
use reversi_server::schema::game_infos::dsl::*;
use super::super::{
  db_connect,
  database::*,
  entitiy::game_info_entity::*
};
use diesel::prelude::*;

// 全検索
pub fn find_all() -> Vec<GameInfoEntitiy> {
  let connection = db_connect::establish_connection();
  let results = game_infos
    .load::<GameInfoEntitiy>(&connection)
    .expect("Error loading posts");
    results
}
// 1件検索
pub fn find_unique(id: i32) -> GameInfoEntitiy {
  let connection = db_connect::establish_connection();
  let results = game_infos
    .filter(game_id.eq(id))
    .limit(1)
    .load::<GameInfoEntitiy>(&connection)
    .expect("Error loading posts");
    results[0]
}

impl Database {
  pub fn find_list_game_info(&self) -> Option<Vec<GameInfoEntitiy>> {
    Some(find_all())
  }

  pub fn find_unique_game_info(&self, id: i32) -> Option<GameInfoEntitiy> {
    Some(find_unique(id))
  }
}


pub fn insert(gameid_param: &i32, state_param: &i32) -> usize {

  let connection = db_connect::establish_connection();
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
