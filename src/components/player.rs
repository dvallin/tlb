use specs::{ Component, HashMapStorage };

pub struct Player {
    pub active: bool,
    pub index: usize,
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
