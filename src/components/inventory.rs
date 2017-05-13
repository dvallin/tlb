use specs::{ Component, HashMapStorage, Entity };

pub struct Inventory {
    pub items: Vec<Entity>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            items: vec![],
        }
    }
}

impl Component for Inventory {
    type Storage = HashMapStorage<Inventory>;
}

impl Inventory {
    pub fn push(&mut self, entity: Entity) {
        self.items.push(entity);
    }

    pub fn pop(&mut self) -> Option<Entity> {
        self.items.pop()
    }
}
