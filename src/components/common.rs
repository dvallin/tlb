use std::collections::VecDeque;
use components::space::{ Position };
use specs::{ Component, HashMapStorage, VecStorage };


pub struct Description {
    pub name: String,
    pub description: String,
}

pub struct CharacterStats {
    pub health: f32,
    pub max_health: f32,
}

pub struct ItemStats {
    pub damage: f32,
    pub range: i32,
}

impl CharacterStats {
    pub fn apply_damage(&mut self, item: &ItemStats) -> f32 {
        let damage = item.damage;
        self.health -= damage;
        damage
    }

    pub fn reset(&mut self) {
        self.health = self.max_health;
    }
}

pub struct MoveToPosition {
    pub path: VecDeque<Position>,
    pub speed: f32,
}

impl Component for Description {
    type Storage = VecStorage<Description>;
}

impl Component for MoveToPosition {
    type Storage = VecStorage<MoveToPosition>;
}

impl Component for CharacterStats {
    type Storage = HashMapStorage<CharacterStats>;
}

impl Component for ItemStats {
    type Storage = HashMapStorage<ItemStats>;
}

pub struct Active;
impl Component for Active {
    type Storage = HashMapStorage<Active>;
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum InTurnState {
    Idle,
    Walking,
    Fighting,
}

pub struct InTurn {
    pub state: InTurnState,
    pub has_walked: bool,
    pub action_points: i32,
}

impl Default for InTurn {
    fn default() -> Self {
        InTurn { state: InTurnState::Idle, has_walked: false, action_points: 2 }
    }
}

impl InTurn {
    pub fn walk(&mut self, cost: i32) {
        self.state = InTurnState::Walking;
        self.has_walked = true;
        self.action_points -= cost;
    }

    pub fn fight(&mut self) {
        self.state = InTurnState::Fighting;
        self.action_points = 0;
    }

    pub fn is_done(&self) -> bool {
        self.state == InTurnState::Idle && self.action_points <= 0
    }

    pub fn action_done(&mut self) {
        self.state = InTurnState::Idle;
    }
}

impl Component for InTurn {
    type Storage = HashMapStorage<InTurn>;
}

pub struct WaitForTurn;
impl Component for WaitForTurn {
    type Storage = HashMapStorage<WaitForTurn>;
}

impl Description {
    pub fn new(name: &str, description: &str) -> Description {
        Description {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}
