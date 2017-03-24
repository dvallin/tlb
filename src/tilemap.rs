use tcod::colors::{ Color };
use tcod::console::{ Console, BackgroundFlag };

const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;

#[derive(Clone, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
    explored: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            explored: false,
            blocked: false,
            block_sight: false,
        }
    }

    pub fn wall() -> Self {
        Tile {
            explored: false,
            blocked: true,
            block_sight: true,
        }
    }
}

pub struct TileMap {
    map: Vec<Vec<Tile>>,
    width: i32,
    height: i32,
}

impl TileMap {
    pub fn new() -> Self {
        let map = vec![vec![Tile::wall(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];

        TileMap {
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            map: map,
        }
    }

    pub fn draw(&self, console: &mut Console) {
        for y in 0..self.height {
            for x in 0..self.width {
                let color = COLOR_DARK_WALL;
                console.set_char_background(x, y, color, BackgroundFlag::Set);
            }
        }
    }
}
