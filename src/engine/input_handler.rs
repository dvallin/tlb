use tcod::input::{ self, Event, Mouse, Key };

#[derive(Default)]
pub struct InputHandler {
    pub mouse: Mouse,
    pub key: Key,
}

impl InputHandler {
    pub fn update(&mut self) {
        match input::check_for_event(input::MOUSE | input::KEY_PRESS) {
            Some((_, Event::Mouse(m))) => self.mouse = m,
            Some((_, Event::Key(k))) => self.key = k,
            _ => self.key = Default::default(),
        }
    }
}
