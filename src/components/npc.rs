use tcod::colors::{ self };
use rand::{ Rng };
use base64::{ encode };
use specs::{ Component, HashMapStorage };
use components::common::{ CharacterStats, Description };
use components::inventory::{ Inventory };
use components::appearance::{ Renderable };

pub struct Npc {
    pub instance: NpcInstance,
}

pub enum NpcInstance {
    Guard,
    Grunt,
    Accountant,
    Technician
}

impl Component for Npc {
    type Storage = HashMapStorage<Npc>;
}

impl Npc {
    pub fn get_renderable(&self) -> Renderable {
        use self::NpcInstance::*;
        match self.instance {
            Guard => Renderable { character: 'G', color: colors::ORANGE },
            Accountant => Renderable { character: 'a', color: colors::GREY },
            Technician => Renderable { character: 'T', color: colors::YELLOW },
        }
    }

    pub fn name<R: Rng>(rng: &mut R) -> &str {
        let names = ["Chopra", "Nagata", "Deep-Ando", "Walker", "Spike", "Rex", "Troy", "Del", "Dash", "Proto", "Ogura"];
        names[(rng.next_u32() % 11) as usize]
    }

    pub fn get_description<R: Rng>(&self, rng: &mut R) -> Description {
        use self::NpcInstance::*;
        let name = Npc::name(rng);
        match self.instance {
            Guard => Description::new(name, "Guard"),
            Grunt => Description::new(encode(name).as_str(), "Grunt"), // they are not just numbers ;)
            Accountant => Description::new(name, "Accountant"),
            Technician => Description::new(name, "Technician"),
        }
    }

    pub fn get_stats(&self) -> CharacterStats {
        CharacterStats { health: 100.0, max_health: 100.0 }
    }
}
