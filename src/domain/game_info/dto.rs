use juniper::{
  graphql_object,
  GraphQLEnum,
};
use super::{
  super::super::{
    app::app_context::AppContext,
    infrastructure::repository::game_info::entitiy::GameInfoEntitiy,
  },
};
use diesel::{Queryable};

// GameInfo
#[derive(Queryable, Debug, Clone, Copy)]
pub struct GameInfoDto {
  pub game_id: i32,
  pub state: GameState,
}

#[graphql_object(context = AppContext)]
impl GameInfoDto {
  /// T
  fn game_id(&self) -> &i32 {
    &self.game_id
  }

  /// 
  fn state(&self) -> &GameState {
    &self.state
  }
}

impl GameInfoDto {
  // pub fn new(id: i32, game_state: GameState) -> GameInfoDto {
  //   GameInfoDto {
  //     game_id: id,
  //     state: game_state,
  //   }
  // }

  pub fn from_entitiy(e: &GameInfoEntitiy) -> GameInfoDto{
    GameInfoDto{
      game_id: e.game_id,
      state: GameState::from_i32(e.state).unwrap(),
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
  pub fn from_i32(i : i32) -> Option<GameState> {
    match i {
      0 => Some(GameState::BlackTurn),
      1 => Some(GameState::WhiteTurn),
      2 => Some(GameState::BlackWon),
      3 => Some(GameState::WhiteWon),
      4 => Some(GameState::Draw),
      _ => None
    }
  }

  // pub fn to_i32(g : GameState) -> i32 {
  //   g as i32
  // }
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
