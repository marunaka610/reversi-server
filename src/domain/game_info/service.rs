use crate::{
    domain::{
        game_info::dto::{GameInfoDto, GameState},
        progress::{
            dto::ProgressDto,
            service::{HaveProgressService, ProgressService},
        },
    },
    infrastructure::repository::game_info::{
        dao::{GameInfoDao, HaveGameInfoDao},
        entitiy::NewGameInfo,
    },
    util::error::CustomError,
};
use chrono::Local;

/// # ゲーム情報サービス
pub trait GameInfoService: HaveGameInfoDao + HaveProgressService {
    // 全検索
    fn find_all(&self) -> Result<Vec<GameInfoDto>, CustomError> {
        let progresses: Vec<ProgressDto> = Vec::new();

        match self.game_info_dao().find_all() {
            Ok(game_infos) => Ok(game_infos
                .iter()
                .map(|e| GameInfoDto::from_entitiy(e, progresses.clone()))
                .collect()),
            Err(msg) => Err(msg),
        }
    }
    // 1件検索
    fn find_unique(&self, id: i32) -> Result<GameInfoDto, CustomError> {
        match self.progress_service().find_all(id) {
            Ok(progressies) => match self.game_info_dao().find_unique(id) {
                Ok(entitiy) => Ok(GameInfoDto::from_entitiy(&entitiy, progressies)),
                Err(msg) => Err(msg),
            },
            Err(msg) => Err(msg),
        }
    }

    // 1件挿入
    fn insert(&self) -> Result<GameInfoDto, CustomError> {
        let new = NewGameInfo {
            state: &GameState::BlackTurn.to_i32(),
            start_time: &Local::now().naive_local(),
        };
        match self.game_info_dao().insert(new) {
            Ok(entitiy) => Ok(GameInfoDto::from_entitiy(&entitiy, vec![])),
            Err(msg) => Err(msg),
        }
    }
}

// ジェネリクス境界を満たすもの全てに実装される？
impl<T: HaveGameInfoDao + HaveProgressService> GameInfoService for T {}

// ↑でAppContextにGameInfoServiceが実装され、このトレイと境界が満たされるので、
// AppContext側で実装できる
pub trait HaveGameInfoService {
    type GameInfoService: GameInfoService;
    fn game_info_service(&self) -> &Self::GameInfoService;
}
