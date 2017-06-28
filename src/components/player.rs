use specs::{ Component, HashMapStorage, Entity };

pub struct Player;

impl Component for Player {
    type Storage = HashMapStorage<Player>;
}

pub struct Fov {
    pub index: usize,
}

impl Component for Fov {
    type Storage = HashMapStorage<Fov>;
}

pub struct Equipment {
    pub active_item: Option<Entity>,
    pub passive_item: Option<Entity>,
    pub clothing: Option<Entity>,
}

impl Component for Equipment {
    type Storage = HashMapStorage<Equipment>;
}

impl Equipment {
    pub fn new() -> Self {
        Equipment {
            active_item: None,
            passive_item: None,
            clothing: None,
        }
    }
}
