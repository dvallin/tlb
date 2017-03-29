use tcod::colors::{ Color };
use tcod::console::{ Console, BackgroundFlag };

use geometry::{ Shape, Rect };

const COLOR_DARK_BEDROCK: Color = Color { r: 0, g: 0, b: 50 };
const COLOR_DARK_WALL: Color = Color { r: 0, g: 0, b: 100 };
const COLOR_DARK_FLOOR: Color = Color { r: 50, g: 50, b: 150 };
const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 43;

#[derive(Clone, Debug)]
struct Tile(i32);

impl Tile {
    pub fn bedrock() -> Self {
        Tile(0)
    }

    pub fn wall() -> Self {
        Tile(1)
    }

    pub fn floor() -> Self {
        Tile(2)
    }

    pub fn color(&self) -> Color {
        match self.0 {
            1 => COLOR_DARK_WALL,
            2 => COLOR_DARK_FLOOR,
            _ => COLOR_DARK_BEDROCK,
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
        let map = vec![vec![Tile::bedrock(); MAP_HEIGHT as usize]; MAP_WIDTH as usize];
        TileMap {
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            map: map,
        }
    }

    pub fn build(&mut self) {
        self.create_room(&Rect::new(10, 10, 15, 15));
    }

    pub fn draw(&self, console: &mut Console) {
        for y in 0..self.height {
            for x in 0..self.width {
                let color = self.map[x as usize][y as usize].color();
                console.set_char_background(x, y, color, BackgroundFlag::Set);
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
}


