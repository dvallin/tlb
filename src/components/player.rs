use specs::{ Component, HashMapStorage };
use components::space::{ Position };

pub struct Player {
    pub spawn: Position,
}

impl Component for Player {
    type Storage = HashMapStorage<Player>;
}

pub struct Fov {
    pub index: usize,
}

impl Component for Fov {
    type Storage = HashMapStorage<Fov>;
}
