use crate::{
    infrastructure::{
        db_connect::establish_connection,
        repository::progress::{dao::*, entitiy::*},
    },
    schema::progresses::dsl::*,
    util::error::CustomError,
};
use diesel::prelude::*;
use std::result::Result;

#[derive(Default, Clone)]
pub struct ProgressPgDao {}

impl ProgressDao for ProgressPgDao {
    // 全検索
    fn find_all(&self, id: i32) -> Result<Vec<ProgressEntitiy>, CustomError> {
        match establish_connection() {
            Ok(connection) => {
                let results = progresses
                    .filter(game_id.eq(id))
                    .load::<ProgressEntitiy>(&connection)
                    .expect("Error loading posts");
                Ok(results)
            }
            Err(msg) => Err(msg),
        }
    }
    // 1件検索
    fn find_unique(&self, id: i32) -> Result<ProgressEntitiy, CustomError> {
        match establish_connection() {
            Ok(connection) => {
                let results = progresses
                    .filter(game_id.eq(id))
                    .limit(1)
                    .load::<ProgressEntitiy>(&connection)
                    .expect("Error loading posts");
                Ok(results[0].clone())
            }
            Err(msg) => Err(msg),
        }
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
        match dao.find_all(1) {
            Ok(results) => {
                dbg!(results.len());
                for item in results {
                    dbg!(item);
                }
            }
            Err(msg) => panic!(msg),
        }
    }

    #[test]
    fn find_unique_test() {
        let dao = ProgressPgDao {};
        match dao.find_unique(1) {
            Ok(result) => dbg!(result),
            Err(msg) => panic!(msg),
        }
    }

    // #[test]
    // fn insert_test() {
    //   let dao = ProgressPgDao{};
    //   let id :i32 = 4;
    //   let st :i32 = 4;
    //   dao.insert(&id, &st);
    // }
}
