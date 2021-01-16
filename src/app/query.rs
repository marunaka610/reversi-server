use juniper::{
  graphql_object,
};
use super::{
  app_context::AppContext,
  super::{
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
  },
};

/**
 * クエリ定義
 */
pub struct Query;

/// The root query object of the schema
#[graphql_object(context = AppContext)]
impl Query {
  
  #[graphql(arguments())]
  fn game_infos(ctx: &AppContext) -> Option<Vec<GameInfoDto>> {
    Some(ctx.game_info_service().find_all_game_info())
  }

  #[graphql(arguments(id(description = "id of the game")))]
  fn game_info(ctx: &AppContext, id: i32) -> Option<GameInfoDto> {
    Some(ctx.game_info_service().find_unique_game_info(id))
  }

  #[graphql(arguments())]
  fn pregresses(ctx: &AppContext) -> Option<Vec<ProgressDto>> {
    Some(ctx.game_info_service().find_all_progress())
  }

  #[graphql(arguments(id(description = "id of game")))]
  fn progress(ctx: &AppContext, id: i32) -> Option<ProgressDto> {
    Some(ctx.progress_service().find_unique_progress(id))
  }
}

