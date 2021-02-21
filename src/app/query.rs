use crate::{
  app::app_context::AppContext,
  domain::{
    game_info::{
      dto::GameInfoDto,
      service::{GameInfoService, HaveGameInfoService},
    },
    progress::{
      dto::ProgressDto,
      service::{HaveProgressService, ProgressService},
    },
  },
};
use juniper::{graphql_object};

/// # クエリ定義
pub struct Query;

/// # クエリ定義
#[graphql_object(context = AppContext)]
impl Query {
  /// # ゲーム情報一覧
  #[graphql(arguments())]
  fn game_infos(ctx: &AppContext) -> Option<Vec<GameInfoDto>> {
    Some(GameInfoService::find_all(ctx.game_info_service()))
  }
  /// # ゲーム情報
  #[graphql(arguments(id(description = "id of the game")))]
  fn game_info(ctx: &AppContext, id: i32) -> Option<GameInfoDto> {
    Some(GameInfoService::find_unique(ctx.game_info_service(), id))
  }
  /// # ゲーム進捗一覧
  #[graphql(arguments(id(description = "id of game")))]
  fn pregresses(ctx: &AppContext, id: i32) -> Option<Vec<ProgressDto>> {
    Some(ProgressService::find_all(ctx.game_info_service(), id))
  }
  /// # ゲーム進捗
  #[graphql(arguments(id(description = "id of game")))]
  fn progress_last(ctx: &AppContext, id: i32) -> Option<ProgressDto> {
    Some(ProgressService::find_unique(ctx.progress_service(), id))
  }
}