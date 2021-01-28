use diesel::{Queryable};
use chrono::NaiveDateTime;
use juniper::{
  graphql_object,
  GraphQLEnum,
};
use crate::{
  app::app_context::AppContext,
  infrastructure::repository::progress::entitiy::ProgressEntitiy,
};

// Progress
#[derive(Queryable, Debug, Clone)]
pub struct ProgressDto {
  pub game_id: i32,
  pub piecies: Vec<i32>,
  pub time: NaiveDateTime,
}

#[graphql_object(context = AppContext)]
impl ProgressDto {
  /// T
  fn game_id(&self) -> &i32 {
    &self.game_id
  }

  /// 
  fn piecies(&self) -> &Vec<i32> {
    &self.piecies
  }

  /// 
  fn time(&self) -> &NaiveDateTime {
    &self.time
  }
}

impl ProgressDto {
  // pub fn new(id: i32, game_state: GameState) -> ProgressDto {
  //   ProgressDto {
  //     game_id: id,
  //     state: game_state,
  //   }
  // }

  pub fn from_entitiy(e: &ProgressEntitiy) -> ProgressDto{
    ProgressDto{
      game_id: e.game_id,
      piecies: e.piecies.iter()
        .map(|x| {
          let i = *x as i32;
          i
        })
        .collect(),
      time: e.time,
    }
  }
}


#[derive(GraphQLEnum, Debug, Clone, Copy)]
pub enum PieceState {
  None = 0,
  Black = 1,
  White = 2,
}
impl PieceState {
  pub fn from_u8(i : u8) -> Option<PieceState> {
    match i {
      0 => Some(PieceState::None),
      1 => Some(PieceState::Black),
      2 => Some(PieceState::White),
      _ => None
    }
  }

  // pub fn to_u8(g : PieceState) -> i32 {
  //   g as u8
  // }
}



#[cfg(test)]
mod tests {
  // use super::*;

  // #[test]
  // fn new() {
  //   let x = ProgressDto::new(1, GameState::BlackTurn);
  //   assert_eq!(1, x.game_id);
  //   // assert_eq!(GameState::BlackTurn, x.state);
  // }


}
