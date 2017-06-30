use std::collections::VecDeque;
use engine::tcod::{ Tcod };
use tcod::pathfinding::{ AStar };
use tcod::colors::{ self, Color };
use tile_map::{ TileMap };
use geometry::{ Ray };
use components::space::{ Viewport, Position };
use entity_map::{ EntityMap, EntityStackMap };
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

pub struct Maps {
    characters: EntityMap,
    items: EntityStackMap,
    tiles: TileMap,
    highlights: Vec<(i32, i32)>,
    highlight_color: Color,
}

impl Maps {
    pub fn new() -> Self {
        Maps {
            characters: EntityMap::new(),
            items: EntityStackMap::new(),
            tiles: TileMap::new(),
            highlights: vec![],
            highlight_color: colors::LIGHT_GREEN,
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
            self.is_planable(entity, end) { 0.0 } else { 1.0 };
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
            .filter_map(|p| self.characters.get(p))
            .collect::<VecDeque<Entity>>()
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

    pub fn is_blocking(&self, p: (i32, i32)) -> bool {
        self.tiles.is_blocking(p)
    }

    pub fn is_projectile_blocking(&self, p: (i32, i32)) -> bool {
        !self.tiles.is_discovered(p) || self.tiles.is_blocking(p)
    }

    pub fn is_planable(&self, entity: &Entity, p: (i32, i32)) -> bool {
        !self.tiles.is_discovered(p) || self.tiles.is_blocking(p)
            || self.characters.get(p)
            .map(|e| e != *entity).unwrap_or(false)
    }

    pub fn is_sight_blocking(&self, p: (i32, i32)) -> bool {
        self.tiles.is_sight_blocking(p)
    }

    pub fn is_occupied(&self, p: (i32, i32)) -> bool {
        self.characters.get(p).is_some()
    }

    pub fn build(&mut self) {
        self.tiles.build();
    }

    pub fn update(&mut self, tcod: &mut Tcod) {
        self.tiles.update(tcod);
    }

    pub fn draw(&self, tcod: &mut Tcod, viewport: &Viewport) {
        self.tiles.draw(tcod, viewport);

        for pos in self.highlights.iter() {
            let pixel = *pos;
            if viewport.visible(pixel) {
                let p = viewport.transform(pixel);
                tcod.highlight(p, self.highlight_color);
            }
        }
    }

    pub fn clear_all(&mut self) {
        self.items.clear();
        self.characters.clear();
    }

    pub fn move_entity(&mut self, map: Map, entity: &Entity, from: (i32, i32), to: (i32, i32)) {
        if from.0 != to.0 || from.1 != to.1 {
            match map {
                Map::Item => {
                    self.items.remove(entity, from);
                    self.items.push(entity, to);
                },
                Map::Character => {
                    self.characters.remove(entity, from);
                    self.characters.push(entity, to);
                },
            }
        }
    }

    pub fn remove(&mut self, map: Map, entity: &Entity, p: (i32, i32)) -> Option<Entity> {
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

    pub fn pop(&mut self, map: Map, p: (i32, i32)) -> Option<Entity> {
        match map {
            Map::Item => self.items.pop(p),
            Map::Character => self.items.pop(p),
        }
    }
}
