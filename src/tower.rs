use specs::{ World, Entity };
use std::collections::VecDeque;
use engine::tcod::{ Tcod };
use tcod::colors::{ self, Color };
use std::collections::{ HashMap };
use maps::{ Maps };

use components::appearance::{ Renderable, Layer0, Layer1 };
use components::player::{ Player, Fov, Equipment };
use components::space::{ Viewport, Spawn, Position, Level };
use components::npc::{ Npc, NpcInstance };
use components::item::{ Item, ItemInstance };
use components::common::{ Active, CharacterStats, Description };
use components::interaction::{ Interactable, InteractableInstance };
use components::inventory::{ Inventory };


pub struct Tower {
    maps: HashMap<Level, Maps>,
    highlights: Vec<(i32, i32)>,
    highlight_color: Color,
}

impl Tower {
    pub fn new(levels: &[Level]) -> Self {
        let mut maps = HashMap::new();
        for level in levels {
            maps.insert(*level, Maps::new());
        }
        Tower {
            maps: maps,
            highlights: vec![],
            highlight_color: colors::LIGHT_GREEN,
        }
    }

    fn create_player(&mut self, x: f32, y: f32, active: bool, name: String,
                     tcod: &mut Tcod, world: &mut World) {
        let fov_map = self.create_fov(tcod);
        let mut builder = world.create_now()
            .with(Player)
            .with(Spawn::for_location(x, y, Level::Tower(0)))
            .with(Renderable { character: '@', color: colors::WHITE })
            .with(CharacterStats { health: 100.0, max_health: 100.0 } )
            .with(Description { name: name, description: "".into() })
            .with(Fov { fov_map: fov_map})
            .with(Inventory::new())
            .with(Equipment::new())
            .with(Layer1);
        if active {
            builder = builder.with(Active);
        }
        builder.build();
    }

    fn create_npc(&mut self, x: f32, y: f32, level: Level,
                  instance: NpcInstance, world: &mut World) -> Entity {
        let n = Npc { instance: instance };
        let builder = world.create_now()
            .with(Spawn::for_location(x, y, level))
            .with(n.get_renderable())
            .with(n.get_description())
            .with(n.get_stats())
            .with(Inventory::new())
            .with(n)
            .with(Layer1);
        builder.build()
    }

    fn create_interactable(&mut self, x: f32, y: f32, level: Level,
                           instance: InteractableInstance, world: &mut World) -> Entity {
        let interactable = Interactable::new(instance);
        let builder = world.create_now()
            .with(Spawn::for_location(x, y, level))
            .with(interactable.get_renderable())
            .with(interactable)
            .with(Layer0);
        builder.build()
    }

    fn create_inventory(&mut self, owner: Entity, items: Vec<ItemInstance>, world: &mut World) {
        let mut inventory = Inventory::new();
        for instance in items {
            let i = Item { instance: instance };
            let mut item = world.create_now()
                .with(Spawn::for_owner(owner))
                .with(i.get_renderable())
                .with(i.get_description());
            if let Some(c) = i.get_stats() {
                item = item.with(c)
            }
            item = item.with(i).with(Layer0);
            inventory.items.push(item.build());
        }
        world.write().insert(owner, inventory);
    }

    fn create_item(&mut self, x: f32, y: f32, level: Level,
                   instance: ItemInstance, world: &mut World) {
        let i = Item { instance: instance };
        let mut builder = world.create_now()
            .with(Spawn::for_location(x, y, level))
            .with(i.get_renderable())
            .with(i.get_description());
        if let Some(c) = i.get_stats() {
            builder = builder.with(c)
        }
        builder
            .with(i)
            .with(Layer0)
            .build();
    }

    pub fn build(&mut self, tcod: &mut Tcod, world: &mut World) {
        {
            let lvl0 = self.get_mut(&Level::Tower(0)).unwrap();
            lvl0.build();
        }

        self.create_player(15.0, 15.0, true, "Colton".into(), tcod, world);
        self.create_player(16.0, 16.0, false, "Gage".into(), tcod, world);

        self.create_interactable(25.0, 21.0, Level::Tower(0),
                                 InteractableInstance::KeyDoor(3, false), world);

        {
            let guard = self.create_npc(31.0, 24.0, Level::Tower(0),
                                        NpcInstance::Guard, world);
            self.create_inventory(guard, vec![ItemInstance::FlickKnife,
                                              ItemInstance::Watch,
                                              ItemInstance::KeyCard(3)], world);
        }
        self.create_npc(29.0, 24.0, Level::Tower(0), NpcInstance::Technician, world);
        self.create_npc(31.0, 29.0, Level::Tower(0), NpcInstance::Accountant, world);

        self.create_item(14.0, 15.0, Level::Tower(0), ItemInstance::FlickKnife, world);
        self.create_item(13.0, 15.0, Level::Tower(0), ItemInstance::DartGun, world);
        self.create_item(33.0, 25.0, Level::Tower(0), ItemInstance::Simstim, world);
        self.create_item(23.0, 25.0, Level::Tower(0), ItemInstance::HitachiRam, world);
        self.create_item(28.0, 21.0, Level::Tower(0), ItemInstance::Shuriken, world);
    }

    pub fn clear(&mut self) {
        for (level, maps) in &mut self.maps {
            maps.clear_all();
        }
    }

    pub fn get_mut(&mut self, level: &Level) -> Option<&mut Maps> {
        self.maps.get_mut(level)
    }

    pub fn get(&self, level: &Level) -> Option<&Maps> {
        self.maps.get(level)
    }

    pub fn update(&mut self, tcod: &mut Tcod) {
        for (level, maps) in &mut self.maps {
            maps.update(tcod);
        }
    }

    pub fn draw(&self, level: &Level, tcod: &mut Tcod, viewport: &Viewport) {
        if let Some(map) = self.maps.get(level) {
            map.draw(tcod, viewport);
        }

        for pos in self.highlights.iter() {
            let pixel = *pos;
            if viewport.visible(pixel) {
                let p = viewport.transform(pixel);
                tcod.highlight(p, self.highlight_color);
            }
        }
    }

    pub fn create_fov(&self, tcod: &mut Tcod) -> HashMap<Level, usize> {
        let mut result = HashMap::new();
        for (level, maps) in &self.maps {
            result.insert(*level, tcod.create_fov());
        }
        result
    }

    pub fn clear_highlights(&mut self) {
        self.highlights.clear();
    }

    pub fn set_highlight_color(&mut self, color: Color) {
        self.highlight_color = color;
    }

    pub fn add_highlights(&mut self, highlights: VecDeque<Position>) {
        self.highlights.extend(
            highlights
                .iter()
                .cloned()
                .map(|p| (p.x as i32, p.y as i32))
        );
    }
}
