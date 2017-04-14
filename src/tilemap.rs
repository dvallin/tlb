use tcod::console::{ Console, BackgroundFlag };

use tcod::colors::{ Color };
use tcod::chars::{ self };

use geometry::{ Shape, Rect };

const COLOR_DARK_BEDROCK: Color = Color { r: 0, g: 0, b: 50 };
const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_FLOOR: Color = Color { r: 50, g: 50, b: 150 };
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;

#[derive(Clone, Debug)]
struct Tile {
    color: Color,
    character: Option<char>,
    blocking: bool,
    wall: bool,
}

impl Tile {
    pub fn create(color: Color, character: Option<char>, blocking: bool, wall: bool) -> Self {
        Tile { color: color, character: character, blocking: blocking, wall: wall }
    }

    pub fn bedrock() -> Self {
        Tile::create(COLOR_DARK_BEDROCK, None, true, false)
    }

    pub fn wall() -> Self {
        Tile::create(COLOR_DARK_WALL, None, true, true)
    }

    pub fn floor() -> Self {
        Tile::create(COLOR_DARK_FLOOR, None, false, false)
    }

    pub fn setCharacter(&mut self, character: char) {
        self.character = Some(character);
    }
}

pub struct TileMap {
    map: Vec<Vec<Tile>>,
    width: i32,
    height: i32,
}

impl TileMap {
    pub fn new() -> Self {
        let map = vec![vec![Tile::bedrock(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
        TileMap {
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            map: map,
        }
    }

    pub fn build(&mut self) {
        self.create_room(&Rect::new(10, 10, 15, 15));
        self.create_room(&Rect::new(20, 20, 15, 15));
        self.create_room(&Rect::new(5, 5, 15, 15));
        self.create_room(&Rect::new(1, 1, 0, 0));
        self.create_room(&Rect::new(3, 1, 1, 1));
        self.create_characters();
    }

    pub fn draw(&self, console: &mut Console) {
        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.map[x as usize][y as usize].color;
                console.set_char_background(x, y, color, BackgroundFlag::Set);

                if let Some(character) = self.map[x as usize][y as usize].character {
                    console.put_char(x, y, character, BackgroundFlag::None);
                }
            }
        }
    }

    fn create_room<T>(self: &mut TileMap, room: &T) where T: Shape {
        for pos in room.into_iter() {
            let tile = if room.is_boundary(pos) {
                Tile::wall()
            } else {
                Tile::floor()
            };
            self.map[pos.x as usize][pos.y as usize] = tile;
        }
    }

    fn get(self: &TileMap, x: i32, y: i32) -> Option<&Tile> {
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            None
        } else {
            Some(&self.map[x as usize][y as usize])
        }
    }

    fn is_blocking(self: &TileMap, x: i32, y: i32) -> bool {
        match self.get(x, y) {
            Some(t) => t.blocking,
            None => true,
        }
    }

    fn is_wall(self: &TileMap, x: i32, y: i32) -> bool {
        match self.get(x, y) {
            Some(t) => t.wall,
            None => false,
        }
    }

    fn create_wall_character(self: &TileMap, x: i32, y: i32) -> char {
        let n = self.is_wall(x, y - 1);
        let e = self.is_wall(x + 1, y);
        let s = self.is_wall(x, y + 1);
        let w = self.is_wall(x - 1, y);
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

            (false, false, false, false) => chars::CHECKBOX_UNSET,
        }
    }

    fn create_characters(self: &mut TileMap) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_wall(x, y) {
                    let c = self.create_wall_character(x, y);
                    self.map[x as usize][y as usize].character = Some(c);
                }
            }
        }
    }
}


