use juniper::{
  graphql_object,
};
use super::super::infrastructure::{
  database::*,
  entitiy::{
    game_info_entity,
  },
};

/**
 * クエリ定義
 */
pub struct Query;

/// The root query object of the schema
#[graphql_object(context = Database)]
impl Query {
  
  #[graphql(arguments(id(description = "id of the game")))]
  fn game_infos(database: &Database) -> Option<Vec<game_info_entity::GameInfoEntitiy>> {
    database.find_list_game_info()
  }

  #[graphql(arguments(id(description = "id of the game")))]
  fn game_info(database: &Database, id: i32) -> Option<game_info_entity::GameInfoEntitiy> {
    database.find_unique_game_info(id)
  }
}

