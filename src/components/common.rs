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
    pub position: Position,
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

pub struct WaitForTurn;
pub struct Active;
pub struct TookTurn;
impl Component for WaitForTurn {
    type Storage = HashMapStorage<WaitForTurn>;
}
impl Component for Active {
    type Storage = HashMapStorage<Active>;
}
impl Component for TookTurn {
    type Storage = HashMapStorage<TookTurn>;
}

impl Description {
    pub fn new(name: &str, description: &str) -> Description {
        Description {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}
