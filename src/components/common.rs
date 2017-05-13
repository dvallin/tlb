use specs::{ Component, HashMapStorage, VecStorage };

pub struct Description {
    pub name: String,
    pub description: String,
}

impl Description {
    pub fn new(name: &str, description: &str) -> Description {
        Description {
            name: name.to_string(),
            description: description.to_string(),
        }
    }
}

pub struct Health {
    pub health: f32,
}

pub struct Weight {
    pub weight: f32,
}

impl Component for Description {
    type Storage = VecStorage<Description>;
}

impl Component for Health {
    type Storage = HashMapStorage<Health>;
}

impl Component for Weight {
    type Storage = VecStorage<Weight>;
}
