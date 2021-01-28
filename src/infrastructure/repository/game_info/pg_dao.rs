use crate::{
  infrastructure::{
    repository::game_info::{
      dao::*,
      entitiy::*,
    },
    db_connect::establish_connection,
  },
  schema::{
    game_infos,
    game_infos::dsl::*
  },
};
use diesel::prelude::*;

#[derive(Default, Clone)]
pub struct GameInfoPgDao {}

impl GameInfoDao for GameInfoPgDao {
  // 全検索
  fn find_all(&self) -> Vec<GameInfoEntitiy> {
    let connection = establish_connection();
    let results = game_infos
      .load::<GameInfoEntitiy>(&connection)
      .expect("Error loading posts");
      results
  }
  // 1件検索
  fn find_unique(&self, id: i32) -> GameInfoEntitiy {
    let connection = establish_connection();
    let results = game_infos
      .filter(game_id.eq(id))
      .limit(1)
      .load::<GameInfoEntitiy>(&connection)
      .expect("Error loading posts");
      results[0]
  }

  // 1件挿入
  fn insert(&self, new_game_info: NewGameInfo) -> usize {

    let connection = establish_connection();
    // let new = NewGameInfo{game_id : gameid_param, state : state_param};

    diesel::insert_into(game_infos::table)
      .values(&new_game_info)
      .execute(&connection)
      .expect("Error saving new post")
  }
}




#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn new() {
    let x = GameInfoEntitiy{game_id: 1, state: 2};
    assert_eq!(1, x.game_id);
    // assert_eq!(GameState::BlackTurn, x.state);
  }

  #[test]
  fn find_all_test() {
    let dao = GameInfoPgDao{};
    let results : Vec<GameInfoEntitiy> = dao.find_all();
    dbg!(results.len());
    for item  in results {
      dbg!(item);
    }
  }

  #[test]
  fn find_unique_test() {
    let dao = GameInfoPgDao{};
    let result : GameInfoEntitiy = dao.find_unique(1);
    dbg!(result);
  }

  #[test]
  fn insert_test() {
    let dao = GameInfoPgDao{};
    let id :i32 = 4;
    let st :i32 = 4;
    let new = NewGameInfo{game_id : &id, state :&st};
    dao.insert(new);
  }
}
