use crate::{
    infrastructure::repository::game_info::entitiy::{GameInfoEntitiy, NewGameInfo},
    util::error::CustomError,
};

pub trait GameInfoDao {
    // 全検索
    fn find_all(&self) -> Result<Vec<GameInfoEntitiy>, CustomError>;
    // 1件検索
    fn find_unique(&self, id: i32) -> Result<GameInfoEntitiy, CustomError>;
    // 1件挿入
    fn insert(&self, new_game_info: NewGameInfo) -> Result<GameInfoEntitiy, CustomError>;
}

pub trait HaveGameInfoDao {
    type GameInfoDao: GameInfoDao;
    fn game_info_dao(&self) -> &Self::GameInfoDao;
}
