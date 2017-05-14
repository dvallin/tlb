use engine::tcod::{ Tcod };
use tile_map::{ TileMap };
use components::space::{ Viewport };
use entity_map::{ EntityMap };
use specs::{ Entity };

pub struct Maps {
    items: EntityMap,
    tiles: TileMap,
}

impl Maps {
    pub fn new() -> Self {
        Maps {
            items: EntityMap::new(),
            tiles: TileMap::new(),
        }
    }

    pub fn is_blocking(&self, x: i32, y: i32) -> bool {
        self.tiles.is_blocking(x, y)
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

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn push_item(&mut self, entity: &Entity, x: i32, y: i32) {
        self.items.push(entity, x, y);
    }

    pub fn pop_item(&mut self, x: i32, y: i32) -> Option<Entity> {
        self.items.pop(x, y)
    }
}
