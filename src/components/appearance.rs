use tcod::colors::{ Color };

use specs::{ Component, HashMapStorage, VecStorage };

pub struct Layer0;
pub struct Layer1;

pub struct Renderable {
    pub character: char,
    pub color: Color,
}

impl Renderable {
    pub fn new(character: char, color: Color) -> Self {
        Renderable {
            character: character,
            color: color,
        }
    }
}


impl Component for Layer0 {
    type Storage = VecStorage<Layer0>;
}

impl Component for Layer1 {
    type Storage = HashMapStorage<Layer1>;
}

impl Component for Renderable {
    type Storage = VecStorage<Renderable>;
}
