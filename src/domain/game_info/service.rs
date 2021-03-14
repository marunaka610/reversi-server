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
        let game_infos = self
            .game_info_dao()
            .find_all()?
            .iter()
            .map(|e| GameInfoDto::from_entitiy(e, progresses.clone()))
            .collect();
        Ok(game_infos)
    }
    // 1件検索
    fn find_unique(&self, id: i32) -> Result<GameInfoDto, CustomError> {
        let entitiy = self.game_info_dao().find_unique(id)?;
        let progressies = self.progress_service().find_all(id)?;
        let game_info = GameInfoDto::from_entitiy(&entitiy, progressies);
        Ok(game_info)
    }

    // 1件挿入
    fn insert(&self) -> Result<GameInfoDto, CustomError> {
        let new = NewGameInfo {
            state: &GameState::BlackTurn.to_i32(),
            start_time: &Local::now().naive_local(),
        };
        let entitiy = self.game_info_dao().insert(new)?;
        Ok(GameInfoDto::from_entitiy(&entitiy, vec![]))
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
