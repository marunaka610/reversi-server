use crate::{
    app::app_context::AppContext,
    domain::game_info::{
        dto::GameInfoDto,
        service::{GameInfoService, HaveGameInfoService},
    },
    // util::error::CustomError,
};
use juniper::{graphql_object, FieldResult, IntoFieldError};

/// # クエリ定義
pub struct Mutation;

#[graphql_object(context = AppContext)]
impl Mutation {
    // # 新規ゲームを登録
    fn create_game_info(ctx: &AppContext) -> FieldResult<GameInfoDto> {
        match GameInfoService::insert(ctx.game_info_service()) {
            Ok(result) => Ok(result),
            Err(msg) => Err(msg.into_field_error()),
        }
    }

    // # 進捗を登録
    // fn create_progress(ctx: &AppContext, game_id: i32, state: i32) -> FieldResult<GameInfoDto> {
    //   Ok(GameInfoService::insert(

    //     ctx.progress_service()
    //   ))
    // }
}
