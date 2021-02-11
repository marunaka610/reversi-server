use crate::{
  infrastructure::{
    db_connect::establish_connection,
    repository::progress::{dao::*, entitiy::*},
  },
  schema::progresses::dsl::*,
};
use diesel::prelude::*;

#[derive(Default, Clone)]
pub struct ProgressPgDao {}

impl ProgressDao for ProgressPgDao {
  // 全検索
  fn find_all(&self, id: i32) -> Vec<ProgressEntitiy> {
    let connection = establish_connection();
    let results = progresses
      .filter(game_id.eq(id))
      .load::<ProgressEntitiy>(&connection)
      .expect("Error loading posts");
    results
  }
  // 1件検索
  fn find_unique(&self, id: i32) -> ProgressEntitiy {
    let connection = establish_connection();
    let results = progresses
      .filter(game_id.eq(id))
      .limit(1)
      .load::<ProgressEntitiy>(&connection)
      .expect("Error loading posts");
    results[0].clone()
  }

  // 1件挿入
  // fn insert(&self, gameid_param: &i32, state_param: &i32) -> usize {

  //   let connection = establish_connection();
  //   let new = NewProgress{game_id : gameid_param, piecies : state_param};

  //   diesel::insert_into(progresses::table)
  //     .values(&new)
  //     .execute(&connection)
  //     .expect("Error saving new post")
  // }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn find_all_test() {
    let dao = ProgressPgDao {};
    let results: Vec<ProgressEntitiy> = dao.find_all(1);
    dbg!(results.len());
    for item in results {
      dbg!(item);
    }
  }

  #[test]
  fn find_unique_test() {
    let dao = ProgressPgDao {};
    let result: ProgressEntitiy = dao.find_unique(1);
    dbg!(result);
  }

  // #[test]
  // fn insert_test() {
  //   let dao = ProgressPgDao{};
  //   let id :i32 = 4;
  //   let st :i32 = 4;
  //   dao.insert(&id, &st);
  // }
}
