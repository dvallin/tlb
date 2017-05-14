use specs::{ Component, HashMapStorage, VecStorage };

pub struct Active;

pub struct Description {
    pub name: String,
    pub description: String,
}

pub struct Health {
    pub health: f32,
}

pub struct Round {
    pub round: i32,
}

impl Component for Description {
    type Storage = VecStorage<Description>;
}

impl Component for Active {
    type Storage = HashMapStorage<Active>;
}

impl Component for Health {
    type Storage = HashMapStorage<Health>;
}

impl Component for Round {
    type Storage = HashMapStorage<Round>;
}

impl Description {
    pub fn new(name: &str, description: &str) -> Description {
        Description {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}
