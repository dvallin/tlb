use components::space::{ Position };
use specs::{ Component, HashMapStorage, VecStorage };


pub struct Description {
    pub name: String,
    pub description: String,
}

pub struct Health {
    pub health: f32,
}

pub struct MoveToPosition {
    pub path: Vec<Position>,
    pub speed: f32,
}

impl Component for Description {
    type Storage = VecStorage<Description>;
}

impl Component for MoveToPosition {
    type Storage = VecStorage<MoveToPosition>;
}


impl Component for Health {
    type Storage = HashMapStorage<Health>;
}

pub struct Active;
impl Component for Active {
    type Storage = HashMapStorage<Active>;
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum InTurnState {
    Idle,
    Walking,
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
