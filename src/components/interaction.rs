use components::player::{ Equipment };
use components::item::{ Item, ItemInstance };
use specs::{ Component, HashMapStorage };

pub enum Interactable {
    KeyDoor(i32, bool)
}

impl Interactable {
    fn key_door_new_state(min_level: i32, open: bool, tool: &Option<&Item>) -> bool {
        if let Some(item) = *tool {
            match item.instance {
                ItemInstance::KeyCard(level) => {
                    if open {
                        false
                    } else {
                        level >= min_level
                    }
                }
                _ => false
            }
        } else {
            false
        }
    }

    pub fn interaction_priority(&self, active: &Option<&Item>, passive: &Option<&Item>, clothing: &Option<&Item>) -> Option<i32> {
        match *self {
            Interactable::KeyDoor(min_level, _) => {
                if let Some(item) = *active {
                    match item.instance {
                        ItemInstance::KeyCard(level) => {
                            if level >= min_level {
                                Some(1)
                            } else {
                                Some(0)
                            }
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }
        }
    }

    pub fn interacts_with(&self, active: &Option<&Item>, passive: &Option<&Item>, clothing: &Option<&Item>) -> bool {
        self.interaction_priority(active, passive, clothing).is_some()
    }

    pub fn interact_with(&mut self, active: &Option<&Item>, passive: &Option<&Item>, clothing: &Option<&Item>) {
        match *self {
            Interactable::KeyDoor(min_level, mut state) => {
                state = Interactable::key_door_new_state(min_level, state, active);
                state = Interactable::key_door_new_state(min_level, state, passive);
            }
        }
    }
}


impl Component for Interactable {
    type Storage = HashMapStorage<Interactable>;
}
