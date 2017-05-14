use tcod::colors::{ self };
use specs::{ Component, HashMapStorage };
use components::common::{ Health, Description };
use components::inventory::{ Inventory };
use components::appearance::{ Renderable };

pub struct Npc {
    pub instance: NpcInstance,
}

pub enum NpcInstance {
    Guard,
    Accountant,
    Technician
}

impl Component for Npc {
    type Storage = HashMapStorage<Npc>;
}

pub fn get_renderable(npc: &Npc) -> Renderable {
    use self::NpcInstance::*;
    match npc.instance {
        Guard => Renderable { character: 'G', color: colors::ORANGE },
        Accountant => Renderable { character: 'a', color: colors::GREY },
        Technician => Renderable { character: 'T', color: colors::YELLOW },
    }
}

pub fn get_description(npc: &Npc) -> Description {
    use self::NpcInstance::*;
    match npc.instance {
        Guard => Description::new("Walker", "Guard"),
        Accountant => Description::new("Phil", "Accountant"),
        Technician => Description::new("Spike", "Technician"),
    }
}

pub fn get_health(_npc: &Npc) -> Health {
    Health { health: 100.0 }
}

pub fn get_inventory(_npc: &Npc) -> Inventory {
    Inventory::new()
}
