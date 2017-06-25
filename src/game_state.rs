pub struct GameState {
    pub is_turn_based: bool,
}

impl Default for GameState {
    fn default() -> Self {
        GameState { is_turn_based: false }
    }
}

impl GameState {
    pub fn reset(&mut self) {
        *self = GameState::default();
    }
}
