use tcod::colors::{ Color };
use tcod::console::{ Console, BackgroundFlag };

use specs::{ Component, VecStorage };

pub struct Renderable {
    pub character: char,
    pub color: Color,
}

impl Component for Renderable {
    type Storage = VecStorage<Renderable>;
}
