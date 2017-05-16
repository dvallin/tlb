use engine::tcod::{ Tcod };
use tile_map::{ TileMap };
use components::space::{ Viewport };
use entity_map::{ EntityMap, EntityStackMap };
use specs::{ Entity };

pub enum Map {
    Item,
    Character,
}

pub struct Maps {
    characters: EntityMap,
    items: EntityStackMap,
    tiles: TileMap,
}

impl Maps {
    pub fn new() -> Self {
        Maps {
            characters: EntityMap::new(),
            items: EntityStackMap::new(),
            tiles: TileMap::new(),
        }
    }

    pub fn is_blocking(&self, x: i32, y: i32) -> bool {
        self.tiles.is_blocking(x, y)
    }

    pub fn is_impassable(&self, entity: &Entity, x: i32, y: i32) -> bool {
        self.tiles.is_blocking(x, y) || self.characters.get(x, y)
            .map(|e| e != *entity).unwrap_or(false)
    }

    pub fn is_sight_blocking(&self, x: i32, y: i32) -> bool {
        self.tiles.is_sight_blocking(x, y)
    }

    pub fn build(&mut self) {
        self.tiles.build();
    }

    pub fn update(&mut self, tcod: &mut Tcod) {
        self.tiles.update(tcod);
    }

    pub fn draw(&self, tcod: &mut Tcod, viewport: &Viewport) {
        self.tiles.draw(tcod, viewport);
    }

    pub fn clear_all(&mut self) {
        self.items.clear();
        self.characters.clear();
    }

    pub fn move_entity(&mut self, map: Map, entity: &Entity, x1: i32, y1: i32, x2: i32, y2: i32) {
        if x1 != x2 || y1 != y2 {
            match map {
                Map::Item => {
                    self.items.remove(entity, x1, y1);
                    self.items.push(entity, x2, y2);
                },
                Map::Character => {
                    self.characters.remove(entity, x1, y1);
                    self.characters.push(entity, x2, y2);
                },
            }
        }
    }

    pub fn push(&mut self, map: Map, entity: &Entity, x: i32, y: i32) {
        match map {
            Map::Item => self.items.push(entity, x, y),
            Map::Character => self.characters.push(entity, x, y),
        }
    }

    pub fn pop(&mut self, map: Map, x: i32, y: i32) -> Option<Entity> {
        match map {
            Map::Item => self.items.pop(x, y),
            Map::Character => self.items.pop(x, y),
        }
    }
}
