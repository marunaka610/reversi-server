//! Appのコンポーネント群
use crate::{
    domain::{game_info::service::HaveGameInfoService, progress::service::HaveProgressService},
    infrastructure::repository::{
        game_info::{dao::HaveGameInfoDao, pg_dao::GameInfoPgDao},
        progress::{dao::HaveProgressDao, pg_dao::ProgressPgDao},
    },
};
use juniper::Context;

/// # Appのコンポーネント群
#[derive(Default, Clone)]
pub struct AppContext {
    pub game_info_dao: GameInfoPgDao,
    pub progress_dao: ProgressPgDao,
}

/// ## 基本メソッド
impl AppContext {
    pub fn new() -> AppContext {
        AppContext {
            game_info_dao: GameInfoPgDao {},
            progress_dao: ProgressPgDao {},
        }
    }
}

impl Context for AppContext {}

impl HaveGameInfoService for AppContext {
    type GameInfoService = Self;
    fn game_info_service(&self) -> &Self::GameInfoService {
        self
    }
}
impl HaveProgressService for AppContext {
    type T = Self;
    fn progress_service(&self) -> &Self::T {
        self
    }
}

impl HaveGameInfoDao for AppContext {
    /// ゲーム情報取得DAOの型
    type GameInfoDao = GameInfoPgDao;
    /// ゲーム情報取得DAOのinstance取得
    fn game_info_dao(&self) -> &Self::GameInfoDao {
        &self.game_info_dao
    }
}

impl HaveProgressDao for AppContext {
    /// ゲーム進捗取得DAOの型
    type ProgressDao = ProgressPgDao;
    /// ゲーム進捗取得DAOのinstance取得
    fn progress_dao(&self) -> &Self::ProgressDao {
        &self.progress_dao
    }
}
