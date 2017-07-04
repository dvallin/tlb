use std::collections::VecDeque;
use engine::tcod::{ Tcod };
use tcod::pathfinding::{ AStar };
use tcod::colors::{ self, Color };
use tile_map::{ TileMap };
use geometry::{ Shape, Ray };
use std::collections::{ HashMap };
use components::space::{ Viewport, Position, Level };
use entity_map::{ EntityMap, Entry };
use specs::{ Entity };

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;
const MAP_Y: i32 = SCREEN_HEIGHT - MAP_HEIGHT;

pub enum Map {
    Item,
    Character,
}

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

    pub fn build(&mut self) {
        for (level, maps) in &mut self.maps {
            maps.build();
        }
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

pub struct Maps {
    characters: EntityMap,
    items: EntityMap,
    tiles: TileMap,
}

impl Maps {
    pub fn new() -> Self {
        Maps {
            characters: EntityMap::new(),
            items: EntityMap::new(),
            tiles: TileMap::new(),
        }
    }

    pub fn screen_to_map(&self, pos: (i32, i32)) -> Option<(i32, i32)> {
        let p = (pos.0, pos.1 - MAP_Y);
        if p.0 >= 0 && p.0 < MAP_WIDTH && p.1 >= 0 && p.1 < MAP_HEIGHT {
            Some(p)
        } else {
            None
        }
    }

    pub fn find_path(&self, entity: &Entity,
                     from: (i32, i32), to: (i32, i32)) -> VecDeque<Position> {
        let callback = |start: (i32,i32), end:(i32,i32) | if
            self.is_not_planable(entity, end) { 0.0 } else { 1.0 };
        let mut astar = AStar::new_from_callback(MAP_WIDTH, MAP_HEIGHT, callback, 0.0);
        astar.find(from, to);
        astar.walk()
            .map(|p| Position { x: p.0 as f32 + 0.5, y: p.1 as f32 + 0.5 })
            .collect::<VecDeque<Position>>()
    }

    pub fn draw_ray(&self, from: (i32, i32), to: (i32, i32), length: i32) -> VecDeque<Position> {
        let p0 = Position { x: from.0 as f32 + 0.5, y: from.1 as f32 + 0.5 };
        Ray::new(from, to).into_iter()
            .skip(1)
            .take_while(|p| !self.is_projectile_blocking(*p))
            .map(|p| Position { x: p.0 as f32 + 0.5, y: p.1 as f32 + 0.5 })
            .take_while(|p| (p0-*p).length() as i32 <= length)
            .collect::<VecDeque<Position>>()
    }

    pub fn collect_characters_with_ray(&self,
                            from: (i32, i32), to: (i32, i32), length: i32) -> VecDeque<Entity> {
        let p0 = Position { x: from.0 as f32 + 0.5, y: from.1 as f32 + 0.5 };
        Ray::new(from, to).into_iter()
            .skip(1)
            .take_while(|p| !self.is_projectile_blocking(*p))
            .take_while(|p| {
                let ps = Position { x: p.0 as f32 + 0.5, y: p.1 as f32 + 0.5 };
                (p0-ps).length() as i32 <= length
            })
            .flat_map(|p| self.characters.get(p))
            .map(|e| e.0)
            .collect::<VecDeque<Entity>>()
    }

    pub fn collect_characters_with_shape<T>(&self, shape: T) -> Vec<Entity> where T: Shape {
        shape.into_iter()
            .flat_map(|p| self.characters.get(p))
            .map(|e| e.0)
            .collect::<Vec<Entity>>()
    }

    pub fn collect_items_with_shape<T>(&self, shape: T) -> Vec<Entity> where T: Shape {
        shape.into_iter()
            .flat_map(|p| self.items.get(p))
            .map(|e| e.0)
            .collect::<Vec<Entity>>()
    }

    pub fn is_blocking(&self, p: (i32, i32)) -> bool {
        self.tiles.is_blocking(p)
    }

    pub fn is_projectile_blocking(&self, p: (i32, i32)) -> bool {
        !self.tiles.is_discovered(p) || self.tiles.is_blocking(p)
    }

    pub fn is_not_planable(&self, entity: &Entity, p: (i32, i32)) -> bool {
        !self.tiles.is_discovered(p) || self.tiles.is_blocking(p)
            || self.characters.get(p).iter().any(|e| e.0 != *entity && e.1 )
    }

    pub fn is_sight_blocking(&self, p: (i32, i32)) -> bool {
        self.tiles.is_sight_blocking(p)
            || self.characters.get(p).iter().any(|e| e.2 )
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

    pub fn move_entity(&mut self, map: Map, entity: &Entity, from: (i32, i32), to: (i32, i32)) {
        if from.0 != to.0 || from.1 != to.1 {
            match map {
                Map::Item => {
                    if let Some(entry) = self.items.remove(entity, from) {
                        self.items.push_entry(entry, to);
                    }                },
                Map::Character => {
                    if let Some(entry) = self.characters.remove(entity, from) {
                        self.characters.push_entry(entry, to);
                    }
                }
            }
        }
    }

    pub fn remove(&mut self, map: Map, entity: &Entity, p: (i32, i32)) -> Option<Entry> {
        match map {
            Map::Item => self.items.remove(entity, p),
            Map::Character => self.characters.remove(entity, p),
        }
    }

    pub fn push(&mut self, map: Map, entity: &Entity, p: (i32, i32)) {
        match map {
            Map::Item => self.items.push(entity, p),
            Map::Character => self.characters.push(entity, p),
        }
    }

    pub fn pop(&mut self, map: Map, p: (i32, i32)) -> Option<Entry> {
        match map {
            Map::Item => self.items.pop(p),
            Map::Character => self.items.pop(p),
        }
    }

    pub fn set_blocking(&mut self, map: Map, entity: &Entity, p: (i32, i32), blocking: bool) {
        match map {
            Map::Item => self.items.set_blocking(entity, p, blocking),
            Map::Character => self.characters.set_blocking(entity, p, blocking),
        }
    }

    pub fn set_sight_blocking(&mut self, map: Map, entity: &Entity, p: (i32, i32), blocking: bool) {
        match map {
            Map::Item => self.items.set_sight_blocking(entity, p, blocking),
            Map::Character => self.characters.set_sight_blocking(entity, p, blocking),
        }
    }
}
