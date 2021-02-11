use crate::{
  app::app_context::AppContext, domain::progress::dto::ProgressDto,
  infrastructure::repository::game_info::entitiy::GameInfoEntitiy,
};
use diesel::Queryable;
use juniper::{graphql_object, GraphQLEnum};

// ゲーム情報
#[derive(Queryable, Debug, Clone)]
pub struct GameInfoDto {
  pub game_id: i32,
  pub state: GameState,
  pub start_time: chrono::NaiveDateTime,
  pub end_time: Option<chrono::NaiveDateTime>,
  pub progresses: Vec<ProgressDto>,
}

#[graphql_object(context = AppContext)]
impl GameInfoDto {
  /// ゲームID
  fn game_id(&self) -> &i32 {
    &self.game_id
  }

  /// ゲームのステータス
  fn state(&self) -> &GameState {
    &self.state
  }

  /// ゲームのステータス
  fn start_time(&self) -> chrono::NaiveDateTime {
    self.start_time
  }

  /// ゲームのステータス
  fn end_time(&self) -> Option<chrono::NaiveDateTime> {
    self.end_time
  }

  /// ゲームの進捗
  fn progresses(&self) -> &Vec<ProgressDto> {
    &self.progresses
  }
}

// impl FromInputValue for GameInfoDto {
//   fn from_input_value(v: &InputValue) -> Option<Self>{
//     GameInfoDto{
//       game_id: v.
//     }
//   }
// }

impl GameInfoDto {
  // pub fn new(id: i32, game_state: GameState) -> GameInfoDto {
  //   GameInfoDto {
  //     game_id: id,
  //     state: game_state,
  //   }
  // }

  pub fn from_entitiy(e: &GameInfoEntitiy, progresses: Vec<ProgressDto>) -> GameInfoDto {
    GameInfoDto {
      game_id: e.game_id,
      state: GameState::from_i32(e.state).unwrap(),
      start_time: e.start_time,
      end_time: e.end_time,
      progresses: progresses,
    }
  }
}

#[derive(GraphQLEnum, Debug, Clone, Copy)]
pub enum GameState {
  BlackTurn = 0,
  WhiteTurn = 1,
  BlackWon = 2,
  WhiteWon = 3,
  Draw = 4,
}
impl GameState {
  pub fn from_i32(i: i32) -> Option<GameState> {
    match i {
      0 => Some(GameState::BlackTurn),
      1 => Some(GameState::WhiteTurn),
      2 => Some(GameState::BlackWon),
      3 => Some(GameState::WhiteWon),
      4 => Some(GameState::Draw),
      _ => None,
    }
  }

  pub fn to_i32(&self) -> i32 {
    *self as i32
  }
}

#[cfg(test)]
mod tests {
  // use super::*;

  // #[test]
  // fn new() {
  //   let x = GameInfoDto::new(1, GameState::BlackTurn);
  //   assert_eq!(1, x.game_id);
  //   // assert_eq!(GameState::BlackTurn, x.state);
  // }
}
