use specs::{ Component, HashMapStorage };

pub struct PlayerControlled {
}

impl Component for PlayerControlled {
    type Storage = HashMapStorage<PlayerControlled>;
}
