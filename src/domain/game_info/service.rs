use crate::{
  domain::{
    game_info::dto::{
      GameInfoDto,
      GameState
    },
    progress::{
      dto::ProgressDto,
      service::{
        HaveProgressService,
        ProgressService,
      }
    }
  },
  infrastructure::repository::{
    game_info::{
      dao::{
        GameInfoDao,
        HaveGameInfoDao
      },
      entitiy::NewGameInfo,
    },
  }
};

pub trait GameInfoService : HaveGameInfoDao + HaveProgressService {

  // 全検索
  fn find_all_game_info(&self) -> Vec<GameInfoDto> {
    let progresses: Vec<ProgressDto> = Vec::new();
    self.game_info_dao()
      .find_all()
      .iter()
      .map(|e| GameInfoDto::from_entitiy(e, progresses.clone()))
      .collect()
  }
  // 1件検索
  fn find_unique_game_info(&self, id: i32) -> GameInfoDto {
  
    let progresses: Vec<ProgressDto> = self.progress_service().find_all_progress(id);
    let entitiy = self.game_info_dao().find_unique(id);
    GameInfoDto::from_entitiy(&entitiy, progresses)
  }

  // 1件挿入
  fn insert_game_info(&self, game_id: i32, state: i32) -> GameInfoDto {
    let new = NewGameInfo{
      game_id : &game_id,
      state: &state
    };
    self.game_info_dao().insert(new);
    GameInfoDto{
      game_id: game_id, 
      state: GameState::from_i32(state).unwrap(),
      progresses: vec![]
    }
  }
}

impl<T:HaveGameInfoDao + HaveProgressService> GameInfoService for T {}

pub trait HaveGameInfoService {
  type T: GameInfoService;
  fn game_info_service(&self) -> &Self::T;
}
