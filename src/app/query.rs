use juniper::{
  graphql_object,
  FieldResult,
};
use crate::{
  app::app_context::AppContext,
  domain::{
    game_info::{
      dto::GameInfoDto,
      service::{
        HaveGameInfoService,
        GameInfoService,
      },
    },
    progress::{
      dto::ProgressDto,
      service::{
        HaveProgressService,
        ProgressService,
      }
    }
  },
};

/// # クエリ定義
pub struct Query;

/// クエリ定義
#[graphql_object(context = AppContext)]
impl Query {
  
  /// # ゲーム情報一覧
  #[graphql(arguments())]
  fn game_infos(ctx: &AppContext) -> Option<Vec<GameInfoDto>> {
    Some(ctx.game_info_service().find_all_game_info())
  }
  
  /// # ゲーム情報
  #[graphql(arguments(id(description = "id of the game")))]
  fn game_info(ctx: &AppContext, id: i32) -> Option<GameInfoDto> {
    Some(ctx.game_info_service().find_unique_game_info(id))
  }
  
  /// # ゲーム進捗一覧
  #[graphql(arguments(id(description = "id of game")))]
  fn pregresses(ctx: &AppContext, id: i32) -> Option<Vec<ProgressDto>> {
    Some(ctx.game_info_service().find_all_progress(id))
  }
  
  /// # ゲーム進捗
  #[graphql(arguments(id(description = "id of game")))]
  fn progress_last(ctx: &AppContext, id: i32) -> Option<ProgressDto> {
    Some(ctx.progress_service().find_unique_progress(id))
  }
}



/// # クエリ定義
pub struct Mutation;

#[graphql_object(context = AppContext)]
impl Mutation {
  fn create_game_info(ctx: &AppContext, game_id: i32, state: i32) -> FieldResult<GameInfoDto> {
    Ok(ctx.game_info_service().insert_game_info(game_id, state))
  }
}