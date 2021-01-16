use super::super::{
  domain::{
    game_info::service::{
      HaveGameInfoService,
    },
    progress::service::{
      HaveProgressService,
    }
  },
  infrastructure::repository::{
    game_info::{
      dao::{
        HaveGameInfoDao,
      },
      pg_dao::GameInfoPgDao,
    },
    progress::{
      dao::{
        HaveProgressDao,
      },
      pg_dao::ProgressPgDao,
    },
  },
};
use juniper::{
  Context,
};


#[derive(Default, Clone)]
pub struct AppContext {
  pub game_info_dao: GameInfoPgDao,
  pub progress_dao: ProgressPgDao,

}

impl AppContext {
  pub fn new() -> AppContext {
    AppContext {
      game_info_dao: GameInfoPgDao{},
      progress_dao: ProgressPgDao{}
    }
  }
}


impl Context for AppContext {}

impl HaveGameInfoService for AppContext{
  type T = Self;
  fn game_info_service(&self) -> &Self::T {
    self
  }
}
impl HaveProgressService for AppContext{
  type T = Self;
  fn progress_service(&self) -> &Self::T {
    self
  }
}

impl HaveGameInfoDao for AppContext {
  type GameInfoDao = GameInfoPgDao;
  fn game_info_dao(&self) -> &Self::GameInfoDao {
    &self.game_info_dao
  } 
}

impl HaveProgressDao for AppContext {
  type ProgressDao = ProgressPgDao;
  fn progress_dao(&self) -> &Self::ProgressDao {
    &self.progress_dao
  } 
}