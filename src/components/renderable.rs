use tcod::colors::{ Color };
use tcod::console::{ Console, BackgroundFlag };

use specs::{ Component, VecStorage };

pub struct Renderable {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}
impl Renderable {
    pub fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
    pub fn clear(&self, con: &mut Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}

impl Component for Renderable {
    type Storage = VecStorage<Renderable>;
}
