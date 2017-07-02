pub struct GameState {
    pub is_turn_based: bool,
    pub fov_needs_update: bool,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            is_turn_based: false,
            fov_needs_update: true
        }
    }
}

impl GameState {
    pub fn reset(&mut self) {
        *self = GameState::default();
    }
}
