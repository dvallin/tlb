use std::collections::{ HashMap };
use specs::{ Component, HashMapStorage, Entity };
use components::space::{ Level };

pub struct Player;

impl Component for Player {
    type Storage = HashMapStorage<Player>;
}

pub struct Fov {
    pub fov_map: HashMap<Level, usize>,
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
