use super::{
  dto::GameInfoDto,
  super::super::infrastructure::repository::game_info::dao::{
    GameInfoDao,
    HaveGameInfoDao
  }
};

pub trait GameInfoService : HaveGameInfoDao {

  // 全検索
  fn find_all_game_info(&self) -> Vec<GameInfoDto> {
    self.game_info_dao()
      .find_all()
      .iter()
      .map(|e| GameInfoDto::from_entitiy(e))
      .collect()
  }
  // 1件検索
  fn find_unique_game_info(&self, id: i32) -> GameInfoDto {
  
    let entitiy = self.game_info_dao().find_unique(id);
    GameInfoDto::from_entitiy(&entitiy)
  }

  // 1件挿入
  fn insert_game_info(&self, gameid_param: &i32, state_param: &i32) -> usize {

    self.game_info_dao().insert(gameid_param, state_param)
  }
}

impl<T:HaveGameInfoDao> GameInfoService for T {}

pub trait HaveGameInfoService {
  type T: GameInfoService;
  fn game_info_service(&self) -> &Self::T;
}
