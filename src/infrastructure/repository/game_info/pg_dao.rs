use crate::{
    infrastructure::{
        db_connect::establish_connection,
        repository::game_info::{dao::*, entitiy::*},
    },
    schema::{game_infos, game_infos::dsl::*},
    util::error::CustomError,
};
use diesel::prelude::*;
use diesel::result::Error;
use std::result::Result;

#[derive(Default, Clone)]
pub struct GameInfoPgDao {}

impl GameInfoDao for GameInfoPgDao {
    // 全検索
    fn find_all(&self) -> Result<Vec<GameInfoEntitiy>, CustomError> {
        let connection = establish_connection()?;
        let results = game_infos
            .load::<GameInfoEntitiy>(&connection)
            .expect("Error find all GameInfo");
        Ok(results)
    }
    // 1件検索
    fn find_unique(&self, id: i32) -> Result<GameInfoEntitiy, CustomError> {
        let connection = establish_connection()?;
        let results = game_infos
            .filter(game_id.eq(id))
            .limit(1)
            .load::<GameInfoEntitiy>(&connection)
            .expect("Error find unique GameInfo");
        Ok(results[0])
    }
    // 1件挿入
    fn insert(&self, new_game_info: NewGameInfo) -> Result<GameInfoEntitiy, CustomError> {
        let connection = establish_connection()?;
        // let new = NewGameInfo{game_id : gameid_param, state : state_param};
        let result = connection.transaction::<_, Error, _>(|| {
            diesel::insert_into(game_infos::table)
                .values(&new_game_info)
                .execute(&connection)
                .expect("Error insert GameInfo");

            let results = game_infos
                .limit(1)
                .order(game_id.desc())
                .load::<GameInfoEntitiy>(&connection)
                .expect("Error loading posts");

            Ok(results[0])
        });
        Ok(result.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;
    #[test]
    fn new() {
        let x = GameInfoEntitiy {
            game_id: 1,
            state: 2,
            start_time: Local::now().naive_local(),
            end_time: Some(Local::now().naive_local()),
        };
        assert_eq!(1, x.game_id);
        // assert_eq!(GameState::BlackTurn, x.state);
    }

    #[test]
    fn find_all_test() {
        let dao = GameInfoPgDao {};
        let results = dao.find_all().unwrap();
        dbg!(results.len());
        for item in results {
            dbg!(item);
        }
    }

    #[test]
    fn find_unique_test() {
        let dao = GameInfoPgDao {};
        let result = dao.find_unique(1).unwrap();
        dbg!(result)
    }

    #[test]
    fn insert_test() {
        let dao = GameInfoPgDao {};
        let st: i32 = 4;
        let new = NewGameInfo {
            state: &st,
            start_time: &Local::now().naive_local(),
        };
        dao.insert(new);
    }
}
