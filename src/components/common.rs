use specs::{ Component, VecStorage };

pub struct Description {
    pub name: String,
}

pub struct Stats {
    pub health: f32,
}

impl Component for Description {
    type Storage = VecStorage<Description>;
}

impl Component for Stats {
    type Storage = VecStorage<Stats>;
}
