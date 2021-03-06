use engine::tcod::{ Tcod };

use tcod::colors::{ self, Color };
use tcod::chars::{ self };

use components::space::{ Viewport };
use geometry::{ Shape, Line, Rect };

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;

#[derive(Clone, Debug)]
struct Tile {
    blocking: bool,
    discovered: bool,
    wall: bool,
    room: Option<i32>,
}

impl Tile {
    pub fn create(blocking: bool, wall: bool, room: Option<i32>) -> Self {
        Tile { blocking: blocking, wall: wall, room: room, discovered: false }
    }

    pub fn bedrock() -> Self {
        Tile::create(true, false, None)
    }

    pub fn wall(room: i32) -> Self {
        Tile::create(true, true, Some(room))
    }

    pub fn floor(room: i32) -> Self {
        Tile::create(false, false, Some(room))
    }

    pub fn character(&self) -> Option<char> {
        if !self.discovered {
            return None;
        }

        if self.wall {
            return Some('#');
        } else if !self.blocking {
            return Some('.');
        }
        None
    }

    pub fn fg_color(&self, visible: bool) -> Color {
        if !self.discovered {
            return colors::BLACK;
        }

        if visible {
            return colors::LIGHTEST_GREY;
        } else {
            return colors::DARK_GREEN;
        }
    }

    pub fn bg_color(&self, visible: bool) -> Color {
        if !self.discovered || !self.blocking || !visible {
            return colors::DARKEST_GREY;
        }

        if visible {
            return colors::DARKER_GREY;
        } else {
            return colors::DARKEST_GREY;
        }
    }

    pub fn update(&mut self, visible: bool) {
        self.discovered = self.discovered || visible;
    }
}

pub struct TileMap {
    map: Vec<Vec<Tile>>,
    width: i32,
    height: i32,
    rooms: i32,
}

impl TileMap {
    pub fn new() -> Self {
        let map = vec![vec![Tile::bedrock(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
        TileMap {
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            map: map,
            rooms: 0
        }
    }

    pub fn update(&mut self, tcod: &Tcod) {
        for y in 0..self.height {
            for x in 0..self.width {
                let visible = tcod.is_in_fov((x,y));
                self.map[x as usize][y as usize].update(visible);
            }
        }
    }

    pub fn draw(&self, tcod: &mut Tcod, viewport: &Viewport) {
        let default = Tile::bedrock();
        for pixel in viewport.into_iter() {
            let tile = self.get(pixel).unwrap_or(&default);
            if let Some(character) = tile.character() {
                let p = viewport.transform(pixel);
                let visible = tcod.is_in_fov(pixel);
                let fg_color = self.map[pixel.0 as usize][pixel.1 as usize].fg_color(visible);
                let bg_color = self.map[pixel.0 as usize][pixel.1 as usize].bg_color(visible);
                tcod.render(p, bg_color, fg_color, character);
            }
        }
    }

    pub fn create_room<T>(self: &mut TileMap, room: &T) where T: Shape {
        let id = self.rooms;
        self.rooms += 1;
        for pos in room.into_iter() {
            let tile = if room.is_boundary(pos) {
                Tile::wall(id)
            } else {
                Tile::floor(id)
            };
            self.map[pos.0 as usize][pos.1 as usize] = tile;
        }
    }

    pub fn create_anti_room<T>(self: &mut TileMap, room: &T) where T: Shape {
        for pos in room.into_iter() {
            if let Some(id) = self.map[pos.0 as usize][pos.1 as usize].room {
                let tile = if room.is_boundary(pos) {
                    Tile::wall(id)
                } else {
                    Tile::bedrock()
                };
                self.map[pos.0 as usize][pos.1 as usize] = tile;
            }
        }
    }

    pub fn create_corridor(self: &mut TileMap, corridor: &Rect) {
        let id = self.rooms;
        self.rooms += 1;
        for pos in corridor.into_iter() {
            let is_wall = corridor.is_boundary(pos);

            let tile;
            if let Some(old_id) = self.map[pos.0 as usize][pos.1 as usize].room {
                let was_wall = self.map[pos.0 as usize][pos.1 as usize].wall;
                tile = if was_wall && is_wall {
                    Tile::wall(old_id)
                } else {
                    Tile::floor(old_id)
                };
            } else {
                tile = if is_wall {
                    Tile::wall(id)
                } else {
                    Tile::floor(id)
                };
            }
            self.map[pos.0 as usize][pos.1 as usize] = tile;
        }
    }

    pub fn draw_line(self: &mut TileMap, line: &Line) {
        for pos in line.into_iter() {
            if let Some(id) = self.map[pos.0 as usize][pos.1 as usize].room {
                self.map[pos.0 as usize][pos.1 as usize] = Tile::wall(id);
            }
        }
    }

    fn get(self: &TileMap, p: (i32, i32)) -> Option<&Tile> {
        if p.0 < 0 || p.0 >= self.width || p.1 < 0 || p.1 >= self.height {
            None
        } else {
            Some(&self.map[p.0 as usize][p.1 as usize])
        }
    }

    pub fn is_discovered(self: &TileMap, p: (i32, i32)) -> bool {
        match self.get(p) {
            Some(t) => t.discovered,
            None => true,
        }
    }

    pub fn is_blocking(self: &TileMap, p: (i32, i32)) -> bool {
        match self.get(p) {
            Some(t) => t.blocking,
            None => true,
        }
    }

    pub fn is_sight_blocking(self: &TileMap, p: (i32, i32)) -> bool {
        match self.get(p) {
            Some(t) => t.blocking,
            None => true,
        }
    }

    pub fn is_wall(self: &TileMap, p: (i32, i32)) -> bool {
        match self.get(p) {
            Some(t) => t.wall,
            None => false,
        }
    }

    fn create_dbox_character(self: &TileMap, n: bool, e: bool, s: bool, w: bool) -> char {
        match (n, e, s, w) {
            (true, true, false, false) => chars::DSW,
            (true, false, false, true) => chars::DSE,
            (false, true, true, false) => chars::DNW,
            (false, false, true, true) => chars::DNE,

            (true, false, true, false)
                | (true, false, false, false)
                | (false, false, true, false) => chars::DVLINE,
            (false, true, false, true)
                | (false, false, false, true)
                | (false, true, false, false) => chars::DHLINE,

            (true, false, true, true) => chars::DTEEW,
            (true, true, true, false) => chars::DTEEE,
            (true, true, false, true) => chars::DTEEN,
            (false, true, true, true) => chars::DTEES,

            (true, true, true, true) => chars::DCROSS,

            (false, false, false, false) => chars::BLOCK3,
        }
    }

    fn create_box_character(self: &TileMap, n: bool, e: bool, s: bool, w: bool) -> char {
        match (n, e, s, w) {
            (true, true, false, false) => chars::SW,
            (true, false, false, true) => chars::SE,
            (false, true, true, false) => chars::NW,
            (false, false, true, true) => chars::NE,

            (true, false, true, false)
                | (true, false, false, false)
                | (false, false, true, false) => chars::VLINE,
            (false, true, false, true)
                | (false, false, false, true)
                | (false, true, false, false) => chars::HLINE,

            (true, false, true, true) => chars::TEEW,
            (true, true, true, false) => chars::TEEE,
            (true, true, false, true) => chars::TEEN,
            (false, true, true, true) => chars::TEES,

            (true, true, true, true) => chars::CROSS,

            (false, false, false, false) => chars::BLOCK3,
        }
    }
}


