use std::collections::HashSet;
use tcod::input::{ self, Event, Mouse, Key, KeyCode };

#[derive(Default)]
pub struct InputHandler {
    pub mouse: Mouse,
    pub mouse_pos: (i32, i32),
    pub key: Key,
    pub pressed_keys: HashSet<char>,
    pub pressed_digit: Option<i32>,
    pub ctrl: bool,
}

impl InputHandler {
    fn register_key(&mut self, key: Key) {
        self.key = key;
        self.ctrl = key.ctrl;
        if key.code == KeyCode::Char {
            if key.pressed {
                self.pressed_keys.insert(key.printable);
            } else {
                self.pressed_keys.remove(&key.printable);
            }
        }
        if key.pressed {
            self.pressed_digit = match key.printable {
                '0' => Some(0),
                '1' => Some(1),
                '2' => Some(2),
                '3' => Some(3),
                '4' => Some(4),
                '5' => Some(5),
                '6' => Some(6),
                '7' => Some(7),
                '8' => Some(8),
                '9' => Some(9),
                _ => None,
            };
        } else {
            self.pressed_digit = None;
        }
    }

    fn register_mouse(&mut self, mouse: Mouse) {
        self.mouse = mouse;
        self.mouse_pos = (self.mouse.cx as i32, self.mouse.cy as i32);
    }

    pub fn is_mouse_pressed(&self) -> bool {
        self.mouse.lbutton_pressed
    }

    pub fn is_char_down(&self, key: char) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn is_char_pressed(&self, key: char) -> bool {
        self.key.printable == key && self.key.pressed
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.key.code == key && self.key.pressed
    }

    pub fn update(&mut self) {
        match input::check_for_event(input::MOUSE | input::KEY_PRESS | input::KEY_RELEASE) {
            Some((_, Event::Mouse(m))) => self.register_mouse(m),
            Some((_, Event::Key(k))) => self.register_key(k),
            _ => {
                self.key = Default::default();
                self.mouse = Default::default();
            },
        }
    }
}
