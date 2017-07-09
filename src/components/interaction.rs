use components::appearance::{ Renderable };
use components::item::{ Item, ItemInstance };
use specs::{ Component, HashMapStorage, Entity };
use tcod::colors::{ self };


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InteractableInstance {
    KeyDoor(i32, bool)
}

pub struct Interaction {
    pub actor: Entity,
}
impl Component for Interaction {
    type Storage = HashMapStorage<Interaction>;
}

pub struct Interactable {
    state: InteractableInstance,
    initial_state: InteractableInstance,
}

impl Interactable {
    pub fn new(instance: InteractableInstance) -> Self {
        Interactable { state: instance, initial_state: instance }
    }

    fn key_door_new_state(min_level: i32, open: bool, tool: Option<&Item>) -> bool {
        if open {
            false
        } else {
            if let Some(item) = tool {
                match item.instance {
                    ItemInstance::KeyCard(level) => level >= min_level,
                    _ => min_level == 0
                }
            } else {
               min_level == 0
            }
        }
    }

    pub fn interact_with(&mut self, active: Option<&Item>, passive: Option<&Item>, clothing: Option<&Item>) {
        use self::InteractableInstance::*;
        self.state = match self.state {
            KeyDoor(min_level, open) => {
                let mut new_open = Interactable::key_door_new_state(min_level, open, active);
                if open == new_open {
                    new_open = Interactable::key_door_new_state(min_level, open, passive);
                }
                KeyDoor(min_level, new_open)
            }
        }
    }

    pub fn get_renderable(&self) -> Renderable {
        use self::InteractableInstance::*;
        match self.state {
            KeyDoor(_level, open) => {
                let mut c = '_';
                if !open {
                    c = 'D';
                }
                let color = colors::GOLD;
                Renderable::new(c, color)
            }
        }
    }

    pub fn is_blocking(&self) -> bool {
        use self::InteractableInstance::*;
        match self.state {
            KeyDoor(_level, open) => {
                !open
            }
        }
    }

    pub fn is_sight_blocking(&self) -> bool {
        use self::InteractableInstance::*;
        match self.state {
            KeyDoor(_level, open) => {
                !open
            }
        }
    }

    pub fn reset(&mut self) {
        self.state = self.initial_state;
    }
}


impl Component for Interactable {
    type Storage = HashMapStorage<Interactable>;
}


