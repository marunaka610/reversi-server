use super::{
  entitiy::{
    GameInfoEntitiy,
    NewGameInfo,
  }
};

pub trait GameInfoDao {
  // 全検索
  fn find_all(&self) -> Vec<GameInfoEntitiy>;
  // 1件検索
  fn find_unique(&self, id: i32) -> GameInfoEntitiy;
  // 1件挿入
  fn insert(&self, new_game_info: NewGameInfo) -> usize;
}

pub trait HaveGameInfoDao {
  type GameInfoDao: GameInfoDao;
  fn game_info_dao(&self) -> &Self::GameInfoDao;
}
