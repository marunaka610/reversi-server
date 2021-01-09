
// GameInfo
pub struct GameInfo {
  pub game_id: i32,
  pub state: GameState,
}

impl GameInfo {
  pub fn new(id: i32, game_state: GameState) -> GameInfo {
    // self.game_id = id;
    // self.state = state;
    GameInfo {
      game_id: id,
      state: game_state,
    }
  }
}


#[derive(Debug)]
pub enum GameState {
  BlackTurn,
  WhiteTurn,
  BlackWon,
  WhiteWon,
  Draw,
}




#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    let x = GameInfo::new(1, GameState::BlackTurn);
    assert_eq!(1, x.game_id);
    // assert_eq!(GameState::BlackTurn, x.state);
  }


}
