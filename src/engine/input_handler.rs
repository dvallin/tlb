use std::collections::HashSet;
use tcod::input::{ self, Event, Mouse, Key };

#[derive(Default)]
pub struct InputHandler {
    pub mouse: Mouse,
    pub key: Key,
    pub pressed_keys: HashSet<char>
}

impl InputHandler {
    fn register_key(&mut self, key: Key) {
        self.key = key;
        if key.pressed {
            self.pressed_keys.insert(key.printable);
        } else {
            self.pressed_keys.remove(&key.printable);
        }
    }

    pub fn is_pressed(&self, key: char) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn update(&mut self) {
        match input::check_for_event(input::MOUSE | input::KEY_PRESS | input::KEY_RELEASE) {
            Some((_, Event::Mouse(m))) => self.mouse = m,
            Some((_, Event::Key(k))) => self.register_key(k),
            _ => self.key = Default::default(),
        }
    }
}
