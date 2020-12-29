pub struct GameInfo{
  game_id : u128,
  state : GameState,
  
}

impl GameInfo {
  pub fn new(id: u128, state : GameState) -> GameInfo {
    // self.game_id = id;
    // self.state = state;
    GameInfo {
      game_id: id,
      state: state
    }
  }
}


pub enum GameState {
  BlackTurn,
  WhiteTurn,
  BlackWon,
  WhiteWon,
  Draw,
}
