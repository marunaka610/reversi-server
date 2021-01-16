use super::{
  entitiy::GameInfoEntitiy,
};

pub trait GameInfoDao {
  // 全検索
  fn find_all(&self) -> Vec<GameInfoEntitiy>;
  // 1件検索
  fn find_unique(&self, id: i32) -> GameInfoEntitiy;
  // 1件挿入
  fn insert(&self, gameid_param: &i32, state_param: &i32) -> usize;
}

pub trait HaveGameInfoDao {
  type GameInfoDao: GameInfoDao;
  fn game_info_dao(&self) -> &Self::GameInfoDao;
}
